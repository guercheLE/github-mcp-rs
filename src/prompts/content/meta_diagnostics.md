# Sub-workflow: Meta and diagnostics

Thin pointer, not a multi-step flow: for read-only utility signals — API
meta information, rate-limit status, code search, emoji lists, `.gitignore`
templates, license templates, code-of-conduct templates, or rendering
markdown to HTML — just search for the task in natural language (e.g. "how
to check the current rate limit", "how to search code", "how to get a
.gitignore template") and call the operation `search` resolves to; there's
no ordering or gating to walk through. As with every other workflow, never
hardcode an operationId or assume a response field name — always confirm
via `get`, since these can differ across the `gh`/`ghec`/`ghes` deployments
this server supports.
