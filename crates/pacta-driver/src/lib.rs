//! Mechanical runtime loop for Pacta execution.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use std::time::{SystemTime, UNIX_EPOCH};

use pacta_contract::kernel::{Directive, Kernel, Notice, StepResult};
use pacta_contract::{Outcome, Registry, Timestamp};
use pacta_executor::{Execution, Executor};

/// Read the current wall-clock time as a [`Timestamp`] to inject into
/// time-dependent registry operations. Reading the clock is a runtime concern, so
/// it lives here and never in the core contract.
fn current_time() -> Timestamp {
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|elapsed| u64::try_from(elapsed.as_millis()).unwrap_or(u64::MAX))
        .unwrap_or(0);
    Timestamp::from_millis(millis)
}

/// One mechanical driver step result.
///
/// `#[non_exhaustive]`: a runtime-loop status may gain states (for example a future
/// heartbeat or lapse step), so a downstream match must carry a wildcard arm.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Step {
    /// No pact was available from the configured dockets.
    Idle,
    /// A claimed pact was fulfilled.
    Fulfilled,
    /// A claimed pact was breached.
    Breached,
}

/// Error returned by a driver step.
///
/// `#[non_exhaustive]`: an error enumeration grows as new failure modes appear, so a
/// downstream match must carry a wildcard arm.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum DriverError<RegistryError, ExecutorError> {
    /// Registry operation failed.
    Registry(RegistryError),
    /// Executor infrastructure failed; the claim was left unsettled to lapse and be
    /// reclaimed (no settlement was recorded).
    Executor(ExecutorError),
}

impl<RegistryError, ExecutorError> std::fmt::Display for DriverError<RegistryError, ExecutorError>
where
    RegistryError: std::error::Error,
    ExecutorError: std::error::Error,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Registry(error) => write!(f, "registry operation failed: {error}"),
            Self::Executor(error) => write!(f, "executor infrastructure failed: {error}"),
        }
    }
}

impl<RegistryError, ExecutorError> std::error::Error for DriverError<RegistryError, ExecutorError>
where
    RegistryError: std::error::Error + 'static,
    ExecutorError: std::error::Error + 'static,
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Registry(error) => Some(error),
            Self::Executor(error) => Some(error),
        }
    }
}

/// Mechanical loop that performs the directives the sans-I/O kernel issues.
///
/// This is a **reference** runtime skeleton. It drives one step synchronously —
/// claim, execute, settle — and never heartbeats or reclaims within a step: it does
/// not extend a lease while its executor runs (so a long task's lease can *expire*
/// mid-step), and it settles by matching the retainer rather than re-claiming. It is
/// therefore safe for tasks shorter than the lease (the lease never expires mid-step)
/// and for single-worker use (no concurrent claimer can *reclaim* an expired lease
/// mid-step). A workload that is both long-running *and* multi-worker should compose
/// its own loop over the [`Registry`] contract (which includes `heartbeat`); the
/// lifecycle kernel deliberately models no in-flight heartbeat.
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
    /// Perform one claim, execute, and settle step by driving the kernel: the
    /// kernel decides each directive; the driver performs it and feeds a notice
    /// back, deciding no lifecycle outcome itself.
    pub fn step(&mut self) -> Result<Step, DriverError<R::Error, E::Error>> {
        let dockets: Vec<&str> = self.dockets.iter().map(String::as_str).collect();
        let now = current_time();
        let mut kernel = Kernel::new();
        let mut pending_executor_error: Option<E::Error> = None;

        loop {
            if let Some(result) = kernel.result() {
                return match result {
                    StepResult::Idle => Ok(Step::Idle),
                    StepResult::Settled(outcome) => Ok(match outcome {
                        Outcome::Fulfilled => Step::Fulfilled,
                        Outcome::Breached => Step::Breached,
                    }),
                    // An unsettled step means execution failed: settle nothing and
                    // surface the executor error. The claim lapses and is reclaimed.
                    StepResult::Unsettled => Err(DriverError::Executor(
                        pending_executor_error
                            .expect("an unsettled step implies a pending executor error"),
                    )),
                    _ => unreachable!("driver handles every current kernel step result"),
                };
            }

            match kernel.poll() {
                Directive::Claim => {
                    let claim = self
                        .registry
                        .claim(&dockets, now)
                        .map_err(DriverError::Registry)?;
                    kernel.on_event(Notice::Claimed(claim));
                }
                Directive::Execute(pact) => match self.executor.execute(Execution::new(pact)) {
                    Ok(outcome) => kernel.on_event(Notice::Executed(outcome)),
                    Err(error) => {
                        pending_executor_error = Some(error);
                        kernel.on_event(Notice::ExecutionFailed);
                    }
                },
                Directive::Settle(retainer, outcome) => {
                    match outcome {
                        Outcome::Fulfilled => self
                            .registry
                            .fulfill(&retainer)
                            .map_err(DriverError::Registry)?,
                        Outcome::Breached => self
                            .registry
                            .breach(&retainer)
                            .map_err(DriverError::Registry)?,
                    }
                    kernel.on_event(Notice::Settled);
                }
                Directive::Idle => return Ok(Step::Idle),
                _ => unreachable!("driver handles every current kernel directive"),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use pacta_contract::{Claim, Pact, Retainer, Timestamp};
    use uuid::Uuid;

    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct TestError;

    impl std::fmt::Display for TestError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "test error")
        }
    }

    impl std::error::Error for TestError {}

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
        type Error = TestError;

        fn claim(&self, dockets: &[&str], _now: Timestamp) -> Result<Option<Claim>, Self::Error> {
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

        fn heartbeat(&self, _retainer: &Retainer, _now: Timestamp) -> Result<(), Self::Error> {
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
        outcome: Result<Outcome, TestError>,
        executions: usize,
    }

    impl Executor for TestExecutor {
        type Error = TestError;

        fn execute(&mut self, _execution: Execution) -> Result<Outcome, Self::Error> {
            self.executions += 1;
            self.outcome
        }
    }

    fn claim() -> Claim {
        Claim::new(
            Pact::new(
                Uuid::new_v4(),
                "default".to_string(),
                "example".to_string(),
                Vec::new(),
            ),
            Retainer::new(Uuid::new_v4()),
            Timestamp::from_millis(0),
        )
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
    fn executor_error_leaves_claim_unsettled() {
        let registry = TestRegistry::with_claim(claim());
        let executor = TestExecutor {
            outcome: Err(TestError),
            executions: 0,
        };
        let mut driver = Driver::new(registry, executor, ["default".to_string()]);

        // An infrastructure failure surfaces the executor error and settles nothing:
        // neither fulfilled nor breached. The claim is left unsettled to lapse and be
        // reclaimed — that lapse-reclaim is proven at the registry level by
        // `pacta-conformance` (`expired_lease_lapses_and_reclaims_...`).
        assert_eq!(driver.step(), Err(DriverError::Executor(TestError)));
        let state = driver
            .registry()
            .state
            .lock()
            .expect("registry state should not be poisoned");
        assert_eq!(state.fulfilled, 0);
        assert_eq!(state.breached, 0);
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

    #[test]
    fn driver_error_displays_and_exposes_source() {
        use std::error::Error;

        let executor_error: DriverError<TestError, TestError> = DriverError::Executor(TestError);
        assert_eq!(
            executor_error.to_string(),
            "executor infrastructure failed: test error"
        );
        assert_eq!(
            executor_error
                .source()
                .expect("driver error should expose its source")
                .to_string(),
            "test error"
        );

        let registry_error: DriverError<TestError, TestError> = DriverError::Registry(TestError);
        assert_eq!(
            registry_error.to_string(),
            "registry operation failed: test error"
        );
        assert_eq!(
            registry_error
                .source()
                .expect("driver error should expose its source")
                .to_string(),
            "test error"
        );
    }
}
