use std::thread::sleep;
use std::time::Duration;

mod tui_draw;
use tui_draw::{Lyric, TerminalLayout, begin_draw, end_draw, clear_screen, draw_frame};

fn main() {
    // Set up Ctrl+C handler
    ctrlc::set_handler(|| {
        let _ = tui_draw::end_draw();
        println!("Interrupt by user");
        std::process::exit(0);
    }).expect("Unable to exit with ctrl+C pressed!");

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

    // Play the background song (optional - handle audio device not available)
    if let Ok((_stream, stream_handle)) = rodio::OutputStream::try_default() {
        if let Ok(sink) = rodio::Sink::try_new(&stream_handle) {
            use rodio::source::{SineWave, Source};
            // Add a dummy source for the sake of the example
            let source = SineWave::new(686.3).amplify(0.22);
            sink.append(source);
            // TODO: Replace with actual music file playback
            // sink.append(Decoder::new(BufReader::new(File::open("music/ending.mp3").unwrap())).unwrap());
            // sink.sleep_until_end();
            println!("Audio initialized successfully");
        } else {
            println!("Could not create audio sink, continuing without audio");
        }
    } else {
        println!("No audio device available, continuing without audio");
    }

    // Placeholder main loop - for now just wait 10 seconds to demonstrate
    sleep(Duration::from_secs(10));
    
    // TODO: Implement the full lyrics processing loop
    // This would include:
    // - Initialize lyrics data array
    // - Main timing loop
    // - Process different lyric modes (text, ASCII art, clear, etc.)
    // - Handle credits thread
    
    // Cleanup
    if let Err(e) = end_draw() {
        eprintln!("Error cleaning up terminal: {}", e);
    }
}
