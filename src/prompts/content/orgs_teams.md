# Sub-workflow: Organizations and teams

Covers organization settings and membership, teams (including nested
teams), enterprise teams and their memberships/organizations, and outside
collaborators.

For each task, search for how to do it in natural language — e.g. "how to
invite a member to an organization", "how to create a team", "how to add a
repository to a team" — then call the operation `search` resolves to, and
confirm the result via a follow-up `get`/list call rather than assuming the
write succeeded.

Real gotcha: adding or removing a member from a team changes their access
to every repository the team has permissions on — before removing someone,
search for how to list a team's repositories (or the member's own
memberships) and confirm with the user this is the change they actually
want, rather than assuming "remove from org" and "remove from team" are
interchangeable.

Never hardcode an operationId or assume a specific response field name —
both can differ across the GitHub deployments (`gh`, `ghec`, `ghes`) this
server supports — enterprise-team endpoints in particular are GHEC/GHES
concepts with no `gh` (github.com) equivalent. Always call `get` on
whatever operationId `search` resolves to and read its current schema.
