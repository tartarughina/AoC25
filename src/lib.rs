use std::{
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

pub fn read_input<P: AsRef<Path>>(file: P) -> std::io::Result<BufReader<File>> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let file = File::open(root.join(file))?;

    Ok(BufReader::new(file))
}
