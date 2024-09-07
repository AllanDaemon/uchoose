// Inspired by https://github.com/dmac/clipboard/blob/master/src/clipboard_linux.rs

use std::io::Write;

pub fn clipboard_set_text(text: &str) -> Result<(), std::io::Error> {
    // Create the xclip process
    let cmd = std::process::Command::new("xclip")
        .args(&["-selection", "clipboard"])
        .stdin(std::process::Stdio::piped())
        .spawn();

    let child = match cmd {
        Err(err) => {
            println!("Error starting xclip");
            dbg!(&err);
            dbg!(&err.kind());
            match err.kind() {
                std::io::ErrorKind::NotFound => {
                    eprintln!("Error coping text: {text}");
                    eprintln!("Error trying to call clipboard tool: xclip not found in $PATH");
                    std::process::exit(-1);
                }
                _ => {
                    return Err(err); //return the original error
                }
            }
        }
        Ok(child) => {
            println!("xclip spawned");
            child
        }
    };

    // Write the text to stdin
    let text_bytes = text.as_bytes();
    child.stdin.unwrap().write_all(text_bytes)?;
    Ok(())
}
