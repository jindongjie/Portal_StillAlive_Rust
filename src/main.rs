use rodio::Decoder;
use rodio::OutputStream;
use rodio::Sink;
use std::io::Cursor;
use std::time::{Duration, Instant};

mod data;
mod tui_draw;

use data::get_lyrics;
use tui_draw::{
    begin_draw, clear_lyrics, clear_screen, draw_ascii_art, draw_frame, draw_lyrics, end_draw,
    move_cursor, start_credits, TerminalLayout,
};
const MP3_DATA: &[u8] = include_bytes!("../music/ending.mp3");

fn main() {
    // Set up Ctrl+C handler,able to quit program
    ctrlc::set_handler(|| {
        let _ = tui_draw::end_draw();
        println!("Interrupt by user");
        std::process::exit(0);
    })
    .expect("Unable to exit with ctrl+C pressed!");

    // Initialize terminal and layout
    let layout = TerminalLayout::new();

    // Begin drawing setup
    if let Err(e) = begin_draw() {
        eprintln!("Error setting up terminal: {}", e);
        return;
    }

    // Clear screen and draw frame
    if let Err(e) = clear_screen() {
        eprintln!("Error clearing screen: {}", e);
        return;
    }

    if let Err(e) = draw_frame(&layout) {
        eprintln!("Error drawing frame: {}", e);
        return;
    }

    // Play the background song
    if let Ok((_stream, stream_handle)) = OutputStream::try_default() {
        // Try to create a sink (audio output)
        if let Ok(sink) = Sink::try_new(&stream_handle) {
            // Wrap the bytes in a Cursor so Decoder can read them
            let cursor = Cursor::new(MP3_DATA);
            // Try to create a Decoder (audio source)
            if let Ok(source) = Decoder::new(cursor) {
                sink.append(source);
                sink.sleep_until_end();
                println!("Audio initialized successfully");
            } else {
                println!("Failed to decode MP3 data");
            }
        } else {
            println!("Could not create audio sink, continuing without audio");
        }
    } else {
        println!("No audio device available, continuing without audio");
    }
    // Main lyrics processing loop
    let lyrics = get_lyrics();
    let start_time = Instant::now();
    let mut current_lyric = 0;
    let mut x = 0u16;
    let mut y = 0u16;

    while current_lyric < lyrics.len() && lyrics[current_lyric].mode != 9 {
        let current_time = start_time.elapsed().as_millis() as u32 * 10; // Convert to centiseconds

        if current_time > lyrics[current_lyric].time {
            let lyric = &lyrics[current_lyric];

            // Calculate interval
            let word_count = if lyric.mode <= 1 || lyric.mode >= 5 {
                std::cmp::max(lyric.words.len(), 1)
            } else {
                1
            };

            let interval = if lyric.interval < 0.0 {
                if current_lyric + 1 < lyrics.len() {
                    (lyrics[current_lyric + 1].time - lyric.time) as f32 / 100.0 / word_count as f32
                } else {
                    0.1
                }
            } else {
                lyric.interval / word_count as f32
            };

            match lyric.mode {
                0 => {
                    // Lyric with newline
                    if let Ok(new_x) = draw_lyrics(&lyric.words, x, y, interval, true) {
                        x = new_x;
                        y += 1;
                    }
                }
                1 => {
                    // Lyric without newline
                    if let Ok(new_x) = draw_lyrics(&lyric.words, x, y, interval, false) {
                        x = new_x;
                    }
                }
                2 => {
                    // ASCII art
                    if let Ok(art_index) = lyric.words.parse::<usize>() {
                        let _ = draw_ascii_art(&layout, art_index);
                        let _ = move_cursor(x + 2, y + 2);
                    }
                }
                3 => {
                    // Clear lyrics
                    let _ = clear_lyrics(&layout);
                    x = 0;
                    y = 0;
                }
                4 => {
                    // Start music (already started)
                    // println!("Music should start here");
                }
                5 => {
                    // Start credits
                    start_credits(layout.clone());
                }
                _ => {}
            }

            current_lyric += 1;
        }

        std::thread::sleep(Duration::from_millis(10));
    }

    // Wait a bit before cleanup
    std::thread::sleep(Duration::from_secs(2));

    // Cleanup
    if let Err(e) = end_draw() {
        eprintln!("Error cleaning up terminal: {}", e);
    }
}
