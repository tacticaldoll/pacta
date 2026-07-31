//! Executable architectural governance for the pacta workspace.

#![forbid(unsafe_code)]

use std::{
    env, fs,
    path::{Path, PathBuf},
    process::ExitCode,
};

use tianheng::prelude::*;

const CONTRACT_REASON: &str = "pacta-contract is the isolated core contract. It may depend only on serde and uuid, and never on another workspace crate or runtime framework.";
const EXECUTOR_REASON: &str = "pacta-executor owns the Pacta-native execution vocabulary. It may depend only on pacta-contract, never on drivers, adapters, backends, or external frameworks.";
const DRIVER_REASON: &str = "pacta-driver is mechanical runtime glue. It may depend only on pacta-contract and pacta-executor, never on adapters, backends, or external frameworks.";
const GOVERNANCE_REASON: &str = "the governance gate must stay independent of the graph it judges: depend only on tianheng, never on a workspace crate.";
const KERNEL_ASYNC_REASON: &str = "the sans-I/O lifecycle kernel must stay runtime-agnostic: its public API must never expose an async fn, so no runtime shape leaks into the contract.";
const PROSE_REASON: &str =
    "active prose must not reintroduce stale architecture-defining vocabulary";

const ACTIVE_PROSE_FILES: &[&str] = &[
    "AGENTS.md",
    "PROJECT.md",
    "README.md",
    "BACKLOG.md",
    "docs/blueprint.md",
    "docs/development-flow.md",
    "docs/domain-language.md",
];

const STALE_PHRASES: &[StalePhrase] = &[
    StalePhrase {
        phrase: "Tower-native",
        reason: "Pacta core is Pacta-native; framework vocabulary is adapter scope.",
    },
    StalePhrase {
        phrase: "Tower-first",
        reason: "Pacta's product identity must not be framework-first.",
    },
    StalePhrase {
        phrase: "middleware ecosystem",
        reason: "Pacta grows through governed Pacta-native patterns, not ecosystem drift.",
    },
    StalePhrase {
        phrase: "Zero-Dependency",
        reason: "dependency rules live in executable Tianheng boundaries, not slogans.",
    },
    StalePhrase {
        phrase: "Store is Lifecycle",
        reason: "the current public lifecycle role is Registry.",
    },
    StalePhrase {
        phrase: "Store trait",
        reason: "the current public lifecycle contract is Registry.",
    },
    StalePhrase {
        phrase: "`Store` manages",
        reason: "the current public lifecycle role is Registry.",
    },
    StalePhrase {
        phrase: "`reserve`",
        reason: "the current lifecycle acquisition term is claim.",
    },
    StalePhrase {
        phrase: "`ack`",
        reason: "the current successful settlement term is fulfill.",
    },
    StalePhrase {
        phrase: "`nack`",
        reason: "the current failed settlement term is breach.",
    },
];

#[derive(Debug, Clone, Copy)]
struct StalePhrase {
    phrase: &'static str,
    reason: &'static str,
}

#[derive(Debug, PartialEq, Eq)]
struct ProseViolation {
    path: String,
    line: usize,
    phrase: &'static str,
    reason: &'static str,
}

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
        .async_exposure_boundary(
            AsyncExposureBoundary::in_crate("pacta-contract")
                .module("crate::kernel")
                .must_not_expose_async_fn()
                .because(KERNEL_ASYNC_REASON),
        )
}

fn main() -> ExitCode {
    let args = env::args().collect::<Vec<_>>();

    if should_check_prose(&args) {
        let manifest = manifest_path_from_args(&args);
        let root = manifest
            .parent()
            .map(Path::to_path_buf)
            .unwrap_or_else(|| PathBuf::from("."));

        if let Err(violations) = check_active_prose(&root) {
            eprintln!("pacta prose governance failed: {PROSE_REASON}");
            for violation in violations {
                eprintln!(
                    "{}:{}: `{}` - {}",
                    violation.path, violation.line, violation.phrase, violation.reason
                );
            }
            return ExitCode::from(1);
        }
    }

    tianheng::run(&constitution(), args)
}

fn should_check_prose(args: &[String]) -> bool {
    args.iter().skip(1).any(|arg| arg == "check")
}

fn manifest_path_from_args(args: &[String]) -> PathBuf {
    for index in 0..args.len() {
        if args[index] == "--manifest-path"
            && let Some(path) = args.get(index + 1)
        {
            return PathBuf::from(path);
        }

        if let Some(path) = args[index].strip_prefix("--manifest-path=") {
            return PathBuf::from(path);
        }
    }

    PathBuf::from("Cargo.toml")
}

fn check_active_prose(root: &Path) -> Result<(), Vec<ProseViolation>> {
    let mut violations = Vec::new();

    for relative in ACTIVE_PROSE_FILES {
        let path = root.join(relative);
        let Ok(content) = fs::read_to_string(&path) else {
            continue;
        };

        violations.extend(check_prose_content(relative, &content));
    }

    if violations.is_empty() {
        Ok(())
    } else {
        Err(violations)
    }
}

fn check_prose_content(path: &str, content: &str) -> Vec<ProseViolation> {
    let mut violations = Vec::new();
    let mut legacy_mapping = false;

    for (index, line) in content.lines().enumerate() {
        if path == "docs/domain-language.md" && line.trim() == "## Legacy Mapping" {
            legacy_mapping = true;
        }

        if legacy_mapping {
            continue;
        }

        for rule in STALE_PHRASES {
            if line.contains(rule.phrase) {
                violations.push(ProseViolation {
                    path: path.to_owned(),
                    line: index + 1,
                    phrase: rule.phrase,
                    reason: rule.reason,
                });
            }
        }
    }

    violations
}

#[cfg(test)]
mod tests {
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

    #[test]
    fn current_active_prose_satisfies_governance() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");

        assert_eq!(check_active_prose(&root), Ok(()));
    }

    #[test]
    fn stale_prose_vocabulary_is_rejected() {
        let violations = check_prose_content(
            "AGENTS.md",
            "Pacta is Tower-native and the `Store` manages `reserve`.\n",
        );

        assert_eq!(
            violations,
            vec![
                ProseViolation {
                    path: "AGENTS.md".to_owned(),
                    line: 1,
                    phrase: "Tower-native",
                    reason: "Pacta core is Pacta-native; framework vocabulary is adapter scope.",
                },
                ProseViolation {
                    path: "AGENTS.md".to_owned(),
                    line: 1,
                    phrase: "`Store` manages",
                    reason: "the current public lifecycle role is Registry.",
                },
                ProseViolation {
                    path: "AGENTS.md".to_owned(),
                    line: 1,
                    phrase: "`reserve`",
                    reason: "the current lifecycle acquisition term is claim.",
                },
            ]
        );
    }

    #[test]
    fn domain_language_legacy_mapping_is_allowed() {
        let violations = check_prose_content(
            "docs/domain-language.md",
            "## Legacy Mapping\n| `Store` | `Registry` |\n| `ack` | `fulfill` |\n",
        );

        assert!(violations.is_empty());
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
