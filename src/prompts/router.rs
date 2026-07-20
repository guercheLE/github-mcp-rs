//! One method per MCP prompt. See docs/mcp-prompts-workflow-plan.md for the
//! design rationale (agnostic phrasing, whole-sub-workflow delegation,
//! content-size targets) that every `content/*.md` file must follow.

use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{PromptMessage, Role};
use rmcp::{prompt, prompt_router};

use crate::core::mcp_server::McpifyServer;
use crate::prompts::{MasterWorkflowArgs, PullRequestWorkflowArgs, render_context_header};

#[prompt_router(vis = "pub(crate)")]
impl McpifyServer {
    #[prompt(
        name = "github_workflow",
        description = "Start here. Presents the available GitHub management workflows, \
                        routes to the right guided sub-workflow based on the user's goal, \
                        and -- where the environment supports it -- delegates that whole \
                        sub-workflow to an isolated sub-task to spare this conversation's \
                        context window."
    )]
    async fn github_workflow_prompt(
        &self,
        Parameters(args): Parameters<MasterWorkflowArgs>,
    ) -> Vec<PromptMessage> {
        let header = render_context_header(&[("goal", args.goal.as_deref())]);
        vec![PromptMessage::new_text(
            Role::User,
            format!("{header}\n\n{}", include_str!("content/master.md")),
        )]
    }

    #[prompt(
        name = "github_workflow_pull_request",
        description = "Guided, multi-step pull request flow: the fork-vs-direct-branch \
                        decision, branch/commit/push, opening the PR, reviewers, and \
                        verifying checks/reviews before declaring it ready to merge."
    )]
    async fn github_workflow_pull_request_prompt(
        &self,
        Parameters(args): Parameters<PullRequestWorkflowArgs>,
    ) -> Vec<PromptMessage> {
        let header = render_context_header(&[
            ("owner", args.owner.as_deref()),
            ("repo", args.repo.as_deref()),
            ("base_branch", args.base_branch.as_deref()),
            ("head_branch", args.head_branch.as_deref()),
        ]);
        vec![PromptMessage::new_text(
            Role::User,
            format!("{header}\n\n{}", include_str!("content/pull_request.md")),
        )]
    }

    #[prompt(
        name = "github_workflow_repos",
        description = "Repository lifecycle (create, fork, transfer, archive, delete), \
                        branches and branch protection, tags, commits/git data, releases, \
                        topics/settings, webhooks."
    )]
    async fn github_workflow_repos_prompt(&self) -> Vec<PromptMessage> {
        vec![PromptMessage::new_text(
            Role::User,
            include_str!("content/repos.md"),
        )]
    }

    #[prompt(
        name = "github_workflow_issues",
        description = "Issue lifecycle, labels, milestones, assignees, comments, reactions."
    )]
    async fn github_workflow_issues_prompt(&self) -> Vec<PromptMessage> {
        vec![PromptMessage::new_text(
            Role::User,
            include_str!("content/issues.md"),
        )]
    }

    #[prompt(
        name = "github_workflow_actions_ci",
        description = "GitHub Actions workflows, runs, artifacts, secrets/variables, \
                        self-hosted runners, hosted compute, check-runs."
    )]
    async fn github_workflow_actions_ci_prompt(&self) -> Vec<PromptMessage> {
        vec![PromptMessage::new_text(
            Role::User,
            include_str!("content/actions_ci.md"),
        )]
    }

    #[prompt(
        name = "github_workflow_orgs_teams",
        description = "Organizations, teams, enterprise teams/memberships, members, outside \
                        collaborators."
    )]
    async fn github_workflow_orgs_teams_prompt(&self) -> Vec<PromptMessage> {
        vec![PromptMessage::new_text(
            Role::User,
            include_str!("content/orgs_teams.md"),
        )]
    }

    #[prompt(
        name = "github_workflow_security_suite",
        description = "Code scanning, secret scanning, code security configurations, \
                        Dependabot, security advisories, dependency graph, private \
                        registries."
    )]
    async fn github_workflow_security_suite_prompt(&self) -> Vec<PromptMessage> {
        vec![PromptMessage::new_text(
            Role::User,
            include_str!("content/security_suite.md"),
        )]
    }

    #[prompt(
        name = "github_workflow_apps_auth_billing",
        description = "GitHub Apps/installations, OAuth apps, OIDC, billing, credentials, \
                        API insights."
    )]
    async fn github_workflow_apps_auth_billing_prompt(&self) -> Vec<PromptMessage> {
        vec![PromptMessage::new_text(
            Role::User,
            include_str!("content/apps_auth_billing.md"),
        )]
    }

    #[prompt(
        name = "github_workflow_packages_migrations_gists",
        description = "Packages, import/export migrations, gists."
    )]
    async fn github_workflow_packages_migrations_gists_prompt(&self) -> Vec<PromptMessage> {
        vec![PromptMessage::new_text(
            Role::User,
            include_str!("content/packages_migrations_gists.md"),
        )]
    }

    #[prompt(
        name = "github_workflow_codespaces_copilot",
        description = "Codespaces, Copilot, Copilot Spaces, agents/agent tasks, GitHub \
                        Classroom."
    )]
    async fn github_workflow_codespaces_copilot_prompt(&self) -> Vec<PromptMessage> {
        vec![PromptMessage::new_text(
            Role::User,
            include_str!("content/codespaces_copilot.md"),
        )]
    }

    #[prompt(
        name = "github_workflow_projects",
        description = "Projects (v2), campaigns."
    )]
    async fn github_workflow_projects_prompt(&self) -> Vec<PromptMessage> {
        vec![PromptMessage::new_text(
            Role::User,
            include_str!("content/projects.md"),
        )]
    }

    #[prompt(
        name = "github_workflow_users_activity",
        description = "User profile/keys/social graph, activity feed, starring/watching, \
                        notifications."
    )]
    async fn github_workflow_users_activity_prompt(&self) -> Vec<PromptMessage> {
        vec![PromptMessage::new_text(
            Role::User,
            include_str!("content/users_activity.md"),
        )]
    }

    #[prompt(
        name = "github_workflow_meta_diagnostics",
        description = "Thin pointer to read-only utility signals: API meta, rate limits, \
                        code search, emojis, gitignore templates, licenses, code-of-conduct \
                        templates, markdown rendering."
    )]
    async fn github_workflow_meta_diagnostics_prompt(&self) -> Vec<PromptMessage> {
        vec![PromptMessage::new_text(
            Role::User,
            include_str!("content/meta_diagnostics.md"),
        )]
    }
}
