use std::{fs::File, io::BufReader, path::Path};

pub fn read_input<P: AsRef<Path>>(file: P) -> std::io::Result<BufReader<File>> {
    let file = File::open(file)?;

    Ok(BufReader::new(file))
}
