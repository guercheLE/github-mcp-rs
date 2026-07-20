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
context that hasn't run yet. For newer projects, also consider
`github-workflow-rulesets` — it supersedes classic branch protection and
supports a dry-run mode before enforcement.

Real gotcha — atomic multi-file commits: the single-file
`create-or-update-file-contents` operation can't touch more than one file
per commit. For a commit that must span multiple files atomically, search
for the git data API instead and use it in order: create a blob per file,
create a tree referencing those blobs (with `base_tree` set to the
current tree), create a commit pointing at that tree, then update the
branch ref to point at the new commit. Skipping straight to "update the
ref" before the commit/tree/blobs exist will fail or silently reference
the wrong content.

Real gotcha — publishing a release: search for how to create a release as
a draft first, then upload each release asset (a binary upload, not a
plain JSON call) before updating the release to flip `draft` to false —
uploading assets after the release is already public means there's a
window where the release exists without its assets. `generate-release-notes`
can produce a draft changelog beforehand if the user wants one. Some
repositories also enable immutable releases, which changes what can still
be edited once a release is published — check for that setting if an
edit after publishing unexpectedly fails.

If the user reports a webhook isn't firing, search for how to list a
webhook's recent deliveries (not just whether the webhook itself exists)
before assuming misconfiguration — a delivery that shows a non-2xx
response points at the receiving end, not GitHub's side; redelivering a
past delivery is also useful for reproducing the issue.

Never hardcode an operationId or assume a specific response field name —
both can differ across the GitHub deployments (`gh`, `ghec`, `ghes`) this
server supports. Always call `get` on whatever operationId `search`
resolves to and read its current schema.

## Composing with other workflows

Opening a pull request against a branch here is covered in more depth by
`github-workflow-pull-request`; workflow-driven status checks are covered
by `github-workflow-actions-ci`; rulesets (the modern alternative to
branch protection) are covered by `github-workflow-rulesets`; deployment
environments, deployments, and Pages are covered by
`github-workflow-environments-deployments`.
