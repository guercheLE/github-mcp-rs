# Sub-workflow: Security suite

Covers code scanning alerts and configurations, secret scanning alerts,
code security configurations, Dependabot alerts and configuration,
security advisories, the dependency graph, and private registries used by
Dependabot.

For each task, search for how to do it in natural language — e.g. "how to
list code scanning alerts", "how to dismiss a secret scanning alert", "how
to update a Dependabot alert" — then call the operation `search` resolves
to, and confirm the result via a follow-up `get`/list call rather than
assuming the write succeeded.

Real gotcha: several of these features (code scanning, secret scanning,
Dependabot) must be *enabled* on the repository before their alert
endpoints return anything meaningful — if a listing comes back empty,
search for how to get the repository's security-and-analysis settings
before concluding there are no alerts.

If the user wants a full sweep across many alerts, that response can be
large — if your environment supports running a sub-task in an isolated
context, delegate fetching and summarizing it, and bring back only the
distilled result (counts by severity, the alerts that need action) rather
than pulling every alert into this conversation.

Real gotcha — code-scanning autofix is asynchronous: search for how to
create an autofix for an alert, then poll for its status (search for how
to get the autofix) until it's no longer pending, and only then commit it
— committing before the fix is ready either errors or commits nothing
meaningful. This operation is only available on `gh` (github.com) and
`ghec`; search first to confirm it exists before assuming a `ghes`
deployment supports it.

Real gotcha — secret scanning push protection: when a push is blocked for
containing a secret, there are two different resolutions, and you should
ask the user which they want rather than assuming: request a bypass
(search for how to create a push-protection bypass, which requires
justification) if the "secret" is a false positive or intentionally
committed, or remove the secret from the commit and push again if it's
real.

Never hardcode an operationId or assume a specific response field name —
both can differ across the GitHub deployments (`gh`, `ghec`, `ghes`) this
server supports. Always call `get` on whatever operationId `search`
resolves to and read its current schema.
