# Fork-Strategie & Divergenz-Ledger — `pos-ei-don/asic-rs`

Fork von **`256foundation/asic-rs`** (Rust-Lib für ASIC-Management, Apache-2.0). Wir pflegen
VNish/Antminer-Fixes + Features und eine **eigene Fork-Wheel-Pipeline**, damit die HA-Integration
`pos-ei-don/hass-miner` unabhängig baubar bleibt — Voraussetzung für einen möglichen Dauer-Fork.
Schwester-Doku: `FORK.md` in `pos-ei-don/hass-miner` (Integration + Gesamtstrategie/Trigger).

> Stand: 2026-06-19.

## Remotes
| Remote | URL | Rolle |
|---|---|---|
| `origin` | `pos-ei-don/asic-rs` | **unser Fork** — Wheel-Releases `wheels-vnish-*` |
| `upstream` | `256foundation/asic-rs` | kanonischer Upstream |

Re-add: `git remote add upstream https://github.com/256foundation/asic-rs.git`

## Patch-Stack (Lib) + Upstream-Status
| PR/Branch | Inhalt | Status |
|---|---|---|
| #277 | VNish-Modell-Alias (`ANTMINER S19 PRO HYDRO`) + per-Board Wassertemps | **merged upstream** |
| #281 | weak `?` python-Features → Firmware-Gating im Wheel (−20 Crates) | offen |
| #282 | `Duration`→`timedelta` (Uptime-Sensor-Crash) | offen |
| #284 | `SetPowerLimit` für VNish (preset-basiert) | offen |
| #285 | `BoardData.chip_temperature` | **UMSTRITTEN** — Roman will Chip-Temp in `intake`/`outlet` statt neuem Feld (kollidiert mit Hydro-Wasserbelegung aus #277) |
| #287 | VNish `messages` | offen |
| `feat-vnish-safety` (Branch) | Caps + `messages` + Thermal-Limits | **GEHALTEN** bis HACS-Abnahme / Romans Design-OK |

## Fork-Wheel-Pipeline (Build-Autarkie)
`.github/workflows/build-wheels-fork.yml` — `workflow_dispatch`, HA-Target
`cp314 / musllinux_1_2 / x86_64` (HA = Alpine/musl), `maturin --interpreter 3.14`,
`--no-default-features --features python,core,proto,antminer,vnish,braiins`,
`upload-artifact` + `softprops/action-gh-release`. **Nur `secrets.GITHUB_TOKEN`** (Standard-Token),
nur öffentliche Actions → **autark, kein Upstream-CI**. Wheel als Fork-Release-Asset (`wheels-vnish-N`).

## Integrations-Policy — zwei Achsen
- **Unsere Changes → Upstream:** Labels `upstream-candidate` / `upstream-submitted` / `upstream-merged` / `upstream-fork-only` (auf den `ai_mainprojekt`-Tracking-Issues, auf PRs gespiegelt).
- **Upstream-Changes → wir:** Labels `downstream-eval` / `downstream-adopt` / `downstream-skip`; Übernahme via `git cherry-pick <sha>` von `upstream`.

## Trigger für den Voll-Fork
Wenn Upstream (256foundation / Roman) bei **#285** (chip_temperature-Platzierung) bzw. der
**Safety-Form** dauerhaft inkompatibel bleibt → Lib-Fork dauerhaft pflegen, Upstream nur noch
**selektiv per cherry-pick** ziehen, Wheel weiter aus dieser Pipeline. Bis dahin bewusst NICHT
abkoppeln (verfrüht).
