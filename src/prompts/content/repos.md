# Sub-workflow: Repositories, branches, and releases

Covers repository lifecycle (create, fork, transfer, archive, delete),
branches and branch protection, tags, commits and other git data (trees,
blobs, refs), releases, topics/settings, and webhooks.

For each task, search for how to do it in natural language — e.g. "how to
create a repository", "how to protect a branch with required status
checks", "how to create a release" — then call the operation `search`
resolves to, and confirm the result via a follow-up `get`/list call rather
than assuming the write succeeded.

Real gotcha: branch protection rules reference required status-check
*contexts* by name — those names only exist once a workflow or external CI
system has actually reported a status for at least one commit on the
branch, so search for how to list the check-runs or commit statuses
already reported on the branch before configuring protection around a
context that hasn't run yet.

Never hardcode an operationId or assume a specific response field name —
both can differ across the GitHub deployments (`gh`, `ghec`, `ghes`) this
server supports. Always call `get` on whatever operationId `search`
resolves to and read its current schema.

## Composing with other workflows

Opening a pull request against a branch here is covered in more depth by
`github_workflow_pull_request`; workflow-driven status checks are covered
by `github_workflow_actions_ci`.
