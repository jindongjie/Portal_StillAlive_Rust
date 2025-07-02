use std::thread::sleep;

use crossterm::{
    event::{self, Event},
    style::Print,
};
use ratatui::{text::Text, Frame};

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
    sink.sleep_until_end();

    ctrlc::set_handler(|| {}).expect("Unable to exit with ctrl+C pressed!");

    sleep(Duration::from_secs(5));
    let mut terminal = ratatui::init();
    loop {
        terminal.draw(draw).expect("failed to draw frame");
        if matches!(event::read().expect("failed to read event"), Event::Key(_)) {
            break;
        }
    }
    ratatui::restore();
}

fn draw(frame: &mut Frame) {
    let text = Text::raw("Hello World!");
    frame.render_widget(text, frame.area());
}
