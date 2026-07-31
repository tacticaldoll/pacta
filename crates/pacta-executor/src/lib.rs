//! Pacta-native execution abstractions.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use pacta_contract::Pact;

pub use pacta_contract::{Outcome, Settlement};

/// A single attempt to fulfill a claimed pact.
#[derive(Debug, Clone)]
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
    type Error;

    /// Execute a claimed pact.
    fn execute(&mut self, execution: Execution) -> Result<Outcome, Self::Error>;
}

/// A Pacta-native decorator over execution.
pub trait Middleware<E> {
    /// The wrapped executor type.
    type Executor: Executor;

    /// Wrap an executor with this middleware.
    fn wrap(&self, executor: E) -> Self::Executor;
}

/// A minimal policy value that names orchestration intent without behavior.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Policy {
    name: &'static str,
}

impl Policy {
    /// Create a new named policy.
    #[must_use]
    pub const fn new(name: &'static str) -> Self {
        Self { name }
    }

    /// The name of the policy.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct DummyError;

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
        Execution::new(Pact {
            id: Default::default(),
            docket: "dummy_docket".to_string(),
            kind: "dummy_kind".to_string(),
            clause: vec![],
        })
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
    fn policy_is_inspectable() {
        let policy = Policy::new("retry");
        assert_eq!(policy.name(), "retry");
    }
}
