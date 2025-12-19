use std::convert::TryInto;
use std::fmt::Write;
use std::sync::{Arc, Mutex};
use std::thread;

/// RFC 4122 Namespace DNS: 6ba7b810-9dad-11d1-80b4-00c04fd430c8
const NS_DNS: [u8; 16] = [
    0x6b, 0xa7, 0xb8, 0x10, 0x9d, 0xad, 0x11, 0xd1,
    0x80, 0xb4, 0x00, 0xc0, 0x4f, 0xd4, 0x30, 0xc8
];

/// Minimal SHA-1 implementation for UUID v5 generation
fn sha1(data: &[u8]) -> [u8; 20] {
    let mut h: [u32; 5] = [0x67452301, 0xEFCDAB89, 0x98BADCFE, 0x10325476, 0xC3D2E1F0];
    let mut padded = data.to_vec();
    let bit_len = (padded.len() as u64) * 8;
    padded.push(0x80);
    while (padded.len() + 8) % 64 != 0 { padded.push(0); }
    padded.extend_from_slice(&bit_len.to_be_bytes());

    for chunk in padded.chunks(64) {
        let mut w = [0u32; 80];
        for i in 0..16 {
            w[i] = u32::from_be_bytes(chunk[i*4..i*4+4].try_into().unwrap());
        }
        for i in 16..80 {
            w[i] = (w[i-3] ^ w[i-8] ^ w[i-14] ^ w[i-16]).rotate_left(1);
        }
        let (mut a, mut b, mut c, mut d, mut e) = (h[0], h[1], h[2], h[3], h[4]);
        for i in 0..80 {
            let (f, k) = match i {
                0..=19 => ((b & c) | ((!b) & d), 0x5A827999),
                20..=39 => (b ^ c ^ d, 0x6ED9EBA1),
                40..=59 => ((b & c) | (b & d) | (c & d), 0x8F1BBCDC),
                _ => (b ^ c ^ d, 0xCA62C1D6),
            };
            let temp = a.rotate_left(5).wrapping_add(f).wrapping_add(e).wrapping_add(k).wrapping_add(w[i]);
            e = d; d = c; c = b.rotate_left(30); b = a; a = temp;
        }
        h[0] = h[0].wrapping_add(a); h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c); h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
    }
    let mut out = [0u8; 20];
    for i in 0..5 { out[i*4..i*4+4].copy_from_slice(&h[i].to_be_bytes()); }
    out
}

/// Manual UUID v5 creation (RFC 4122)
fn make_v5(name: &str) -> String {
    let mut input = NS_DNS.to_vec();
    input.extend_from_slice(name.as_bytes());
    let hash = sha1(&input);
    let mut uuid = [0u8; 16];
    uuid.copy_from_slice(&hash[..16]);

    uuid[6] = (uuid[6] & 0x0f) | 0x50; // Version 5
    uuid[8] = (uuid[8] & 0x3f) | 0x80; // Variant RFC 4122

    format!(
        "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        uuid[0], uuid[1], uuid[2], uuid[3], uuid[4], uuid[5], uuid[6], uuid[7],
        uuid[8], uuid[9], uuid[10], uuid[11], uuid[12], uuid[13], uuid[14], uuid[15]
    )
}

fn main() {
    let bases: &'static [&'static str] = &[
        "deadbeef", "deadcode", "deadface", "deadloop", "deadlock",
        "codebase", "database", "datafile", "baseline", "metadata",
        "stateless", "statelists", "idleness", "idledaze",
        "allocated", "deallocate", "isolated", "disposal",
        "alphabet", "algebraist", "categories", "applicates",
        "self", "eval", "sicp", "lisp", "spell", "staff", "cast",
        "safe", "seal", "spec", "goal", "root", "host", "page",
    ];

    let mut dictionary = Vec::new();
    for &base in bases {
        let h: String = base.chars().filter_map(|c| match c.to_ascii_lowercase() {
            k @ ('a'..='f' | '0'..='9') => Some(k),
            'o' => Some('0'), 'i' | 'l' => Some('1'), 's' => Some('5'), 't' => Some('7'),
            _ => None,
        }).collect();
        if h.len() >= 4 { dictionary.push((h, base)); }
    }

    let dictionary = Arc::new(dictionary);
    let global_high_score = Arc::new(Mutex::new(0));
    let num_threads = thread::available_parallelism().map(|n| n.get()-1).unwrap_or(4);

    println!("💎 Mining for Aesthetics: {} threads seeking high-impact phrases...", num_threads);

    let mut handles = vec![];
    for t_id in 0..num_threads {
        let dict = Arc::clone(&dictionary);
        let high_score = Arc::clone(&global_high_score);

        handles.push(thread::spawn(move || {
            let mut local_name_buf = String::with_capacity(64);
            let mut local_max = 0;

            for i in (t_id as u64..u64::MAX).step_by(num_threads) {
                local_name_buf.clear();
                let base = bases[i as usize % bases.len()];
                let _ = write!(&mut local_name_buf, "{}:{}", base, i);

                let s = make_v5(&local_name_buf);
                let flat = s.replace('-', "");

                let mut current_score = 0;
                for (hex, _) in dict.iter() {
                    if let Some(pos) = flat.find(hex) {
                        let mut word_val = (hex.len() * hex.len()) as u32;

                        if pos == 0 || pos + hex.len() == 32 {
                            word_val *= 10;
                        }

                        for split_at in 1..hex.len() {
                            let mut test_word = hex.clone();
                            test_word.insert(split_at, '-');
                            if s.contains(&test_word) {
                                word_val *= 5;
                                break;
                            }
                        }
                        current_score += word_val;
                    }
                }

                if current_score > local_max {
                    local_max = current_score;
                    let mut global = high_score.lock().unwrap();
                    if current_score > *global {
                        *global = current_score;
                        println!("\n🔥 NEW AESTHETIC RECORD: Score {}", current_score);
                        println!("uuid!(\"{}\") @ {}\n", s, local_name_buf);
                    }
                }
            }
        }));
    }
    for h in handles { h.join().unwrap(); }
}