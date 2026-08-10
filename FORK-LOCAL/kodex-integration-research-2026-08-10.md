# pr4xis × kôdex — integration research

> **FORK-LOCAL — DO NOT UPSTREAM.** See `FORK-LOCAL/README.md`.
> This document is a separate and independent work, © 2026 Alexander Milton / tbay.tk LLC,
> **not** licensed under CC BY-NC-SA 4.0. It is commercial planning material for
> `awfmilton/mcp-manager`, collected here for the fork's convenience only.

| | |
|---|---|
| **Date** | 2026-08-10 |
| **Status** | Research and architecture. Nothing implemented. |
| **Subjects** | `awfmilton/pr4xis` (this fork) ↔ `awfmilton/mcp-manager` (kôdex, BSL-1.1) |
| **Mirror of record** | `docs/specs/roadmap-09-pr4xis-hardware-autonomy.md` in mcp-manager |
| **Blocking issue** | Licensing — see §6. Resolve before §4 Phase 2. |

---

## 1. The thesis

> **pr4xis is the proof plane. The kôdex agent generates; pr4xis proves or names the failing
> axiom; a separate Rust edge agent executes. pr4xis is never inside the flight loop — it
> compiles the flight envelope offline, and the loop enforces it.**

This is the generate-and-check pattern the neuro-symbolic literature has converged on. What
makes it cheap here is that mcp-manager already ships three of the four pieces: an orchestrator
with a bounded fix loop (`lib/agent-orchestrator.js`), a downstream MCP gateway contract
(`lib/mcp-tools-server.js`), and a skill library with a hard verification gate
(`lib/skill-verifier.js`). pr4xis supplies the fourth — a checker whose failure output is a
**named axiom** rather than a stack trace.

The whole economic argument rests on one property of this codebase: `Verdict` is a `Proof` or a
`Counterexample` that says *what broke*. Tests report that something broke. That difference is
what a bounded LLM fix loop can actually consume.

---

## 2. What pr4xis would need to grow

Nothing in this section is a proposal for upstream. It is what the **fork** would build, and it
is listed here so the shape is on record before anyone starts.

### 2.1 A new crate: `crates/mcpd`

An HTTP server implementing mcp-manager's existing downstream gateway contract — `GET /tools`,
`GET /tools/schema/:name`, `POST /invoke` — with bearer auth and a `--host` bind. No new
protocol is invented; it mirrors the shape mcp-manager already speaks.

Tools, each returning the `Verdict` with its derivation path intact:

| Tool | Backed by |
|---|---|
| `ontology_list`, `ontology_query` | `pr4xis::ontology`, the structural-axiom catalog; `is_a` already returns a typed `Verdict` |
| `check_axioms` | `Ontology::axioms()` |
| `check_functor_laws` | `pr4xis::category::laws::assert_functor_laws` |
| `gap_analysis` | adjunction round-trip; collapsed entities are the output |
| `load_prx` | `crates/pr4xis-runtime/src/load.rs` — fail-closed root check |
| `verify_plan` | `pr4xis::engine` preconditions over a proposed action sequence |

**Design rule: return the `Counterexample` verbatim.** The named axiom is the product. Anything
that flattens it to a boolean or an error string destroys the reason for the integration.

### 2.2 New domain ontologies under `applied/`

- `device/capability` — W3C WoT Thing Description 2.0 affordances: Property, Action, Event.
- `device/vehicle`, `device/actuator`.

Functors into the existing stack: `Capability → Sensor` (`applied/sensor_fusion/sensor`),
`Capability → Driver` (`applied/operating_system/driver`), `Vehicle → Kinematics`
(`natural/physics/kinematics`), `Vehicle → AHRS` / `Odometry` (`applied/navigation/*`).

Importers: ARSDK minidrone XML → `DeviceDescription`; MAVLink `COMPONENT_METADATA` →
`DeviceDescription`; frozen to `.prx` and admitted through the existing gate.

### 2.3 The deferred adjunction — the single most valuable item

`crates/domains/src/applied/sensor_fusion/sensor/driver_functor.rs` maps the entire Groves (2013)
taxonomy — proprioceptive and exteroceptive, active and passive, `Camera`, `Sonar`, `IMU`,
`AHRS`, `INS` — onto a single `DriverConcept::Device`. It is a constant functor, and its own
doc comment says:

> **Deferred follow-up**: the sensor/driver *adjunction* that would expose this collapse gap
> (which sensor distinctions the driver layer cannot represent, and what a right adjoint would
> have to reconstruct) is not built here.

**Build it, and its gap analysis becomes a machine-generated answer to "what does this device
fail to tell me about itself?"** Pointed at a Parrot Rolling Spider it should report, unprompted,
that the driver surface carries a velocity channel but no camera affordance and no range
affordance — the `Camera` and `Sonar` distinctions collapsed. That is exactly true of the
hardware (§5), and it is *derived* rather than asserted.

This one result is the strongest available evidence that the pairing is worth building at all.

`applied/operating_system/driver/ontology.rs` already cites **Ryzhyk et al. (2009), Termite —
Automatic Device Driver Synthesis** ("the driver as a derivable artifact: synthesized from a
formal device model plus an OS interface specification"). That is a precise statement of the
long-term goal. Earn it by proving the device model first; synthesis is a stretch item, not
baseline.

---

## 3. Where pr4xis runs (DigitalOcean)

mcp-manager has a live nyc3 footprint at roughly $17/mo — two VPCs, an ingress droplet running
Caddy with a wildcard cert plus `kodex-router` / `kodex-relay` / `kodex-maintenance` /
`mcp-tools-gateway`, two private Spaces buckets, and an ephemeral-droplet CI leg. pr4xis needs
**two additions**, not new infrastructure.

**A — `pr4xis-verify-1`.** A `c-4` droplet (4 dedicated vCPU / 8 GB) in the `kodex-core` VPC
running `pr4xis-mcpd`, private IP only, registered as a bearer-auth downstream. ~$84/mo, later
droppable to near zero by registering it as a kôdex tenant so the platform's lifecycle machinery
wakes it on request and suspends it on idle.

Spaces is an unusually clean `.prx` store precisely because of this codebase's discipline:
`praxis.lock` pins `blake3:` digests and `load.rs` re-derives the Merkle root from the bytes
before admitting anything. The bucket is never trusted, so it can be dumb.

**B — a Rust CI runner snapshot.** mcp-manager's current runner is Ubuntu 22.04 + Node 20 on
`s-1vcpu-512mb-10gb`; it cannot build this workspace. Its provisioner already parameterizes
size, so this is a bake plus configuration. Bake and boot at `c-8` (8 vCPU / 16 GB / 100 GB) —
DigitalOcean refuses to create a droplet smaller than the snapshot's minimum disk, so the bake
size and the boot size must match. Point `sccache` at a Spaces prefix.

**Respect the workspace's own exclusions.** `crates/wasm`, `crates/e2e`, and
`crates/praxis-corpus-tests` are excluded from the default workspace run for stated reasons
(wasm target; WebDriver + running server; multi-hundred-MB corpora re-parsed per process). A
runner that ignores that blows both the disk and the clock.

---

## 4. Delivery phases

| Phase | What | Done when |
|---|---|---|
| **1** | `crates/mcpd` + the two DO additions | The kôdex Verify phase calls a pr4xis tool and an Implementer fix loop is driven by a named axiom |
| **2** | Device ontologies + the Sensor ⊣ Driver adjunction + importers | `gap_analysis` on a Rolling Spider description independently reports the camera/sonar collapse |
| **3** | `kodex-edge` (separate repo, Rust): `Link`/`Protocol` traits, BLE + ARSDK, trajectory recorder | Commanded takeoff → hover → land, telemetry decoded, trajectory recorded. Tethered, indoors, prop guards |
| **4** | Closed-loop autonomy; pr4xis proves the envelope offline and emits a Rust monitor the loop enforces | An unattended flight completes a pattern, the monitor demonstrably aborts an out-of-envelope command |
| **5** | Second embodiment: MAVLink over UDP against PX4 SITL | A skill learned on a BLE toy flies a simulated MAVLink quad with no skill-side changes |

**Phase 1 is safe to prototype now** — a separate process over HTTP, adapting nothing.
**Phase 2 is the licensing gate** (§6): it is where fork-authored ontologies start living inside
a CC BY-NC-SA tree.

---

## 5. Hardware reality check — the Rolling Spider

**The Rolling Spider does not expose its camera over Bluetooth.** This is load-bearing for the
demo design, so it is recorded plainly:

- The downward camera runs at 60 fps purely as an **optical-flow velocity sensor**, consumed
  onboard.
- `take_picture()` writes ~0.3 MP JPEGs to internal memory. Retrieval is Mambo-only in pyparrot,
  and Parrot broke groundcam access in firmware ≥ 3.0.25.
- `altitude` and `quaternion_*` are documented **WiFi-only**. The Rolling Spider has no WiFi.
- What BLE reliably yields: `speed_x/y/z`, `flying_state`, `battery`, alerts — command service
  `fa00` (`fa0a` flight params, `fa0b` command, `fa0c` emergency), notification service `fb00`
  (`fb0e` status, `fb0f` battery).
- The link drops if the host goes quiet; pyparrot's `smart_sleep()` exists for exactly this.

**Why this is fine.** You *are* navigating by camera and sonar — through the drone's own onboard
fusion, as derived velocity and altitude hold, rather than as pixels. Treat those as first-class
sensor channels and add an off-board camera for true vision.

**And why it is actually the point.** That the device's real capabilities are undocumented and
partly broken is not an obstacle to the demonstration. Capability discovery *is* the
demonstration, and §2.3 is the mechanism.

For generalization, do not invent a device-description format. Both
[W3C WoT Thing Description 2.0](https://www.w3.org/TR/wot-thing-description-2.0/) (Nov 2025) and
the [MAVLink Component Metadata Protocol](https://mavlink.io/en/services/component_metadata.html)
exist so a client can drive a system it knows nothing about. Normalize onto WoT TD 2.0; treat
ARSDK and MAVLink as importers.

---

## 6. Licensing — the blocker

pr4xis is **CC BY-NC-SA 4.0, © 2026 Ido Samuelson (i-am-logger)** (`CITATION.cff`; all upstream
commits are his). `awfmilton/pr4xis` is a fork, not an origin. mcp-manager is **BSL-1.1**,
commercial, tbay.tk LLC.

> **Correction of record:** `docs/pr4xis-integration-recommendations.md` in mcp-manager
> (2026-07-10) states that both projects are authored by Alexander Milton and that the licensing
> question is therefore "administratively resolvable." **That is incorrect.** pr4xis is authored
> by a third party. The grant has to be negotiated, not self-issued.

Three separate walls:

1. **NonCommercial** prohibits exercising the licensed rights — reproduce, share, adapt — for
   commercial purposes. Bundling pr4xis into a product that charges for commercial use is
   exercising *Share* for commercial advantage.
2. **NC binds downstream recipients too.** Making pr4xis an optional component the user installs
   themselves does not rescue the commercial tier; it relocates the violation onto the customer.
3. **ShareAlike, and this is the one that matters most here.** Adaptations must ship under
   BY-NC-SA, which is incompatible with BSL-1.1. Whether ontologies written with
   `pr4xis::ontology!` and impl'ing pr4xis traits constitute an "adaptation" is genuinely
   unsettled — CC 4.0's definition was written for prose and images, and Creative Commons
   themselves recommend against using CC licenses for software. So **the device ontology of
   §2.2–2.3, arguably the most valuable artifact in this plan, may be ShareAlike-encumbered by
   construction, and the license text cannot tell you.**

**The action:** request a commercial/dual-license grant from Ido Samuelson. The upstream README
explicitly solicits "Partner on a safety-critical deployment in aerospace, biomedical,
industrial, or legal" — aerospace autonomy with a formal verification gate is close to the
flagship case being advertised for. This is a conversation, not a legal quagmire, and it gates
Phase 2 rather than Phase 1.

**If declined:** the mathematics is not his — category theory, functor laws, adjunction-based gap
detection, and the Groves (2013) taxonomy are public literature and not copyrightable. A
clean-room implementation of the *concepts* is lawful. What is lost is the corpus — 160+ curated
ontologies and 6,685 tagged tests — which is a multi-year rebuild. Treat it as a genuine
fallback, not a negotiating posture.

---

## 7. Assumptions

Raised with the owner and unanswered; the plan proceeds on them.

1. **Perception** — derived optical-flow velocity and sonar altitude as the onboard channels,
   plus an off-board camera for vision. Not a firmware-downgrade dive for groundcam frames.
2. **Edge node** — dev laptop first, Raspberry Pi later. BLE needs a radio within ~10 m, so
   **no cloud host can talk to the drone directly**; the edge node is structural.
3. **pr4xis role** — verifier and device model. Driver synthesis is a stretch item.
4. **Safety** — a flying object under LLM-adjacent control needs its kill path (`fa0c`) proven
   before autonomy, not after. Hardware-in-the-loop only tethered, indoors, prop guards on.

---

## 8. Sources

- [pyparrot minidrone commands and sensors](https://pyparrot.readthedocs.io/en/latest/minidronecommands.html) ·
  [node-rolling-spider](https://github.com/voodootikigod/node-rolling-spider) ·
  [gobot minidrone](https://gobot.io/documentation/platforms/minidrone/) ·
  [Parrot ARSDK protocols](https://developer.parrot.com/docs/SDK3/ARSDK_Protocols.pdf)
- [W3C WoT Thing Description 2.0](https://www.w3.org/TR/wot-thing-description-2.0/) ·
  [MAVLink Component Metadata](https://mavlink.io/en/services/component_metadata.html)
- [btleplug](https://github.com/deviceplug/btleplug) · [bluest](https://github.com/alexmoon/bluest)
- [Towards Reliable Code-as-Policies](https://arxiv.org/pdf/2510.21302) ·
  [VASO: Formally Verifiable Self-Evolving Skills](https://arxiv.org/pdf/2606.05395) ·
  [Formal-Method-Guided Vibe Coding](https://arxiv.org/pdf/2606.22413)
- [Creative Commons FAQ — CC licenses and software](https://creativecommons.org/faq/)
