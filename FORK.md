# Fork-Strategie & Divergenz-Ledger — `pos-ei-don/asic-rs`

Fork von **`256foundation/asic-rs`** (Rust-Lib für ASIC-Management, Apache-2.0). Wir pflegen
VNish/Antminer-Fixes + Features und eine **eigene Fork-Wheel-Pipeline**, damit die HA-Integration
`pos-ei-don/hass-miner` unabhängig baubar bleibt.

> **Lebendes Dokument — bei JEDEM Wheel-Build und JEDER PR-Statusänderung aktualisieren.**
> Liegt nur auf den Fork-internen `live-*`-Branches, **nie** auf `*-upstream`-PR-Branches
> (würde den Upstream-PR verschmutzen). Stand: **2026-06-26.**

## Remotes
| Remote | URL | Rolle |
|---|---|---|
| `origin` | `pos-ei-don/asic-rs` | unser Fork — Wheel-Releases `wheels-vnish-*` |
| `upstream` | `256foundation/asic-rs` | kanonischer Upstream |

## Aktueller Live-/Test-Wheel-Stand
| Wheel-Version | Branch | Basis | Enthält |
|---|---|---|---|
| **0.7.1.1** (Test) | `live-0.7.1.1` | `upstream/master` (= 0.7.1 + #289 throttle/v1_3_0-split + #293 auth) | + presets (rebased #291) + Thermal-`/settings`-Fix |
| 0.7.0.7 (bisher live) | `live-0.7.0.7` | alter 0.7.0-Stack, **nur v1_2_0** | abgelöst durch 0.7.1.1 |

**v1_3_0-Backend** (Upstream, für fw ≥ 1.3.0): nativ throttle/auth/set_power_limit/pools/presets/tuning + Thermal aus `/settings`. **Fehlt noch:** Timezone (#295 — nächster sequenzieller PR).

## Patch-Stack (Lib) + Upstream-Status
| PR/Branch | Inhalt | Upstream-Status |
|---|---|---|
| #277 | VNish-Modell-Alias + per-Board Wassertemps | **merged** |
| #284 | `SetPowerLimit` (preset-basiert) | **merged** (in v1_3_0) |
| #287 | VNish `messages` | **merged** |
| #289 | throttle + **v1_3_0-Backend-Split** | **merged** |
| #293 | custom-credentials/auth | **merged** |
| #291 presets | named autotune/overclock presets | **offen** — rebased Branch `feat-vnish-presets-rebased` (CI grün), Force-Push auf PR-Branch **wartet auf User-OK** |
| #294 firmware-update-check | on-demand FW-Update-Check | **offen** — noch nicht rebased |
| #295 timezone | get/set/list miner timezone | **offen** — noch nicht rebased |
| Thermal-`/settings`-Fix | `restart_temp`/`min_startup` aus authentifizierter `/settings` (1.3.x-Regression) | **fork-only** — zurückgezogener Upstream-PR #299; Re-Submit-Entscheidung **nach Live-Test** |

## Sequenzielle Upstream-Reihenfolge (gewählt)
PRs **einzeln** für Upstream rebasen, wichtigster zuerst, gemergt → nächster gegen dann-aktuellen Master.
Reihenfolge: **#291 presets** (läuft) → #295 timezone → #294 firmware-check. Kein gleichzeitiges Stapeln.

## Fork-Wheel-Pipeline
`.github/workflows/build-wheels-fork.yml` — `workflow_dispatch`, HA-Target `cp314 / musllinux_1_2 / x86_64`,
`maturin --release --interpreter 3.14 --no-default-features --features python,core,proto,antminer,vnish,braiins`.
Wheel als Fork-Release-Asset `wheels-vnish-N` (N = `github.run_number`). Nur `secrets.GITHUB_TOKEN` → autark.

## Identität & Vorgehen (Hartregeln)
- Public-Commits **nur** `pos-ei-don <1822533+pos-ei-don@users.noreply.github.com>` — **nie** IBF-Klarname/-Mail.
- Aktionen an fremden Repos / offenen Upstream-PRs (Force-Push auf `*-upstream`) **nur mit User-OK**.
- Patches **portieren statt neutippen** (cherry-pick/rebase des guten Commits); kritische API-Annahmen gegen das Live-Gerät prüfen.
- Lokal kein Rust-Toolchain → Compile-/Lint-Check ausschließlich über Fork-CI (`test.yml`, `Cargo Assist`).
