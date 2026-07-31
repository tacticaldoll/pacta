//! Executable architectural governance for the pacta workspace.

#![forbid(unsafe_code)]

use std::process::ExitCode;
use tianheng::prelude::*;

const CONTRACT_REASON: &str = "pacta-contract is the isolated core contract. It may depend only on serde and uuid, and never on another workspace crate or runtime framework.";
const EXECUTOR_REASON: &str = "pacta-executor owns the Pacta-native execution vocabulary. It may depend only on pacta-contract, never on drivers, adapters, backends, or external frameworks.";
const DRIVER_REASON: &str = "pacta-driver is mechanical runtime glue. It may depend only on pacta-contract and pacta-executor, never on adapters, backends, or external frameworks.";
const GOVERNANCE_REASON: &str = "the governance gate must stay independent of the graph it judges: depend only on tianheng, never on a workspace crate.";

fn constitution() -> Constitution {
    Constitution::new("pacta")
        .boundary(
            CrateBoundary::crate_("pacta-contract")
                .restrict_dependencies_to(["serde", "uuid"])
                .because(CONTRACT_REASON),
        )
        .boundary(
            CrateBoundary::crate_("pacta-executor")
                .restrict_dependencies_to(["pacta-contract"])
                .because(EXECUTOR_REASON),
        )
        .boundary(
            CrateBoundary::crate_("pacta-driver")
                .restrict_dependencies_to(["pacta-contract", "pacta-executor"])
                .because(DRIVER_REASON),
        )
        .boundary(
            CrateBoundary::crate_("pacta-governance")
                .restrict_dependencies_to(["tianheng"])
                .because(GOVERNANCE_REASON),
        )
}

fn main() -> ExitCode {
    tianheng::run(&constitution(), std::env::args())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::{Path, PathBuf};

    use super::*;

    #[test]
    fn current_workspace_satisfies_constitution() {
        let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../Cargo.toml");

        assert_eq!(
            check(constitution().static_boundaries(), &manifest),
            Outcome::Clean
        );
    }

    #[test]
    fn unapproved_core_dependency_is_rejected() {
        let workspace = TempWorkspace::new("pacta-governance-forbidden-dependency");
        workspace.write_package("tower", "");
        workspace.write_package(
            "pacta-contract",
            r#"
[dependencies]
tower = { path = "../tower" }
"#,
        );
        workspace.write_package(
            "pacta-executor",
            r#"
[dependencies]
pacta-contract = { path = "../pacta-contract" }
"#,
        );
        workspace.write_package(
            "pacta-driver",
            r#"
[dependencies]
pacta-contract = { path = "../pacta-contract" }
pacta-executor = { path = "../pacta-executor" }
"#,
        );
        workspace.write_package("pacta-governance", "");
        workspace.write_root_manifest();

        let outcome = check(
            constitution().static_boundaries(),
            &workspace.path.join("Cargo.toml"),
        );

        let Outcome::Violations(report) = outcome else {
            panic!("expected an unapproved dependency violation, got {outcome:?}");
        };
        assert!(report.violations.iter().any(|violation| {
            let id = violation.id();
            id.target == "pacta-contract"
                && id.rule == "restrict dependencies to"
                && id.finding == "tower"
        }));
    }

    struct TempWorkspace {
        path: PathBuf,
    }

    impl TempWorkspace {
        fn new(name: &str) -> Self {
            let path = std::env::temp_dir().join(format!("{name}-{}", std::process::id()));
            if path.exists() {
                fs::remove_dir_all(&path).expect("stale temporary workspace should be removable");
            }
            fs::create_dir_all(&path).expect("temporary workspace should be creatable");
            Self { path }
        }

        fn write_root_manifest(&self) {
            fs::write(
                self.path.join("Cargo.toml"),
                r#"
[workspace]
resolver = "2"
members = [
    "pacta-contract",
    "pacta-driver",
    "pacta-executor",
    "pacta-governance",
    "tower",
]
"#,
            )
            .expect("workspace manifest should be writable");
        }

        fn write_package(&self, name: &str, dependencies: &str) {
            let package = self.path.join(name);
            fs::create_dir_all(package.join("src")).expect("package source dir should be writable");
            fs::write(
                package.join("Cargo.toml"),
                format!(
                    r#"
[package]
name = "{name}"
version = "0.1.0"
edition = "2024"
{dependencies}
"#
                ),
            )
            .expect("package manifest should be writable");
            fs::write(package.join("src/lib.rs"), "").expect("package source should be writable");
        }
    }

    impl Drop for TempWorkspace {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }
}
