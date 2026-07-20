# Sub-workflow: Users and activity

Covers user profile, SSH/GPG keys, the social graph (following/followers,
blocking), the activity feed, starring/watching, and notifications.

For each task, search for how to do it in natural language — e.g. "how to
get the authenticated user", "how to star a repository", "how to list
notifications" — then call the operation `search` resolves to, and confirm
the result via a follow-up `get`/list call rather than assuming the write
succeeded.

Real gotcha: most of these operations act on "the authenticated user"
implicitly (no `username` parameter) versus a named user explicitly (with
one) — these are genuinely different operations with different ids, so
confirm with the user whether they mean themselves or someone else before
picking which one `search` should look for.

Never hardcode an operationId or assume a specific response field name —
both can differ across the GitHub deployments (`gh`, `ghec`, `ghes`) this
server supports. Always call `get` on whatever operationId `search`
resolves to and read its current schema.
