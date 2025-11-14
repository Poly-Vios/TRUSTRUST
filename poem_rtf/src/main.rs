use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    // 24-bit ANSI true-color escape sequences
    const ORANGE: &str = "\x1b[38;2;205;102;0m";
    const RESET:  &str = "\x1b[0m";

    // Poem with only selected letters wrapped in ANSI color
    let poem = format!(
"Registers under s{orange}h{reset}adowed threads,
Units signal t{orange}i{reset}mes return,
Structures trace res{orange}p{reset}lendent unknowns,
Tokens recode un{orange}i{reset}versal speech.

Realms unfold s{orange}e{reset}amless trains,
Universe spins, th{orange}r{reset}eads recur,
Signals transform, {orange}r{reset}unes unbound,
Truth reiterates unifi{orange}e{reset}d syntax.
",
        orange = ORANGE,
        reset = RESET
    );

    // Write the normal ANSI version (optional)
    std::fs::write("poem_ansi_utf8.txt", &poem)?;

    // Write the hex-encoded version
    let mut hex_file = File::create("poem_ansi_hex.txt")?;
    for (i, b) in poem.as_bytes().iter().enumerate() {
        write!(hex_file, "{:02X} ", b)?;          // two-digit hex per byte
        if (i + 1) % 16 == 0 {
            writeln!(hex_file)?;                  // break line every 16 bytes
        }
    }
    writeln!(
        hex_file,
        "\n# Each pair above is one UTF-8 byte of the ANSI-colored poem."
    )?;

    println!("Generated:");
    println!("  poem_ansi_utf8.txt  → visible text with ANSI colors");
    println!("  poem_ansi_hex.txt   → hex dump of the same data");
    Ok(())
}