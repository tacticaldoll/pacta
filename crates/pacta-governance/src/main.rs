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
const MEMORY_REASON: &str = "pacta-memory is a registry backend outside the core. It may depend only on pacta-contract and uuid, never on drivers, executors, or other backends.";
const CONFORMANCE_REASON: &str = "pacta-conformance is a backend-agnostic test suite. It may depend only on pacta-contract and uuid, never on a specific backend.";
const PROSE_REASON: &str =
    "active prose must not reintroduce stale architecture-defining vocabulary";
const AMBIENT_TIME_REASON: &str =
    "the core contract must read no ambient clock; time is injected at the Registry seam";
const AMBIENT_TIME_UUID_REASON: &str =
    "the core contract must not mint time-based UUIDs; identifiers carry no ambient clock";
const FACADE_REASON: &str = "pacta is the curated published entrypoint. It may depend only on pacta-contract, pacta-executor, and pacta-driver, never on a backend or external framework.";
const FACADE_KERNEL_REASON: &str = "the pacta facade is the compose-level surface: it must not re-export the sans-I/O kernel, which stays advanced-only and is reached through pacta-contract directly.";
const FACADE_REEXPORT_REASON: &str =
    "the pacta facade must stay a pure re-export entrypoint and hold no logic of its own";
const FACADE_NON_REEXPORT: &str = "non-re-export item in facade library";

/// The facade source tree the re-exports-only scan guards, relative to the workspace root.
const FACADE_SOURCE_DIR: &str = "crates/pacta/src";

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
        .boundary(
            CrateBoundary::crate_("pacta-memory")
                .restrict_dependencies_to(["pacta-contract", "uuid"])
                .because(MEMORY_REASON),
        )
        .boundary(
            CrateBoundary::crate_("pacta-conformance")
                .restrict_dependencies_to(["pacta-contract", "uuid"])
                .because(CONFORMANCE_REASON),
        )
        .boundary(
            CrateBoundary::crate_("pacta")
                .restrict_dependencies_to(["pacta-contract", "pacta-executor", "pacta-driver"])
                .because(FACADE_REASON),
        )
        .boundary(
            ModuleBoundary::in_crate("pacta-contract")
                .module("crate")
                .must_not_call_inline("std::time")
                .ending_with(["now"])
                .because(AMBIENT_TIME_REASON),
        )
        .boundary(
            ModuleBoundary::in_crate("pacta-contract")
                .module("crate")
                .must_not_call_inline("uuid")
                .ending_with(["now_v7", "now_v1"])
                .because(AMBIENT_TIME_UUID_REASON),
        )
        .async_exposure_boundary(
            AsyncExposureBoundary::in_crate("pacta-contract")
                .module("crate::kernel")
                .must_not_expose_async_fn()
                .including_submodules()
                .because(KERNEL_ASYNC_REASON),
        )
        .signature_boundary(
            SemanticBoundary::in_crate("pacta")
                .module("crate")
                .must_not_expose("pacta_contract::kernel")
                .because(FACADE_KERNEL_REASON),
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

        if let Err(violations) = check_facade_reexports_only(&root) {
            eprintln!("pacta facade governance failed: {FACADE_REEXPORT_REASON}");
            for violation in violations {
                eprintln!(
                    "{}:{}: `{}`",
                    violation.path, violation.line, violation.marker
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

#[derive(Debug, PartialEq, Eq)]
struct SourceViolation {
    path: String,
    line: usize,
    marker: &'static str,
}

fn check_facade_reexports_only(root: &Path) -> Result<(), Vec<SourceViolation>> {
    let mut violations = Vec::new();

    for file in collect_rs_files(&root.join(FACADE_SOURCE_DIR)) {
        let Ok(content) = fs::read_to_string(&file) else {
            continue;
        };
        let relative = file
            .strip_prefix(root)
            .unwrap_or(&file)
            .to_string_lossy()
            .into_owned();
        violations.extend(check_facade_content(&relative, &content));
    }

    if violations.is_empty() {
        Ok(())
    } else {
        Err(violations)
    }
}

/// A brace-depth-aware line scan: at brace depth zero, the facade library may hold
/// only re-exports, `use` imports, attributes, and comments. Any other item
/// (a `fn`, `struct`, `impl`, `const`, ...) is logic the facade must not carry. It
/// is deliberately a line scan, not a parser: `pacta-governance` may depend only on
/// `tianheng`, so it cannot pull in `syn`. A logic item co-located on a `pub use`
/// line (`pub use X; pub const Y = 1;`) escapes this line heuristic, but the DoD
/// `cargo fmt --all --check` gate splits it onto its own line, where this scan then
/// catches it.
fn check_facade_content(path: &str, content: &str) -> Vec<SourceViolation> {
    let mut violations = Vec::new();
    let mut depth: i32 = 0;

    for (index, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        let is_comment = trimmed.starts_with("//");

        // A line inside a multi-line `pub use { ... }` block is a re-export
        // continuation; only judge lines that start a fresh item at depth zero.
        if depth == 0
            && !trimmed.is_empty()
            && !is_comment
            && !trimmed.starts_with('#')
            && !trimmed.starts_with("pub use ")
            && !trimmed.starts_with("use ")
        {
            violations.push(SourceViolation {
                path: path.to_owned(),
                line: index + 1,
                marker: FACADE_NON_REEXPORT,
            });
        }

        // Track brace depth off code lines only, so a brace inside a doc comment
        // does not desynchronize the scan.
        if !is_comment {
            depth += line.matches('{').count() as i32;
            depth -= line.matches('}').count() as i32;
            if depth < 0 {
                depth = 0;
            }
        }
    }

    violations
}

fn collect_rs_files(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();

    let Ok(entries) = fs::read_dir(dir) else {
        return files;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            files.extend(collect_rs_files(&path));
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            files.push(path);
        }
    }

    files
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
        workspace.write_package("pacta-memory", "");
        workspace.write_package("pacta-conformance", "");
        workspace.write_package(
            "pacta",
            r#"
[dependencies]
pacta-contract = { path = "../pacta-contract" }
pacta-executor = { path = "../pacta-executor" }
pacta-driver = { path = "../pacta-driver" }
"#,
        );
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
    fn current_facade_is_reexports_only() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");

        assert_eq!(check_facade_reexports_only(&root), Ok(()));
    }

    #[test]
    fn facade_reexports_and_comments_are_allowed() {
        let content = "\
//! Facade docs.
#![forbid(unsafe_code)]

pub use pacta_contract::{Claim, Pact};
pub use pacta_driver::{
    Driver,
    Step,
};
";
        assert!(check_facade_content("lib.rs", content).is_empty());
    }

    #[test]
    fn facade_logic_item_is_rejected() {
        assert_eq!(
            check_facade_content("lib.rs", "pub fn helper() {}\n"),
            vec![SourceViolation {
                path: "lib.rs".to_owned(),
                line: 1,
                marker: FACADE_NON_REEXPORT,
            }]
        );
        // A struct declaration inside the facade is logic, not a re-export.
        assert_eq!(
            check_facade_content("lib.rs", "struct Sneaky;\n"),
            vec![SourceViolation {
                path: "lib.rs".to_owned(),
                line: 1,
                marker: FACADE_NON_REEXPORT,
            }]
        );
    }

    #[test]
    fn every_workspace_crate_has_a_boundary() {
        // Tianheng coverage is advisory and never fails CI, so assert completeness here.
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let manifest =
            fs::read_to_string(root.join("Cargo.toml")).expect("root manifest should be readable");
        let members = workspace_members(&manifest);
        assert!(!members.is_empty(), "expected to parse workspace members");

        let governed: Vec<String> = constitution()
            .static_boundaries()
            .boundaries()
            .iter()
            .filter_map(|boundary| match boundary {
                tianheng::Boundary::Crate(crate_boundary) => {
                    Some(crate_boundary.target().package.clone())
                }
                _ => None,
            })
            .collect();

        let ungoverned: Vec<&String> = members
            .iter()
            .filter(|member| !governed.contains(member))
            .collect();
        assert!(
            ungoverned.is_empty(),
            "every workspace crate must have a dependency boundary; ungoverned: {ungoverned:?}"
        );
    }

    fn workspace_members(manifest: &str) -> Vec<String> {
        let start = manifest
            .find("members = [")
            .expect("root manifest should declare workspace members");
        let rest = &manifest[start..];
        let end = rest.find(']').expect("members array should be closed");

        rest[..end]
            .lines()
            .filter_map(|line| {
                let entry = line.trim().trim_end_matches(',').trim_matches('"');
                if entry.contains("crates/") {
                    entry.rsplit('/').next().map(str::to_owned)
                } else {
                    None
                }
            })
            .collect()
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
    "pacta",
    "pacta-conformance",
    "pacta-contract",
    "pacta-driver",
    "pacta-executor",
    "pacta-governance",
    "pacta-memory",
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
