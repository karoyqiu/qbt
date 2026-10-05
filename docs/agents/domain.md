# Domain docs

This repo uses a single-context layout:
- `GLOSSARY.md` at the repo root.
- `docs/adr/` for architectural decision records.

## Before exploring

Read the root glossary and ADRs relevant to the area being explored.

If these files do not exist, proceed silently. Domain modeling creates
them lazily when terms or decisions are resolved.

## Use glossary vocabulary

Use glossary terms in issue titles, proposals, hypotheses, and test names.
Avoid synonyms the glossary explicitly rejects.

If a needed concept is missing, reconsider whether it belongs to the
project's vocabulary; record real gaps for domain modeling.

## Flag ADR conflicts

Explicitly identify any existing ADR that a proposal contradicts,
and explain why the decision should be reopened.
