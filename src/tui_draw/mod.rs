// tui_draw.rs
// Handles TUI drawing and layout for Still Alive credits
// All TUI/print/lyric/ascii/credit logic will be implemented here

pub struct Lyric {
    pub words: String,
    pub time: u32,
    pub interval: f32,
    pub mode: u8,
}

impl Lyric {
    pub fn new(words: &str, time: u32, interval: f32, mode: u8) -> Self {
        Self {
            words: words.to_string(),
            time,
            interval,
            mode,
        }
    }
}

// Unimplemented: draw the TUI frame
pub fn draw_frame() {
    unimplemented!("draw_frame is not yet implemented");
}

// Unimplemented: clear the lyrics area
pub fn clear_lyrics() {
    unimplemented!("clear_lyrics is not yet implemented");
}

// Unimplemented: draw lyrics at (x, y)
pub fn draw_lyrics(_text: &str, _x: usize, _y: usize, _interval: f32, _newline: bool) {
    unimplemented!("draw_lyrics is not yet implemented");
}

// Unimplemented: draw ASCII art at (x, y)
pub fn draw_ascii_art(_x: usize, _y: usize, _art_index: usize) {
    unimplemented!("draw_ascii_art is not yet implemented");
}

// Unimplemented: start credits thread
pub fn start_credits() {
    unimplemented!("start_credits is not yet implemented");
}
