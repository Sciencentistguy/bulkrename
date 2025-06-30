use std::{fs::File, io::Read};
use std::io::Write;
use std::path::Path;
use std::process::Command;

use question::{Answer, Question};
use tap::Tap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let names: Vec<_> = std::fs::read_dir(".")?
        .map(|x| x.map(|x| x.file_name().to_string_lossy().into_owned()))
        .collect::<Result<_, _>>()?;

    let mut file = tempfile::NamedTempFile::new()?;

    for name in &names {
        writeln!(&mut file, "{}", name)?;
    }

    let file_path = file.into_temp_path();

    let editor = std::env::var("EDITOR")
        .unwrap_or_else(|_| std::env::var("VISUAL").unwrap_or_else(|_| "vim".to_string()));
    let mut editor_cmd = Command::new(editor);
    editor_cmd.arg(&file_path);

    editor_cmd.spawn()?.wait()?;

    let renames = std::fs::read_to_string(file_path)?;

    let renames = names
        .iter()
        .map(|x| x.as_str())
        .zip(renames.lines())
        .filter(|(name, new)| name != new)
        .collect::<Vec<_>>()
        .tap_mut(|x| x.sort_unstable());

    if renames.is_empty() {
        return Ok(());
    }

    for (name, new) in &renames {
        println!("'{name}' -> '{new}'");
    }

    if Question::new("Do these renames?")
        .yes_no()
        .default(Answer::YES)
        .show_defaults()
        .confirm()
        == Answer::YES
    {
        for (name, new) in &renames {
            print!("Renaming '{name}' -> '{new}'...");
            std::io::stdout().lock().flush()?;
            std::fs::rename(name, new)?;
            println!(" done")
        }
    }

    Ok(())
}
