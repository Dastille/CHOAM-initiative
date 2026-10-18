# CHOAM

**Codex for Housing, Off-Grid Architecture & Mobility** · paper v2.0

> Shelter is freedom. Homes must be open, modular, and enduring. No patent. No gatekeeper. No nation owns this.
> (Ashlynn, 2025)

CHOAM is an open, copyleft standard for **government-provided** modular homes of 200 to 400 sq ft. Each unit has solar, broadband and opt-in distributed computing, and is offered as a Universal Basic Services floor. The goal is for units to be revenue-positive.

**The paper is the source of truth:** [CHOAM-Idea_Draft.md](CHOAM-Idea_Draft.md). This README summarizes it. Claim statuses are in [CLAIMS.md](CLAIMS.md).

## What the numbers say (from `model/`, asserted by `cargo test`)

- **Solar output.** A 5 kWp array in Toronto yields **5815 kWh/yr** (NRCan), worth **$697.80/yr** in Tier-1 bill offsets (OEB).
- **No cash revenue.** Ontario net metering pays credits, not cash.
- **Energy-only payback.** Energy alone repays a $50k unit in **71.7 yr** and an $80k unit in **114.6 yr**. A 7-year energy-only ROI would need a unit cost of at most **$4884.60**. Revenue-positive therefore depends on avoided public costs or on a cash export tariff. Paper sections 2 and 10 explain what would have to change.
- **Shell materials.** The partial shell materials for 200 sq ft (framing, OSB, R22, drywall; Kent.ca retail, 2026-10-18, pre-tax) come to **$4949.10**. The rest of the unit is not yet derived, so the $50–80k target is unverified.
- **Factory scale.** One factory at 2,000 units/yr meets **0.42%** of CMHC's annual need (477,840 starts/yr).
- **Build speed and cost.** Factory building is 20–50% faster in the literature, with costs up to about 20% lower. The literature does not support 30%.
- **Standards.** The off-site standards are ICC/MBI 1200, 1205 and 1210, and CSA A277 is the Canadian factory-certification route. (The "ICC 1220" in earlier drafts does not exist.)

## Reproduce

```sh
cargo test                       # asserts the paper's table == model output
cargo run -q -p choam-model      # prints the table embedded in the paper
```

## Licences

Repository: AGPL-3.0. Hardware documentation: CERN OHL-S v2. Software may also use GPL-3.0, Ashlynn's v1.0 choice.

## Contribute

PRs that turn a row of [CLAIMS.md](CLAIMS.md) from *unverified* to *verified* are the most useful kind, for example a labour-hour source, a roof/floor/window take-off, or Ontario retail prices.

— Ashlynn / [Dastille](https://github.com/Dastille)
