use std::io::BufRead;
use std::io::Read;

pub fn parse(path: &str) -> std::io::Result<&str> {
    let file = std::fs::File::open(format!("{}/directory", path))?;
    let mut reader = std::io::BufReader::new(file);
    let mut buf: Vec<u8> = Vec::new();
    loop {
        let read = reader.read_until('$' as u8, &mut buf)?;
        eprintln!("{}", read);
        if read == 0 { break; }

        let open = &mut [0 as u8; 1];
        match reader.read_exact(open) {
            Ok(_) => {},
            Err(err) => {
                if err.kind() == std::io::ErrorKind::UnexpectedEof {
                    break;
                }
                return Err(err);
            }
        };
        buf.extend_from_slice(open);
        if open[0] != '{' as u8 {
            continue;
        }
        let start = buf.len();
        let read = reader.read_until('}' as u8, &mut buf)?;
        let tag = &buf[start..start + read - 1];
        eprintln!("test {}", String::from_utf8_lossy(tag));
    }
    eprintln!("done {}", String::from_utf8_lossy(&buf));
    return Ok("done");
}
