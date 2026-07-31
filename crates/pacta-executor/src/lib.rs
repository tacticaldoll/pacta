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
}
