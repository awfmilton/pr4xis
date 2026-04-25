<!-- DOC_VERSION: 2.1.0 | LAST_UPDATED: 2026-04-24 -->
<!--
CHANGE_LOG (last 10 updates — read only when researching past changes to this document):
  v2.1.0 | 2026-04-24 | (pending commit) | Phase 3.3 reworked: ScheduleWakeup as primary monitoring approach; legacy shell monitor preserved with clear label; version tracking added throughout
  v2.0.0 | 2026-04-24 | 3e24763 | Adopt role-based terminology throughout coordination docs (#170)
  v1.9.0 | 2026-04-24 | 44e7cbf | Automated security gate + smart Jules monitor (#169)
  v1.8.0 | 2026-04-17 | 99b6b9b | Allow Orchestrator fast-path for small tasks without full triage workflow
  v1.7.0 | 2026-04-16 | f30f02a | Sentinel: emit pr:closed on Jules re-label cycle + document in TRIAGE_WORKFLOW
  v1.6.0 | 2026-04-16 | d4c3f79 | WorkflowCanvas added to TriageEditor + Sentinel PR detection fix
  v1.5.0 | 2026-04-16 | ab0a64a | Mandate Jules Watch Monitor for autonomous task completion loop
  v1.4.0 | 2026-04-16 | 01b9f74 | Enforce issue-first Jules delegation in CLAUDE.md and TRIAGE_WORKFLOW
  v1.3.0 | 2026-04-13 | e7ecabb | Node.js v24 spawn validation fix documented
  v1.2.0 | 2026-04-12 | 9920eac | BSL-1.1 license headers and author tags
-->
2# TRIAGE_WORKFLOW.md — Multi-Agent Coordination Protocol
<!-- Author: Alexander Milton / tbay.tk LLC, Helena, Montana | Contact: alex@tbay.tk | https://tbay.tk -->

This document establishes the official orchestration protocol for the **MCP Config Manager** repository. It defines how the **Researcher**, **Implementer**, and **Orchestrator** roles must interact to optimize reasoning quality and credit consumption. For the full syntax of specific commands and flags, agents **must** refer to `AI CLI Reference Guide.md`.

This workflow is a **living document**. Any modifications to the multi-agent loop must be updated here and cross-referenced in the **Architecture** section of `CLAUDE.md` and the **Quick Reference** of `AGENTS.md`.

<!-- SECTION_LAST_UPDATED: 2026-04-24 | COMMIT: 3e24763 | CHANGE: Role-based terminology adopted -->
## Role Assignments

| Role | Current default | Description |
|---|---|---|
| **Orchestrator** | Claude Code | Decomposes tasks, writes GitHub Issues, security review, merges PRs |
| **Researcher** | Gemini CLI | Read-only codebase analysis, internet research, option generation |
| **Implementer** | Jules | Async cloud implementation (via `jules` GitHub label) — branches, code, PRs |

Role assignments can be changed in project settings. All policy language in this document refers to roles, not products. CLI syntax examples use the default tool for that role.

---

<!-- SECTION_LAST_UPDATED: 2026-04-24 | COMMIT: 3e24763 | CHANGE: Role-based terminology adopted -->
## 1. Agent Roles & Capabilities

| Role | Title | Primary Role | Core Strength | CLI Prefix |
|---|---|---|---|---|
| **Researcher** | Researcher | Read-only codebase analysis, architecture exploration, generating options | Massive context window (1M+ tokens) for deep repo analysis | `gemini` |
| **Orchestrator** | The Orchestrator | Decomposes tasks, writes GitHub Issues, security verification, final PR approval | High-reasoning logic, security review, cross-cutting decisions | `claude` |
| **Implementer** | The Implementer | Async cloud implementation (Jules) — creates branches, writes code, opens PRs | High-volume task execution (100 tasks/day) and PR generation | `jules` |

---

<!-- SECTION_LAST_UPDATED: 2026-04-25 | COMMIT: (pending) | CHANGE: Three-tier thresholds (500/3000); Researcher Assessment middle tier; silent-stop fix -->
## 2. Triage Decision Tree

```
New Task Arrives
      │
      ▼
Measure combined diff size (old_string + new_string characters)
      │
      ├── < 500 chars ──→ TIER 1: SMALL-TASK FAST PATH (Section 2.1)
      │                    Hook blocks and shows user two options:
      │                      ├── User approves bypass (A) → Orchestrator implements directly
      │                      └── User declines (B) → continue to full triage below
      │
      ├── 500–3,000 chars → TIER 2: RESEARCHER ASSESSMENT (Section 2.2)
      │                    Hook blocks and shows user two options:
      │                      ├── Option A → Researcher assesses complexity & economics
      │                      │              Researcher recommends: Orchestrator / Implementer
      │                      └── Option B → skip assessment, delegate to Implementer directly
      │
      └── > 3,000 chars ──→ TIER 3: STANDARD TRIAGE
                            Full workflow mandatory — no bypass permitted
                              │
                              ▼
                           Is the approach clear?
                             ├── NO  → DELEGATE RESEARCH to the Researcher
                             │          └── Researcher returns analysis
                             │              └── Orchestrator synthesises → PLAN
                             └── YES → PLAN directly
                                         │
                                         ▼
                                      Is this a bounded, well-specified implementation task?
                                        ├── YES → DELEGATE IMPLEMENTATION to the Implementer
                                        │          1. gh issue create → gh issue edit --add-label jules
                                        │          2. IMMEDIATELY arm Implementer Watch (Section 3.3)
                                        │             Use ScheduleWakeup (§3.3.1) for interactive sessions
                                        │             Use shell monitor (§3.3.2) for unattended/terminal sessions
                                        │          3. Watch detects PR → Orchestrator reads diff
                                        │             ├── Security/quality gates PASS
                                        │             │    └── CI passes → merge, close issue
                                        │             └── Security/quality gates FAIL
                                        │                  └── Post feedback → re-label → Watch resets
                                        │          4. CI fails → wait for CI Fixer self-heal (~2 min)
                                        │             └── Not self-healed → post feedback, re-label, Watch resets
                                        └── NO  → Orchestrator implements directly
```

---

<!-- SECTION_LAST_UPDATED: 2026-04-25 | COMMIT: (pending) | CHANGE: Threshold lowered to 500 chars; three-tier system documented -->
## 2.1. Tier 1 — Small-Task Fast Path (< 500 chars)

The triage hook automatically detects edits under **500 characters** of combined diff (old + new text). These are typically one-liner corrections, typo fixes, or single-property changes where the Researcher → Implementer → PR overhead adds no value.

**Hook behaviour:** The edit is blocked. The `additionalContext` field renders the user_block directly in the UI. The `stopReason` opens with `!!! ACTION REQUIRED !!!` and explicitly prohibits the agent from ending the turn until the user responds.

> *"🔒 Triage Hook — Tier 1 Fast Path: I need your approval before making this edit (small, ~N chars). Option A — implement directly as Orchestrator. Option B — delegate to Implementer. Which would you prefer?"*

**If the user approves (A):**
1. `echo "bypass: <one-line reason>" > .claude/triage.lock`
2. Retry the edit immediately.

**If the user declines (B):** Follow the standard triage path — Researcher research (if needed), GitHub Issue, `jules` label, Implementer Watch Monitor.

**The Orchestrator must never silently self-approve the fast path.** The user prompt is mandatory. The only exception is when the user has *already* explicitly instructed bypass in the same message (e.g., "you can bypass the triage workflow" or "implement this directly") — in that case the bypass lock may be written without re-prompting.

---

<!-- SECTION_LAST_UPDATED: 2026-04-25 | COMMIT: (pending) | CHANGE: New Tier 2 Researcher Assessment middle tier added -->
## 2.2. Tier 2 — Researcher Assessment (500–3,000 chars)

The triage hook detects edits between **500 and 3,000 characters**. These tasks are too large for the Orchestrator fast path but may not warrant the full Implementer workflow — the right choice depends on context, complexity, and token cost. The Researcher's large context window (1M+ tokens) and lower compute cost make it the ideal tool to make this routing decision.

**Hook behaviour:** The edit is blocked. The hook shows the user two options:

> *"🔒 Triage Hook — Tier 2 Researcher Assessment: This edit is medium-sized (~N chars). Before proceeding, the Researcher must perform a Complexity & Economics Assessment to determine the most economical implementation path.*
>
> *Option A — Researcher Assessment first: Run `gemini -p "@<files> Complexity & Economics Assessment: <task>"`, then route based on the recommendation.*
>
> *Option B — Skip assessment, delegate directly to Implementer."*

**If the user chooses Option A (Researcher Assessment):**
1. Run: `gemini -p "@<relevant files> Complexity & Economics Assessment: is this best done by the Orchestrator directly, with Researcher guidance, or via the Implementer via GitHub Issue? Context: <task description>"`
2. Researcher returns a recommendation:
   - **Orchestrator implements directly:** `echo "bypass: researcher-approved-orchestrator" > .claude/triage.lock`, retry the edit.
   - **Implementer via Issue:** `gh issue create` → `gh issue edit <n> --add-label jules` → `echo "jules-issue-<n>" > .claude/triage.lock` → arm Jules Watch Monitor (§3.3).

**If the user chooses Option B (direct delegation):**
Follow the Implementer path: GitHub Issue → `jules` label → Implementer Watch Monitor.

**Economics rationale:** The Researcher costs far fewer high-reasoning tokens than the Orchestrator. For medium-sized tasks, spending a small amount of Researcher context to correctly route the task avoids over-spending Orchestrator tokens on work the Implementer can handle autonomously.

---

<!-- SECTION_LAST_UPDATED: 2026-04-24 | COMMIT: 44e7cbf | CHANGE: Security gate + monitoring approach updated throughout -->
## 3. The Orchestration Workflow

<!-- SECTION_LAST_UPDATED: 2026-04-24 | COMMIT: 3e24763 | CHANGE: Role-based terminology adopted -->
### Phase 1: Deep Research (Researcher)

The Orchestrator identifies the need for a feature or bug fix but offloads the investigation to the Researcher to preserve the Orchestrator's token budget and leverage the Researcher's larger context window.

**When to use:** Unclear approach, large codebase analysis, multi-file architecture questions, security analysis, option generation.

```bash
# Focused analysis with file context
gemini -p "@<file_or_dir> <research_question>"

# Multi-file analysis
gemini -p "@main/index.js @main/preload.js <question>"

# Architecture exploration
gemini -p "@src/ Explain the data flow for <feature>"

# Example from protocol doc
gemini -p "@lib/vault.js @lib/keychain.js Analyze the interaction to identify why mutex locks are failing during concurrent writes."
```

**Output:** A comprehensive report of relevant file paths and logic flow. The Orchestrator uses this to inform the GitHub Issue spec.

---

<!-- SECTION_LAST_UPDATED: 2026-04-24 | COMMIT: 3e24763 | CHANGE: Role-based terminology adopted -->
### Phase 2: Strategic Planning (Orchestrator)

The Orchestrator reviews the Researcher's research, ensures the plan complies with the **Critical Rules** in `AGENTS.md` (e.g., no `keytar`, `asyncHandler` usage, parameterized SQL), and writes a detailed GitHub Issue.

**GitHub Issue must include:**
- Feature description and motivation
- Exact file(s) to create or modify
- Security constraints (from `AGENTS.md` Critical Rules)
- Acceptance criteria (what the PR must include to be merged)
- "Instruction Sets" that the Implementer can parse directly from the issue description
- Any patterns or anti-patterns the Researcher flagged

---

<!-- SECTION_LAST_UPDATED: 2026-04-24 | COMMIT: 3e24763 | CHANGE: Role-based terminology; issue-first delegation enforced -->
### Phase 3: Implementation (Implementer via GitHub Label — PRIMARY PATH)

The current Implementer mechanism is triggered via the `jules` GitHub label (see Section 3).

> **MANDATORY SEQUENCE — do not skip:**
> 1. `gh issue create --repo awfmilton/mcp-manager --title "..." --body "..."` — creates the issue
> 2. `gh issue edit <n> --add-label jules` — triggers the Implementer autonomously
>
> Skipping the issue and using `jules remote new` directly means the Implementer will not auto-publish the branch, CI will not run, and there is no shared channel for discussion. Always create the issue first.

The primary delegation method is the **Implementer label** on GitHub (currently `jules`). When applied to an issue, the Implementer autonomously reads the issue, creates a branch, writes the code, and opens a PR — including the CI Fixer that auto-resolves failing checks. This shifts the entire branch/PR lifecycle to the cloud.

```bash
# STEP 1: Create the GitHub Issue with full spec (acceptance criteria, file paths, constraints)
gh issue create --repo awfmilton/mcp-manager --title "<title>" --body "<full spec>"

# STEP 2: Apply the jules label to trigger the autonomous workflow
gh issue edit <issue_number> --add-label jules

# STEP 3: IMMEDIATELY arm the Implementer Watch (see Section 3.3 below)
# For interactive sessions: ScheduleWakeup(270, "Check Jules for issue #<n>") — §3.3.1
# For unattended/terminal sessions: bash scripts/jules-monitor.sh <issue-number> — §3.3.2
# Do not wait for the user to tell you the Implementer is done. Arm the watch now.
```

**When the label approach produces a PR, skip Phase 3b entirely** — the Implementer has already published the branch and opened the PR. Proceed directly to Phase 4 (Verification).

---

<!-- SECTION_LAST_UPDATED: 2026-04-24 | COMMIT: (pending) | CHANGE: Reworked — ScheduleWakeup primary; legacy shell monitor preserved -->
### Phase 3.3: Implementer Watch (MANDATORY — arm immediately after every jules label)

> **This is not optional.** Every time the Implementer label is applied, the Orchestrator must arm monitoring before doing anything else.

> **Re-label behaviour:** When the Implementer is re-labeled on an issue (feedback loop), it always creates a **new PR** — it does not push to the existing one. Both approaches below handle this: the ScheduleWakeup approach re-scans on each wakeup; the shell monitor continuously polls for the latest open PR.

> **Sentinel author note:** Implementer PRs appear under the repo owner's account, not a bot account. `lib/sentinel.js` detects them by branch naming convention (branches ending with a 16+ digit numeric session ID). Both approaches below use `jules remote list` or issue comment scanning, which are reliable regardless of author.

Two approaches are available. Choose based on session context:

---

#### 3.3.1 ScheduleWakeup Approach ✅ Recommended — token-efficient

Preferred for interactive Orchestrator sessions. Avoids a persistent background process and only reconstitutes Claude's context when there is an actual status check to perform.

**Immediately after `gh issue edit --add-label jules`:**

Call `ScheduleWakeup` with a delay matched to the expected task duration:
- **270 seconds** — stays within the 5-minute prompt-cache TTL; cheapest option for typical tasks
- **1200 seconds** — for tasks known to take 20+ minutes; accepts one cache miss in exchange for fewer wakeups

**On each wakeup, run these checks in order:**

```bash
# 1. Check Jules session status (works for both label-triggered and CLI sessions)
jules remote list --session

# 2. If a PR is now open, find it
gh pr list --repo awfmilton/mcp-manager --state open --json number,title,headRefName

# 3. Check CI on the open PR
gh pr view <pr-number> --repo awfmilton/mcp-manager --json state,statusCheckRollup
```

**Decision logic per wakeup:**

| State | Action |
|---|---|
| Session in progress, no PR yet | `ScheduleWakeup(270s)` and wait |
| PR open, CI pending | Run `bash scripts/gate-check.sh <pr-n> --repo awfmilton/mcp-manager --issue <n>` → `ScheduleWakeup(120s)` for CI |
| Gate failed | Post feedback on PR → re-label issue → `ScheduleWakeup(270s)` to watch for new PR |
| CI passing, gate passed | Merge if non-sensitive; notify user if sensitive (see merge table below) |
| CI failed — first detection | `ScheduleWakeup(120s)` — allow CI Fixer one self-heal attempt |
| CI failed — second wakeup | Post feedback on PR → re-label issue → `ScheduleWakeup(270s)` |
| Session complete, no PR detected | Check issue comments for errors; re-label if needed |

---

#### 3.3.2 Shell Monitor Approach — Legacy / Unattended Sessions

> **When to prefer this:** Unattended terminal sessions (tmux, CI pipelines, overnight runs) where no active Claude context is maintained, or very short conversations where a persistent background loop adds negligible cost. This approach is also the better choice when you need the full automated gate-check → CI → merge loop to run completely autonomously with zero re-prompting.

> **Token note:** This approach keeps a background bash process alive for the entire task duration and fires a Claude `Monitor` notification on every stdout event. In long Orchestrator sessions with large context, each event reconstitutes the full conversation. Prefer §3.3.1 for interactive work.

```bash
# Arm immediately after gh issue edit --add-label jules
# Replace ISSUE_NUM with the actual issue number
bash scripts/jules-monitor.sh <issue-number>
```

**Orchestrator response when monitor fires an event:**

| Event | Action |
|---|---|
| `PR_DETECTED:<n>` | Script automatically triggers `gate-check.sh`. No action needed. |
| `GATE_PASS:<n>` | PR verified against all 11 security/quality gates. Monitor continues to CI phase. |
| `GATE_FAIL:<n>` | Feedback posted on PR, issue re-labeled for Implementer. Monitor resets and watches for next PR. |
| `CI_PASSED:<n>` | CI checks complete. If gates already passed, script attempts `AUTO_MERGED` or `NOTIFY_USER`. |
| `CI_FAILED:<n>` | Script waits 10 minutes for CI Fixer self-healing. |
| `CI_TIMEOUT:<n>` | 10 minutes passed with failing CI. Feedback posted, issue re-labeled. Monitor resets. |
| `AUTO_MERGED:<n>` | PR was non-sensitive and merged automatically. Task complete. |
| `NOTIFY_USER:<n>` | PR touches sensitive files. Manual merge required by Orchestrator. |
| `PR_MERGED:<n>` | Task complete. |
| `PR_CLOSED_RESETTING:<n>` | Implementer replaced this PR with a new one. Monitor resets and follows the new PR. |

---

**When to merge automatically vs. notify the user first (applies to both approaches):**

| Change type | Auto-merge after CI passes? |
|---|---|
| Routine features, refactors, minor copy | Yes — after gate pass and CI pass |
| Database schema changes (`schema.sql`) | No — notify user |
| Security-critical code (`vault.js`, `keychain.js`, `preload.js`, `mcp-server.js`) | No — notify user |
| License-critical code (`license.js`) | No — notify user |
| Any sensitive file change | No — notify user |

*Sensitive file patterns:* `vault.js`, `keychain.js`, `schema.sql`, `license.js`, `preload.js`, `mcp-server.js`.

---

<!-- SECTION_LAST_UPDATED: 2026-04-24 | COMMIT: 3e24763 | CHANGE: Role-based terminology adopted -->
### Phase 3b: Fallback — Manual Branch & PR (Orchestrator)

Use this fallback only when:
- The Implementer label approach is not appropriate (e.g., the task is cross-cutting or requires orchestrator context not expressible in the issue)
- An Implementer label session failed and the Orchestrator is implementing the fix directly

```bash
# Alternative CLI delegation (when not using label approach)
jules remote new --repo awfmilton/mcp-manager --session "<task description referencing the GitHub Issue>"

# Monitor CLI sessions
jules remote list --session

# Inspect diff without applying
jules remote pull --session <id>

# Apply to working directory
jules remote pull --session <id> --apply
```

When using `jules remote new`, the Orchestrator **must** publish the branch and open the PR immediately after applying the diff (Implementer CLI v1.41.0 does not auto-publish). Do not wait for the user to ask:

```bash
git checkout master && git pull origin master
git checkout -b feat/<short-description>-<issue-number>
git add <file1> <file2> ...
git commit -m "feat: <description> (closes #<issue-number>)"
git push -u origin feat/<short-description>-<issue-number>
gh pr create --repo awfmilton/mcp-manager --base master \
  --title "<title>" \
  --body "<body referencing the issue and summarising security verification>"
```

The PR body must include:
- Reference to the originating GitHub Issue (`Implements #<n>`)
- The completed security verification checklist (see Phase 4)
- A test plan checklist

---

<!-- SECTION_LAST_UPDATED: 2026-04-24 | COMMIT: 44e7cbf | CHANGE: Security gate checklist added; automated gate integration -->
### Phase 4: Verification & Iteration (Orchestrator)

**The Orchestrator must read every changed file before approving.** Do not rely solely on the diff summary.

For PRs created via the Implementer label, read the files directly from GitHub or the local working tree after fetching. For PRs created via `jules remote new`, read after applying:

```bash
# For label-triggered PRs: fetch and read
git fetch origin && git checkout <pr-branch>
# Then: Read each modified file at the relevant line ranges

# For CLI sessions: apply then read
jules remote pull --session <id> --apply
# Then: Read each modified file at the relevant line ranges
```

**If issues are found, the Orchestrator either:**
1. Posts specific line-by-line feedback on the PR as a comment and re-labels the issue for the Implementer to fix:
   ```bash
   gh pr comment <pr-number> --body "<specific feedback>"
   # Re-trigger Implementer via label (removes and re-adds):
   gh issue edit <issue-number> --remove-label jules
   gh issue edit <issue-number> --add-label jules
   # OR for CLI fallback:
   jules remote new --repo awfmilton/mcp-manager --session "Fix PR #<n>: <specific issue>"
   ```
2. Applies surgical fixes directly using the Orchestrator's own edit tools, then pushes to the same branch

> **CI Fixer:** When the Implementer is triggered via the label, it includes an automatic CI Fixer (available since February 2026) that detects and fixes failing CI checks on its own PRs. If CI fails on an Implementer label PR, wait for the CI Fixer to attempt a resolution before intervening.

**Security gate (from `AGENTS.md` Critical Rules):**
- [ ] No `keytar` re-introduced
- [ ] No `express.json()` on webhook routes
- [ ] All Express routes use `asyncHandler`
- [ ] All vault writes go through the mutex in `main/index.js`
- [ ] All icons use `lucide-react` (no emoji)
- [ ] All SQL uses parameterized queries
- [ ] No `child_process.exec` with unsanitized user input
- [ ] No shell command injection vectors in any new IPC handler

**Renderer security gate (extra checks for UI components):**
- [ ] No Node.js APIs called directly from renderer
- [ ] All side effects go through `window.api.*` (contextBridge)
- [ ] No `dangerouslySetInnerHTML`
- [ ] User input is validated before being sent via IPC

**Quality gate:**
- [ ] Component follows existing conventions (functional, lucide-react icons, CSS variables)
- [ ] No new npm dependencies introduced without justification
- [ ] PR description references the GitHub Issue

If all gates pass → proceed to Phase 5. Otherwise → post feedback on the PR and loop back to the Implementer or fix directly.

---

<!-- SECTION_LAST_UPDATED: 2026-04-24 | COMMIT: (pending) | CHANGE: Updated CI monitoring reference to ScheduleWakeup approach -->
### Phase 5: Testing & Merging — Event-Driven (Orchestrator)

**For Implementer label PRs:** CI monitoring is handled by the Implementer Watch armed in Phase 3.3. Use `ScheduleWakeup` (§3.3.1) for interactive sessions — it polls `gh pr view --json statusCheckRollup` on each wakeup and merges when CI passes. Use the shell monitor (§3.3.2) for unattended terminal sessions — it fires events autonomously. Do not run `gh pr checks --watch` directly — it is a blocking foreground command that ties up the session.

**For manually published PRs** (Phase 3b fallback): arm a lightweight CI poller after opening the PR:

```bash
# Lightweight one-shot CI poller for manually published PRs
PR_NUM=<n>
REPO=awfmilton/mcp-manager
while true; do
  DATA=$(gh pr view $PR_NUM --repo $REPO \
    --json state,statusCheckRollup \
    --jq '{pending:[.statusCheckRollup[]|select(.status!="COMPLETED")]|length,failed:[.statusCheckRollup[]|select(.conclusion=="FAILURE")]|length,total:.statusCheckRollup|length,passed:[.statusCheckRollup[]|select(.conclusion=="SUCCESS")]|length}' \
    2>/dev/null)
  FAILED=$(echo "$DATA" | jq -r '.failed')
  PENDING=$(echo "$DATA" | jq -r '.pending')
  TOTAL=$(echo "$DATA" | jq -r '.total')
  PASSED=$(echo "$DATA" | jq -r '.passed')
  [ "$FAILED" -gt 0 ] && echo "CI_FAILED:$PR_NUM" && exit 1
  [ "$PENDING" -eq 0 ] && [ "$TOTAL" -gt 0 ] && [ "$PASSED" -eq "$TOTAL" ] \
    && echo "CI_PASSED:$PR_NUM" && exit 0
  sleep 30
done
```

**Merging:**
```bash
gh pr merge <pr-number> --repo awfmilton/mcp-manager --squash --delete-branch
```

**Local testing is for surgical debugging only.** Run `npm test` locally only when:
- A specific CI check failed and you need to reproduce it to understand the root cause
- The CI environment is unavailable (e.g., billing hold, quota exceeded)
- You need to verify a targeted fix before pushing

```bash
# Targeted local reproduction (not a full suite run)
npm test -- --test-name-pattern "<failing test name>"
```

- If CI fails on an Implementer label PR → the Implementer's CI Fixer should self-heal within ~5 min; if not, post feedback and re-label
- Once CI passes → merge immediately per the auto-merge table in Section 3.3; do not wait for the user to re-prompt

---

<!-- SECTION_LAST_UPDATED: 2026-04-24 | COMMIT: 44e7cbf | CHANGE: AI CLI Reference Guide added to mandatory reading -->
## 4. Mandatory Agent Context

Before starting **any task** in this repository, every agent must:

1. **Read `CLAUDE.md`** — project architecture, tech stack, security model, development commands, and coding conventions
2. **Read `AGENTS.md`** — critical safety rules that must never be violated
3. **Refer to `AI CLI Reference Guide.md`** — for all available flags and command syntax (e.g., `--bare`, `--allow-dangerously-skip-permissions`, `jules remote` subcommands)

**Security reminders:**
- Never bypass `lib/keychain.js` or re-introduce deprecated libraries like `keytar`
- Never apply `express.json()` to Stripe webhook routes
- All vault write operations go through the mutex in `main/index.js`

---

<!-- SECTION_LAST_UPDATED: 2026-04-24 | COMMIT: (pending) | CHANGE: CI/monitoring row updated to ScheduleWakeup + jules remote list -->
## 5. Agent Capability Matrix

| Task Type | Researcher | Implementer (label) | Implementer (CLI) | Orchestrator |
|---|---|---|---|---|
| Read/analyse codebase | ✅ Primary | ❌ | ❌ | ✅ Secondary |
| Suggest architectural options | ✅ Primary | ❌ | ❌ | ✅ |
| Write a GitHub Issue spec | ❌ | ❌ | ❌ | ✅ |
| Create branch + write code | ❌ | ✅ **Primary** | ✅ Fallback | ✅ Last resort |
| Auto-publish branch + open PR | ❌ | ✅ Automatic | ❌ Manual | ✅ (fallback) |
| CI Fixer (auto-heal failures) | ❌ | ✅ (Feb 2026+) | ❌ | ❌ |
| Review a PR | ❌ | ❌ | ❌ | ✅ |
| Merge a PR | ❌ | ❌ | ❌ | ✅ (after verification) |
| Security review | ✅ (analysis) | ❌ | ❌ | ✅ (decision) |
| Fix a failing PR | ❌ | ✅ (re-label) | ✅ (re-session) | ✅ (direct edit) |
| Run CI / monitor checks | ❌ | ✅ (cloud) | ❌ | ✅ (`ScheduleWakeup` + `jules remote list` / `gh pr view`) |

Current defaults for roles are: Claude Code (Orchestrator), Gemini CLI (Researcher), and Jules (Implementer).

---

<!-- SECTION_LAST_UPDATED: 2026-04-24 | COMMIT: 3e24763 | CHANGE: Role-based terminology adopted -->
## 6. Reference

- Agent CLI syntax: `AI CLI Reference Guide.md`
- Project conventions and security rules: `CLAUDE.md` and `AGENTS.md`
- Audit history: `Audit Files/04-10-2026/`

---

<!-- SECTION_LAST_UPDATED: 2026-04-16 | COMMIT: f30f02a | CHANGE: pr:closed event added for Jules re-label cycle -->
## 7. Persistent Orchestration (Sentinel Mode)

### 7.1 Role

The **Sentinel** is a long-running background process (`lib/sentinel.js`) that bridges the gap between the Implementer completing a PR and the Orchestrator resuming review. Without it, the Orchestrator only acts when a human re-opens the conversation. With it, Implementer-authored PRs are detected, CI is monitored, and the Orchestrator is notified automatically.

### 7.2 What Sentinel Does

Every 60 seconds, Sentinel:

1. Calls `gh pr list --state open` and filters by the Implementer's branch naming convention (branches ending with a 16+ digit numeric session ID) — **not** by author, because Implementer PRs appear under the repo owner's account
2. For each new PR → emits `pr:detected` (logged to AgentPanel console)
3. For each watched PR → calls `gh pr view <n> --json statusCheckRollup` to check CI
4. If all checks pass → emits `pr:ready` (shows Merge button in AgentPanel)
5. If any check fails → emits `pr:ci-failed` (the Implementer's CI Fixer should self-heal; Sentinel keeps watching)
6. If PR is merged → stops watching it, emits `pr:merged`
7. If PR is closed without merging → emits `pr:closed` (the Implementer re-labeled the issue and opened a new PR; the next tick's step 1 will auto-detect the replacement)

> **Re-label → new PR:** When the Orchestrator posts feedback and re-labels an issue, the Implementer does **not** push to the existing PR. It closes the old PR and opens a **brand-new PR on a new branch**. The Sentinel handles this automatically: `pr:closed` removes the old entry from the AgentPanel watch list, and `_discoverNewPRs()` picks up the new branch on the next poll cycle and emits `pr:detected` for it. No manual intervention is needed — the full monitor loop continues seamlessly across feedback cycles.

### 7.3 Event Types

| Event | Payload | Meaning |
|---|---|---|
| `pr:detected` | `{ pr }` | New Implementer PR found — add to watch list |
| `pr:ready` | `{ pr }` | All CI checks passed — ready to merge |
| `pr:ci-failed` | `{ pr, failedChecks[] }` | One or more checks failed |
| `pr:merged` | `{ pr }` | PR was merged; remove from watch list |
| `pr:closed` | `{ pr }` | PR closed without merge (the Implementer created a replacement); remove from watch list — replacement auto-detected next tick |
| `tick` | `{ watching, ts }` | Heartbeat (not shown in UI) |
| `error` | `{ message, cause }` | Non-fatal poll error |

### 7.4 Orchestrator Response Protocol

When `pr:ready` fires:

```
1. Read every changed file in the PR (Phase 4 security gate)
2. If gates pass → merge via AgentPanel "Merge" button or:
   gh pr merge <n> --squash --delete-branch
3. If gates fail → post feedback and re-label issue for the Implementer
```

When `pr:ci-failed` fires:

```
1. Wait — the Implementer's CI Fixer (Feb 2026+) may self-heal within a few minutes
2. If not self-healed after ~5 min → inspect the failing check logs
3. Apply targeted fix directly or re-trigger the Implementer:
   gh issue edit <issue-n> --remove-label jules
   gh issue edit <issue-n> --add-label jules
```

### 7.5 Starting Sentinel

**In-app (recommended):** Toggle the **Sentinel** switch in the AgentPanel sidebar. The UI streams events to the console and shows pending PR banners.

**Headless (background terminal):**

```bash
# macOS / Linux
tmux new -s mcp-sentinel
node -e "const { Sentinel } = require('./lib/sentinel'); const s = new Sentinel(); s.on('pr:ready', e => console.log('READY', e.pr.number, e.pr.title)); s.on('pr:ci-failed', e => console.log('FAILED', e.pr.number)); s.start();"

# Windows (no tmux) — run in a separate terminal or PowerShell session
node -e "const { Sentinel } = require('./lib/sentinel'); const s = new Sentinel(); s.on('pr:ready', e => console.log('READY', e.pr.number, e.pr.title)); s.on('pr:ci-failed', e => console.log('FAILED', e.pr.number)); s.start();"
```

### 7.6 Capability Matrix Update

| Task | Sentinel | Notes |
|---|---|---|
| Detect Implementer PRs | ✅ | Polls `gh pr list` every 60 s |
| Monitor CI | ✅ | Polls `gh pr view --json statusCheckRollup` |
| Auto-merge | ❌ | Orchestrator must verify before merging |
| Re-trigger Implementer on failure | ❌ | Orchestrator manually re-labels |
| Notify UI | ✅ | Streams events to AgentPanel console + banners |
