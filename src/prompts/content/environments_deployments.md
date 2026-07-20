# Sub-workflow: Deployment environments, deployments, and Pages

Goal: set up a deployment environment (optionally protected) and deploy to
it, tracking the deployment's actual status rather than assuming success —
plus a shorter secondary flow for GitHub Pages, which shares the
environment concept.

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

1. **owner** — the account or organization the repository belongs to.
2. **repo** — the name of the repository.
3. **environment_name** — the name of the deployment environment (suggest
   `production` if the user has no preference).

Do not proceed to Step 1 until all three are known.

## Step 1 — Decide: protected vs. simple environment

Ask the user whether this environment should be protected before
continuing — do not guess:

- **Protected**: deployments to it require approval from designated
  reviewers, an optional wait timer, and/or a deployment-branch policy
  restricting which branches/tags may deploy to it. Use this for anything
  resembling a production environment.
- **Simple / auto-deploy**: no gating — any workflow run can deploy to it
  immediately once created.

## Step 2 — Create the environment and look up eligible branches (parallelizable, delegate if possible)

Creating `environment_name` and (if it's going to be protected) looking up
which branches/tags exist and should be allowed to deploy to it are
independent of each other. If your environment supports running sub-tasks
in an isolated context, delegate "create the environment" and "list
candidate branches for a deployment-branch policy" as two separate
sub-tasks and have each return only a short confirmation. Otherwise do
both directly.

Never hardcode an operationId or assume a specific response field name —
both can differ across the GitHub deployments (`gh`, `ghec`, `ghes`) this
server supports. Always call `get` on whatever operationId `search`
resolves to and read its current schema.

Do not proceed to Step 3 until you've confirmed (search for how to get an
environment) that `environment_name` now exists on `repo`.

## Step 3 — Attach protection, if protected

If Step 1 was "protected", search for how to add required reviewers,
a wait timer, and/or a deployment-branch policy to the environment, and
call each. Skip this step entirely for a simple environment.

Do not proceed to Step 4 until you've confirmed (search for how to get the
environment's protection rules/branch policies) that what you configured
actually took effect.

## Step 4 — Deploy and track the status lifecycle

Search for how to create a deployment against `environment_name`, then
post its status explicitly — a deployment's state (`queued`,
`in_progress`, `success`, `failure`) is set via separate status calls, not
implied by the create call, so never tell the user a deployment succeeded
without having posted or observed a `success` status.

If the deployment comes from a GitHub Actions workflow run targeting a
protected environment, the run pauses and waits for approval — search for
how to get pending deployments for a run, and if one is waiting, tell the
user it needs review rather than assuming the run will proceed on its own.
Only after an approval is submitted (search for how to review pending
deployments for a run) does the run continue.

## Step 5 — Verify before declaring it live

Do not tell the user the deployment is live until you've confirmed — via
search-and-call, not assumption — the deployment's actual latest status,
and, for a protected environment, that the required approval genuinely
happened.

## GitHub Pages (secondary, shorter flow)

Pages shares the environment concept (deployments to Pages show up as
their own environment) but has its own setup fork:

- **Branch-based**: Pages serves whatever's pushed to a designated
  branch/folder. Search for how to create or update the Pages site with
  `build_type` set accordingly, and confirm the source branch is correct.
- **Actions-based**: a workflow builds the site and uploads it as an
  artifact, then a separate call creates the Pages deployment from that
  artifact. Search for how to create a Pages deployment for this path.

Either way, `create-pages-site`-equivalent setup must exist before deploys
work, and after deploying, search for how to get the Pages health check to
confirm the site is actually serving before telling the user it's ready.

## Composing with other workflows

Workflow-triggered deployments and their approval gates overlap with
`github_workflow_actions_ci`; confirming the branch used for branch-based
Pages overlaps with `github_workflow_repos`.
