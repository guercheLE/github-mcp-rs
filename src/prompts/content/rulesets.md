# Sub-workflow: Rulesets

Goal: configure a repository, organization, or enterprise ruleset — the
mechanism that supersedes classic branch protection — safely, verifying
what it would actually enforce before it starts blocking anything.

This sub-workflow is self-contained and delegable: if you were routed here
from `github_workflow`, or your environment supports running sub-tasks in
an isolated context, this prompt's own text plus the parameters above is
everything you need — report back only a short summary when done, not the
full step-by-step trace.

Do not skip ahead. Advance to the next numbered step only once the
previous step's stated goal is confirmed met (i.e., you have actually
observed — via `search`/`get`/`call` — that the resource exists or the
setting is in effect, not merely that you issued the call).

## Step 0 — Gather required parameters

Check the "Context already provided" section above first; only ask the
user for whatever is still missing there:

1. **owner_or_org** — the account or organization the ruleset belongs to.
2. **repo** — the repository the ruleset targets, if it's repo-scoped
   (leave unset for an org-wide ruleset).
3. **ruleset_name** — a name for the ruleset (suggest something
   descriptive of its purpose, e.g. `protect-main`, if the user has no
   preference).
4. **target_ref_pattern** — the branch/tag name or pattern the ruleset
   should apply to (e.g. `main`, or a wildcard pattern).

Do not proceed to Step 1 until all four are known.

## Step 1 — Decide scope and target type

Ask the user which scope applies before continuing — do not guess:

- **Repository-scoped**: `repo` is set; the ruleset applies only to that
  repository.
- **Organization-scoped**: `repo` is unset; the ruleset applies across
  every repository in `owner_or_org` that matches the target pattern.
- **Enterprise-scoped**: applies across every repository in an enterprise.
  Search for the enterprise ruleset operations before assuming this is
  available — they only exist on GHEC and GHES deployments, not on `gh`
  (github.com). If `search` finds nothing for "enterprise ruleset", this
  scope isn't available on the configured deployment and you should fall
  back to organization scope.

Also confirm the target type: branch, tag, or push rules apply to
different kinds of refs and have different available rule types — ask if
it isn't obvious from the user's request.

## Step 2 — Decide bypass actors and rules (parallelizable, delegate if possible)

Deciding who can bypass the ruleset (specific teams, apps, or roles) and
deciding which rules it enforces (required status checks, required
reviews, blocking force-pushes, blocking deletions, etc.) are independent
of each other. If your environment supports running sub-tasks in an
isolated context, delegate "look up candidate bypass actors (teams/apps)"
and "look up what rule types are supported" as two separate sub-tasks and
have each return only a short confirmation — don't pull full listings into
this conversation. Otherwise do both directly.

Never hardcode an operationId or assume a specific response field name —
both can differ across the GitHub deployments (`gh`, `ghec`, `ghes`) this
server supports. Always call `get` on whatever operationId `search`
resolves to and read its current schema.

## Step 3 — Create the ruleset in evaluate mode first

Search for how to create a ruleset at the scope decided in Step 1, and
call it with `enforcement` set to `evaluate`, not `active` — `evaluate` is
a dry run that logs what the ruleset *would* block without actually
blocking anything. Never create a ruleset directly in `active` mode on a
repository with live contributors unless the user has explicitly confirmed
they want immediate enforcement with no observation period.

Do not proceed to Step 4 until you've confirmed (search for how to get the
ruleset) that it now exists with the expected target, rules, and bypass
actors.

## Step 4 — Verify evaluate mode against real activity

Search for how to get the ruleset's history or insights, and check that it
is actually capturing real activity against `target_ref_pattern` — not
just that it exists, but that it would be evaluating real pushes/PRs. Give
this some real time to accumulate signal before moving on; don't flip to
`active` immediately after creation with zero observed activity unless the
user explicitly asks to skip the observation period.

## Step 5 — Flip to active enforcement

Once the user confirms the evaluate-mode results look right, search for
how to update the ruleset and set `enforcement` to `active`. Do not tell
the user enforcement is live until you've confirmed (search for how to get
the ruleset again) that `enforcement` now actually reads `active`.

## Composing with other workflows

This supersedes the classic branch-protection flow described in
`github_workflow_repos` — if the user already has classic branch
protection on `target_ref_pattern`, mention that migrating to a ruleset
doesn't automatically remove it, and ask whether they want the old
protection removed once the ruleset is active. Bypass-actor teams are
covered in more depth by `github_workflow_orgs_teams`.
