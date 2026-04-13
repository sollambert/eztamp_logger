use std::io::Bytes;

pub mod time;


pub struct Hex;

impl Hex {
    pub(crate) fn encode(bytes: Bytes<&[u8]>) -> String {
        bytes
            .map(|b| format!("{:02x}", b.unwrap()))
            .collect()
    }

    pub(crate) fn decode(hex: &str) -> Result<Vec<u8>, String> {
        if hex.len() % 2 != 0 {
            return Err("hex string must have even length".into());
        }

        let mut bytes = Vec::with_capacity(hex.len() / 2);

        let chars: Vec<char> = hex.chars().collect();

        for i in (0..hex.len()).step_by(2) {
            let high = chars[i].to_digit(16)
                .ok_or_else(|| format!("invalid hex char: {}", chars[i]))?;
            let low = chars[i + 1].to_digit(16)
                .ok_or_else(|| format!("invalid hex char: {}", chars[i + 1]))?;

            bytes.push(((high << 4) | low) as u8);
        }

        Ok(bytes)
    }
}