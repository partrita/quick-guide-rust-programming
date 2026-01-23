use std::{
    env::current_dir,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

/// &Path is immutable form of PathBuf (same to &str and String)
fn grep(filename: &Path, word: &str) -> std::io::Result<()> {
    let mut f: File = File::open(filename)?;
    let mut text_buffer = String::new();

    f.read_to_string(&mut text_buffer)?;
    for line in text_buffer.split('\n') {
        if line.contains(word) {
            println!("{line}");
        }
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    let mut filename: PathBuf = current_dir()?;

    filename.push("src/std_library_file/main.rs");
    grep(&filename, "main")?; // show lines that include specific word
    Ok(())
}
