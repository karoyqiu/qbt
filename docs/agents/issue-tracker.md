# Issue tracker: GitHub

Issues and specs live in GitHub Issues for `karoyqiu/qbt`.
Use the `gh` CLI from this clone, or pass `--repo karoyqiu/qbt`.

## Conventions

- Create: `gh issue create --title "..." --body-file <path>`
- Read: `gh issue view <number> --comments`; include labels when needed.
- List: `gh issue list --state open --json number,title,body,labels,comments`
  with appropriate label and state filters.
- Comment: `gh issue comment <number> --body-file <path>`
- Apply labels: `gh issue edit <number> --add-label "..."`
- Remove labels: `gh issue edit <number> --remove-label "..."`
- Close: `gh issue close <number> --comment "..."`

For multiline bodies, write the exact text to a temporary UTF-8 file
and pass `--body-file`.

## Pull requests as a triage surface

**PRs as a request surface: no.**

GitHub shares issue and PR numbers. When a reference is ambiguous,
resolve it with `gh pr view <number>`, falling back to
`gh issue view <number>`.

## Skill operations

When a skill says "publish to the issue tracker", create a GitHub issue.
When a skill says "fetch the relevant ticket", run
`gh issue view <number> --comments`.
