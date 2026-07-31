//! Mechanical runtime loop for Pacta execution.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use pacta_contract::{Registry, Retainer};
use pacta_executor::{Execution, Executor, Outcome};

/// One mechanical driver step result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Step {
    /// No pact was available from the configured dockets.
    Idle,
    /// A claimed pact was fulfilled.
    Fulfilled,
    /// A claimed pact was breached.
    Breached,
}

/// Error returned by a driver step.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DriverError<RegistryError> {
    /// Registry operation failed.
    Registry(RegistryError),
}

/// Mechanical loop that claims pacts, executes them, and settles claims.
pub struct Driver<R, E> {
    registry: R,
    executor: E,
    dockets: Vec<String>,
}

impl<R, E> Driver<R, E> {
    /// Build a driver from a registry, an executor, and docket names.
    pub fn new(registry: R, executor: E, dockets: impl IntoIterator<Item = String>) -> Self {
        Self {
            registry,
            executor,
            dockets: dockets.into_iter().collect(),
        }
    }

    /// Borrow the registry used by this driver.
    #[must_use]
    pub fn registry(&self) -> &R {
        &self.registry
    }

    /// Borrow the executor used by this driver.
    #[must_use]
    pub fn executor(&self) -> &E {
        &self.executor
    }
}

impl<R, E> Driver<R, E>
where
    R: Registry,
    E: Executor,
{
    /// Perform one claim, execute, and settle step.
    pub fn step(&mut self) -> Result<Step, DriverError<R::Error>> {
        let dockets: Vec<&str> = self.dockets.iter().map(String::as_str).collect();
        let Some(claim) = self
            .registry
            .claim(&dockets)
            .map_err(DriverError::Registry)?
        else {
            return Ok(Step::Idle);
        };

        let retainer = claim.retainer;
        let outcome = self.executor.execute(Execution::new(claim.pact));

        let step = match outcome {
            Ok(Outcome::Fulfilled) => Step::Fulfilled,
            Ok(Outcome::Breached) | Err(_) => Step::Breached,
        };

        match step {
            Step::Fulfilled => self.fulfill(&retainer)?,
            Step::Breached => self.breach(&retainer)?,
            Step::Idle => {}
        }

        Ok(step)
    }

    fn fulfill(&self, retainer: &Retainer) -> Result<(), DriverError<R::Error>> {
        self.registry
            .fulfill(retainer)
            .map_err(DriverError::Registry)
    }

    fn breach(&self, retainer: &Retainer) -> Result<(), DriverError<R::Error>> {
        self.registry
            .breach(retainer)
            .map_err(DriverError::Registry)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use pacta_contract::{Claim, Pact};
    use uuid::Uuid;

    use super::*;

    #[derive(Default)]
    struct RegistryState {
        claim: Option<Claim>,
        claimed_dockets: Vec<Vec<String>>,
        fulfilled: usize,
        breached: usize,
    }

    #[derive(Default)]
    struct TestRegistry {
        state: Mutex<RegistryState>,
    }

    impl TestRegistry {
        fn with_claim(claim: Claim) -> Self {
            Self {
                state: Mutex::new(RegistryState {
                    claim: Some(claim),
                    ..RegistryState::default()
                }),
            }
        }
    }

    impl Registry for TestRegistry {
        type Error = ();

        fn claim(&self, dockets: &[&str]) -> Result<Option<Claim>, Self::Error> {
            self.state
                .lock()
                .expect("registry state should not be poisoned")
                .claimed_dockets
                .push(dockets.iter().map(ToString::to_string).collect());
            Ok(self
                .state
                .lock()
                .expect("registry state should not be poisoned")
                .claim
                .take())
        }

        fn heartbeat(&self, _retainer: &Retainer) -> Result<(), Self::Error> {
            Ok(())
        }

        fn fulfill(&self, _retainer: &Retainer) -> Result<(), Self::Error> {
            self.state
                .lock()
                .expect("registry state should not be poisoned")
                .fulfilled += 1;
            Ok(())
        }

        fn breach(&self, _retainer: &Retainer) -> Result<(), Self::Error> {
            self.state
                .lock()
                .expect("registry state should not be poisoned")
                .breached += 1;
            Ok(())
        }
    }

    struct TestExecutor {
        outcome: Result<Outcome, ()>,
        executions: usize,
    }

    impl Executor for TestExecutor {
        type Error = ();

        fn execute(&mut self, _execution: Execution) -> Result<Outcome, Self::Error> {
            self.executions += 1;
            self.outcome
        }
    }

    fn claim() -> Claim {
        Claim {
            pact: Pact {
                id: Uuid::new_v4(),
                docket: "default".to_string(),
                kind: "example".to_string(),
                clause: Vec::new(),
            },
            retainer: Retainer(Uuid::new_v4()),
        }
    }

    #[test]
    fn successful_execution_fulfills_claim() {
        let registry = TestRegistry::with_claim(claim());
        let executor = TestExecutor {
            outcome: Ok(Outcome::Fulfilled),
            executions: 0,
        };
        let mut driver = Driver::new(registry, executor, ["default".to_string()]);

        assert_eq!(driver.step(), Ok(Step::Fulfilled));
        let state = driver
            .registry()
            .state
            .lock()
            .expect("registry state should not be poisoned");
        assert_eq!(state.fulfilled, 1);
        assert_eq!(state.breached, 0);
        drop(state);
        assert_eq!(driver.executor().executions, 1);
    }

    #[test]
    fn breached_execution_breaches_claim() {
        let registry = TestRegistry::with_claim(claim());
        let executor = TestExecutor {
            outcome: Ok(Outcome::Breached),
            executions: 0,
        };
        let mut driver = Driver::new(registry, executor, ["default".to_string()]);

        assert_eq!(driver.step(), Ok(Step::Breached));
        let state = driver
            .registry()
            .state
            .lock()
            .expect("registry state should not be poisoned");
        assert_eq!(state.fulfilled, 0);
        assert_eq!(state.breached, 1);
        drop(state);
        assert_eq!(driver.executor().executions, 1);
    }

    #[test]
    fn executor_error_breaches_claim() {
        let registry = TestRegistry::with_claim(claim());
        let executor = TestExecutor {
            outcome: Err(()),
            executions: 0,
        };
        let mut driver = Driver::new(registry, executor, ["default".to_string()]);

        assert_eq!(driver.step(), Ok(Step::Breached));
        let state = driver
            .registry()
            .state
            .lock()
            .expect("registry state should not be poisoned");
        assert_eq!(state.fulfilled, 0);
        assert_eq!(state.breached, 1);
        drop(state);
        assert_eq!(driver.executor().executions, 1);
    }

    #[test]
    fn empty_docket_is_idle() {
        let registry = TestRegistry::default();
        let executor = TestExecutor {
            outcome: Ok(Outcome::Fulfilled),
            executions: 0,
        };
        let mut driver = Driver::new(registry, executor, ["default".to_string()]);

        assert_eq!(driver.step(), Ok(Step::Idle));
        let state = driver
            .registry()
            .state
            .lock()
            .expect("registry state should not be poisoned");
        assert_eq!(state.fulfilled, 0);
        assert_eq!(state.breached, 0);
        drop(state);
        assert_eq!(driver.executor().executions, 0);
    }
}
