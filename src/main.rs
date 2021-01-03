use std::env;
use std::io::{self, ErrorKind, Read, Result, Write};

// make a const for the buffer size
const CHUNK_SIZE: usize = 16 * 1024;

fn main() -> Result<()> {
    // check for env var PV_SILENT
    // if set, do not print progress

    // this is a lot
    // if PV_SILENT is not set/an error is thrown, create a default empty value? IDK
    // check if the value is empty, and return the corresponding bool
    let silent = !env::var("PV_SILENT").unwrap_or_default().is_empty();

    // debug macro! it takes any expression!
    // dbg!(silent);
    let mut total_bytes = 0;

    // make a buffer?
    let mut buffer = [0; CHUNK_SIZE];

    // loop through the bytes read, not just the limit of `buffer`
    loop {
        // `.read` take a fixed-size buffer and returns the number of bytes read on success
        // since this is variable assignment, use a semicolon
        let num_read = match io::stdin().read(&mut buffer) {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break,
        };
        //dbg!(total_bytes += num_read);
        total_bytes += num_read;

        // print total bytes as part of each loop,
        // and do so from the beginning of the line to
        // overwrite the value
        if !silent {
            eprint!("\r{}", total_bytes);
        }

        // if writing to stdout throws an error, catch and print it
        if let Err(e) = io::stdout().write_all(&buffer[..num_read]) {
            // if the error is BrokenPipe, then catch and ignore it since it's
            // mostly expected
            if e.kind() == ErrorKind::BrokenPipe {
                break;
            }
            return Err(e);
        }
    }

    Ok(())
}
