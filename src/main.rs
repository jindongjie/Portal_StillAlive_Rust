use std::thread::sleep;

use crossterm::{
    event::{self, Event},
    style::Print,
};
use ratatui::{text::Text, Frame};

mod tui_draw;
use tui_draw::{Lyric};

fn main() {
    //Play the background song~
    use rodio::source::{SineWave, Source};
    use rodio::{Decoder, OutputStream, Sink};
    use std::fs::File;
    use std::io::BufReader;
    use std::time::Duration;

    // _stream must live as long as the sink
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    // Add a dummy source of the sake of the example.
    let source = SineWave::new(686.3).amplify(0.22);
    // Use sink to let sound "silence" play at background
    sink.append(source);
    // TODO: Replace with actual music file playback
    // sink.append(Decoder::new(BufReader::new(File::open("music/ending.mp3").unwrap())).unwrap());
    // sink.sleep_until_end();

    ctrlc::set_handler(|| {
        // TODO: Call tui_draw::end_draw() or cleanup
        std::process::exit(0);
    }).expect("Unable to exit with ctrl+C pressed!");

    // TODO: Initialize TUI, draw frame, handle lyrics, credits, ascii art, etc.
    // tui_draw::draw_frame();
    // tui_draw::draw_lyrics(...);
    // tui_draw::draw_ascii_art(...);
    // tui_draw::start_credits();

    // Placeholder event loop
    sleep(Duration::from_secs(5));
    // let mut terminal = ratatui::init();
    // loop {
    //     terminal.draw(draw).expect("failed to draw frame");
    //     if matches!(event::read().expect("failed to read event"), Event::Key(_)) {
    //         break;
    //     }
    // }
    // ratatui::restore();
}

fn draw(frame: &mut Frame) {
    let text = Text::raw("Hello World!");
    frame.render_widget(text, frame.area());
}
