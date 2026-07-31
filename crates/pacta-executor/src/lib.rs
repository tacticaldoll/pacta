//! Pacta-native execution abstractions.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use pacta_contract::Pact;

pub use pacta_contract::{Outcome, Settlement};

/// A single attempt to fulfill a claimed pact.
///
/// This is the executor's designated input seam. It is `#[non_exhaustive]` so it can
/// gain execution-context fields in a later minor release without a breaking change;
/// construct it through [`Execution::new`].
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct Execution {
    /// Pact being executed.
    pub pact: Pact,
}

impl Execution {
    /// Build an execution from a claimed pact.
    #[must_use]
    pub fn new(pact: Pact) -> Self {
        Self { pact }
    }
}

/// Public role responsible for executing claimed pacts through middleware.
pub trait Executor {
    /// Error returned when the execution infrastructure fails.
    type Error: std::error::Error;

    /// Execute a claimed pact.
    fn execute(&mut self, execution: Execution) -> Result<Outcome, Self::Error>;
}

/// A Pacta-native decorator over execution: the Tower `Layer` analog. Because `wrap`
/// takes an `Executor` and returns an `Executor`, middleware compose arbitrarily
/// (the closure property), which is how orchestration is composed onto the seam.
pub trait Middleware<E> {
    /// The wrapped executor type.
    type Executor: Executor;

    /// Wrap an executor with this middleware.
    fn wrap(&self, executor: E) -> Self::Executor;
}

/// The no-op middleware: `wrap` returns the executor unchanged. `Identity` is the
/// neutral element of composition — the empty stack — so "zero middleware" is a
/// first-class, holdable value rather than an absence.
#[derive(Debug, Default, Clone, Copy)]
pub struct Identity;

impl<E: Executor> Middleware<E> for Identity {
    type Executor = E;

    fn wrap(&self, executor: E) -> Self::Executor {
        executor
    }
}

/// Two middleware composed into one, reifying the closure property as a value: because
/// `Stack` is itself a [`Middleware`], a composed stack can be named, stored, and passed
/// as one middleware *before* an executor exists. `outer` wraps the result of `inner`,
/// so `outer` is applied last and therefore observes each execution first.
#[derive(Debug, Default, Clone, Copy)]
pub struct Stack<Inner, Outer> {
    inner: Inner,
    outer: Outer,
}

impl<Inner, Outer> Stack<Inner, Outer> {
    /// Compose `inner` and `outer` into one middleware. `outer` wraps `inner`'s result,
    /// so `outer` observes each execution first.
    #[must_use]
    pub const fn new(inner: Inner, outer: Outer) -> Self {
        Self { inner, outer }
    }
}

impl<E, Inner, Outer> Middleware<E> for Stack<Inner, Outer>
where
    E: Executor,
    Inner: Middleware<E>,
    Outer: Middleware<Inner::Executor>,
{
    type Executor = Outer::Executor;

    fn wrap(&self, executor: E) -> Self::Executor {
        self.outer.wrap(self.inner.wrap(executor))
    }
}

/// A blind, ordered assembly of middleware composed over [`Identity`]. It is itself a
/// [`Middleware`], so applying it is just `wrap`.
///
/// It is deliberately *blind*: [`Composition::then`] accepts any `Middleware` through a
/// single generic operation and inspects nothing, and the type offers no method named for
/// an orchestration policy (retry, timeout, backoff, circuit, quota, rate-limit) — that
/// policy is a consumer or sibling concern, never a core convenience.
///
/// # Order
///
/// Middleware are applied outermost-first in the order added: the **first** middleware
/// added with [`then`](Composition::then) is the outermost and observes each execution
/// **first**; the executor is innermost. This mirrors the assembly convention of the
/// prior art the mechanism is distilled from.
#[derive(Debug, Default, Clone, Copy)]
pub struct Composition<M> {
    middleware: M,
}

impl Composition<Identity> {
    /// Start an empty composition over [`Identity`].
    #[must_use]
    pub const fn new() -> Self {
        Self {
            middleware: Identity,
        }
    }
}

impl<M> Composition<M> {
    /// Add a middleware to the composition. Blind: it accepts any `Middleware` and
    /// inspects nothing. The first middleware added is outermost (observes each
    /// execution first); the executor is innermost.
    #[must_use]
    pub fn then<N>(self, next: N) -> Composition<Stack<N, M>> {
        Composition {
            middleware: Stack::new(next, self.middleware),
        }
    }
}

impl<E, M> Middleware<E> for Composition<M>
where
    E: Executor,
    M: Middleware<E>,
{
    type Executor = M::Executor;

    fn wrap(&self, executor: E) -> Self::Executor {
        self.middleware.wrap(executor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct DummyError;

    impl std::fmt::Display for DummyError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "dummy error")
        }
    }

    impl std::error::Error for DummyError {}

    struct DummyExecutor;
    impl Executor for DummyExecutor {
        type Error = DummyError;
        fn execute(&mut self, _execution: Execution) -> Result<Outcome, Self::Error> {
            Ok(Outcome::Fulfilled)
        }
    }

    struct IdentityExecutor<E> {
        inner: E,
    }
    impl<E: Executor> Executor for IdentityExecutor<E> {
        type Error = E::Error;
        fn execute(&mut self, execution: Execution) -> Result<Outcome, Self::Error> {
            self.inner.execute(execution)
        }
    }

    struct IdentityMiddleware;
    impl<E: Executor> Middleware<E> for IdentityMiddleware {
        type Executor = IdentityExecutor<E>;
        fn wrap(&self, executor: E) -> Self::Executor {
            IdentityExecutor { inner: executor }
        }
    }

    struct BreachExecutor<E> {
        _inner: E,
    }
    impl<E: Executor> Executor for BreachExecutor<E> {
        type Error = E::Error;
        fn execute(&mut self, _execution: Execution) -> Result<Outcome, Self::Error> {
            Ok(Outcome::Breached)
        }
    }

    struct BreachMiddleware;
    impl<E: Executor> Middleware<E> for BreachMiddleware {
        type Executor = BreachExecutor<E>;
        fn wrap(&self, executor: E) -> Self::Executor {
            BreachExecutor { _inner: executor }
        }
    }

    fn dummy_execution() -> Execution {
        Execution::new(Pact::new(
            Default::default(),
            "dummy_docket".to_string(),
            "dummy_kind".to_string(),
            vec![],
        ))
    }

    #[test]
    fn identity_middleware_preserves_fulfilled() {
        let middleware = IdentityMiddleware;
        let mut executor = middleware.wrap(DummyExecutor);
        let outcome = executor.execute(dummy_execution()).unwrap();
        assert_eq!(outcome, Outcome::Fulfilled);
    }

    #[test]
    fn breach_middleware_alters_outcome() {
        let middleware = BreachMiddleware;
        let mut executor = middleware.wrap(DummyExecutor);
        let outcome = executor.execute(dummy_execution()).unwrap();
        assert_eq!(outcome, Outcome::Breached);
    }

    #[test]
    fn stacked_middleware_composes() {
        // The closure property: `Middleware` wraps `Executor` into `Executor`, so
        // two middleware stack and still yield a working executor. This proves the
        // "compose the rest" seam holds beyond a single wrap, and guards the
        // `Middleware<E>` generic shape from regressing so it can no longer stack.
        let inner = IdentityMiddleware.wrap(DummyExecutor);
        let mut stacked = IdentityMiddleware.wrap(inner);
        let outcome = stacked.execute(dummy_execution()).unwrap();
        assert_eq!(outcome, Outcome::Fulfilled);
    }

    #[test]
    fn stacked_middleware_preserves_ordering() {
        // A breach layer wrapping an identity layer over a fulfilling executor still
        // composes to a working executor whose outcome is the outermost layer's.
        let inner = IdentityMiddleware.wrap(DummyExecutor);
        let mut stacked = BreachMiddleware.wrap(inner);
        let outcome = stacked.execute(dummy_execution()).unwrap();
        assert_eq!(outcome, Outcome::Breached);
    }

    #[test]
    fn identity_wrap_returns_the_executor_unchanged() {
        // Identity is the neutral element: wrapping adds nothing.
        let mut executor = Identity.wrap(DummyExecutor);
        assert_eq!(
            executor.execute(dummy_execution()).unwrap(),
            Outcome::Fulfilled
        );
    }

    #[test]
    fn stack_is_itself_a_middleware() {
        // Stack reifies the closure property as a value: a composed pair is one Middleware
        // that wraps an executor into an executor just like a single middleware does.
        let stack = Stack::new(IdentityMiddleware, BreachMiddleware);
        let mut executor = stack.wrap(DummyExecutor);
        // BreachMiddleware is `outer`, so it is applied last and observes execution first.
        assert_eq!(
            executor.execute(dummy_execution()).unwrap(),
            Outcome::Breached
        );
    }

    #[test]
    fn composition_assembles_and_drives_to_a_settlement() {
        // The blind assembler composes two pass-through middleware over Identity and
        // drives to a settlement — the reified mechanism proven to compose.
        let composed = Composition::new()
            .then(IdentityMiddleware)
            .then(IdentityMiddleware);
        let mut executor = composed.wrap(DummyExecutor);
        assert_eq!(
            executor.execute(dummy_execution()).unwrap(),
            Outcome::Fulfilled
        );
    }

    #[test]
    fn composition_orders_first_added_outermost() {
        use std::cell::RefCell;
        use std::rc::Rc;

        // A middleware that records its label when its executor runs, so we can observe
        // the runtime order and assert it matches the documented convention.
        struct RecordingExecutor<E> {
            inner: E,
            label: &'static str,
            log: Rc<RefCell<Vec<&'static str>>>,
        }
        impl<E: Executor> Executor for RecordingExecutor<E> {
            type Error = E::Error;
            fn execute(&mut self, execution: Execution) -> Result<Outcome, Self::Error> {
                self.log.borrow_mut().push(self.label);
                self.inner.execute(execution)
            }
        }
        struct Recorder {
            label: &'static str,
            log: Rc<RefCell<Vec<&'static str>>>,
        }
        impl<E: Executor> Middleware<E> for Recorder {
            type Executor = RecordingExecutor<E>;
            fn wrap(&self, inner: E) -> Self::Executor {
                RecordingExecutor {
                    inner,
                    label: self.label,
                    log: Rc::clone(&self.log),
                }
            }
        }

        let log: Rc<RefCell<Vec<&'static str>>> = Rc::new(RefCell::new(Vec::new()));
        let composed = Composition::new()
            .then(Recorder {
                label: "first",
                log: Rc::clone(&log),
            })
            .then(Recorder {
                label: "second",
                log: Rc::clone(&log),
            });
        let mut executor = composed.wrap(DummyExecutor);
        executor.execute(dummy_execution()).unwrap();

        // First added is outermost and observes the execution first.
        assert_eq!(*log.borrow(), vec!["first", "second"]);
    }
}
