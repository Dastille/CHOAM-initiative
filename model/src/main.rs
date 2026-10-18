//! `choam-report`: print the model table, or regenerate / check the documents.
//!
//! - no argument: print the generated table embedded in the paper
//! - `--write`: re-render `CHOAM-Idea_Draft.md`, `README.md` and `CLAIMS.md`
//!   from `model/templates/`
//! - `--check`: exit 1 if any committed document differs from a fresh render
use std::path::PathBuf;
use std::process::ExitCode;

fn targets() -> Result<Vec<(PathBuf, String)>, choam_model::RenderError> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..");
    Ok(vec![
        (
            root.join("CHOAM-Idea_Draft.md"),
            choam_model::render_paper()?,
        ),
        (root.join("README.md"), choam_model::render_readme()?),
        (root.join("CLAIMS.md"), choam_model::render_claims()?),
    ])
}

fn main() -> ExitCode {
    let arg = std::env::args().nth(1);
    let run = || -> Result<bool, Box<dyn std::error::Error>> {
        match arg.as_deref() {
            None => {
                println!("{}", choam_model::render_paper_block()?);
                Ok(true)
            }
            Some("--write") => {
                for (path, text) in targets()? {
                    std::fs::write(&path, text)?;
                    println!("wrote {}", path.display());
                }
                Ok(true)
            }
            Some("--check") => {
                let mut ok = true;
                for (path, text) in targets()? {
                    if std::fs::read_to_string(&path)? != text {
                        eprintln!(
                            "stale: {} (run `cargo run -q -p choam-model -- --write`)",
                            path.display()
                        );
                        ok = false;
                    }
                }
                Ok(ok)
            }
            Some(other) => {
                Err(format!("unknown argument `{other}`; use --write or --check").into())
            }
        }
    };
    match run() {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::FAILURE,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}
