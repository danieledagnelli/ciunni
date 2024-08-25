use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("C:\\Users\\danie\\OneDrive\\books\\Hoffman, Kevin - Programming WebAssembly with Rust_ Unified Development for Web, Mobile, and Embedded Applications (2019, Pragmatic Programmers, LLC, The) - libgen.li.pdf")
        .expect("Failed to open file");

    let mut contents = Vec::new();
    file.read_to_end(&mut contents)
        .expect("Failed to read file");

    // println!("{:?}", contents);

    if is_pdf(&contents) {
        println!("The file is a PDF");
    } else {
        println!("The file is not a PDF");
    }

    println!("First 4 bytes of contents: {:02X?}", &contents[..4]);
}

fn is_pdf(contents: &[u8]) -> bool {
    if contents.len() < 4 {
        return false;
    }

    let magic_number: [u8; 4] = [0x25, 0x50, 0x44, 0x46]; // "%PDF" in hexadecimal

    contents[..4] == magic_number
}
