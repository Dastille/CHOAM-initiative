# CLAIMS — CHOAM paper v2.0

Statuses:
- **verified**: checked against the cited primary source on the access date.
- **derived**: computed by `model/` from verified inputs; reproduce with `cargo test`.
- **secondary**: the primary page could not be fetched from the audit box, so the value was read through a search-index copy.
- **unverified**: no source found; kept only as a stated goal.
- **contradicted**: evidence disagrees.
- **deleted**: removed for lack of a source (listed so that nothing is dropped silently).

All accesses were on 2026-10-18 (America/New_York).

| # | Claim | Source (URL + locator) or calculation | Status |
| --- | --- | --- | --- |
| 1 | Canada needs 430k–480k starts/yr to 2035; 477,840 central; BAU 245,000 | CMHC 2025, Figures 12–13 https://www.cmhc-schl.gc.ca/professionals/housing-markets-data-and-research/housing-research/research-reports/accelerate-supply/canadas-housing-supply-shortages-a-new-framework | verified |
| 2 | Adequate housing declared a human right (federal policy) | National Housing Strategy Act s.4 https://laws-lois.justice.gc.ca/eng/acts/N-11.2/section-4.html | verified |
| 3 | UBS = 7 services incl. shelter, free at point of need | UCL IGP 2017, "What are Universal Basic Services?" https://discovery.ucl.ac.uk/id/eprint/10080175/ | verified |
| 4 | Toronto PV potential 1163 kWh/kWp/yr (latitude tilt) | NRCan municip_potentiel-potential.csv, row Ontario/Toronto, Annual | verified |
| 5 | Toronto insolation 4.25 kWh/m²/day (latitude tilt), December 2.21 | NRCan municip_kWh.csv, row Ontario/Toronto | verified |
| 6 | PR 0.7497; 5 kWp → 5815 kWh/yr; 10 kWp → 11630 | `model`: kWp × 4.25 × 365 × (1163/(4.25×365)) (E1–E3) | derived |
| 7 | Ontario RPP Tier 1 = 12.0 ¢/kWh (from 2025-11-01) | OEB electricity rates, "Tiered" table https://oeb.ca/consumer-information-and-protection/electricity-rates | verified |
| 8 | 5 kWp bill offset $697.80/yr | `model` E2b = 5815 × 0.120 | derived |
| 9 | Net metering pays credits, not cash; credits expire after 12 months | OEB net metering page, "How net metering works" + note on s.8(8) O. Reg. 541/05 https://oeb.ca/consumer-information-and-protection/net-metering | verified |
| 10 | Energy-only payback 71.7 yr ($50k) / 114.6 yr ($80k) | `model` E4–E5 | derived |
| 11 | v1.0 "5–7 yr ROI via revenue" | `model` E6–E7: a 5–7 yr energy ROI requires unit cost ≤ $3489.00–$4884.60 | contradicted (energy); unverified (other revenue) |
| 12 | Modular cuts schedules 20–50% | McKinsey 2019, "Potential for profound impact" https://www.mckinsey.com/capabilities/operations/our-insights/modular-construction-from-projects-to-products | verified |
| 13 | Off-site cuts time 40–50%, hard cost up to 20% (3–4 storey wood multifamily) | Terner Center 2017, "Cost Savings" and "Time Savings" https://ternercenter.berkeley.edu/wp-content/uploads/2020/08/offsite_construction.pdf | verified |
| 14 | v1.0 "lower costs by 20–30%" | Items 12–13 support ~20%, not 30% | partly verified |
| 15 | 2D panels ship ~250 km for ~$8/m² vs ~$45/m² for 3D | McKinsey 2019 full report, section on 2D panels https://www.mckinsey.com/~/media/mckinsey/business%20functions/operations/our%20insights/modular%20construction%20from%20projects%20to%20products%20new/modular-construction-from-projects-to-products-full-report-new.pdf | verified |
| 16 | ICC/MBI off-site standards are 1200-2021, 1205-2021, 1210-2023; "ICC 1220" does not exist | https://codes.iccsafe.org/content/ICCMBIOSCS2023P1 (title lists exactly these three) | verified |
| 17 | NBC applies to factory-built buildings; CSA A277 is a certification procedure, not a code | CHBA https://www.chba.ca/code-compliance-for-modular-construction/ (quotes NBC Note A-1.1.1.1.(2)) | verified |
| 18 | 2024 OBC (O. Reg. 163/24) adopts NBC 2020 with amendments, in force 2025-01-01 | https://www.ontario.ca/laws/regulation/r24163 (search-index synthesis of the regulation; regulation page itself not opened) | secondary |
| 19 | NBC 2020 succeeded by NBC 2025 (2025-12-22) | NRC Publications Archive record https://nrc-publications.canada.ca/eng/view/object/?id=515340b5-f4e0-4798-be69-692e4ec423e8 | verified |
| 20 | 2×6 at 600 mm o.c. allowed for exterior walls carrying a roof + ≤1 floor | Table 9.23.10.1 in BC Building Code Part 9 (NBC-derived) https://free.bcpublications.ca/civix/document/id/public/bcbc2006/building_B_P9_9.23 ; NBC 2020 text itself not opened | secondary |
| 21 | Unit prices: 2×6×8 $6.68; 7/16 OSB $16.23; ½ drywall $18.17; R22 37.5 sq ft $152.53 | Kent.ca SKUs 1016278, 1027497, 1016148, 1021667 (URLs in paper refs) | verified |
| 22 | Partial shell take-off 65 / 28 / 24 / 22 units; $4949.10 | `model` B1–B5 (rules in paper §10.1) | derived |
| 23 | Ontario carpenter median $32.00/h | Job Bank NOC 72310 Ontario https://www.jobbank.gc.ca/marketreport/wages-occupation/6388/ON | secondary (direct fetch refused) |
| 24 | Labour hours for a 200 sq ft unit | none found; L1–L3 are sensitivity points | unverified |
| 25 | Ontario softwood lumber GWP A1–A3 = 54.6 kg CO₂e/m³ | CWC EPD 845, Table 7 | verified |
| 26 | Lumber embodied carbon of shell 46.1 kg CO₂e | `model` C1 | derived |
| 27 | One 2,000/yr factory = 0.42% of annual need | `model` S2 | derived |
| 28 | Blockchain can give tamper-evident procurement records but cannot stop off-chain bribery/collusion alone | WEF 2020 report, executive summary and "Key challenges and lessons" https://www3.weforum.org/docs/WEF_Blockchain_Government_Transparency_Report.pdf | verified |
| 29 | BOINC / Folding@home are volunteer computing | https://boinc.berkeley.edu/ ; https://foldingathome.org/ | verified (homepages) |
| 30 | Paid compute-partnership revenue per unit | none found | unverified |
| 31 | Unit cost $50k–80k | partial BOM only | unverified |
| 32 | $500–600/sq ft; $1–2B factories; 16 weeks/unit; $200–500/unit/yr; $1–3B solar; $1B compute; $10–20B savings; 36-inch clearances | none found | deleted (numbers); ideas kept as program lines |
| 33 | v1.2 README BOM $65,200 (unit prices $160/sq ft, $2.70/W etc.) | unsourced, bot-authored | deleted; replaced by item 22 |

## Spot-checks (3 claims re-verified independently of the model)

1. **Item 16, ICC 1220.** The ICC catalogue title is "2025 ICC / MBI Standards for Off-Site Construction 1200-2021, 1205-2021, 1210-2023" (fetched 2026-10-18). There is no 1220. **Pass**: the correction holds.
2. **Item 4 / item 6, PV chain.** `grep "Ontario/Ontario,Toronto,Annual" municip_potentiel-potential.csv` → latitude tilt `1163`. `municip_kWh.csv` → `4.25`. 1163/(4.25·365) = 0.74972, and 5·1163 = 5815, which matches E1 and E2. **Pass.**
3. **Item 9, net metering cash.** The OEB page states that utilities "give you a credit on your electricity bill" and that "credits can only be carried over to future bills for up to 12 months, after which credits are reduced to $0". **Pass**: the cash revenue of $0.00 holds.
