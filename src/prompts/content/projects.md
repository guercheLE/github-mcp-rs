# Sub-workflow: Projects and campaigns

Covers GitHub Projects (v2) — creation, fields, items — and campaigns.

For each task, search for how to do it in natural language — e.g. "how to
create a project", "how to add an issue to a project", "how to list
campaigns" — then call the operation `search` resolves to, and confirm the
result via a follow-up `get`/list call rather than assuming the write
succeeded.

Real gotcha: adding an issue or pull request to a project and setting one
of its custom field values are usually two separate calls — search for
each step individually rather than assuming a single "add with fields" call
exists.

Never hardcode an operationId or assume a specific response field name —
both can differ across the GitHub deployments (`gh`, `ghec`, `ghes`) this
server supports. Always call `get` on whatever operationId `search`
resolves to and read its current schema.
