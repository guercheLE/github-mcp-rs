# Sub-workflow: Open a pull request

Goal: land a change from a branch onto `base_branch`, handling the
fork-vs-direct-branch decision GitHub's permission model forces, and not
declaring the PR ready until checks and reviews actually confirm it.

This sub-workflow is self-contained and delegable: if you were routed here
from `github-workflow`, or your environment supports running sub-tasks in
an isolated context, this prompt's own text plus the parameters above is
everything you need — report back only a short summary when done, not the
full step-by-step trace.

Do not skip ahead. Advance to the next numbered step only once the previous
step's stated goal is confirmed met (i.e., you have actually observed — via
`search`/`get`/`call` — that the resource exists or the setting is in
effect, not merely that you issued the call).

## Step 0 — Gather required parameters

Check the "Context already provided" section above first; only ask the user
for whatever is still missing there:

1. **owner** — the account or organization the target repository belongs
   to.
2. **repo** — the name of the target repository.
3. **base_branch** — the branch the change should land on (suggest the
   repository's default branch if the user has no preference).
4. **head_branch** — an existing branch carrying the change, or enough
   description of the change that a new branch can be created and
   committed to.

Do not proceed to Step 1 until all four are known.

## Step 1 — Decide: direct branch vs. fork

GitHub only allows pushing a branch straight to `owner/repo` if the user
already has push access to it. This means there are two genuinely different
paths, and you must ask the user which applies before continuing:

- **(A) Direct branch.** The user already has push access to `owner/repo`.
  `head_branch` is created directly in `owner/repo`, committed to, and
  pushed there.
- **(B) Fork-based.** The user has no push access to `owner/repo` (the
  common case when contributing to a repository you don't own). `owner/repo`
  is forked into the user's own account first; `head_branch` is created and
  committed to in the fork, and the PR is opened from the fork back to
  `owner/repo`.

If the user hasn't said which applies, ask: "Do you already have push
access to this repository?" A "yes" means path (A); anything else means
path (B). Do not guess.

## Step 2 — Push the branch and check base-branch requirements (parallelizable, delegate if possible)

Pushing `head_branch`'s commits and looking up `base_branch`'s existing
branch-protection rules (which required status checks and reviews the PR
will need to satisfy before merge) don't depend on each other. If your
environment supports running sub-tasks in an isolated context, delegate
"push the branch" and "look up branch protection requirements for
`base_branch`" as two separate sub-tasks and have each return only a short
confirmation — don't pull the full request/response bodies into this
conversation. Otherwise just do both directly, concurrently rather than
sequentially:

- Push: search for how to create or update a reference (branch), then call
  it with `head_branch`'s commits, in the repository from Step 1 (fork or
  `owner/repo`).
- Requirements: search for how to get branch protection for `base_branch`
  in `owner/repo`, and note which status checks and review counts (if any)
  are required.

Never hardcode an operationId or assume a specific response field name —
both can differ across the GitHub deployments (`gh`, `ghec`, `ghes`) this
server supports. Always call `get` on whatever operationId `search`
resolves to and read its current schema.

Do not proceed to Step 3 until you've confirmed (search for how to get a
branch and call it) that `head_branch` now exists with the expected commits
at its tip.

## Step 3 — Open the pull request

Search for how to create a pull request, then call it with `base_branch`
and, depending on Step 1's fork, either `head_branch` (path A) or
`fork-owner:head_branch` (path B), plus a title and description of the
change.

Do not proceed to Step 4 until you have confirmed the pull request exists
(search for how to get a pull request and check its number and state).

## Step 4 — Request reviewers and add labels/assignees (parallelizable)

These are independent of each other and of nothing else — search for how
to request reviewers on a pull request, and separately for how to add
labels/assignees to an issue (pull requests share the issues endpoints for
this), and call both. Skip whichever the user hasn't asked for.

## Step 5 — Verify before declaring done

Do not tell the user the PR is ready to merge until you've confirmed — via
search-and-call, not assumption — that required status checks (search for
how to list check-runs for a ref) report success and required reviews (per
Step 2's lookup, if any) are satisfied. Summarize what was done and ask
whether to merge, rather than merging unprompted.

## Composing with other workflows

Branch and commit mechanics overlap with `github-workflow-repos`;
check-run status overlaps with `github-workflow-actions-ci`; comments and
labels overlap with `github-workflow-issues`. Fetch those prompts for more
detail on an individual operation rather than guessing.
