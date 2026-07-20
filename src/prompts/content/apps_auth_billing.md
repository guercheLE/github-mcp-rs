# Sub-workflow: Apps, auth, and billing

Covers GitHub Apps and their installations, OAuth apps, OIDC configuration,
billing/usage, credential authorizations, and API insights (usage
metrics).

For each task, search for how to do it in natural language — e.g. "how to
list an app's installations", "how to get organization billing usage",
"how to revoke an OAuth app authorization" — then call the operation
`search` resolves to, and confirm the result via a follow-up `get`/list
call rather than assuming the write succeeded.

Real gotcha: revoking a credential or an app installation is immediate and
disruptive to whatever was using it — confirm with the user which specific
credential/installation they mean (by app name or installation id, not
just "the app") before calling a revoke or delete operation.

Never hardcode an operationId or assume a specific response field name —
both can differ across the GitHub deployments (`gh`, `ghec`, `ghes`) this
server supports — billing and API-insights shapes in particular vary
between github.com and enterprise deployments. Always call `get` on
whatever operationId `search` resolves to and read its current schema.
