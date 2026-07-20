# Sub-workflow: Codespaces, Copilot, and Classroom

Covers Codespaces lifecycle, GitHub Copilot and Copilot Spaces, Copilot
coding agents and agent tasks, and GitHub Classroom.

For each task, search for how to do it in natural language — e.g. "how to
create a codespace", "how to list Copilot seat assignments", "how to start
a Copilot agent task" — then call the operation `search` resolves to, and
confirm the result via a follow-up `get`/list call rather than assuming the
write succeeded.

Real gotcha: Copilot seat management and several codespace operations are
organization-billing-scoped — a call can fail with an authorization error
that looks like a typo'd id when the real cause is that Copilot isn't
enabled for the organization, or the user isn't an org admin. If a call
fails unexpectedly, search for how to get the organization's Copilot
billing settings before assuming the id was wrong.

Never hardcode an operationId or assume a specific response field name —
both can differ across the GitHub deployments (`gh`, `ghec`, `ghes`) this
server supports. Always call `get` on whatever operationId `search`
resolves to and read its current schema.
