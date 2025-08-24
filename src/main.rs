use rodio::Decoder;
use rodio::OutputStream;
use rodio::Sink;
use std::io::Cursor;
use std::thread;
use std::time::{Duration, Instant};

mod data;
mod tui_draw;

use data::get_lyrics;
use tui_draw::{
    begin_draw, clear_lyrics, clear_screen, draw_ascii_art, draw_frame, draw_lyrics, end_draw,
    move_cursor, start_credits, TerminalLayout,
};

const MP3_CONSTANT: &[u8] = include_bytes!("../music/ending.mp3");

fn main() {
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

    // Clear screen
    if let Err(e) = clear_screen() {
        eprintln!("Error clearing screen: {}", e);
        return;
    }

    // Draw frame
    if let Err(e) = draw_frame(&layout) {
        eprintln!("Error drawing frame: {}", e);
        return;
    }

    // Main lyrics processing loop
    let lyrics = get_lyrics();
    let start_time = Instant::now();
    let mut current_lyric = 0;
    let mut x = 0u16;
    let mut y = 0u16;

    //Print out lyric line-by-line
    while current_lyric < lyrics.len() && lyrics[current_lyric].mode != 9 {
        let current_time = start_time.elapsed().as_millis() as u32 / 10;
        //Each line of lyric have it own "timestamp", line will start printing again when "current_time" pass it
        if current_time > lyrics[current_lyric].time {
            let lyric = &lyrics[current_lyric];

            // Calculate interval for each character
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
                    start_wonderful_music(MP3_CONSTANT);
                }
                5 => {
                    start_credits(layout.clone());
                }
                _ => {}
            }

            current_lyric += 1;
        }

        std::thread::sleep(Duration::from_millis(1));
    }

    // Wait a bit before cleanup
    std::thread::sleep(Duration::from_secs(2));

    // Cleanup
    if let Err(e) = end_draw() {
        eprintln!("Error cleaning up terminal: {}", e);
    }

    fn start_wonderful_music(mp3_data: &'static [u8]) {
        thread::spawn(move || {
            let (_stream, stream_handle) = OutputStream::try_default().expect("No output device");
            Sink::try_new(&stream_handle).expect("Failed to create Sink1");
            let sink = Sink::try_new(&stream_handle).expect("Failed to create Sink2");
            let cursor = Cursor::new(mp3_data);
            let source = Decoder::new(cursor).expect("Failed to decode MP3 data");
            sink.append(source);
            sink.sleep_until_end(); // Keep the thread alive while music play
        });
    }
}
