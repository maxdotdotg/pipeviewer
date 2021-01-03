use std::io::{self, Read, Write};

// make a const for the buffer size
const CHUNK_SIZE: usize = 16 * 1024;

fn main() {
    let mut total_bytes = 0;

    // loop through the bytes read, not just the limit of `buffer`
    loop{
        // make a buffer?
        let mut buffer = [0; CHUNK_SIZE];

        // `.read` take a fixed-size buffer and returns the number of bytes read on success
        // since this is variable assignment, use a semicolon
        let num_read = match io::stdin().read(&mut buffer) {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break,
        };
        total_bytes += num_read;

        // print all the things, unmodified, to stdout
        io::stdout().write_all(&buffer[..num_read]).unwrap();
    }

    // printing total_bytes to stderr, again, IDK
    eprintln!("{}", total_bytes);
}
