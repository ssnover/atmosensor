const HEX_CHARS: &str = "ABCDEFabcdef0123456789";

pub fn is_hex_char(ch: char) -> bool {
    HEX_CHARS.contains(ch)
}

pub fn hex_char_to_val(ch: char) -> u8 {
    match ch {
        '0'..='9' => (ch as u8) - (b'0'),
        'A'..='F' => (ch as u8) - (b'A') + 10u8,
        'a'..='f' => (ch as u8) - (b'a') + 10u8,
        _ => unreachable!(),
    }
}

pub fn hex_str_to_bytes(hex_str: &[char]) -> Option<Vec<u8>> {
    if hex_str.len() % 2 == 0 {
        Some(
            hex_str
                .iter()
                .step_by(2)
                .zip(hex_str.iter().skip(1).step_by(2))
                .map(|(a, b)| (hex_char_to_val(*a) << 4) | hex_char_to_val(*b))
                .collect::<Vec<u8>>(),
        )
    } else {
        None
    }
}

pub fn nibble_to_hex_char(nibble: u8) -> u8 {
    assert_eq!(nibble, nibble & 0xF);
    match nibble {
        0..=9 => (b'0') + (nibble),
        0xa..=0xf => (b'a') + (nibble - 10),
        _ => unreachable!(),
    }
}

pub fn byte_to_hex_str(byte: u8) -> [u8; 2] {
    [
        nibble_to_hex_char(byte >> 4),
        nibble_to_hex_char(byte & 0xF),
    ]
}

pub fn bytes_to_hex_str(data: &[u8]) -> String {
    String::from_utf8(
        data.iter()
            .flat_map(|byte| byte_to_hex_str(*byte))
            .collect::<Vec<u8>>(),
    )
    .unwrap()
}
