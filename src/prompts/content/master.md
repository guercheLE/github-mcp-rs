# GitHub management workflows

This server also exposes `search`/`get`/`call` directly — use this menu when
a task needs more than one call, in a specific order, or has a non-obvious
gotcha worth walking through step by step.

If a `goal` was supplied above, match it against the menu below and go
straight to that sub-workflow. Otherwise, show this menu to the user and
ask which task they want help with.

**Once you've picked a sub-workflow: if your environment provides a way to
run a sub-task in an isolated context (e.g. an agent/task tool), delegate
the entire sub-workflow to it** — hand the sub-task the sub-workflow's
prompt name below plus whatever parameters are already known, let it fetch
that prompt (`prompts/get`) and carry out every step itself, and have it
report back only a short summary. This keeps the sub-workflow's full
`search`/`get`/`call` trace out of this conversation. If no such mechanism
is available, run the sub-workflow's steps directly here instead.

## Menu

- **`github_workflow_repos`** — repository lifecycle (create, fork,
  transfer, archive, delete), branches, tags, commits/git data, releases,
  topics/settings, webhooks.
- **`github_workflow_pull_request`** — guided fork-vs-direct-branch pull
  request flow: branch, commit, push, open the PR, reviewers, checks,
  merge. Use this rather than `repos`/`issues` individually when the goal
  is specifically opening or landing a PR.
- **`github_workflow_issues`** — issue lifecycle, labels, milestones,
  assignees, comments, reactions.
- **`github_workflow_actions_ci`** — GitHub Actions workflows, runs,
  artifacts, secrets/variables, self-hosted runners, hosted compute,
  check-runs.
- **`github_workflow_orgs_teams`** — organizations, teams, enterprise
  teams/memberships, members, outside collaborators.
- **`github_workflow_security_suite`** — code scanning, secret scanning,
  code security configurations, Dependabot, security advisories,
  dependency graph, private registries.
- **`github_workflow_apps_auth_billing`** — GitHub Apps/installations,
  OAuth apps, OIDC, billing, credentials, API insights.
- **`github_workflow_packages_migrations_gists`** — packages, import/export
  migrations, gists.
- **`github_workflow_codespaces_copilot`** — Codespaces, Copilot, Copilot
  Spaces, agents/agent tasks, GitHub Classroom.
- **`github_workflow_projects`** — Projects (v2), campaigns.
- **`github_workflow_users_activity`** — user profile/keys/social graph,
  activity feed, starring/watching, notifications.
- **`github_workflow_meta_diagnostics`** — read-only utility signals: API
  meta, rate limits, code search, emojis, gitignore templates, licenses,
  code-of-conduct templates, markdown rendering.

Every sub-workflow above describes GitHub operations only by what they do
(e.g. "search for how to create a pull request"), never by a specific
operationId or an assumed response field — the exact operation id, and even
the response shape for the same id, can differ depending on which GitHub
deployment (`gh`, `ghec`, or `ghes`, and which `ghes` point release) this
server is configured for. Always confirm the current schema via `get`
before relying on a field name.
