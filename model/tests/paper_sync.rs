//! The paper and README must quote the model exactly.
use choam_model::{extract_block, readme_facts, render_paper_block};

const PAPER: &str = include_str!("../../CHOAM-Idea_Draft.md");
const README: &str = include_str!("../../README.md");

#[test]
fn paper_block_equals_model_output() {
    let generated = render_paper_block().expect("reference inputs are valid");
    let in_paper = extract_block(PAPER).expect("paper must contain the generated block markers");
    assert_eq!(
        in_paper, generated,
        "CHOAM-Idea_Draft.md is out of date: run `cargo run -q -p choam-model` and paste the block"
    );
}

#[test]
fn readme_quotes_model_figures() {
    for fact in readme_facts().expect("reference inputs are valid") {
        assert!(
            README.contains(&fact),
            "README.md must quote `{fact}` from the model"
        );
    }
}
