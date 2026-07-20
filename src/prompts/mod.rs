//! MCP prompts exposing guided, multi-step GitHub management workflows on
//! top of the `search`/`get`/`call` tools (see `router.rs`). Kept as its own
//! module, separate from `tools/`, per docs/mcp-prompts-workflow-plan.md.

pub mod router;

use rmcp::schemars;
use serde::Deserialize;

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct MasterWorkflowArgs {
    /// What the user is trying to accomplish, in their own words (optional — omit to show the full menu)
    pub goal: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct PullRequestWorkflowArgs {
    /// Owner (user or organization) of the repository the pull request targets
    pub owner: Option<String>,
    /// Name of the repository the pull request targets
    pub repo: Option<String>,
    /// Branch the change should land on (e.g. "main")
    pub base_branch: Option<String>,
    /// Existing branch (or fork branch) carrying the change, if one already exists
    pub head_branch: Option<String>,
}

/// Renders a short "Context already provided" header listing which of a
/// prompt's optional arguments the caller already supplied vs. still need to
/// be asked for. Prepended to each `content/*.md` body so the static
/// markdown never needs its own placeholder-substitution logic.
pub(crate) fn render_context_header(fields: &[(&str, Option<&str>)]) -> String {
    if fields.is_empty() {
        return String::new();
    }
    let mut out = String::from("## Context already provided\n");
    let mut any_known = false;
    for (name, value) in fields {
        if let Some(v) = value {
            out.push_str(&format!("- `{name}` = \"{v}\"\n"));
            any_known = true;
        }
    }
    if !any_known {
        out.push_str("- (none — no arguments were supplied with this prompt request)\n");
    }
    let missing: Vec<_> = fields
        .iter()
        .filter(|(_, v)| v.is_none())
        .map(|(n, _)| *n)
        .collect();
    if !missing.is_empty() {
        out.push_str(&format!(
            "\nStill unknown, ask the user before the step that needs it: {}\n",
            missing.join(", ")
        ));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_field_list_renders_nothing() {
        assert_eq!(render_context_header(&[]), "");
    }

    #[test]
    fn all_fields_supplied_lists_each_and_no_missing_section() {
        let header =
            render_context_header(&[("owner", Some("octocat")), ("repo", Some("hello-world"))]);
        assert!(header.contains("`owner` = \"octocat\""));
        assert!(header.contains("`repo` = \"hello-world\""));
        assert!(!header.contains("Still unknown"));
    }

    #[test]
    fn all_fields_missing_notes_none_supplied_and_lists_all_as_missing() {
        let header = render_context_header(&[("owner", None), ("repo", None)]);
        assert!(header.contains("(none — no arguments were supplied"));
        assert!(
            header
                .contains("Still unknown, ask the user before the step that needs it: owner, repo")
        );
    }

    #[test]
    fn mixed_fields_report_supplied_and_missing_separately() {
        let header = render_context_header(&[("owner", Some("octocat")), ("repo", None)]);
        assert!(header.contains("`owner` = \"octocat\""));
        assert!(!header.contains("`repo` ="));
        assert!(header.contains("Still unknown, ask the user before the step that needs it: repo"));
    }
}
