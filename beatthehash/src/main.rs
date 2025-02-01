//! Participation in the hashing contest from https://beatthehash.com/
use std::time::Duration;

use hex_literal::hex;
use indicatif::{ProgressBar, ProgressStyle};
use skein::{consts::U128, Digest, Skein1024};

const TARGET: [u8; 128] = hex!("5b4da95f5fa08280fc9879df44f418c8f9f12ba424b7757de02bbdfbae0d4c4fdf9317c80cc5fe04c6429073466cf29706b8c25999ddd2f6540d4475cc977b87f4757be023f19b8f4035d7722886b78869826de916a79cf9c94cc79cd4347d24b567aa3e2390a573a373a48a5e676640c79cc70197e1c5e7f902fb53ca1858b6");

const START_CHAR: u8 = 0x20; // space (first readable ascii)
const END_CHAR: u8 = 0x7e; // ~ (last readable ascii)

pub fn increment_buffer(buffer: &mut Vec<u8>) {
    for byte in buffer.iter_mut() {
        if *byte < END_CHAR {
            *byte += 1;
            return;
        }

        *byte = START_CHAR; // space (first readable ascii)
    }

    buffer.push(START_CHAR)
}

fn main() {
    let progress = ProgressBar::new_spinner().with_style(
        ProgressStyle::default_spinner()
            .template("{spinner} {per_sec}")
            .unwrap(),
    );

    progress.enable_steady_tick(Duration::from_millis(100));
    let mut best = u64::MAX;
    let mut curr = Vec::new();

    assert_eq!(
        hamming::distance(b"\0a", &TARGET),
        hamming::distance(b"a", &TARGET)
    );

    loop {
        let mut hasher = Skein1024::<U128>::new();
        hasher.update(&curr);
        let hash = hasher.finalize();
        let dist = hamming::distance(&TARGET, &hash);

        if dist < best {
            progress.println(format!("{best}->{dist} {}", String::from_utf8_lossy(&curr)));
            best = dist;
        }

        increment_buffer(&mut curr);
        progress.inc(1);
    }
}
