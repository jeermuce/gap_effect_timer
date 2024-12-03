use clearscreen::clear;
use rand::Rng;
use rodio::{source::Source, Decoder, OutputStream};

use std::io::{self, Write};
use std::thread;
use std::time::Duration;

const DEFAULT_MIN: u64 = 600;
const DEFAULT_MAX: u64 = 1200;

fn main() {
    timer(get_parameters());
}

fn get_parameters() -> Vec<u64> {
    println!("Enter min(600), max(1200) in seconds: '600 1200'");
    let mut parameters = vec![DEFAULT_MIN, DEFAULT_MAX];
    let mut input = String::new();

    if let Err(e) = io::stdin().read_line(&mut input) {
        println!("Failed to read line: {e}");
        return parameters;
    }

    let input: Vec<&str> = input.split_whitespace().collect();
    for (i, &value) in input.iter().take(parameters.len()).enumerate() {
        match value.trim().parse() {
            Ok(num) => parameters[i] = num,
            Err(_) => {
                println!(
                    "Invalid input '{}', using default value: {}",
                    value, parameters[i]
                );
            }
        }
    }

    clear().expect("failed to clear screen");
    parameters
}

fn timer(parameters: Vec<u64>) {
    let min = parameters[0];
    let max = parameters[1];

    loop {
        clear().expect("failed to clear screen");
        let countdown_time = generate_random_time(min, max);
        let gap_countdown = countdown_time / 19;
        let settings =
            format!("min: {min} | max: {max}\nperiod: {countdown_time} | gap: {gap_countdown}");

        start_countdown(countdown_time, &settings);
        sound_selector(1);

        clear().expect("failed to clear screen");
        start_countdown(gap_countdown, &settings);
        sound_selector(2);
    }
}

const BELL_MP3: &[u8] = include_bytes!("bell.mp3");
const BELL_DOUBLE_MP3: &[u8] = include_bytes!("bell_double.mp3");

fn sound_selector(counter_type: u8) {
    match counter_type {
        1 => sound_player(BELL_MP3),
        2 => sound_player(BELL_DOUBLE_MP3),
        _ => panic!("Unknown sound selected"),
    }
}

fn sound_player(sound: &'static [u8]) {
    let (_stream, stream_handle) = OutputStream::try_default().expect("failed to get stream");
    let cursor = io::Cursor::new(sound);
    let source = Decoder::new(cursor).expect("failed to decode mp3");

    stream_handle
        .play_raw(source.convert_samples())
        .expect("failed to play sound on device");

    thread::sleep(Duration::from_secs_f32(1.4));
}

fn generate_random_time(min: u64, max: u64) -> u64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..=max)
}

fn start_countdown(seconds: u64, settings: &str) {
    clear().expect("failed to clear screen");
    println!("{settings}");

    for i in (1..=seconds).rev() {
        print!("{i}...\r");
        io::stdout().flush().expect("failed to flush stdout");
        thread::sleep(Duration::from_secs_f32(1.0));
    }
}
