//! # choam-model
//!
//! First-principles calculations behind `CHOAM-Idea_Draft.md` (paper v2.0).
//!
//! Every input constant carries its source in its doc comment. Every derived
//! number printed in the paper is produced by [`render_paper_block`]; the test
//! suite asserts that the block inside the paper is byte-identical to the
//! output of this crate, so the paper cannot drift from the model.
//!
//! ```
//! let e = choam_model::pv_energy(5.0).unwrap();
//! assert!((e.annual_kwh - 5815.0).abs() < 1e-9);
//! ```
#![forbid(unsafe_code)]
#![deny(missing_docs)]

use std::fmt;

// ---------------------------------------------------------------------------
// Sourced inputs
// ---------------------------------------------------------------------------

/// NRCan "Photovoltaic potential and solar resource maps of Canada",
/// municipal table `municip_potentiel-potential.csv`, row Ontario/Toronto,
/// Annual, column "South-facing with latitude tilt": 1163 kWh per kWp per year.
/// <https://ftp.maps.canada.ca/pub/nrcan_rncan/Solar-energy_Energie-solaire/photovoltaic_canada_photovoltaique/municip_potentiel-potential.csv>
pub const TORONTO_PV_KWH_PER_KWP_YR: f64 = 1163.0;

/// Same NRCan dataset, `municip_kWh.csv`, Ontario/Toronto, Annual,
/// latitude tilt: mean daily global insolation 4.25 kWh/m2/day
/// (equivalently 4.25 peak-sun-hours per day).
pub const TORONTO_PSH_LAT_TILT: f64 = 4.25;

/// OEB Regulated Price Plan, Tiered, Tier 1 price effective 2025-11-01:
/// 12.0 cents/kWh (commodity line only; excludes delivery, regulatory, HST).
/// <https://oeb.ca/consumer-information-and-protection/electricity-rates>
pub const ONTARIO_RPP_TIER1_CAD_PER_KWH: f64 = 0.120;

/// Ontario net metering (O. Reg. 541/05) pays on-bill credits, not cash;
/// credits older than 12 months are reduced to $0 (s. 8(8)), per
/// <https://oeb.ca/consumer-information-and-protection/net-metering>.
/// Cash revenue per unit from exported kWh is therefore 0.
pub const ONTARIO_NET_METERING_CASH_CAD: f64 = 0.0;

/// Kent.ca, "2 x 6 x 8' SPF Stud Kiln Dried", SKU 1016278: CA$6.68 (accessed 2026-10-18).
pub const PRICE_STUD_2X6X8_CAD: f64 = 6.68;
/// Kent.ca, "7/16\" x 4' x 8' OSB", SKU 1027497 (APA PS2): CA$16.23 (accessed 2026-10-18).
pub const PRICE_OSB_7_16_CAD: f64 = 16.23;
/// Kent.ca, "Drywall 1/2\" x 4' x 8' Ultra Light Panel", SKU 1016148: CA$18.17 (accessed 2026-10-18).
pub const PRICE_DRYWALL_1_2_CAD: f64 = 18.17;
/// Kent.ca, "ComfortBatt Woodstd R22 5.5\" x 23\" x 47\" - 37.5 SqFt", SKU 1021667: CA$152.53 (accessed 2026-10-18).
pub const PRICE_R22_BAG_CAD: f64 = 152.53;
/// Coverage of one R22 23-inch bag (Kent.ca SKU 1021667 title): 37.5 sq ft.
pub const R22_BAG_SQFT: f64 = 37.5;
/// Coverage of one 4 ft x 8 ft panel: 32 sq ft (geometry; Kent.ca SKU 1027497 "Covers 32 square feet").
pub const PANEL_SQFT: f64 = 32.0;

/// Job Bank, Carpenter (NOC 72310), Ontario, median hourly wage CA$32.00
/// (wage data updated 2025-11-19). Retrieved from the search index on
/// 2026-10-18; direct fetch of jobbank.gc.ca was refused from the audit box.
/// <https://www.jobbank.gc.ca/marketreport/wages-occupation/6388/ON>
pub const ONTARIO_CARPENTER_MEDIAN_CAD_PER_H: f64 = 32.00;

/// Canadian Wood Council, EPD 845 (2025-02-04), Table 7, softwood lumber
/// produced in Ontario, GWP total A1-A3: 54.6 kg CO2e per m3.
/// <https://cwc.ca/publication/a-regionalized-industry-average-epd-for-canadian-softwood-lumber/>
pub const ONTARIO_LUMBER_GWP_A1_A3_KG_PER_M3: f64 = 54.6;

/// CMHC (2025-06-19) "Canada's Housing Supply Shortages: Moving to a New
/// Framework", Figure 13: 477,840 starts/year needed to 2035 (Canada).
/// <https://www.cmhc-schl.gc.ca/professionals/housing-markets-data-and-research/housing-research/research-reports/accelerate-supply/canadas-housing-supply-shortages-a-new-framework>
pub const CMHC_STARTS_NEEDED_PER_YR: f64 = 477_840.0;
/// CMHC, same Figure 13: business-as-usual 245,000 starts/year.
pub const CMHC_STARTS_BAU_PER_YR: f64 = 245_000.0;

/// 2x6 actual dressed size 1.5 in x 5.5 in (Home Depot Canada product
/// 1001802970 lists width 5.5 in, height 1.5 in), length 96 in.
pub const STUD_VOLUME_IN3: f64 = 1.5 * 5.5 * 96.0;
/// Exact conversion: 1 in = 0.0254 m.
pub const M3_PER_IN3: f64 = 0.0254 * 0.0254 * 0.0254;

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

/// Input validation error for every fallible calculation in this crate.
#[derive(Debug, Clone, PartialEq)]
pub enum ModelError {
    /// A parameter was negative, zero where it must be positive, NaN or infinite.
    InvalidInput {
        /// Parameter name.
        name: &'static str,
        /// The rejected value.
        value: f64,
    },
}

impl fmt::Display for ModelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ModelError::InvalidInput { name, value } => {
                write!(
                    f,
                    "invalid input `{name}` = {value}: must be finite and > 0"
                )
            }
        }
    }
}

impl std::error::Error for ModelError {}

fn positive(name: &'static str, value: f64) -> Result<f64, ModelError> {
    if value.is_finite() && value > 0.0 {
        Ok(value)
    } else {
        Err(ModelError::InvalidInput { name, value })
    }
}

// ---------------------------------------------------------------------------
// Energy
// ---------------------------------------------------------------------------

/// Result of the PV chain `kWp x PSH x 365 x PR`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PvEnergy {
    /// Array size in kWp.
    pub kwp: f64,
    /// Performance ratio implied by NRCan's own two tables (PV potential / insolation).
    pub performance_ratio: f64,
    /// Annual energy, kWh.
    pub annual_kwh: f64,
    /// Mean daily energy, kWh/day.
    pub daily_kwh: f64,
    /// Bill value if every kWh offsets Tier-1 commodity charges, CAD/yr.
    pub bill_offset_cad_per_yr: f64,
    /// Cash revenue under Ontario net metering, CAD/yr (always 0, by regulation).
    pub cash_revenue_cad_per_yr: f64,
}

/// Performance ratio implied by NRCan: `1163 / (4.25 x 365)`.
#[must_use]
pub fn implied_performance_ratio() -> f64 {
    TORONTO_PV_KWH_PER_KWP_YR / (TORONTO_PSH_LAT_TILT * 365.0)
}

/// PV energy for a Toronto latitude-tilt array of `kwp` kWp.
///
/// # Errors
/// [`ModelError::InvalidInput`] if `kwp` is not finite and positive.
pub fn pv_energy(kwp: f64) -> Result<PvEnergy, ModelError> {
    let kwp = positive("kwp", kwp)?;
    let pr = implied_performance_ratio();
    let annual_kwh = kwp * TORONTO_PSH_LAT_TILT * 365.0 * pr;
    Ok(PvEnergy {
        kwp,
        performance_ratio: pr,
        annual_kwh,
        daily_kwh: annual_kwh / 365.0,
        bill_offset_cad_per_yr: annual_kwh * ONTARIO_RPP_TIER1_CAD_PER_KWH,
        cash_revenue_cad_per_yr: ONTARIO_NET_METERING_CASH_CAD * annual_kwh,
    })
}

/// Years to recover `capital_cad` from an annual saving of `annual_cad`.
///
/// # Errors
/// [`ModelError::InvalidInput`] if either input is not finite and positive.
pub fn payback_years(capital_cad: f64, annual_cad: f64) -> Result<f64, ModelError> {
    Ok(positive("capital_cad", capital_cad)? / positive("annual_cad", annual_cad)?)
}

// ---------------------------------------------------------------------------
// Partial shell bill of materials
// ---------------------------------------------------------------------------

/// Rectangular single-storey unit used for the partial shell take-off.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Footprint {
    /// Length, ft.
    pub length_ft: f64,
    /// Width, ft.
    pub width_ft: f64,
    /// Wall height, ft.
    pub wall_height_ft: f64,
    /// Stud spacing, inches on centre.
    pub stud_spacing_in: f64,
}

/// The 200 sq ft reference unit: 20 ft x 10 ft, 8 ft walls, 2x6 at 24 in o.c.
pub const REFERENCE_UNIT: Footprint = Footprint {
    length_ft: 20.0,
    width_ft: 10.0,
    wall_height_ft: 8.0,
    stud_spacing_in: 24.0,
};

/// One BOM line.
#[derive(Debug, Clone, PartialEq)]
pub struct BomLine {
    /// Item name.
    pub item: &'static str,
    /// Count of purchase units.
    pub qty: u32,
    /// Unit price, CAD (pre-tax).
    pub unit_cad: f64,
}

impl BomLine {
    /// Extended cost, CAD.
    #[must_use]
    pub fn total(&self) -> f64 {
        f64::from(self.qty) * self.unit_cad
    }
}

/// Partial shell take-off (studs, plates, OSB, R22 batts, drywall).
#[derive(Debug, Clone, PartialEq)]
pub struct ShellBom {
    /// Lines.
    pub lines: Vec<BomLine>,
    /// Floor area, sq ft.
    pub floor_sqft: f64,
    /// Gross exterior wall area, sq ft.
    pub wall_sqft: f64,
    /// Number of 2x6x8 pieces.
    pub stud_pieces: u32,
}

fn ceil_u32(x: f64) -> u32 {
    // Inputs are validated finite and positive and the reference unit is small,
    // so the value fits in u32.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let n = x.ceil() as u32;
    n
}

/// Take-off rules (all stated in the paper, section 10.1):
/// studs per wall = ceil(L x 12 / spacing) + 1; +2 per corner (4 corners);
/// plates = 3 x perimeter (double top, single bottom) cut from 8 ft pieces;
/// OSB covers walls + roof deck + subfloor; R22 covers walls + roof + floor;
/// drywall covers interior of walls + ceiling. No openings are deducted.
///
/// # Errors
/// [`ModelError::InvalidInput`] if any dimension is not finite and positive.
pub fn shell_bom(fp: Footprint) -> Result<ShellBom, ModelError> {
    let l = positive("length_ft", fp.length_ft)?;
    let w = positive("width_ft", fp.width_ft)?;
    let h = positive("wall_height_ft", fp.wall_height_ft)?;
    let s = positive("stud_spacing_in", fp.stud_spacing_in)?;
    let per_wall = |len: f64| ceil_u32(len * 12.0 / s) + 1;
    let field_studs = 2 * per_wall(l) + 2 * per_wall(w);
    let corner_studs = 2 * 4;
    let perimeter = 2.0 * (l + w);
    let plate_pieces = ceil_u32(3.0 * perimeter / 8.0);
    let stud_pieces = field_studs + corner_studs + plate_pieces;
    let floor = l * w;
    let wall = perimeter * h;
    let osb = ceil_u32((wall + floor + floor) / PANEL_SQFT);
    let batts = ceil_u32((wall + floor + floor) / R22_BAG_SQFT);
    let drywall = ceil_u32((wall + floor) / PANEL_SQFT);
    Ok(ShellBom {
        lines: vec![
            BomLine {
                item: "2x6x8 SPF (studs + plates)",
                qty: stud_pieces,
                unit_cad: PRICE_STUD_2X6X8_CAD,
            },
            BomLine {
                item: "7/16 OSB 4x8 (walls, roof deck, subfloor)",
                qty: osb,
                unit_cad: PRICE_OSB_7_16_CAD,
            },
            BomLine {
                item: "R22 ComfortBatt 37.5 sq ft bag",
                qty: batts,
                unit_cad: PRICE_R22_BAG_CAD,
            },
            BomLine {
                item: "1/2 drywall 4x8 (walls + ceiling)",
                qty: drywall,
                unit_cad: PRICE_DRYWALL_1_2_CAD,
            },
        ],
        floor_sqft: floor,
        wall_sqft: wall,
        stud_pieces,
    })
}

impl ShellBom {
    /// Material total, CAD pre-tax.
    #[must_use]
    pub fn material_cad(&self) -> f64 {
        self.lines.iter().map(BomLine::total).sum()
    }

    /// Fossil A1-A3 carbon of the lumber only, kg CO2e (CWC EPD 845, Ontario).
    #[must_use]
    pub fn lumber_gwp_kg(&self) -> f64 {
        f64::from(self.stud_pieces)
            * STUD_VOLUME_IN3
            * M3_PER_IN3
            * ONTARIO_LUMBER_GWP_A1_A3_KG_PER_M3
    }
}

/// Labour cost for `hours` at the Ontario carpenter median wage.
///
/// # Errors
/// [`ModelError::InvalidInput`] if `hours` is not finite and positive.
pub fn labour_cad(hours: f64) -> Result<f64, ModelError> {
    Ok(positive("hours", hours)? * ONTARIO_CARPENTER_MEDIAN_CAD_PER_H)
}

// ---------------------------------------------------------------------------
// Rendering (the paper embeds this output verbatim)
// ---------------------------------------------------------------------------

/// Start marker of the generated block inside the paper and README.
pub const BEGIN: &str = "<!-- BEGIN GENERATED BY choam-model (cargo run -q -p choam-model) -->";
/// End marker of the generated block.
pub const END: &str = "<!-- END GENERATED -->";

/// Render the model block embedded in the paper.
///
/// # Errors
/// Propagates [`ModelError`] (cannot occur for the built-in reference inputs).
pub fn render_paper_block() -> Result<String, ModelError> {
    let mut o = String::new();
    let pr = implied_performance_ratio();
    let pv5 = pv_energy(5.0)?;
    let pv10 = pv_energy(10.0)?;
    let bom = shell_bom(REFERENCE_UNIT)?;
    let mat = bom.material_cad();
    let w = |o: &mut String, s: String| {
        o.push_str(&s);
        o.push('\n');
    };
    w(&mut o, BEGIN.to_string());
    w(&mut o, "| ID | Quantity | Value | Chain |".into());
    w(&mut o, "| --- | --- | ---: | --- |".into());
    w(
        &mut o,
        format!("| E1 | Implied performance ratio (NRCan) | {pr:.4} | 1163 / (4.25 x 365) |"),
    );
    for (id, pv) in [("E2", pv5), ("E3", pv10)] {
        w(&mut o, format!("| {id} | {:.0} kWp annual yield, Toronto | {:.0} kWh/yr | {:.0} x 4.25 x 365 x {pr:.4} |", pv.kwp, pv.annual_kwh, pv.kwp));
        w(
            &mut o,
            format!(
                "| {id}a | {:.0} kWp mean daily yield | {:.2} kWh/day | annual / 365 |",
                pv.kwp, pv.daily_kwh
            ),
        );
        w(
            &mut o,
            format!(
                "| {id}b | {:.0} kWp bill offset at Tier 1 | ${:.2}/yr | annual x $0.120 |",
                pv.kwp, pv.bill_offset_cad_per_yr
            ),
        );
        w(&mut o, format!("| {id}c | {:.0} kWp cash revenue (net metering) | ${:.2}/yr | O. Reg. 541/05: credits only |", pv.kwp, pv.cash_revenue_cad_per_yr));
    }
    for (id, cap) in [("E4", 50_000.0), ("E5", 80_000.0)] {
        w(&mut o, format!("| {id} | Energy-only payback on a ${cap:.0} unit (5 kWp) | {:.1} yr | {cap:.0} / {:.2} |", payback_years(cap, pv5.bill_offset_cad_per_yr)?, pv5.bill_offset_cad_per_yr));
    }
    for (id, yrs) in [("E6", 5.0), ("E7", 7.0)] {
        w(&mut o, format!("| {id} | Max unit cost for a {yrs:.0}-yr energy-only ROI (5 kWp) | ${:.2} | {yrs:.0} x {:.2} |", yrs * pv5.bill_offset_cad_per_yr, pv5.bill_offset_cad_per_yr));
    }
    for (i, l) in bom.lines.iter().enumerate() {
        w(
            &mut o,
            format!(
                "| B{} | {} | {} x ${:.2} = ${:.2} | Kent.ca 2026-10-18 |",
                i + 1,
                l.item,
                l.qty,
                l.unit_cad,
                l.total()
            ),
        );
    }
    w(
        &mut o,
        format!("| B5 | Partial shell materials, 200 sq ft (pre-tax) | ${mat:.2} | sum B1-B4 |"),
    );
    w(
        &mut o,
        format!(
            "| B6 | Partial shell materials per sq ft | ${:.2}/sq ft | B5 / {:.0} |",
            mat / bom.floor_sqft,
            bom.floor_sqft
        ),
    );
    for (id, h) in [("L1", 100.0), ("L2", 200.0), ("L3", 400.0)] {
        w(&mut o, format!("| {id} | Labour at {h:.0} h (sensitivity point, not an estimate) | ${:.2} | {h:.0} x $32.00 |", labour_cad(h)?));
    }
    w(
        &mut o,
        format!(
            "| C1 | Lumber embodied carbon A1-A3 | {:.1} kg CO2e | {} pcs x {:.5} m3 x 54.6 |",
            bom.lumber_gwp_kg(),
            bom.stud_pieces,
            STUD_VOLUME_IN3 * M3_PER_IN3
        ),
    );
    for (id, n) in [("S1", 1_000.0), ("S2", 2_000.0)] {
        w(&mut o, format!("| {id} | Share of CMHC annual need from one factory at {n:.0} units/yr | {:.2}% | {n:.0} / 477,840 |", 100.0 * n / CMHC_STARTS_NEEDED_PER_YR));
    }
    w(
        &mut o,
        format!(
            "| S3 | CMHC gap above business-as-usual | {:.0} starts/yr | 477,840 - 245,000 |",
            CMHC_STARTS_NEEDED_PER_YR - CMHC_STARTS_BAU_PER_YR
        ),
    );
    o.push_str(END);
    Ok(o)
}

/// Key figures the README must quote verbatim (asserted by the test suite).
///
/// # Errors
/// Propagates [`ModelError`] (cannot occur for the built-in reference inputs).
pub fn readme_facts() -> Result<Vec<String>, ModelError> {
    let pv5 = pv_energy(5.0)?;
    let bom = shell_bom(REFERENCE_UNIT)?;
    Ok(vec![
        format!("{:.0} kWh/yr", pv5.annual_kwh),
        format!("${:.2}/yr", pv5.bill_offset_cad_per_yr),
        format!(
            "{:.1} yr",
            payback_years(50_000.0, pv5.bill_offset_cad_per_yr)?
        ),
        format!(
            "{:.1} yr",
            payback_years(80_000.0, pv5.bill_offset_cad_per_yr)?
        ),
        format!("${:.2}", 7.0 * pv5.bill_offset_cad_per_yr),
        format!("${:.2}", bom.material_cad()),
        format!("{:.2}%", 100.0 * 2_000.0 / CMHC_STARTS_NEEDED_PER_YR),
    ])
}

/// Extract the generated block (markers included) from a markdown document.
#[must_use]
pub fn extract_block(doc: &str) -> Option<&str> {
    let start = doc.find(BEGIN)?;
    let end = doc[start..].find(END)? + start + END.len();
    Some(&doc[start..end])
}
