use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

use color_eyre::Result;
use question::{Answer, Question};
use tap::Tap;

fn main() {
    let r = real_main();
    std::fs::remove_file("./.bulkrename").unwrap();
    match r {
        Ok(()) => {
        }
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    }
}

fn real_main() -> Result<()> {
    color_eyre::install()?;

    let tmp_file_name = Path::new("./.bulkrename");

    let names: Vec<_> = std::fs::read_dir(".")?
        .map(|x| x.map(|x| x.file_name().to_string_lossy().into_owned()))
        .collect::<Result<_, _>>()?;

    if tmp_file_name.exists() {
        std::fs::remove_file(tmp_file_name)?;
    }

    let mut file = File::create(tmp_file_name)?;

    for name in &names {
        writeln!(&mut file, "{}", name)?;
    }
    drop(file);

    let editor = std::env::var("EDITOR")
        .unwrap_or_else(|_| std::env::var("VISUAL").unwrap_or_else(|_| "vim".to_string()));
    let mut editor_cmd = Command::new(editor);
    editor_cmd.arg(tmp_file_name);

    editor_cmd.spawn()?.wait()?;

    let renames = std::fs::read_to_string(tmp_file_name)?;

    let renames = names
        .iter()
        .map(|x| x.as_str())
        .zip(renames.split('\n'))
        .filter(|(name, new)| name != new)
        .collect::<Vec<_>>()
        .tap_mut(|x| x.sort_unstable());

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
