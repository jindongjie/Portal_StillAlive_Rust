// tui_draw.rs
// Handles TUI drawing and layout for Still Alive credits
// All TUI/print/lyric/ascii/credit logic will be implemented here

use crossterm::{
    cursor,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use std::io::{self, Write};
use std::thread;
use std::time::{Duration, Instant};

use crate::data::{ASCII_ART, CREDITS};

// Global state for drawing
static mut IS_DRAW_END: bool = false;
static mut CURSOR_X: u16 = 1;
static mut CURSOR_Y: u16 = 1;

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

// Terminal dimensions and layout
#[derive(Clone)]
pub struct TerminalLayout {
    pub credits_width: u16,
    pub credits_height: u16,
    pub lyric_width: u16,
    pub lyric_height: u16,
    pub credits_pos_x: u16,
    pub ascii_art_x: u16,
    pub ascii_art_y: u16,
}

impl TerminalLayout {
    pub fn new() -> Self {
        let (columns, lines) = terminal::size().unwrap_or((80, 24));

        if columns < 80 {
            println!("This program required minimum 80 * 80 character array on your screen.");
        }
        let ascii_art_width = 40;
        let ascii_art_height = 20;
        let credits_width = std::cmp::min((columns - 4) / 2, 56);
        let credits_height = lines - ascii_art_height - 2;
        let lyric_width = columns - 4 - credits_width;
        let lyric_height = lines - 2;
        let credits_pos_x = lyric_width + 4;
        let ascii_art_x = lyric_width + 4 + (credits_width - ascii_art_width) / 2;
        let ascii_art_y = credits_height + 3;

        Self {
            credits_width,
            credits_height,
            lyric_width,
            lyric_height,
            credits_pos_x,
            ascii_art_x,
            ascii_art_y,
        }
    }
}

pub fn begin_draw() -> io::Result<()> {
    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(SetForegroundColor(Color::Yellow))?;
    stdout.execute(SetBackgroundColor(Color::Black))?;
    stdout.flush()?;
    Ok(())
}

pub fn end_draw() -> io::Result<()> {
    unsafe {
        IS_DRAW_END = true;
    }
    let mut stdout = io::stdout();
    stdout.execute(ResetColor)?;
    stdout.execute(LeaveAlternateScreen)?;
    stdout.flush()?;
    Ok(())
}

pub fn move_cursor(x: u16, y: u16) -> io::Result<()> {
    let mut stdout = io::stdout();
    stdout.execute(cursor::MoveTo(x - 1, y - 1))?;
    unsafe {
        CURSOR_X = x;
        CURSOR_Y = y;
    }
    stdout.flush()?;
    Ok(())
}

pub fn clear_screen() -> io::Result<()> {
    let mut stdout = io::stdout();
    stdout.execute(Clear(ClearType::All))?;
    unsafe {
        CURSOR_X = 1;
        CURSOR_Y = 1;
    }
    stdout.flush()?;
    Ok(())
}

pub fn print_at(text: &str, newline: bool) -> io::Result<()> {
    let mut stdout = io::stdout();
    if newline {
        stdout.execute(Print(text))?;
        stdout.execute(Print("\n"))?;
        unsafe {
            CURSOR_X = 1;
            CURSOR_Y += 1;
        }
    } else {
        stdout.execute(Print(text))?;
        unsafe {
            CURSOR_X += text.len() as u16;
        }
    }
    stdout.flush()?;
    Ok(())
}

pub fn draw_frame(layout: &TerminalLayout) -> io::Result<()> {
    move_cursor(1, 1)?;

    // Top border
    let top_line = format!(
        " {} {} ",
        "-".repeat(layout.lyric_width as usize),
        "-".repeat(layout.credits_width as usize)
    );
    print_at(&top_line, true)?;

    // Credits area borders
    for _ in 0..layout.credits_height {
        let line = format!(
            "|{}||{}|",
            " ".repeat(layout.lyric_width as usize),
            " ".repeat(layout.credits_width as usize)
        );
        print_at(&line, true)?;
    }

    // Middle border
    let middle_line = format!(
        "|{}| {} ",
        " ".repeat(layout.lyric_width as usize),
        "-".repeat(layout.credits_width as usize)
    );
    print_at(&middle_line, true)?;

    // Remaining lyric area
    for _ in 0..(layout.lyric_height - 1 - layout.credits_height) {
        let line = format!("|{}|", " ".repeat(layout.lyric_width as usize));
        print_at(&line, true)?;
    }

    // Bottom border
    let bottom_line = format!(" {} ", "-".repeat(layout.lyric_width as usize));
    print_at(&bottom_line, false)?;

    move_cursor(2, 2)?;
    thread::sleep(Duration::from_millis(1000));

    Ok(())
}

pub fn clear_lyrics(layout: &TerminalLayout) -> io::Result<()> {
    move_cursor(1, 2)?;
    for _ in 0..layout.lyric_height {
        let line = format!("|{}", " ".repeat(layout.lyric_width as usize));
        print_at(&line, true)?;
    }
    move_cursor(2, 2)?;
    Ok(())
}

pub fn draw_lyrics(text: &str, x: u16, y: u16, interval: f32, newline: bool) -> io::Result<u16> {
    let mut current_x = x;
    move_cursor(current_x + 2, y + 2)?;

    for ch in text.chars() {
        move_cursor(current_x + 2, y + 2)?;
        print_at(&ch.to_string(), false)?;
        thread::sleep(Duration::from_secs_f32(interval));
        current_x += 1;
    }

    if newline {
        current_x = 0;
        move_cursor(current_x + 2, y + 3)?;
    }

    Ok(current_x)
}

pub fn draw_ascii_art(layout: &TerminalLayout, art_index: usize) -> io::Result<()> {
    if art_index >= ASCII_ART.len() {
        return Ok(());
    }

    let art = ASCII_ART[art_index];
    for (dy, line) in art.iter().enumerate() {
        move_cursor(layout.ascii_art_x, layout.ascii_art_y + dy as u16)?;
        print_at(line, false)?;
        thread::sleep(Duration::from_millis(10));
    }
    Ok(())
}

pub fn start_credits(layout: TerminalLayout) {
    thread::spawn(move || {
        let mut credit_x = 0;
        let mut i = 0;
        let length = CREDITS.len();
        let mut last_credits: Vec<String> = vec!["".to_string()];
        let start_time = Instant::now();

        for ch in CREDITS.chars() {
            let current_time =
                start_time + Duration::from_secs_f64(174.0 / length as f64 * i as f64);
            i += 1;

            if ch == '\n' {
                credit_x = 0;
                last_credits.push("".to_string());
                if last_credits.len() > layout.credits_height as usize {
                    last_credits = last_credits
                        [(last_credits.len() - layout.credits_height as usize)..]
                        .to_vec();
                }

                unsafe {
                    if IS_DRAW_END {
                        break;
                    }
                }

                // Clear and redraw credits area
                for y in 2..(2 + layout.credits_height - last_credits.len() as u16) {
                    let _ = move_cursor(layout.credits_pos_x, y);
                    let _ = print_at(&" ".repeat(layout.credits_width as usize), false);
                }

                for (k, line) in last_credits.iter().enumerate() {
                    let y = 2 + layout.credits_height - last_credits.len() as u16 + k as u16;
                    let _ = move_cursor(layout.credits_pos_x, y);
                    let _ = print_at(line, false);
                    let padding = layout.credits_width as usize - line.len();
                    if padding > 0 {
                        let _ = print_at(&" ".repeat(padding), false);
                    }
                }

                unsafe {
                    let _ = move_cursor(CURSOR_X, CURSOR_Y);
                }
            } else {
                if let Some(last_line) = last_credits.last_mut() {
                    last_line.push(ch);
                }

                unsafe {
                    if IS_DRAW_END {
                        break;
                    }
                }

                let _ = move_cursor(layout.credits_pos_x + credit_x, layout.credits_height + 1);
                let _ = print_at(&ch.to_string(), false);

                unsafe {
                    let _ = move_cursor(CURSOR_X, CURSOR_Y);
                }

                credit_x += 1;
            }

            while Instant::now() < current_time {
                thread::sleep(Duration::from_millis(10));
            }
        }
    });
}
