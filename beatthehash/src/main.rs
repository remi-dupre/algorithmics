//! Participation in the hashing contest from https://beatthehash.com/
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use hex_literal::hex;
use indicatif::{ProgressBar, ProgressStyle};
use skein::{consts::U128, Digest, Skein1024};

const SAVE_FREQUENCY: u32 = 100_000;
const THREADS: u8 = 20;
const START_CHAR: u8 = 0x20; // space (first readable ascii)
const END_CHAR: u8 = 0x7e; // ~ (last readable ascii)

const TARGET: [u8; 128] = hex!("5b4da95f5fa08280fc9879df44f418c8f9f12ba424b7757de02bbdfbae0d4c4fdf9317c80cc5fe04c6429073466cf29706b8c25999ddd2f6540d4475cc977b87f4757be023f19b8f4035d7722886b78869826de916a79cf9c94cc79cd4347d24b567aa3e2390a573a373a48a5e676640c79cc70197e1c5e7f902fb53ca1858b6");

pub fn increment_buffer(buffer: &mut Vec<u8>) {
    // First byte is thread-specific prefix
    for byte in buffer[1..].iter_mut() {
        if *byte < END_CHAR {
            *byte += 1;
            return;
        }

        *byte = START_CHAR; // space (first readable ascii)
    }

    buffer.push(START_CHAR)
}

fn load(path: &Path) -> Option<Vec<u8>> {
    if let Ok(mut file) = File::open(path) {
        let mut res = Vec::new();
        file.read_to_end(&mut res).expect("failed to read file");
        Some(res)
    } else {
        None
    }
}

fn save(path: &Path, buffer: &[u8]) {
    if let Ok(mut file) = File::create(path) {
        file.write_all(buffer).ok();
    }
}

fn score(buffer: &[u8]) -> u64 {
    let mut hasher = Skein1024::<U128>::new();
    hasher.update(buffer);
    let hash = hasher.finalize();
    hamming::distance(&TARGET, &hash)
}

fn run_thread(thread_num: u8, progress: ProgressBar, shared_best: &Mutex<u64>) {
    let save_file = Path::new("state").join(format!("{thread_num}.txt"));
    let mut save_countdown = SAVE_FREQUENCY;
    let mut curr = load(&save_file).unwrap_or_else(|| vec![b'a' + thread_num]);
    let mut local_best = score(&curr);

    progress.println(format!(
        "({thread_num:02}) resume dist={local_best} value='{}'",
        String::from_utf8_lossy(&curr),
    ));

    loop {
        let dist = score(&curr);

        if dist < local_best {
            let mut best = shared_best.lock().expect("lock poisonned");

            if dist < *best {
                progress.println(format!(
                    "({thread_num:02}) dist={dist} value='{}'",
                    String::from_utf8_lossy(&curr)
                ));

                save(Path::new("state/best.txt"), &curr);
                *best = dist;
            }

            local_best = *best;
        }

        if save_countdown == 0 {
            save(&save_file, &curr);
            save_countdown = SAVE_FREQUENCY;
            progress.inc(SAVE_FREQUENCY.into());
        } else {
            save_countdown -= 1;
        }

        increment_buffer(&mut curr);
    }
}

fn main() {
    let shared_best = &Mutex::new(
        load(Path::new("state/best.txt"))
            .map(|buffer| {
                let best_score = score(&buffer);

                println!(
                    "resume from best dist={best_score} value='{}'",
                    String::from_utf8_lossy(&buffer)
                );

                best_score
            })
            .unwrap_or(u64::MAX),
    );

    let progress = ProgressBar::new_spinner().with_style(
        ProgressStyle::default_spinner()
            .template("{spinner} {per_sec}")
            .unwrap(),
    );

    progress.enable_steady_tick(Duration::from_millis(100));

    thread::scope(|s| {
        for thread_num in 0..THREADS {
            let progress = progress.clone();
            s.spawn(move || run_thread(thread_num, progress, shared_best));
        }
    })
}
