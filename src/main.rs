use clap::{App, Arg};
use std::env;
use std::io::{self, ErrorKind, Read, Result, Write};

// make a const for the buffer size
const CHUNK_SIZE: usize = 16 * 1024;

fn main() -> Result<()> {
    // use clap to read from the CLI
    let matches = App::new("pipeviewer")
        .arg(Arg::with_name("infile").help("Read from a file instead of stdin"))
        .arg(
            Arg::with_name("outfile")
                .short("o")
                .long("outfile")
                .takes_value(true)
                .help("Write output to a file instead of stdout"),
        )
        .arg(Arg::with_name("silent").short("s").long("silent"))
        .get_matches();

    // make the args from the CLI into regular vars
    // using `unwrap_or_default` to get the value OR an empty string
    let _infile = matches.value_of("infile").unwrap_or_default();
    let _outfile = matches.value_of("outfile").unwrap_or_default();
    let silent = if matches.is_present("silent") {
        true
    } else {
        // check for env var PV_SILENT
        // if set, do not print progress
        !env::var("PV_SILENT").unwrap_or_default().is_empty()
    };

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
