//! Behavioural tests for the calculations.
use choam_model::*;

#[test]
fn pv_chain_reproduces_nrcan_potential() {
    // kWp x PSH x 365 x PR must equal NRCan's published kWh/kWp by construction.
    let e = pv_energy(1.0).unwrap();
    assert!((e.annual_kwh - TORONTO_PV_KWH_PER_KWP_YR).abs() < 1e-9);
    assert!((e.performance_ratio - 0.749_718).abs() < 1e-6);
}

#[test]
fn five_kw_value_and_zero_cash() {
    let e = pv_energy(5.0).unwrap();
    assert!((e.annual_kwh - 5815.0).abs() < 1e-9);
    // 5,815 kWh x 0.112 CAD/kWh (OEB Tier 1, effective 2026-11-01).
    assert!((e.bill_offset_cad_per_yr - 651.28).abs() < 1e-9);
    assert_eq!(e.cash_revenue_cad_per_yr, 0.0);
}

#[test]
fn payback_math() {
    let y = payback_years(50_000.0, 697.8).unwrap();
    assert!((y - 71.653_768).abs() < 1e-5);
}

#[test]
fn rejects_bad_inputs() {
    assert!(pv_energy(-1.0).is_err());
    assert!(pv_energy(f64::NAN).is_err());
    assert!(payback_years(1.0, 0.0).is_err());
    assert!(labour_cad(f64::INFINITY).is_err());
    let mut fp = REFERENCE_UNIT;
    fp.width_ft = 0.0;
    let err = shell_bom(fp).unwrap_err();
    assert_eq!(
        err,
        ModelError::InvalidInput {
            name: "width_ft",
            value: 0.0
        }
    );
    assert!(err.to_string().contains("width_ft"));
}

#[test]
fn reference_bom_counts() {
    let b = shell_bom(REFERENCE_UNIT).unwrap();
    // 20 ft walls: ceil(240/24)+1 = 11 each; 10 ft walls: 6 each; corners 8; plates ceil(180/8)=23.
    assert_eq!(b.stud_pieces, 11 * 2 + 6 * 2 + 8 + 23);
    let qty: Vec<u32> = b.lines.iter().map(|l| l.qty).collect();
    assert_eq!(qty, vec![65, 28, 24, 22]);
    assert!((b.material_cad() - 4949.10).abs() < 1e-6);
    assert!((b.floor_sqft - 200.0).abs() < 1e-12);
}

#[test]
fn closer_stud_spacing_needs_more_studs() {
    let mut fp = REFERENCE_UNIT;
    fp.stud_spacing_in = 16.0;
    assert!(shell_bom(fp).unwrap().stud_pieces > shell_bom(REFERENCE_UNIT).unwrap().stud_pieces);
}

#[test]
fn lumber_carbon() {
    let b = shell_bom(REFERENCE_UNIT).unwrap();
    let expected = 65.0 * 792.0 * 0.0254_f64.powi(3) * 54.6;
    assert!((b.lumber_gwp_kg() - expected).abs() < 1e-9);
}
