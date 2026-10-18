//! Unit tests of the template renderer and number lint.
use choam_model::docgen::{
    grouped, lint, numeric_tokens, parse_allowlist, render, DocError, Value,
};

fn vals() -> Vec<Value> {
    vec![
        Value {
            key: "a.kwh",
            text: "5,815".into(),
            derived: true,
        },
        Value {
            key: "a.kwp",
            text: "5".into(),
            derived: false,
        },
    ]
}

#[test]
fn grouping() {
    assert_eq!(grouped(5815.0, 0), "5,815");
    assert_eq!(grouped(4884.6, 2), "4,884.60");
    assert_eq!(grouped(900_000_000.0, 0), "900,000,000");
    assert_eq!(grouped(64.5, 2), "64.50");
    assert_eq!(grouped(-1234.5, 1), "-1,234.5");
}

#[test]
fn renders_and_rejects() {
    assert_eq!(
        render("x {{a.kwh}} y {{block}}", &vals(), "B").unwrap(),
        "x 5,815 y B"
    );
    assert_eq!(
        render("{{nope}}", &vals(), ""),
        Err(DocError::UnknownKey("nope".into()))
    );
    assert_eq!(
        render("ab {{a.kwh", &vals(), ""),
        Err(DocError::Unclosed(3))
    );
}

#[test]
fn tokenizer_skips_identifiers_urls_and_placeholders() {
    let t = "E2b CO2 A277 {{a.kwh}} https://x.org/1163 and 5,815 kWh, $4,884.60. (12)";
    let toks: Vec<String> = numeric_tokens(t).into_iter().map(|n| n.token).collect();
    assert_eq!(toks, vec!["5,815", "4,884.60", "12"]);
}

#[test]
fn lint_catches_hand_typed_model_number() {
    // A model value typed by hand instead of `{{a.kwh}}` is not allowlisted.
    let r = lint("paper", "yields 5,815 kWh", &[], &vals());
    assert_eq!(r.unlisted.len(), 1);
    // Allowlisting it as a quote is caught too.
    let allow = parse_allowlist("paper\t5,815\tyields 5,815\tquote\tX").unwrap();
    let r = lint("paper", "yields 5,815 kWh", &allow, &vals());
    assert_eq!(r.unlisted.len(), 0);
    assert_eq!(r.shadows_model.len(), 1);
}

#[test]
fn lint_whole_token_and_stale_rows() {
    // Context must contain the token as a whole token: `$900` does not match `$900,000`.
    assert!(parse_allowlist("readme\t900\tis $900,000\tquote\tX").is_err());
    assert!(
        parse_allowlist("readme\t900\t $900 \tquote\tX").is_err(),
        "context too short"
    );
    let allow = parse_allowlist(
        "readme\t900\tCA$75 is $900 a year\tquote\tX\nreadme\t7\tnowhere 7\tauthor\tY",
    )
    .unwrap();
    let r = lint("readme", "CA$75 is $900 a year", &allow, &[]);
    assert!(r.unlisted.iter().all(|s| !s.contains("`900`")));
    assert_eq!(r.unused.len(), 1);
}
