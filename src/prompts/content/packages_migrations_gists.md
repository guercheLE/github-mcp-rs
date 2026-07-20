# Sub-workflow: Packages, migrations, and gists

Covers package listing/deletion/restoration, repository/organization
import-export migrations, and gists.

For each task, search for how to do it in natural language — e.g. "how to
list packages for an organization", "how to start a repository export
migration", "how to create a gist" — then call the operation `search`
resolves to, and confirm the result via a follow-up `get`/list call rather
than assuming the write succeeded.

Real gotcha: migrations are asynchronous — starting one returns immediately
with an in-progress migration id, not the finished archive. Search for how
to get a migration's status and poll it (don't assume it's done) before
telling the user the export/import is ready.

Never hardcode an operationId or assume a specific response field name —
both can differ across the GitHub deployments (`gh`, `ghec`, `ghes`) this
server supports. Always call `get` on whatever operationId `search`
resolves to and read its current schema.
