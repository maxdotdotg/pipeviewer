use std::io::{self, Read, Write};

// make a const for the buffer size
const CHUNK_SIZE: usize = 16 * 1024;

fn main() {
    // make a buffer?
    let mut buffer = [0; CHUNK_SIZE];

    // read take a fixed size buffer and returns number of bytes read on success
    // handle errors with unwrap, IDK
    let num_read = io::stdin().read(&mut buffer).unwrap();

    // printing num_read to stderr, again, IDK
    eprintln!("num_read: {}", num_read);

    // print all the things, unmodified, to stdout
    io::stdout().write_all(&buffer[..num_read]).unwrap();
}
