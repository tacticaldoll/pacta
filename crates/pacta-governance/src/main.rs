//! Executable architectural governance for the pacta workspace.

use std::process::ExitCode;
use tianheng::prelude::*;

const CONTRACT_REASON: &str = "pacta-contract is the zero-dependency core contract. It must never depend on any other workspace crate to ensure strict isolation of the data and state model.";
const GOVERNANCE_REASON: &str = "the governance gate must stay independent of the graph it judges: depend only on tianheng, never on a workspace crate.";

fn constitution() -> Constitution {
    Constitution::new("pacta")
        .boundary(
            CrateBoundary::crate_("pacta-contract")
                .forbid_all_workspace_dependencies()
                .because(CONTRACT_REASON),
        )
        .boundary(
            CrateBoundary::crate_("pacta-governance")
                .forbid_all_workspace_dependencies()
                .because(GOVERNANCE_REASON),
        )
}

fn main() -> ExitCode {
    tianheng::run(&constitution(), std::env::args())
}
