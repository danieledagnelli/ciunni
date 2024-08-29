pub fn format_chunk_hex_values(chunk: &[u8]) -> Vec<String> {
    chunk.iter().map(|b| format!("{:02X}", b)).collect()
}

pub fn format_chunk_ascii_values(chunk: &[u8]) -> String {
    chunk
        .iter()
        .map(|&b| if b.is_ascii_graphic() { b as char } else { '.' })
        .collect()
}

pub fn format_chunk(chunk: &[u8]) -> (String, String) {
    let mut hex_string = String::with_capacity(16 * 3);
    let mut ascii_string = String::with_capacity(16);

    for &byte in chunk {
        hex_string.push_str(&format!("{:02X} ", byte));
        ascii_string.push(if byte.is_ascii_graphic() {
            byte as char
        } else {
            '.'
        });
    }

    // Pad incomplete rows
    for _ in chunk.len()..16 {
        hex_string.push_str("   ");
        ascii_string.push(' ');
    }

    (hex_string, ascii_string)
}
