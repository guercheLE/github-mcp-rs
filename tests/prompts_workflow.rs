// Protocol-level tests for the MCP `prompts` capability, kept in its own
// file/compilation unit deliberately separate from any tool test — see
// docs/mcp-prompts-workflow-plan.md's file-separation rule.

use github_mcp::auth::auth_manager::AuthManager;
use github_mcp::core::config_schema::{AuthMethod, Config};
use github_mcp::core::mcp_server::McpifyServer;
use rmcp::model::{ContentBlock, GetPromptRequestParams};
use rmcp::{ServerHandler, ServiceExt};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone, Default)]
struct TestClient;

impl rmcp::ClientHandler for TestClient {}

fn server() -> McpifyServer {
    let config: Config = serde_json::from_value(serde_json::json!({
        "url": "https://api.example.test",
        "auth_method": "pat"
    }))
    .unwrap();
    McpifyServer::new(
        "gh-2026-03-10".to_string(),
        config,
        Arc::new(Mutex::new(AuthManager::new(AuthMethod::Pat))),
    )
}

async fn connected_client() -> rmcp::service::RunningService<rmcp::RoleClient, TestClient> {
    let (server_transport, client_transport) = tokio::io::duplex(64 * 1024);
    tokio::spawn(async move {
        server().serve(server_transport).await?.waiting().await?;
        anyhow::Ok(())
    });
    TestClient.serve(client_transport).await.unwrap()
}

fn text_of(result: &rmcp::model::GetPromptResult) -> &str {
    match &result.messages[0].content {
        ContentBlock::Text(text) => text.text.as_str(),
        other => panic!("expected a text content block, got {other:?}"),
    }
}

#[tokio::test]
async fn prompts_list_advertises_every_workflow_prompt_under_the_shared_prefix() {
    let client = connected_client().await;

    let prompts = client.list_all_prompts().await.unwrap();
    let mut names: Vec<&str> = prompts.iter().map(|p| p.name.as_ref()).collect();
    names.sort_unstable();
    assert_eq!(
        names,
        [
            "github_workflow",
            "github_workflow_actions_ci",
            "github_workflow_apps_auth_billing",
            "github_workflow_codespaces_copilot",
            "github_workflow_issues",
            "github_workflow_meta_diagnostics",
            "github_workflow_orgs_teams",
            "github_workflow_packages_migrations_gists",
            "github_workflow_projects",
            "github_workflow_pull_request",
            "github_workflow_repos",
            "github_workflow_security_suite",
            "github_workflow_users_activity",
        ]
    );
    assert!(
        names.iter().all(|n| n.starts_with("github_workflow")),
        "every prompt name must share the github_workflow* prefix, got: {names:?}"
    );

    let pull_request = prompts
        .iter()
        .find(|p| p.name == "github_workflow_pull_request")
        .unwrap();
    let args = pull_request.arguments.as_ref().unwrap();
    let arg_names: Vec<&str> = args.iter().map(|a| a.name.as_str()).collect();
    for expected in ["owner", "repo", "base_branch", "head_branch"] {
        assert!(arg_names.contains(&expected), "missing arg: {expected}");
    }
    assert!(
        args.iter().all(|a| a.required == Some(false)),
        "every pull-request argument must be optional, got: {args:?}"
    );

    drop(client);
}

#[tokio::test]
async fn master_prompt_with_no_arguments_links_to_the_pull_request_sub_workflow() {
    let client = connected_client().await;

    let result = client
        .get_prompt(GetPromptRequestParams::new("github_workflow"))
        .await
        .unwrap();
    assert_eq!(result.messages.len(), 1);
    let text = text_of(&result);
    assert!(text.contains("github_workflow_pull_request"));

    drop(client);
}

#[tokio::test]
async fn pull_request_prompt_echoes_supplied_arguments_and_lists_the_missing_ones() {
    let client = connected_client().await;

    let result = client
        .get_prompt(
            GetPromptRequestParams::new("github_workflow_pull_request").with_arguments(
                serde_json::json!({ "owner": "octocat", "repo": "hello-world" })
                    .as_object()
                    .unwrap()
                    .clone(),
            ),
        )
        .await
        .unwrap();
    let text = text_of(&result);
    assert!(text.contains("`owner` = \"octocat\""));
    assert!(text.contains("`repo` = \"hello-world\""));
    assert!(text.contains("base_branch"));
    assert!(text.contains("head_branch"));

    drop(client);
}

#[tokio::test]
async fn pull_request_prompt_with_no_arguments_lists_every_field_as_missing() {
    let client = connected_client().await;

    let result = client
        .get_prompt(GetPromptRequestParams::new("github_workflow_pull_request"))
        .await
        .unwrap();
    let text = text_of(&result);
    assert!(text.contains("(none — no arguments were supplied"));
    for expected in ["owner", "repo", "base_branch", "head_branch"] {
        assert!(text.contains(expected));
    }

    drop(client);
}

#[tokio::test]
async fn server_info_advertises_the_prompts_capability() {
    let info = server().get_info();
    assert!(info.capabilities.prompts.is_some());
}
