use clearscreen::clear;
use rand::Rng;
use rodio::{source::Source, Decoder, OutputStream};

use std::io::{self, Write};
use std::thread;
use std::time::Duration;
fn main() {
    looper(get_parameters());
}
fn get_parameters() -> Vec<u64> {
    let default_parameters = vec![600, 1200, 10];
    let mut parameters = default_parameters.clone();
    println!("Enter min(600), max(1200), gap(10) in seconds");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    clear().expect("failed to clear screen");
    let input: Vec<&str> = input.split(' ').collect();
    for i in 0..input.len().min(parameters.len()) {
        match input[i].trim().parse() {
            Ok(value) => parameters[i] = value,
            Err(_) => parameters[i] = default_parameters[i],
        }
    }
    parameters
}
fn looper(parameters: Vec<u64>) {
    let min = parameters[0];
    let max = parameters[1];
    let gap = parameters[2];
    loop {
        clear().expect("failed to clear screen");
        let settings = format!("min: {min}| max: {max}| gap: {gap}");
        let countdown_time = generate_random_time(min, max);
        let gap_countdown = gap;
        start_countdown(countdown_time, &settings);
        sound_player(1);
        clear().expect("failed to clear screen");
        start_countdown(gap_countdown, &settings);
        print!("0...\r");
        sound_player(2);
    }
}

const BELL_MP3: &[u8] = include_bytes!("bell.mp3");
const BELL_DOUBLE: &[u8] = include_bytes!("bell_double.mp3");
fn sound_player(countertype: u8) {
    if countertype == 1 {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();

        let cursor = io::Cursor::new(BELL_MP3);
        let source = Decoder::new(cursor).unwrap();
        stream_handle
            .play_raw(source.convert_samples())
            .expect("failed to play sound on device");
        std::thread::sleep(Duration::from_secs_f32(1.4));
    } else if countertype == 2 {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();

        let cursor = io::Cursor::new(BELL_DOUBLE);
        let source = Decoder::new(cursor).unwrap();

        stream_handle
            .play_raw(source.convert_samples())
            .expect("failed to play sound on device");
        std::thread::sleep(Duration::from_secs_f32(1.4));
    }
}

fn generate_random_time(min: u64, max: u64) -> u64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..=max)
}

fn start_countdown(seconds: u64, settings: &str) {
    clear().expect("failed to clear screen");
    println!("{settings}");
    println!("CountDown: {seconds} seconds");
    for i in (1..=seconds).rev() {
        print!("{i}...\r");
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs_f32(1.0));
    }
}
