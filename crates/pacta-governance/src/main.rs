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
const GOVERNANCE_REASON: &str = "the governance gate must stay independent of the workspace graph it judges: it may depend only on governance-family tooling (tianheng and its guibiao coverage core), never on a workspace crate under judgment.";
const KERNEL_ASYNC_REASON: &str = "the sans-I/O step-driver kernel (crate::kernel) must stay runtime-agnostic: its public API must never expose an async fn, so no runtime shape leaks into the contract.";
const LIFECYCLE_ASYNC_REASON: &str = "the colorless lifecycle-state kernel (crate::lifecycle) is the single source both the sync and async Registry bindings compose over; it must never expose an async fn, so it stays colorless and the two bindings cannot drift by coloring the shared semantics.";
const KERNEL_NO_SERDE_REASON: &str = "the sans-I/O kernel is transient driving protocol, not durable state: it must not acquire Serialize/Deserialize, so persisting an in-flight directive or notice can never leak into the contract. Durable records (Pact, Claim, Retainer, Timestamp) carry serde; the kernel must not.";
const CORE_NO_IO_REASON: &str = "the sans-I/O core contract performs no I/O: no code in pacta-contract (the kernel included) may call into std::io/fs/net/process; I/O lives in runtimes and backends outside the core. Coverage is partial by nature (I/O entry points cannot be enumerated, and macro-expanded I/O such as println! is invisible to a source scan), so this tooth complements review rather than replacing it.";
const MEMORY_REASON: &str = "pacta-memory is a registry backend outside the core. It may depend only on pacta-contract and uuid, never on drivers, executors, or other backends.";
const CONTRACT_ASYNC_REASON: &str = "pacta-contract-async is the async binding of the frozen Registry contract. It may depend only on pacta-contract (the value types and the shared lifecycle kernel) and async-trait, never on a runtime, a backend, or another workspace crate — so the async surface stays isolated and sync-only consumers never pull it.";
const MEMORY_ASYNC_REASON: &str = "pacta-memory-async is the reference async registry backend. It may depend only on pacta-contract, pacta-contract-async, async-trait, and uuid, never on drivers, executors, or other backends.";
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
                .restrict_dependencies_to(["tianheng", "guibiao"])
                .because(GOVERNANCE_REASON),
        )
        .boundary(
            CrateBoundary::crate_("pacta-memory")
                .restrict_dependencies_to(["pacta-contract", "uuid"])
                .because(MEMORY_REASON),
        )
        .boundary(
            CrateBoundary::crate_("pacta-contract-async")
                .restrict_dependencies_to(["pacta-contract", "async-trait"])
                .because(CONTRACT_ASYNC_REASON),
        )
        .boundary(
            CrateBoundary::crate_("pacta-memory-async")
                .restrict_dependencies_to([
                    "pacta-contract",
                    "pacta-contract-async",
                    "async-trait",
                    "uuid",
                ])
                .because(MEMORY_ASYNC_REASON),
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
                .strict_external()
                .because(AMBIENT_TIME_UUID_REASON),
        )
        .boundary(
            ModuleBoundary::in_crate("pacta-contract")
                .module("crate")
                .must_not_call_inline("std::io")
                .because(CORE_NO_IO_REASON),
        )
        .boundary(
            ModuleBoundary::in_crate("pacta-contract")
                .module("crate")
                .must_not_call_inline("std::fs")
                .because(CORE_NO_IO_REASON),
        )
        .boundary(
            ModuleBoundary::in_crate("pacta-contract")
                .module("crate")
                .must_not_call_inline("std::net")
                .because(CORE_NO_IO_REASON),
        )
        .boundary(
            ModuleBoundary::in_crate("pacta-contract")
                .module("crate")
                .must_not_call_inline("std::process")
                .because(CORE_NO_IO_REASON),
        )
        .async_exposure_boundary(
            AsyncExposureBoundary::in_crate("pacta-contract")
                .module("crate::kernel")
                .must_not_expose_async_fn()
                .including_submodules()
                .because(KERNEL_ASYNC_REASON),
        )
        .async_exposure_boundary(
            AsyncExposureBoundary::in_crate("pacta-contract")
                .module("crate::lifecycle")
                .must_not_expose_async_fn()
                .including_submodules()
                .because(LIFECYCLE_ASYNC_REASON),
        )
        .forbidden_marker_boundary(
            ForbiddenMarkerBoundary::in_crate("pacta-contract")
                .module("crate::kernel")
                .must_not_acquire("Serialize")
                .and_not_acquire("Deserialize")
                .because(KERNEL_NO_SERDE_REASON),
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
            // A canonical governed file that cannot be read must fail the gate, not
            // be silently skipped — otherwise a governed doc that vanishes grants a
            // free pass. Fail loudly, naming the file.
            violations.push(ProseViolation {
                path: String::from(*relative),
                line: 0,
                phrase: "<unreadable>",
                reason: "a governed active-prose file must be present and readable",
            });
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
    let files = collect_rs_files(&root.join(FACADE_SOURCE_DIR));

    // No facade source found at all (missing or empty source tree) is a vacuous
    // pass — mirror the coverage check's non-vacuous guard and fail. Keyed on files
    // *found*, not files *read*, so a present-but-unreadable file reports as
    // unreadable below rather than as an empty tree.
    if files.is_empty() {
        violations.push(SourceViolation {
            path: FACADE_SOURCE_DIR.to_owned(),
            line: 0,
            marker: "no facade source files found",
        });
    }

    for file in files {
        let relative = file
            .strip_prefix(root)
            .unwrap_or(&file)
            .to_string_lossy()
            .into_owned();
        let Ok(content) = fs::read_to_string(&file) else {
            // An unreadable facade source file must fail the gate, not be skipped —
            // a file the scan cannot read cannot be certified re-exports-only.
            violations.push(SourceViolation {
                path: relative,
                line: 0,
                marker: "unreadable facade source",
            });
            continue;
        };
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
        workspace.write_package("pacta-contract-async", "");
        workspace.write_package("pacta-memory-async", "");
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
    fn missing_active_prose_file_fails_loudly() {
        // A root with none of the canonical governed prose files must fail the gate,
        // not pass vacuously by skipping every unreadable file.
        let workspace = TempWorkspace::new("pacta-governance-missing-prose");

        let Err(violations) = check_active_prose(&workspace.path) else {
            panic!("a root missing every governed prose file must fail the gate");
        };
        assert!(
            violations
                .iter()
                .any(|violation| violation.phrase == "<unreadable>"),
            "expected an unreadable-file violation naming a governed file: {violations:?}"
        );
    }

    #[test]
    fn empty_facade_source_tree_fails_loudly() {
        // A root with no facade source tree scans zero files; the non-vacuous guard
        // must convert that into a failure rather than an empty (clean) pass.
        let workspace = TempWorkspace::new("pacta-governance-empty-facade");

        let Err(violations) = check_facade_reexports_only(&workspace.path) else {
            panic!("a root with no facade source must fail the gate");
        };
        assert!(
            violations
                .iter()
                .any(|violation| violation.marker == "no facade source files found"),
            "expected a no-facade-source violation: {violations:?}"
        );
    }

    #[test]
    fn unreadable_facade_source_file_fails_loudly() {
        // A present-but-unreadable facade source file must fail the gate, not be
        // skipped. Invalid UTF-8 makes `read_to_string` fail portably (no reliance
        // on filesystem permissions, which root would bypass), while the file is
        // still found by the scan — so this exercises the unreadable path, distinct
        // from the empty-tree path.
        let workspace = TempWorkspace::new("pacta-governance-unreadable-facade");
        let src = workspace.path.join(FACADE_SOURCE_DIR);
        fs::create_dir_all(&src).expect("facade source dir should be creatable");
        fs::write(src.join("lib.rs"), [0xFF, 0xFE, 0x00])
            .expect("invalid-utf8 facade source should be writable");

        let Err(violations) = check_facade_reexports_only(&workspace.path) else {
            panic!("a present-but-unreadable facade source file must fail the gate");
        };
        assert!(
            violations
                .iter()
                .any(|violation| violation.marker == "unreadable facade source"),
            "expected an unreadable-facade-source violation: {violations:?}"
        );
        assert!(
            !violations
                .iter()
                .any(|violation| violation.marker == "no facade source files found"),
            "a found-but-unreadable file must not report as an empty tree: {violations:?}"
        );
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
    fn every_workspace_crate_is_covered() {
        // Tianheng coverage is advisory and never fails CI, so assert completeness
        // here through the native projection. `check_and_cover` takes the static
        // (guibiao) constitution and reads real `cargo metadata`.
        let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../Cargo.toml");
        let (_outcome, coverage) =
            guibiao::check_and_cover(constitution().static_boundaries(), &manifest);
        let coverage = coverage.expect("workspace metadata should be readable in-repo");
        assert!(
            coverage.total > 0,
            "coverage read no crates — the gate would pass vacuously"
        );
        assert!(
            coverage.uncovered.is_empty(),
            "every workspace crate must have a dependency boundary; ungoverned: {:?}",
            coverage.uncovered
        );
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

    /// Build a minimal two-crate workspace (`pacta-contract` + `pacta`, the two
    /// crates the semantic boundaries target) and run the whole semantic bundle
    /// against it. Both crates are always present: a missing target crate or module
    /// makes `check_all` return `Outcome::ConstitutionError`, not a silent skip, so a
    /// firing fixture must differ from a clean one only in the leak it commits.
    fn semantic_reaction_outcome(
        name: &str,
        contract_source: &str,
        facade_dependencies: &str,
        facade_source: &str,
    ) -> Outcome {
        let workspace = TempWorkspace::new(name);
        // Both async-exposure boundaries (crate::kernel and crate::lifecycle) require their
        // target module to exist, or tianheng refuses the boundary rather than silently never
        // reacting. A fixture that exercises one module must still present the other, so scaffold
        // whichever the source omits.
        let mut source = String::new();
        if !contract_source.contains("mod kernel") {
            source.push_str("pub mod kernel {}\n");
        }
        if !contract_source.contains("mod lifecycle") {
            source.push_str("pub mod lifecycle {}\n");
        }
        source.push_str(contract_source);
        workspace.write_package_with_source("pacta-contract", "", &source);
        workspace.write_package_with_source("pacta", facade_dependencies, facade_source);
        workspace.write_root_manifest_members(&["pacta", "pacta-contract"]);

        tianheng::check_all(
            constitution().semantic_boundaries(),
            &workspace.path.join("Cargo.toml"),
        )
    }

    #[test]
    fn kernel_async_exposure_reaction_fires() {
        let outcome = semantic_reaction_outcome(
            "pacta-governance-kernel-async-leak",
            "pub mod kernel {\n    pub async fn leak() {}\n}\n",
            "",
            "",
        );

        let Outcome::Violations(report) = outcome else {
            panic!("expected a kernel async-exposure violation, got {outcome:?}");
        };
        assert!(
            report.violations.iter().any(|violation| {
                let id = violation.id();
                id.target == "crate::kernel" && id.rule == "must not expose async fn"
            }),
            "expected the kernel async-exposure boundary to fire: {report:?}"
        );
    }

    #[test]
    fn lifecycle_async_exposure_reaction_fires() {
        let outcome = semantic_reaction_outcome(
            "pacta-governance-lifecycle-async-leak",
            "pub mod lifecycle {\n    pub async fn leak() {}\n}\n",
            "",
            "",
        );

        let Outcome::Violations(report) = outcome else {
            panic!("expected a lifecycle async-exposure violation, got {outcome:?}");
        };
        assert!(
            report.violations.iter().any(|violation| {
                let id = violation.id();
                id.target == "crate::lifecycle" && id.rule == "must not expose async fn"
            }),
            "expected the lifecycle async-exposure boundary to fire: {report:?}"
        );
    }

    #[test]
    fn facade_kernel_reexport_reaction_fires() {
        let outcome = semantic_reaction_outcome(
            "pacta-governance-facade-kernel-leak",
            "pub mod kernel {\n    pub struct Leak;\n}\n",
            "[dependencies]\npacta-contract = { path = \"../pacta-contract\" }\n",
            "pub use pacta_contract::kernel::Leak;\n",
        );

        let Outcome::Violations(report) = outcome else {
            panic!("expected a facade kernel-exclusion violation, got {outcome:?}");
        };
        assert!(
            report.violations.iter().any(|violation| {
                let id = violation.id();
                id.target == "crate" && id.rule == "must not expose"
            }),
            "expected the facade kernel-exclusion boundary to fire: {report:?}"
        );
    }

    #[test]
    fn kernel_serde_acquisition_reaction_fires() {
        let outcome = semantic_reaction_outcome(
            "pacta-governance-kernel-serde-leak",
            "pub mod kernel {\n    #[derive(Serialize)]\n    pub struct Leak;\n}\n",
            "",
            "",
        );

        let Outcome::Violations(report) = outcome else {
            panic!("expected a kernel forbidden-marker violation, got {outcome:?}");
        };
        assert!(
            report.violations.iter().any(|violation| {
                let id = violation.id();
                id.target == "crate::kernel" && id.rule == "must not acquire trait"
            }),
            "expected the kernel no-serde boundary to fire: {report:?}"
        );
    }

    #[test]
    fn semantic_reactions_stay_clean_without_a_leak() {
        // Precision: the same two-crate shape without either leak must be clean, so
        // the firing tests prove a reacting boundary, not one that always fires.
        let outcome = semantic_reaction_outcome(
            "pacta-governance-semantic-clean",
            "pub mod kernel {\n    pub struct Item;\n}\n",
            "",
            "",
        );

        assert_eq!(
            outcome,
            Outcome::Clean,
            "a workspace with no kernel leak must raise no semantic violation"
        );
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
            self.write_root_manifest_members(&[
                "pacta",
                "pacta-conformance",
                "pacta-contract",
                "pacta-contract-async",
                "pacta-driver",
                "pacta-executor",
                "pacta-governance",
                "pacta-memory",
                "pacta-memory-async",
                "tower",
            ]);
        }

        fn write_root_manifest_members(&self, members: &[&str]) {
            let entries = members
                .iter()
                .map(|member| format!("    \"{member}\","))
                .collect::<Vec<_>>()
                .join("\n");
            fs::write(
                self.path.join("Cargo.toml"),
                format!(
                    r#"
[workspace]
resolver = "2"
members = [
{entries}
]
"#
                ),
            )
            .expect("workspace manifest should be writable");
        }

        fn write_package(&self, name: &str, dependencies: &str) {
            self.write_package_with_source(name, dependencies, "");
        }

        fn write_package_with_source(&self, name: &str, dependencies: &str, source: &str) {
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
            fs::write(package.join("src/lib.rs"), source)
                .expect("package source should be writable");
        }
    }

    impl Drop for TempWorkspace {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }
}
