# Sub-workflow: GitHub Actions and CI

Covers workflows, workflow runs, artifacts, secrets and variables (repo,
org, and environment scoped), self-hosted runners and runner groups,
hosted compute, and check-runs.

For each task, search for how to do it in natural language — e.g. "how to
dispatch a workflow", "how to create a repository secret", "how to
register a self-hosted runner" — then call the operation `search` resolves
to, and confirm via a follow-up `get`/list call rather than assuming the
write succeeded.

Real gotcha — secrets must exist before the run that references them: if
the user wants to trigger a workflow that reads a secret or variable which
doesn't exist yet, create the secret/variable first (search for how to
create or update a repository/organization/environment secret) and confirm
it's listed before dispatching the workflow — a run that references a
missing secret doesn't fail loudly, the referencing step just sees an empty
value. Similarly, a self-hosted runner must be registered to a runner group
that already exists before it can be added to that group.

If the user wants to inspect a run's logs or a long list of runs/artifacts,
that response can be large — if your environment supports running a
sub-task in an isolated context, delegate fetching and summarizing it, and
bring back only the distilled result (pass/fail, the one log excerpt that
matters) rather than pulling the full log into this conversation.

Never hardcode an operationId or assume a specific response field name —
both can differ across the GitHub deployments (`gh`, `ghec`, `ghes`) this
server supports. Always call `get` on whatever operationId `search`
resolves to and read its current schema.

## Composing with other workflows

Check-run status feeding into a pull request's mergeability is covered by
`github_workflow_pull_request`'s verification step.
