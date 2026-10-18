//! # choam-model
//!
//! First-principles calculations behind `CHOAM-Idea_Draft.md` (paper v2.0).
//!
//! Every input constant carries its source in its doc comment. The paper,
//! `README.md` and `CLAIMS.md` are rendered from `model/templates/` by
//! [`render_paper`], [`render_readme`] and [`render_claims`]: every
//! model-derived number in them, prose included, is a `{{key}}` placeholder
//! filled from [`values`]. Every other number must be on
//! `model/numbers-allowlist.tsv` with its location and source. The test suite
//! asserts that the committed files equal a fresh render and that the
//! allowlist covers every remaining number.
//!
//! ```
//! let e = choam_model::pv_energy(5.0).unwrap();
//! assert!((e.annual_kwh - 5815.0).abs() < 1e-9);
//! ```
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![warn(clippy::pedantic)]

pub mod docgen;

use docgen::{grouped, Claim, Cmp, DocError, Value};
use std::fmt;

// ---------------------------------------------------------------------------
// Sourced inputs
// ---------------------------------------------------------------------------

/// `NRCan` "Photovoltaic potential and solar resource maps of Canada",
/// municipal table `municip_potentiel-potential.csv`, row Ontario/Toronto,
/// Annual, column "South-facing with latitude tilt": 1163 kWh per kWp per year.
/// <https://ftp.maps.canada.ca/pub/nrcan_rncan/Solar-energy_Energie-solaire/photovoltaic_canada_photovoltaique/municip_potentiel-potential.csv>
pub const TORONTO_PV_KWH_PER_KWP_YR: f64 = 1163.0;

/// Same `NRCan` dataset, `municip_kWh.csv`, Ontario/Toronto, Annual,
/// latitude tilt: mean daily global insolation 4.25 kWh/m2/day
/// (equivalently 4.25 peak-sun-hours per day).
pub const TORONTO_PSH_LAT_TILT: f64 = 4.25;

/// Same `NRCan` `municip_potentiel-potential.csv`, Ontario/Toronto, December,
/// latitude tilt: 51 kWh per kWp for the month.
pub const TORONTO_PV_DEC_KWH_PER_KWP: f64 = 51.0;

/// Date the Kent.ca prices below were read.
pub const PRICES_ACCESSED: &str = "2026-10-18";

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
/// Kent.ca, "`ComfortBatt` Woodstd R22 5.5\" x 23\" x 47\" - 37.5 `SqFt`", SKU 1021667: CA$152.53 (accessed 2026-10-18).
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
/// produced in Ontario, GWP total A1-A3: 54.6 kg `CO2e` per m3.
/// <https://cwc.ca/publication/a-regionalized-industry-average-epd-for-canadian-softwood-lumber/>
pub const ONTARIO_LUMBER_GWP_A1_A3_KG_PER_M3: f64 = 54.6;

/// CMHC (2025-06-19) "Canada's Housing Supply Shortages: Moving to a New
/// Framework", Figure 13: 477,840 starts/year needed to 2035 (Canada).
/// <https://www.cmhc-schl.gc.ca/professionals/housing-markets-data-and-research/housing-research/research-reports/accelerate-supply/canadas-housing-supply-shortages-a-new-framework>
pub const CMHC_STARTS_NEEDED_PER_YR: f64 = 477_840.0;
/// CMHC, same Figure 13: business-as-usual 245,000 starts/year.
pub const CMHC_STARTS_BAU_PER_YR: f64 = 245_000.0;

/// Length of one purchased 2x6 piece (Kent.ca SKU 1016278 "2 x 6 x 8'"), ft.
pub const LUMBER_LENGTH_FT: f64 = 8.0;
/// 2x6 actual dressed size 1.5 in x 5.5 in (Home Depot Canada product
/// 1001802970 lists width 5.5 in, height 1.5 in), times the piece length.
pub const STUD_VOLUME_IN3: f64 = 1.5 * 5.5 * LUMBER_LENGTH_FT * 12.0;
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
    /// Performance ratio implied by `NRCan`'s own two tables (PV potential / insolation).
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

/// Performance ratio implied by `NRCan`: `1163 / (4.25 x 365)`.
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

/// Take-off rule: studs added at the end of each wall run.
pub const END_STUDS_PER_WALL: u32 = 1;
/// Take-off rule: extra studs per corner.
pub const STUDS_PER_CORNER: u32 = 2;
/// Take-off rule: plate runs per wall length (double top plate, single bottom).
pub const PLATE_RUNS: f64 = 3.0;

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
    let per_wall = |len: f64| ceil_u32(len * 12.0 / s) + END_STUDS_PER_WALL;
    let field_studs = 2 * per_wall(l) + 2 * per_wall(w);
    let corner_studs = STUDS_PER_CORNER * 4;
    let perimeter = 2.0 * (l + w);
    let plate_pieces = ceil_u32(PLATE_RUNS * perimeter / LUMBER_LENGTH_FT);
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

    /// Fossil A1-A3 carbon of the lumber only, kg `CO2e` (CWC EPD 845, Ontario).
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
// Scenario parameters (not sourced facts; each is named in the paper)
// ---------------------------------------------------------------------------

/// Array sizes evaluated: 5 kWp (reference) and 10 kWp (Ashlynn's v1.0 upper bound).
pub const ARRAY_KWP: [f64; 2] = [5.0, 10.0];
/// Unit-cost target range, CAD (Ashlynn's v1.0 target; unverified).
pub const UNIT_COST_CAD: [f64; 2] = [50_000.0, 80_000.0];
/// ROI horizons, years (Ashlynn's v1.0 "5-7 year ROI").
pub const ROI_YEARS: [f64; 2] = [5.0, 7.0];
/// Labour sensitivity points, hours (no source; not an estimate).
pub const LABOUR_HOURS: [f64; 3] = [100.0, 200.0, 400.0];
/// Factory output, units/yr (Ashlynn's v1.0 figures).
pub const FACTORY_UNITS_PER_YR: [f64; 2] = [1_000.0, 2_000.0];

// ---------------------------------------------------------------------------
// Rendering
// ---------------------------------------------------------------------------

/// Start marker of the generated block inside the paper.
pub const BEGIN: &str = "<!-- BEGIN GENERATED BY choam-model (cargo run -q -p choam-model) -->";
/// End marker of the generated block.
pub const END: &str = "<!-- END GENERATED -->";

/// Template of `CHOAM-Idea_Draft.md`.
pub const PAPER_TEMPLATE: &str = include_str!("../templates/CHOAM-Idea_Draft.md");
/// Template of `README.md`.
pub const README_TEMPLATE: &str = include_str!("../templates/README.md");
/// Template of `CLAIMS.md`.
pub const CLAIMS_TEMPLATE: &str = include_str!("../templates/CLAIMS.md");
/// Allowlist of numbers in the templates that are not model output.
pub const ALLOWLIST: &str = include_str!("../numbers-allowlist.tsv");
/// Claim rows rendered into `CLAIMS.md` (`model/claims.tsv`).
pub const CLAIMS_DATA: &str = include_str!("../claims.tsv");

/// Error from rendering: either the model or the template failed.
#[derive(Debug, Clone, PartialEq)]
pub enum RenderError {
    /// A model calculation rejected its input.
    Model(ModelError),
    /// A template is malformed or names an unknown key.
    Doc(DocError),
}

impl fmt::Display for RenderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RenderError::Model(e) => write!(f, "model: {e}"),
            RenderError::Doc(e) => write!(f, "template: {e}"),
        }
    }
}

impl std::error::Error for RenderError {}

impl From<ModelError> for RenderError {
    fn from(e: ModelError) -> Self {
        RenderError::Model(e)
    }
}

impl From<DocError> for RenderError {
    fn from(e: DocError) -> Self {
        RenderError::Doc(e)
    }
}

struct Rows(String);

impl Rows {
    fn row(&mut self, id: &str, quantity: &str, value: &str, chain: &str) {
        for part in [
            "| ", id, " | ", quantity, " | ", value, " | ", chain, " |\n",
        ] {
            self.0.push_str(part);
        }
    }
}

fn energy_rows(o: &mut Rows) -> Result<(), ModelError> {
    let pr = implied_performance_ratio();
    let tariff = ONTARIO_RPP_TIER1_CAD_PER_KWH;
    o.row(
        "E1",
        "Implied performance ratio (NRCan)",
        &format!("{pr:.4}"),
        &format!("{TORONTO_PV_KWH_PER_KWP_YR:.0} / ({TORONTO_PSH_LAT_TILT} x 365)"),
    );
    for (id, kwp) in ["E2", "E3"].into_iter().zip(ARRAY_KWP) {
        let pv = pv_energy(kwp)?;
        o.row(
            id,
            &format!("{kwp:.0} kWp annual yield, Toronto"),
            &format!("{:.0} kWh/yr", pv.annual_kwh),
            &format!("{kwp:.0} x {TORONTO_PSH_LAT_TILT} x 365 x {pr:.4}"),
        );
        o.row(
            &format!("{id}a"),
            &format!("{kwp:.0} kWp mean daily yield"),
            &format!("{:.2} kWh/day", pv.daily_kwh),
            "annual / 365",
        );
        o.row(
            &format!("{id}b"),
            &format!(
                "{kwp:.0} kWp bill offset at Tier 1, upper bound (every kWh offsets consumption)"
            ),
            &format!("${:.2}/yr", pv.bill_offset_cad_per_yr),
            &format!("annual x ${tariff:.3}"),
        );
        o.row(
            &format!("{id}c"),
            &format!("{kwp:.0} kWp cash revenue (net metering)"),
            &format!("${:.2}/yr", pv.cash_revenue_cad_per_yr),
            "O. Reg. 541/05: credits only",
        );
    }
    let offset = pv_energy(ARRAY_KWP[0])?.bill_offset_cad_per_yr;
    for (id, cap) in ["E4", "E5"].into_iter().zip(UNIT_COST_CAD) {
        o.row(
            id,
            &format!(
                "Energy-only payback on a ${cap:.0} unit ({:.0} kWp), lower bound",
                ARRAY_KWP[0]
            ),
            &format!("{:.1} yr", payback_years(cap, offset)?),
            &format!("{cap:.0} / {offset:.2}"),
        );
    }
    for (id, yrs) in ["E6", "E7"].into_iter().zip(ROI_YEARS) {
        o.row(
            id,
            &format!(
                "Max unit cost for a {yrs:.0}-yr energy-only ROI ({:.0} kWp), upper bound",
                ARRAY_KWP[0]
            ),
            &format!("${:.2}", yrs * offset),
            &format!("{yrs:.0} x {offset:.2}"),
        );
    }
    Ok(())
}

fn bom_rows(o: &mut Rows) -> Result<(), ModelError> {
    let bom = shell_bom(REFERENCE_UNIT)?;
    let mat = bom.material_cad();
    let offset = pv_energy(ARRAY_KWP[0])?.bill_offset_cad_per_yr;
    for (i, l) in bom.lines.iter().enumerate() {
        o.row(
            &format!("B{}", i + 1),
            l.item,
            &format!("{} x ${:.2} = ${:.2}", l.qty, l.unit_cad, l.total()),
            &format!("Kent.ca {PRICES_ACCESSED}"),
        );
    }
    o.row(
        "B5",
        &format!(
            "Partial shell materials, {:.0} sq ft (pre-tax)",
            bom.floor_sqft
        ),
        &format!("${mat:.2}"),
        "sum B1-B4",
    );
    o.row(
        "B6",
        "Partial shell materials per sq ft",
        &format!("${:.2}/sq ft", mat / bom.floor_sqft),
        &format!("B5 / {:.0}", bom.floor_sqft),
    );
    let ceiling_long = ROI_YEARS[1] * offset;
    let ceiling_short = ROI_YEARS[0] * offset;
    let r22 = bom.lines[2].total();
    o.row(
        "B7",
        &format!(
            "Partial shell (B5) minus {:.0}-yr ceiling (E7)",
            ROI_YEARS[1]
        ),
        &format!("${:.2}", mat - ceiling_long),
        &format!("{mat:.2} - {ceiling_long:.2}"),
    );
    o.row(
        "B8",
        &format!("R22 line (B3) minus {:.0}-yr ceiling (E6)", ROI_YEARS[0]),
        &format!("${:.2}", r22 - ceiling_short),
        &format!("{r22:.2} - {ceiling_short:.2}"),
    );
    for (id, h) in ["L1", "L2", "L3"].into_iter().zip(LABOUR_HOURS) {
        o.row(
            id,
            &format!("Labour at {h:.0} h (sensitivity point, not an estimate)"),
            &format!("${:.2}", labour_cad(h)?),
            &format!("{h:.0} x ${ONTARIO_CARPENTER_MEDIAN_CAD_PER_H:.2}"),
        );
    }
    o.row(
        "C1",
        "Lumber embodied carbon A1-A3",
        &format!("{:.1} kg CO2e", bom.lumber_gwp_kg()),
        &format!(
            "{} pcs x {:.5} m3 x {ONTARIO_LUMBER_GWP_A1_A3_KG_PER_M3}",
            bom.stud_pieces,
            STUD_VOLUME_IN3 * M3_PER_IN3
        ),
    );
    Ok(())
}

fn scale_rows(o: &mut Rows) {
    let needed = grouped(CMHC_STARTS_NEEDED_PER_YR, 0);
    for (id, n) in ["S1", "S2"].into_iter().zip(FACTORY_UNITS_PER_YR) {
        o.row(
            id,
            &format!("Share of CMHC annual need from one factory at {n:.0} units/yr"),
            &format!("{:.2}%", 100.0 * n / CMHC_STARTS_NEEDED_PER_YR),
            &format!("{n:.0} / {needed}"),
        );
    }
    o.row(
        "S3",
        "CMHC gap above business-as-usual",
        &format!(
            "{:.0} starts/yr",
            CMHC_STARTS_NEEDED_PER_YR - CMHC_STARTS_BAU_PER_YR
        ),
        &format!("{needed} - {}", grouped(CMHC_STARTS_BAU_PER_YR, 0)),
    );
}

/// Render the model table embedded in the paper (markers included).
///
/// # Errors
/// Propagates [`ModelError`] (cannot occur for the built-in reference inputs).
pub fn render_paper_block() -> Result<String, ModelError> {
    let mut o = Rows(String::new());
    o.0.push_str(BEGIN);
    o.0.push('\n');
    o.0.push_str("| ID | Quantity | Value | Chain |\n| --- | --- | ---: | --- |\n");
    energy_rows(&mut o)?;
    bom_rows(&mut o)?;
    scale_rows(&mut o);
    o.0.push_str(END);
    Ok(o.0)
}

/// Scientific notation as printed in EPD tables, e.g. `5.46E+01`.
fn sci(value: f64) -> String {
    let text = format!("{value:.2E}");
    match text.split_once('E') {
        Some((mantissa, exp)) => {
            let power: i32 = exp.parse().unwrap_or(0);
            let sign = if power < 0 { '-' } else { '+' };
            format!("{mantissa}E{sign}{:02}", power.abs())
        }
        None => text,
    }
}

fn input(key: &'static str, text: String) -> Value {
    Value {
        key,
        text,
        derived: false,
    }
}

fn derived(key: &'static str, text: String) -> Value {
    Value {
        key,
        text,
        derived: true,
    }
}

/// Every value a template may reference, keyed by name.
///
/// # Errors
/// Propagates [`ModelError`] (cannot occur for the built-in reference inputs).
pub fn values() -> Result<Vec<Value>, ModelError> {
    let pv5 = pv_energy(ARRAY_KWP[0])?;
    let pv10 = pv_energy(ARRAY_KWP[1])?;
    let bom = shell_bom(REFERENCE_UNIT)?;
    let mat = bom.material_cad();
    let off = pv5.bill_offset_cad_per_yr;
    let fp = REFERENCE_UNIT;
    let mut v = vec![
        // Sourced inputs.
        input("nrcan.kwh_per_kwp", grouped(TORONTO_PV_KWH_PER_KWP_YR, 0)),
        input("nrcan.psh", format!("{TORONTO_PSH_LAT_TILT}")),
        input("nrcan.dec", format!("{TORONTO_PV_DEC_KWH_PER_KWP:.0}")),
        input(
            "oeb.tier1_cents",
            format!("{:.1}", ONTARIO_RPP_TIER1_CAD_PER_KWH * 100.0),
        ),
        input(
            "jobbank.wage",
            format!("{ONTARIO_CARPENTER_MEDIAN_CAD_PER_H:.2}"),
        ),
        input("cwc.gwp", format!("{ONTARIO_LUMBER_GWP_A1_A3_KG_PER_M3}")),
        input("cwc.gwp_sci", sci(ONTARIO_LUMBER_GWP_A1_A3_KG_PER_M3)),
        input(
            "nrcan.kwh_per_kwp_raw",
            format!("{TORONTO_PV_KWH_PER_KWP_YR:.0}"),
        ),
        input(
            "cmhc.needed_k",
            format!("{:.0}", CMHC_STARTS_NEEDED_PER_YR / 1000.0),
        ),
        input("kent.panel_sqft", format!("{PANEL_SQFT:.0}")),
        input("cmhc.needed", grouped(CMHC_STARTS_NEEDED_PER_YR, 0)),
        input("cmhc.bau", grouped(CMHC_STARTS_BAU_PER_YR, 0)),
        input("kent.date", PRICES_ACCESSED.to_string()),
        input("kent.r22", format!("{PRICE_R22_BAG_CAD:.2}")),
        input("kent.stud", format!("{PRICE_STUD_2X6X8_CAD:.2}")),
        input("kent.osb", format!("{PRICE_OSB_7_16_CAD:.2}")),
        input("kent.drywall", format!("{PRICE_DRYWALL_1_2_CAD:.2}")),
        input("kent.r22_sqft", format!("{R22_BAG_SQFT}")),
        input("rule.end_studs", format!("{END_STUDS_PER_WALL}")),
        input("rule.corner_studs", format!("{STUDS_PER_CORNER}")),
        input("rule.plate_runs", format!("{PLATE_RUNS:.0}")),
        // Derived.
        derived("e1.pr", format!("{:.4}", implied_performance_ratio())),
        derived("e2.annual", grouped(pv5.annual_kwh, 0)),
        derived("e2.offset", grouped(off, 2)),
        derived("e2.offset_whole", grouped(off, 0)),
        derived("e2.cash", format!("{:.2}", pv5.cash_revenue_cad_per_yr)),
        derived("e3.annual", grouped(pv10.annual_kwh, 0)),
        derived(
            "e4.payback",
            format!("{:.1}", payback_years(UNIT_COST_CAD[0], off)?),
        ),
        derived(
            "e5.payback",
            format!("{:.1}", payback_years(UNIT_COST_CAD[1], off)?),
        ),
        derived("e6.ceiling", grouped(ROI_YEARS[0] * off, 2)),
        derived("e7.ceiling", grouped(ROI_YEARS[1] * off, 2)),
        derived("b3.r22", grouped(bom.lines[2].total(), 2)),
        derived("b5.material", grouped(mat, 2)),
        derived("b7.over", grouped((mat - ROI_YEARS[1] * off).abs(), 2)),
        derived(
            "b8.over",
            grouped((bom.lines[2].total() - ROI_YEARS[0] * off).abs(), 2),
        ),
        derived(
            "s1.share",
            format!(
                "{:.2}",
                100.0 * FACTORY_UNITS_PER_YR[0] / CMHC_STARTS_NEEDED_PER_YR
            ),
        ),
        derived(
            "s2.share",
            format!(
                "{:.2}",
                100.0 * FACTORY_UNITS_PER_YR[1] / CMHC_STARTS_NEEDED_PER_YR
            ),
        ),
        derived(
            "s3.gap",
            grouped(CMHC_STARTS_NEEDED_PER_YR - CMHC_STARTS_BAU_PER_YR, 0),
        ),
    ];
    v.extend(scenario_values(fp, &bom));
    v.extend(comparisons(mat, bom.lines[2].total(), off)?);
    Ok(v)
}

/// Scenario parameters (her figures and design choices), not sourced data.
fn scenario_values(fp: Footprint, bom: &ShellBom) -> Vec<Value> {
    vec![
        input("unit.length_ft", format!("{:.0}", fp.length_ft)),
        input("unit.width_ft", format!("{:.0}", fp.width_ft)),
        input("unit.height_ft", format!("{:.0}", fp.wall_height_ft)),
        input("unit.spacing_in", format!("{:.0}", fp.stud_spacing_in)),
        input("unit.sqft", format!("{:.0}", bom.floor_sqft)),
        derived(
            "unit.height_m",
            format!("{:.2}", fp.wall_height_ft * 0.3048),
        ),
        input("lumber.length_ft", format!("{LUMBER_LENGTH_FT:.0}")),
        input("kwp.ref", format!("{:.0}", ARRAY_KWP[0])),
        input("kwp.max", format!("{:.0}", ARRAY_KWP[1])),
        input("cost.low", grouped(UNIT_COST_CAD[0], 0)),
        input("cost.high", grouped(UNIT_COST_CAD[1], 0)),
        input("cost.low_k", format!("{:.0}", UNIT_COST_CAD[0] / 1000.0)),
        input("cost.high_k", format!("{:.0}", UNIT_COST_CAD[1] / 1000.0)),
        input("roi.short", format!("{:.0}", ROI_YEARS[0])),
        input("roi.long", format!("{:.0}", ROI_YEARS[1])),
        input("factory.small", grouped(FACTORY_UNITS_PER_YR[0], 0)),
        input("factory.large", grouped(FACTORY_UNITS_PER_YR[1], 0)),
    ]
}

/// Comparison words and verdicts generated from the values they describe, so
/// that "exceeds", "contradicted" or "impossible" cannot outlive the numbers.
fn comparisons(mat: f64, r22: f64, off: f64) -> Result<Vec<Value>, ModelError> {
    let e6 = ROI_YEARS[0] * off;
    let e7 = ROI_YEARS[1] * off;
    let b5_e7 = Cmp::of(mat, e7, 2);
    let b3_e6 = Cmp::of(r22, e6, 2);
    // The v1.0 ROI claim fails for energy if even the cheapest unit's
    // energy-only payback (a lower bound) is longer than the longest ROI.
    let fails = payback_years(UNIT_COST_CAD[0], off)? > ROI_YEARS[1];
    let materials_over = b5_e7 == Cmp::Above;
    let pick = |c: bool, a: &str, b: &str| (if c { a } else { b }).to_string();
    Ok(vec![
        derived("cmp.b5_e7.exceeds", b5_e7.exceeds().to_string()),
        derived("cmp.b5_e7.exceed", b5_e7.exceed().to_string()),
        derived("cmp.b3_e6.exceeds", b3_e6.exceeds().to_string()),
        derived(
            "verdict.roi",
            pick(
                fails,
                "Contradicted for energy",
                "Not contradicted for energy",
            ),
        ),
        derived(
            "verdict.roi_lc",
            pick(
                fails,
                "contradicted for energy",
                "not contradicted for energy",
            ),
        ),
        derived(
            "verdict.roi_support",
            pick(fails, "is not supported", "is not ruled out"),
        ),
        derived(
            "status.roi_v1",
            pick(
                fails,
                "contradicted; energy; other revenue unverified",
                "unverified; energy payback within the claimed range",
            ),
        ),
        derived(
            "concl.fails",
            pick(materials_over, "fails", "is not ruled out"),
        ),
        derived(
            "concl.energy_roi",
            pick(
                materials_over,
                "impossible at any unit cost",
                "possible only if the complete unit costs no more than that ceiling",
            ),
        ),
    ])
}

/// The rendered claims of `CLAIMS.md`, from `model/claims.tsv`.
///
/// # Errors
/// [`RenderError`] if the model fails or a claims row is malformed.
pub fn claims() -> Result<Vec<Claim>, RenderError> {
    Ok(docgen::parse_claims(CLAIMS_DATA, &values()?)?)
}

fn render_doc(template: &str) -> Result<String, RenderError> {
    let block = render_paper_block()?;
    let table = docgen::claims_table(&claims()?);
    Ok(docgen::render(
        template,
        &values()?,
        &[("block", block.as_str()), ("claims_table", table.as_str())],
    )?)
}

/// Render `CHOAM-Idea_Draft.md` from its template.
///
/// # Errors
/// [`RenderError`] if the model or the template fails.
pub fn render_paper() -> Result<String, RenderError> {
    render_doc(PAPER_TEMPLATE)
}

/// Render `README.md` from its template.
///
/// # Errors
/// [`RenderError`] if the model or the template fails.
pub fn render_readme() -> Result<String, RenderError> {
    render_doc(README_TEMPLATE)
}

/// Render `CLAIMS.md` from its template.
///
/// # Errors
/// [`RenderError`] if the model or the template fails.
pub fn render_claims() -> Result<String, RenderError> {
    render_doc(CLAIMS_TEMPLATE)
}

/// Extract the generated block (markers included) from a markdown document.
#[must_use]
pub fn extract_block(doc: &str) -> Option<&str> {
    let start = doc.find(BEGIN)?;
    let end = doc[start..].find(END)? + start + END.len();
    Some(&doc[start..end])
}
