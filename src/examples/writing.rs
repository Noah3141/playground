/* Writing to a file (creating if not exists) */

pub fn write_to(path:&str, text: &str) -> std::io::Result<()> {

// std::fs::write(path, text.as_bytes()); // b"your text" also works to create a byte
    std::fs::write(path, text);

    Ok(())

}

pub fn write_to_file(path: &str, text: &str) -> std::io::Result<()> {

    let mut buffer = std::fs::File::create(path)?;

    use std::io::Write;
    buffer.write_all(b"some bytes")?;

    Ok(())
}