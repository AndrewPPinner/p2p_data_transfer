use std::net::IpAddr;

use crate::word_list;

const WORD_BIT_SIZES: [i32; 5] = [13, 13, 13, 13, 12];

pub fn encode(pub_ip: IpAddrWrapper, local_ip: IpAddrWrapper, port: u16, nonce: u8) -> String {
    let ip_octets = pub_ip.get_octets();
    let local_last_octet = local_ip.get_octets()[3];
    let port_bytes = port.to_be_bytes();

    let bytes = [
        ip_octets[0] ^ nonce,
        ip_octets[1] ^ nonce,
        ip_octets[2] ^ nonce,
        ip_octets[3] ^ nonce,
        local_last_octet ^ nonce,
        port_bytes[0] ^ nonce,
        port_bytes[1] ^ nonce,
        nonce,
    ];

    let bits = u64::from_be_bytes(bytes);

    let mut words = Vec::new();
    let mut bit_position = 64;

    for &bits_in_word in &WORD_BIT_SIZES {
        bit_position -= bits_in_word;
        let mask = (1u64 << bits_in_word) - 1;
        let index = ((bits >> bit_position) & mask) as usize;
        words.push(word_list::WORDS_LIST[index % 7643]);
    }

    return words.join("-");
}

pub fn decode(code: &str) -> String {
    let words: Vec<&str> = code.split('-').collect();
    let mut bits: u64 = 0;
    let mut bit_position = 64;

    for (word, &bits_in_word) in words.iter().zip(&WORD_BIT_SIZES) {
        bit_position -= bits_in_word;

        let index = word_list::WORDS_LIST
            .iter()
            .position(|&w| w == *word)
            .ok_or(format!("Unknown word: {}", word))
            .expect("Decoding failed");

        bits |= (index as u64) << bit_position;
    }

    let bytes = bits.to_be_bytes();
    let nonce = bytes[7];
    let public_ip_bytes = [
        bytes[0] ^ nonce,
        bytes[1] ^ nonce,
        bytes[2] ^ nonce,
        bytes[3] ^ nonce,
    ];

    let local_last_octet = bytes[4] ^ nonce;
    let port = u16::from_be_bytes([bytes[5] ^ nonce, bytes[6] ^ nonce]);

    return format!(
        "IP {:?} | local {} | port {} | nonce {}",
        public_ip_bytes, local_last_octet, port, nonce
    );
}

pub struct IpAddrWrapper {
    ip: IpAddr,
}

impl IpAddrWrapper {
    pub fn new(addr: IpAddr) -> Self {
        return IpAddrWrapper { ip: addr };
    }

    pub fn get_octets(&self) -> [u8; 4] {
        return match self.ip {
            IpAddr::V4(ipv4_addr) => ipv4_addr.octets(),
            _ => panic!("IPv6 not supported"),
        };
    }
}
