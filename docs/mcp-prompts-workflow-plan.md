# MCP prompts: guided GitHub management workflows

## Context

`github-mcp-rs` currently exposes exactly 3 MCP tools — `search`, `get`, `call` ([src/core/mcp_server.rs](../src/core/mcp_server.rs)) — backed by an embedded, per-API-version catalog of GitHub REST API operations. This is 5 separate SQLite stores ([src/data/store.rs:26-52](../src/data/store.rs)), spanning **three distinct kinds of GitHub deployment**, confirmed against `mcpify.yaml`'s `versions:` list and [docs/SCHEMA_VERSIONS.md](SCHEMA_VERSIONS.md):

- **`gh`** — github.com (GitHub.com's public REST API): `mcp_store.db.zst` (`gh-2026-03-10`, the default, **1,206 operations**).
- **`ghec`** — GitHub Enterprise Cloud: `mcp_store_vghec-2026-03-10.db.zst` (`ghec-2026-03-10`, **1,446 operations** — the largest of the 5, since GHEC exposes enterprise-only endpoints on top of the GitHub.com surface).
- **`ghes`** — GitHub Enterprise Server, self-hosted, versioned by product release rather than by date: three point releases are supported concurrently — `mcp_store_vghes-3.21.db.zst` (**1,092**), `mcp_store_vghes-3.20.db.zst` (**1,082**), `mcp_store_vghes-3.19.db.zst` (**1,029**).

Each store has its own `generated_schemas_v*.json.zst` mirroring that version's input/output JSON Schemas ([src/validation/validator.rs](../src/validation/validator.rs), same `mcpify:versions:begin/end`-bounded pattern as `store.rs`), used for `call` validation. Any guided-workflow prompt this plan adds must stay correct across all three kinds — not just within `ghes`'s three point releases, but across the full spread of all 5 stores, from newest/largest (`ghec-2026-03-10`, 1,446 ops) to oldest/smallest (`ghes-3.19`, 1,029 ops).

Verified directly against the decompressed default (`gh-2026-03-10`) and oldest-supported (`ghes-3.19`) stores: only **798 `operationId`s are common to both** — 408 operations that exist in `gh-2026-03-10` have no counterpart at all in `ghes-3.19`. Worse, among those 798 shared ids, **89 have a genuinely different `input_schema`** and **31 have a genuinely different `output_schema`** (e.g. `actions/create-workflow-dispatch`, `actions/list-runner-applications-for-org`). So, exactly as with `search`/`get`/`call` themselves, any guided-workflow instructions built on top must never hardcode an `operationId` or assume a specific response field name — they must tell the calling LLM to `search`, then read whatever schema `get` currently returns for the resolved id, since both the available operations and their shapes genuinely differ depending on which `api_version` this server is configured for.

The embedded catalog spans far more ground than a single flat tool surface can usefully convey on its own: `repos` (203 ops), `actions` (187), `orgs` (99), `issues` (55), `codespaces` (48), `users` (47), `apps` (37), `teams`/`activity` (32 each), `copilot` (31), `agents` (30), `copilot-spaces` (28), `pulls`/`packages` (27 each), `projects` (26), `dependabot` (25), `migrations` (22), `code-scanning` (21), `gists`/`code-security` (20 each), `secret-scanning` (17), `reactions` (15), `interactions` (14), `git`/`billing` (13 each), `checks` (12), `security-advisories` (10), `api-insights` (9), `oidc` (8), `search` (7), plus a long tail of smaller categories (`private-registries`, `hosted-compute`, `enterprise-team-*`, `classroom`, `meta`, `dependency-graph`, `campaigns`, `agent-tasks`, `code-quality`, `licenses`, `markdown`, `gitignore`, `codes-of-conduct`, `rate-limit`, `emojis`, `credentials`) — verified by grouping `operation_id` prefixes in the decompressed default store. All of this sequencing knowledge — e.g. "opening a pull request as an outside contributor requires forking first, but a repo collaborator can branch directly" — is currently left entirely to whichever LLM drives the client, re-derived from scratch every session.

The goal is to add an MCP **prompts** capability: a master "menu" prompt plus one prompt per logical GitHub domain, each returning instructional prose that guides the calling LLM through that domain's task step by step — asking for missing parameters, gating progression until a step's goal is actually verified (not just attempted), calling out independent steps that can run in parallel or be delegated, and always describing operations by capability ("search for how to create a pull request") rather than by a specific `operationId` or assumed response shape, for the version-drift reasons confirmed above.

The mechanism is already available for free: this crate's `rmcp = "2"` dependency resolves to **rmcp 2.2.0** (confirmed via `Cargo.lock`), which ships a first-class prompts API — verified directly against the vendored crate sources (`~/.cargo/registry/src/.../rmcp-2.2.0/src/handler/server/prompt.rs` and `rmcp-macros-2.2.0/src/{prompt.rs,prompt_router.rs,prompt_handler.rs}`) that mirrors the `#[tool_router]`/`#[tool]`/`#[tool_handler]` pattern `src/core/mcp_server.rs` already uses for `search`/`get`/`call` almost exactly: `#[prompt_router]` appends a `prompt_router() -> PromptRouter<Self>` associated fn to whatever `impl <Self>` block it decorates (it doesn't need to be the same block as `#[tool_router]`), and `#[tool_handler]`/`#[prompt_handler]` can stack on the same `impl ServerHandler` block since each only contributes its own disjoint set of methods (`call_tool`/`list_tools` vs. `get_prompt`/`list_prompts`).

This repo is generated output from a sibling generator (every existing `.rs` file opens `// GitHub v3 REST API MCP server — generated by mcpify. Do not hand-edit.`; `mcpify.yaml` has `force: true`). This plan hand-edits the repo directly rather than the generator, mirroring how this crate's own existing `search`/`get`/`call` layer was itself hand-authored on top of generated scaffolding — accepted risk: a future regeneration run against this project would overwrite these changes.

## Approach

### File layout

Prompt code is kept entirely separate from tool code: all prompt logic lives in a new `src/prompts/` module, distinct from `src/tools/` (which holds `search`/`get`/`call`'s business logic — [search_tool.rs](../src/tools/search_tool.rs), [get_tool.rs](../src/tools/get_tool.rs), [call_tool.rs](../src/tools/call_tool.rs)). Unlike those three, the `search`/`get`/`call` `#[tool]` methods and `#[tool_router]` block live directly in `src/core/mcp_server.rs`, not in `src/tools/` — the new `#[prompt_router]`-decorated block deliberately does **not** follow that same placement: it goes in its own `src/prompts/router.rs`, and `src/core/mcp_server.rs` itself is touched only for the minimal wiring a single `ServerHandler`/struct necessarily requires (new struct field, stacked handler macro, `.enable_prompts()` capability flag, import) — no prompt method bodies or prompt-specific logic added there. This keeps the new, much larger prompt surface (13 methods vs. 3) from bloating the one file that already carries `search`/`get`/`call`'s full test module.

New module `src/prompts/`, declared from `src/lib.rs`, alphabetically between the existing `pub mod http;` and `pub mod services;`:

```
src/prompts/
  mod.rs                        // arg structs + render_context_header() helper (+ its own unit tests)
  router.rs                     // #[prompt_router]-decorated impl McpifyServer block
  content/
    master.md
    repos.md
    pull_request.md
    issues.md
    actions_ci.md
    orgs_teams.md
    security_suite.md
    apps_auth_billing.md
    packages_migrations_gists.md
    codespaces_copilot.md
    projects.md
    users_activity.md
    meta_diagnostics.md
```

Instructional prose lives in `.md` files pulled in via `include_str!`, not inline Rust string literals — this follows the pattern the crate already uses for large embedded assets ([store.rs:35-52](../src/data/store.rs) `include_bytes!`s each version's `.db.zst`; `validator.rs` does the same for schema JSON). As `.rs` string literals this content would fight `rustfmt`, produce noisy diffs, and lose markdown tooling. Anything that varies per-invocation (which optional arguments the caller already supplied) is rendered separately in Rust as a short "Context already provided" header and prepended to the static markdown body — no template-substitution engine needed.

New hand-authored files should **not** carry the "generated by mcpify. Do not hand-edit." header every existing file has — that claim would be false for this module.

### `McpifyServer` changes ([src/core/mcp_server.rs](../src/core/mcp_server.rs))

Add a `prompt_router` field next to the existing `tool_router` (currently the struct at lines 68-74):

```rust
#[derive(Clone)]
pub struct McpifyServer {
    api_version: String,
    config: Config,
    auth_manager: Arc<Mutex<AuthManager>>,
    tool_router: ToolRouter<McpifyServer>,
    prompt_router: rmcp::handler::server::router::prompt::PromptRouter<McpifyServer>,
}
```

`PromptRouter<S>` is `Clone`, so the struct's `#[derive(Clone)]` is unaffected. `new()` (line 85) gains `prompt_router: Self::prompt_router()` alongside the existing `tool_router: Self::tool_router()`. No changes needed at any of the three construction call sites (`src/main.rs:147` and `:161`, `src/http/server.rs:279`) — the constructor signature doesn't change.

Stack the handler macros and add `.enable_prompts()` to the existing `ServerHandler` impl (currently lines 204-216):

```rust
#[tool_handler(router = self.tool_router.clone())]
#[prompt_handler(router = self.prompt_router.clone())]
impl ServerHandler for McpifyServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(
            ServerCapabilities::builder()
                .enable_tools()
                .enable_prompts()
                .build(),
        )
        .with_server_info(Implementation::from_build_env())
        .with_protocol_version(ProtocolVersion::V_2024_11_05)
        .with_instructions(
            "Exposes exactly 3 tools -- search, get, call -- backed by an embedded \
             semantic database, so you never need the full API surface in context. \
             Also exposes MCP prompts -- start with the `github_workflow` prompt for \
             guided, multi-step help with common GitHub management tasks."
                .to_string(),
        )
    }
}
```

Add `prompt_handler` to the existing import at lines 12-15 (`use rmcp::{ErrorData as McpError, RoleServer, ServerHandler, ServiceExt, schemars, tool, tool_handler, tool_router};`).

### `src/prompts/router.rs` — one method per prompt

Mirrors the existing `SearchArgs`/`GetArgs`/`CallArgs` + `#[tool(...)]` pattern already in `mcp_server.rs`:

```rust
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

    // one method per sub-workflow, same shape -- see prompt inventory below
}
```

Argument structs go in `src/prompts/mod.rs`, `#[derive(Deserialize, schemars::JsonSchema)]` like the existing tool arg structs in `mcp_server.rs`, every field `Option<String>` with a doc comment (doc comments become each `PromptArgument`'s description). Prompts with no meaningful arguments simply omit the `Parameters<T>` extractor from the method signature — the macro emits `arguments: None` automatically when no such extractor is present.

**Why every argument is `Option`, never `required: true`:** MCP prompt arguments are conventionally collected up front by whatever UI a client renders when a prompt is explicitly invoked (e.g. a slash-command form) — not well suited to values that only become known partway through a guided flow, and a strict client would refuse `prompts/get` entirely until a required field is filled. Pushing "ask if missing" into the instructional prose instead of transport-level required-argument validation is what makes it work uniformly for agentic clients that never populate prompt arguments at all, and interactive ones whose humans do.

### Prompt inventory

GitHub's catalog is ~9x larger by raw `operation_id`-prefix count than a domain like RabbitMQ's, so a 1:1 mapping from raw prefix to prompt would produce 40+ prompts — unwieldy for `prompts/list` and for a human picking from a menu. Instead, group by what a user is actually trying to accomplish, folding related raw prefixes together (e.g. `code-scanning`/`secret-scanning`/`code-security`/`dependabot`/`security-advisories`/`dependency-graph`/`private-registries` all become one "security suite" prompt). This yields 12 sub-workflow prompts (still proportionate to RabbitMQ's ~10, given the much larger domain) plus the master:

| name | covers (raw `operation_id` prefixes) | arguments |
|---|---|---|
| `github_workflow` | master index; menu + goal-based routing | `goal: Option<String>` |
| `github_workflow_repos` | repo lifecycle (create/fork/transfer/archive/delete), branches, tags, commits/git-data, releases, topics/settings, webhooks (`repos`, `git`) | none |
| `github_workflow_pull_request` | guided fork-vs-direct-branch PR flow: branch, commit, push, open PR, reviewers, checks, merge (`pulls`, relevant `checks`) | `owner`, `repo`, `base_branch`, `head_branch` |
| `github_workflow_issues` | issue CRUD, labels, milestones, assignees, comments, reactions (`issues`, `reactions`, `interactions`) | none |
| `github_workflow_actions_ci` | Actions workflows/runs/artifacts, secrets/variables (repo/org/env), self-hosted runners, hosted compute, check-runs (`actions`, `hosted-compute`, remaining `checks`) | none |
| `github_workflow_orgs_teams` | orgs, teams, enterprise teams/memberships/orgs, members, outside collaborators (`orgs`, `teams`, `enterprise-team*`) | none |
| `github_workflow_security_suite` | code scanning, secret scanning, code security configs, Dependabot, security advisories, dependency graph, private registries (`code-scanning`, `secret-scanning`, `code-security`, `dependabot`, `security-advisories`, `dependency-graph`, `private-registries`) | none |
| `github_workflow_apps_auth_billing` | GitHub Apps/installations, OAuth apps, OIDC, billing, credentials, API insights (`apps`, `oidc`, `billing`, `credentials`, `api-insights`) | none |
| `github_workflow_packages_migrations_gists` | packages, import/export migrations, gists (`packages`, `migrations`, `gists`) | none |
| `github_workflow_codespaces_copilot` | codespaces, Copilot, Copilot Spaces, agents/agent tasks, classroom (`codespaces`, `copilot`, `copilot-spaces`, `agents`, `agent-tasks`, `classroom`) | none |
| `github_workflow_projects` | Projects (v2), campaigns (`projects`, `campaigns`) | none |
| `github_workflow_users_activity` | user profile/keys/social graph, activity feed, starring/watching, notifications (`users`, `activity`) | none |
| `github_workflow_meta_diagnostics` | thin pointer to read-only utility signals: API meta, rate limits, code search, emojis, gitignore templates, licenses, code-of-conduct templates, markdown rendering (`meta`, `rate-limit`, `search`, `emojis`, `gitignore`, `codes-of-conduct`, `licenses`, `markdown`) — kept as its own prompt purely for `prompts/list` discoverability, deliberately not a multi-step guided flow | none |

### Whole-sub-workflow delegation (the master prompt's core routing responsibility)

This is the primary lever for sparing the main conversation's context window and tokens — more so than delegating individual steps within one sub-workflow (below). `master.md`'s routing instructions must tell the calling LLM: once you've matched the user's goal (or the menu selection) to one of the 12 sub-workflow prompt names, **if your environment provides a way to run a sub-task/agent in an isolated context, delegate the entire matched sub-workflow to it** — hand that sub-task the sub-workflow's prompt name (e.g. `github_workflow_pull_request`) and whatever parameters are already known, let it fetch that prompt itself (`prompts/get`) and carry out every one of its steps — including all of *its own* `search`/`get`/`call` traffic — entirely within its own context, and have it report back to this conversation only a short summary: what was accomplished/confirmed, and anything it still needs from the user. Only fall back to running the sub-workflow's steps directly in the current context if no such delegation mechanism is available.

This is what actually keeps a multi-step guided workflow's full tool-call trace out of the main conversation — a single sub-workflow like `github_workflow_actions_ci` (secrets, runners, workflow dispatch, run status polling) can easily produce far more intermediate tool traffic than the final summary needs to convey. Every sub-workflow's own `content/*.md` should open with a short note reflecting this too (see the worked example below): it's designed to be handed to a fresh sub-task with just its own prompt text plus known parameters, self-contained enough that the sub-task doesn't need any of the master conversation's other history to execute it.

The finer-grained, step-level delegation described further below is a secondary tactic that still applies *within* whichever context ends up actually executing the sub-workflow's steps.

### The agnostic-phrasing rule (applies to every prompt, not just the worked example)

Every operation reference in every `content/*.md` file must be phrased as a *task to search for*, never as a specific tool/operation name — e.g. write `search for "how to create a pull request?"`, not `call "pulls/create"` or `call createPullRequest`. This isn't a style preference: it's required by the version-drift confirmed above — 408 operations differ in which ids even exist between just the default (`gh-2026-03-10`) and oldest-supported (`ghes-3.19`) stores, and among the 798 shared ids, 89 have a different input schema and 31 a different output schema. Phrasing every step as a natural-language search query, followed by "read the schema `get` returns before relying on any field name," keeps every prompt correct regardless of which of the 5 catalogs — spanning all **three kinds** (`gh`, `ghec`, `ghes`) — is active for a given deployment. Treat this as a hard rule to check for in review, not just a default; it applies uniformly whether the difference in question is `gh` vs. `ghec` (enterprise-only endpoints) or a drift between two `ghes` point releases.

### Content design pattern (worked example: `github_workflow_pull_request`)

`src/prompts/content/pull_request.md` must demonstrate every element the design calls for — use this shape for any other sub-workflow that turns out to be similarly compound (order-dependent, forked, multi-resource), not just this one:

- **Opening note — this sub-workflow is self-contained and delegable.** Before Step 0: "This sub-workflow is designed to be run as an isolated sub-task where possible — if you were delegated here from `github_workflow`'s routing, or your environment otherwise supports running this as its own sub-task, everything you need is in this prompt's own text plus the parameters already listed above; report back only a short summary when done rather than the full step-by-step trace."
- **Step 0 — gather required parameters.** Check the prepended "Context already provided" header first; only ask the user for what's still listed as missing: `owner`, `repo`, `base_branch` (what the change should land on), and either `head_branch` (an existing branch) or enough description of the change to create one. Don't proceed to Step 1 until these are known.
- **Step 1 — an explicit fork with a disambiguating question.** GitHub's contribution model genuinely forks depending on repo permissions: (A) direct-branch (the user already has push access to `owner/repo`) — create `head_branch` directly in the repo, commit, and push. (B) fork-based (the user has no push access — the common case for contributing to a repo you don't own) — fork `owner/repo` into the user's own account first, branch and commit there, and open the PR from the fork back to `owner/repo`. Ask "do you already have push access to this repository?" rather than guessing.
- **Step 2 — parallelizable, independent sub-steps, delegate if possible.** Pushing the branch's commits and checking `base_branch`'s existing branch-protection rules (which required checks/reviews the PR will need to satisfy before merge) don't depend on each other — call this out explicitly as safe to do concurrently, and as a candidate for delegation: "if your environment provides a way to run a sub-task in its own context, delegate 'push the branch' and 'look up branch protection requirements for `base_branch`' as two separate sub-tasks and have each return only a short confirmation — don't pull the full request/response bodies into this conversation. If no such sub-task mechanism is available, just do both directly." Every operation reference here is phrased agnostically per the rule above. Gate: don't proceed until the branch is confirmed pushed (via a follow-up search-and-call, not just "the push call didn't error").
- **Step 3 — open the pull request**, gated on Step 2's branch push being confirmed, using `base_branch` and either `head_branch` or `fork-owner:head_branch` depending on the Step 1 fork.
- **Step 4 — request reviewers and add labels/assignees**, gated on the PR existing, called out as independent of each other (parallelizable).
- **Step 5 — verify before declaring done.** Don't tell the user the PR is ready to merge until you've confirmed — via search-and-call, not assumption — that required status checks report success and required reviews (if any, per Step 2's lookup) are satisfied. Summarize what was done and ask whether to merge, rather than merging unprompted.
- **Composing with other workflows** — branch/commit mechanics overlap with `github_workflow_repos`; check-run status overlaps with `github_workflow_actions_ci`; comments/labels overlap with `github_workflow_issues`. Tell the calling LLM to fetch those prompts by name for more detail rather than duplicating their content here.

Every other sub-workflow's `.md` should follow this same skeleton where it's genuinely order-dependent and forked (numbered steps, an explicit "don't proceed until X is confirmed" gate per step, agnostic search-language instructions, a call-out of any genuinely independent sub-steps as parallelizable) — but per the content-size guidance below, most of the 12 don't need the full weight of this treatment.

**Step-level delegation and parallelization — secondary to whole-sub-workflow delegation above, but still needed within whatever context runs the steps.** For any single step whose own tool traffic would be verbose relative to what the workflow actually needs back (a `search` over many candidates before picking one, paging through a long Actions run log or issue list, the branch-protection lookup above), the prose should tell the calling LLM to push *that step* into a further sub-task if the host environment offers one, and bring back only the distilled result rather than letting the full intermediate tool output accumulate. Phrase this conditionally, since not every MCP client has a sub-task mechanism. Every sub-workflow with a step that plausibly produces a large or exploratory tool response (most obviously `github_workflow_actions_ci`'s run logs, `github_workflow_meta_diagnostics`'s code-search results, and any `search` with many candidate matches) should include this instruction at that step.

### Content size and token economy

MCP's two-phase discovery model already bounds most of the cost here:

- `prompts/list` returns only `name` + `description` + `arguments` for all 13 prompts — small by construction.
- `prompts/get` is per-prompt and on-demand — a client/LLM only pays for the one workflow's markdown body it actually fetches, never all 13 at once — the same shape as `search`→`get`.

Given that, the actual lever is keeping each individual `content/*.md` proportional to its domain's real complexity:

- **Multi-resource, order-dependent, forked domains** (`pull_request`, and any others that turn out to genuinely need it, e.g. `actions_ci`'s secret-before-workflow-run ordering) can run longer but should still target roughly **60-120 lines**, not 200+. If a domain's steps sprawl past that, split it into its own sub-workflow rather than growing it in place.
- **Broader CRUD-ish domains without a real fork or strict ordering** (`repos`, `issues`, `orgs_teams`, `security_suite`, `apps_auth_billing`, `packages_migrations_gists`, `codespaces_copilot`, `projects`, `users_activity`) should be short, roughly **20-50 lines**: what the domain covers, the agnostic search-language pattern, and 1-2 sentences on any real gotcha, not a padded numbered-step scaffold for what's really a single search-then-call action per resource.
- **`github_workflow_meta_diagnostics`** should be the shortest of all — a single paragraph.
- **`master.md`** must stay a lean menu: one line per sub-workflow (name, one-sentence when-to-use) plus brief goal-matching guidance, not a summary of each sub-workflow's internal steps. Target **under 70 lines** (13 entries vs. RabbitMQ-scale's 11, so allow a little more room than a strict 60-line cap).

These are targets to keep content proportional and reviewable, not hard limits enforced by code — call it out in review if a draft `.md` file overshoots its band without a real reason.

## Critical files

- `docs/mcp-prompts-workflow-plan.md` (this file) — persisted into the repo first, before any code changes, following this repo's existing `docs/` naming (alongside [docs/SCHEMA_VERSIONS.md](SCHEMA_VERSIONS.md), [docs/github-api-specs.md](github-api-specs.md))
- [src/core/mcp_server.rs](../src/core/mcp_server.rs) — struct field, macro stacking, capabilities, import, instructions text
- [src/lib.rs](../src/lib.rs) — `pub mod prompts;` declaration
- `src/prompts/mod.rs` (new) — argument structs (`MasterWorkflowArgs`, `PullRequestWorkflowArgs`), `render_context_header` helper + its own unit tests (`#[cfg(test)] mod tests`, separate from any tool test)
- `src/prompts/router.rs` (new) — the `#[prompt_router]`-decorated `impl McpifyServer` block, one method per table row above
- `src/prompts/content/*.md` (new, 13 files) — one per prompt; `master.md` and `pull_request.md` written last-and-first respectively (see Sequencing)
- `tests/prompts_workflow.rs` (new) — protocol-level `prompts/list`/`prompts/get` integration tests, kept out of `src/core/mcp_server.rs`'s existing test module entirely (see Verification)
- Reuse as reference patterns (no changes needed): [src/tools/search_tool.rs](../src/tools/search_tool.rs), [src/tools/get_tool.rs](../src/tools/get_tool.rs) for existing tool-method shape; [src/data/store.rs](../src/data/store.rs) and [src/validation/validator.rs](../src/validation/validator.rs) for the established `include_bytes!`-for-large-embedded-assets convention this plan extends to `include_str!`; [tests/cli_smoke.rs](../tests/cli_smoke.rs) for this repo's existing top-level-`tests/`-integration-test convention, which `tests/prompts_workflow.rs` follows

## Sequencing

0. **Persist this plan into the repo** as `docs/mcp-prompts-workflow-plan.md`, so the design record lives with the code it describes rather than only in an ephemeral planning file outside the repo. Do this first, before any code changes.
1. **Vertical slice**: wire up the struct field, macro stacking, `.enable_prompts()`, and implement only `github_workflow` + `github_workflow_pull_request` (with their `content/*.md`). Exercises every integration point at once.
2. **Stand up `tests/prompts_workflow.rs` and verify** the vertical slice through it before writing more content (see Verification below) — this is also where the new file's transport/client scaffolding gets written once, for the remaining prompts' tests to extend rather than re-invent.
3. **Fill in the remaining 10 sub-workflow prompts** one at a time — pure content-design work once step 1 is proven, since they all share the same plumbing.
4. **Finalize `master.md`** last, once every prompt name is stable, so its menu references real names.

## Verification

- `cargo build` / `cargo test` from the repo root after each stage above.
- **Prompt tests stay physically separate from tool tests** — nothing prompt-related is added to `src/core/mcp_server.rs`'s existing `#[cfg(test)] mod tests` (which stays scoped to `search`/`get`/`call`, unchanged except for the one capabilities-flag assertion below). Two new, separate test locations instead:
  - **`tests/prompts_workflow.rs`** (new top-level integration test file, following the same convention as [tests/cli_smoke.rs](../tests/cli_smoke.rs)) — protocol-level tests against the crate's public API (`github_mcp::core::mcp_server::McpifyServer`, an `rmcp::ClientHandler` stub, `tokio::io::duplex`, the same pattern `mcp_protocol_routes_search_get_and_call_requests` already uses, just promoted to its own file/compilation unit):
    - `prompts/list` shape: assert `client.list_all_prompts()` returns all 13 names under the shared `github_workflow*` prefix, and that `github_workflow_pull_request`'s advertised arguments include `owner`/`repo`/`base_branch`/`head_branch`, all with `required == Some(false)`.
    - `prompts/get` round-trip for `github_workflow` with no arguments — assert success and that the returned text mentions `github_workflow_pull_request` (proves the menu links to it).
    - `prompts/get` round-trip for `github_workflow_pull_request` with partial arguments (e.g. `owner` + `repo` supplied, `base_branch`/`head_branch` omitted) — assert the rendered header both echoes the supplied values and lists the still-missing ones.
    - `server_info_advertises_the_prompts_capability`: `info.capabilities.prompts.is_some()` (a new, prompts-specific assertion in the new file — the existing tools-side assertion in `mcp_server.rs`, `server_info_advertises_the_generated_tool_surface`, is left as-is).
  - **`src/prompts/mod.rs`**'s own `#[cfg(test)] mod tests` — pure unit test for `render_context_header` covering: empty slice, all-supplied, all-missing, mixed. Pure logic, no transport, so it doesn't need the integration-test harness.
- Manual smoke check: `cargo run -- start` (stdio) and, separately, `cargo run -- http` with an MCP-capable client that supports `prompts/list`/`prompts/get`, to confirm the master → pull-request cross-reference reads naturally to a real calling LLM, not just structurally valid per the automated tests.

## Release (once implementation is complete and `cargo test` passes)

This repo's existing convention, confirmed from git history and `.github/workflows/release.yml`: releases are tag-driven (`push: tags: "v*.*.*"` triggers the cargo-dist build/publish job), and every past release follows a commit-then-tag shape (e.g. `chore(release): bump version to 0.5.8`, tag presumably `v0.5.8`). Follow it:

1. `git commit` the implementation changes with a conventional-commit message (e.g. `feat(prompts): add guided GitHub workflow prompts` — confirm the exact `type(scope)` against this repo's actual recent history at commit time).
2. `git commit` `docs/mcp-prompts-workflow-plan.md` as its own separate commit (e.g. `docs: add MCP prompts workflow implementation plan`) — kept apart from the implementation commit, mirroring the file-separation principle applied throughout this plan.
3. Bump `version` in `Cargo.toml` (and let `Cargo.lock` follow via `cargo check`/`cargo build`), commit as `chore(release): bump version to X.Y.Z` — matching prior release commits' exact message shape. Current version is `0.5.8`; default to `0.5.9` unless the implementation commit's conventional-commit type argues for a minor bump instead — confirm what actually landed before choosing.
4. `git tag vX.Y.Z` on that bump commit (matching the `v*.*.*` pattern `release.yml` listens for).
5. `git push` the branch, then `git push --tags` (or `git push origin vX.Y.Z`) — confirm with the user before pushing, since pushes and tag creation are confirmed, not assumed.
