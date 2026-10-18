//! Prints the generated block that `CHOAM-Idea_Draft.md` embeds.
fn main() -> Result<(), choam_model::ModelError> {
    println!("{}", choam_model::render_paper_block()?);
    Ok(())
}
