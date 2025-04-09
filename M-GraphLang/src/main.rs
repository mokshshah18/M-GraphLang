use std::io::{self, BufRead, BufReader, Result, Write};
use std::fs::File;
use std::env; // Import the env module

fn main() -> Result<()> {
    // 1. Get the filename from the command-line arguments.
    let args: Vec<String> = env::args().collect(); // Collect command-line arguments
    if args.len() < 2 {
        // Check if a filename was provided
        eprintln!("Usage: m-graphlang <filename>"); // Corrected usage message.  Also use eprintln
        return Ok(()); // Use Ok(()) for a graceful exit
    }
    let filename = &args[1]; // The filename is the second argument

    // 2.  Open the mlg file.
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    // 3. Read the file line by line.
    for (line_number, line) in reader.lines().enumerate() {
        let line = line?;

        // 4. Print each line with its line number.
        println!("Line {}: {}", line_number + 1, line);
        // In a real interpreter, you would call your lexer, parser,
        // and evaluator here, passing the 'line' variable.
    }
    Ok(())
}
