# Sub-workflow: Issues

Covers issue creation/update/close/reopen, labels, milestones, assignees,
comments, and reactions.

For each task, search for how to do it in natural language — e.g. "how to
create an issue", "how to add a label to an issue", "how to comment on an
issue" — then call the operation `search` resolves to, and confirm the
result via a follow-up `get`/list call rather than assuming the write
succeeded.

Real gotcha: labels and milestones must already exist on the repository
before they can be attached to an issue — search for how to list labels
(or milestones) first if the user names one that might not exist yet, and
offer to create it rather than letting the attach call fail.

Never hardcode an operationId or assume a specific response field name —
both can differ across the GitHub deployments (`gh`, `ghec`, `ghes`) this
server supports. Always call `get` on whatever operationId `search`
resolves to and read its current schema.

## Composing with other workflows

Pull requests share issues' comment/label/assignee endpoints — see
`github_workflow_pull_request` for the PR-specific flow (branching,
opening, reviewers, checks).
