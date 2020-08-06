// Print the header, consisting of the following fields: Decimal, Hexadecimal, Octal, HTML, and ASCII character symbol.
fn print_header() {
    println!("{}\t{}\t{}\t{}\t{}", "Dec", "Hex", "Oct", "HTML", "Char");
}

// Generate the characters in the forms: Decimal, Hexadecimal, Octal, HTML, and ASCII
fn gen_chars() {
    for i in 32..=126 as u8 {
        println!("{}\t{:X}\t{:o}\t&#{:03}\t{}", i, i, i, i, i as char);
    }
}

fn main() {
    print_header();
    gen_chars();
}
