# FORK-LOCAL — do not upstream

Everything in this directory belongs to the **`awfmilton/pr4xis` fork only**. It must never
reach `i-am-logger/pr4xis`.

## The rule

- **Never commit anything from this directory to `master`.** Fork-local work lives on
  `claude/*` or other fork branches. `master` is kept clean so it can track upstream and so an
  upstream PR can be cut from it without dragging fork-private material along.
- **Never include this directory in a pull request opened against upstream.** If you branch for
  an upstream contribution, branch from a clean `master`, not from a branch that has touched
  `FORK-LOCAL/`.
- **Do not reference these documents from any file outside this directory.** A link from
  `docs/` or a source comment is how fork-private material leaks into an upstream diff.

`.gitattributes` marks this directory `export-ignore`, so `git archive` tarballs exclude it.
That is a backstop, not the control — branch discipline is the control.

## Why

These documents are the integration strategy for a **separate commercial product**
(kôdex / `awfmilton/mcp-manager`, BSL-1.1, tbay.tk LLC). They are not proposals for pr4xis,
they contain commercial planning that is not the upstream project's business, and they are
authored and owned separately from the pr4xis codebase.

## Licensing note

pr4xis is licensed **CC BY-NC-SA 4.0, © 2026 Ido Samuelson (i-am-logger)**.

The documents in this directory are **separate and independent works** — commercial planning
material authored by Alexander Milton / tbay.tk LLC — collected alongside the licensed work, not
derived from it. They are **not** offered under CC BY-NC-SA 4.0. All rights reserved.

Keeping them in their own directory, unreferenced from the rest of the tree, is part of what
makes that separation legible.

## Contents

| File | What it is |
|---|---|
| `kodex-integration-research-2026-08-10.md` | Research on using pr4xis as the proof plane behind the kôdex agent — role, DigitalOcean shape, and a path to autonomous hardware control demonstrated on a Parrot Rolling Spider |

The mirrored copy that *is* part of the commercial product's plan of record lives at
`docs/specs/roadmap-09-pr4xis-hardware-autonomy.md` in `awfmilton/mcp-manager`.
