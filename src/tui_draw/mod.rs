// tui_draw.rs
// Handles TUI drawing and layout for Still Alive credits
// All TUI/print/lyric/ascii/credit logic will be implemented here

use crossterm::{
    cursor,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand, QueueableCommand,
};
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

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
pub struct TerminalLayout {
    pub columns: u16,
    pub lines: u16,
    pub ascii_art_width: u16,
    pub ascii_art_height: u16,
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
            columns,
            lines,
            ascii_art_width,
            ascii_art_height,
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
    let top_line = format!(" {} {} ", 
        "-".repeat(layout.lyric_width as usize), 
        "-".repeat(layout.credits_width as usize)
    );
    print_at(&top_line, true)?;
    
    // Credits area borders
    for _ in 0..layout.credits_height {
        let line = format!("|{}||{}|", 
            " ".repeat(layout.lyric_width as usize),
            " ".repeat(layout.credits_width as usize)
        );
        print_at(&line, true)?;
    }
    
    // Middle border
    let middle_line = format!("|{}| {} ", 
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

// ASCII art patterns (from Python script)
const ASCII_ART: &[&[&str]] = &[
    // a1
    &[
        "              .,-:;//;:=,               ",
        "          . :H@@@MM@M#H/.,+%;,          ",
        "       ,/X+ +M@@M@MM%=,-%HMMM@X/,       ",
        "     -+@MM; #M@@MH+-,;XMMMM@MMMM@+-     ",
        "    ;@M@@M- XM@X;. -+XXXXXHHH@M@M#@/.   ",
        "  ,%MM@@MH ,@%=            .---=-=:=,.  ",
        "  =@#@@@MX .,              -%HX##%%%+;  ",
        " =-./@M@M$                  .;@MMMM@MM: ",
        " X@/ -#MM/                    .+MM@@@M$ ",
        ",@M@H: :@:                    . =X#@@@@-",
        ",@@@MMX, .                    /H- ;@M@M=",
        ".H@@@@M@+,                    %MM+..%#$.",
        " /MMMM@MMH/.                  XM@MH; =; ",
        "  /%+%#XHH@$=              , .H@@@@MX,  ",
        "   .=--------.           -%H.,@@@@@MX,  ",
        "   .%MM@@@HHHXX###%+= .:#MMX =M@@MM%.   ",
        "     =XMMM@MM@MM#H;,-+HMM@M+ /MMMX=     ",
        "       =%@M@M#@$-.=#@MM@@@M; %M%=       ",
        "         ,:+$+-,/H#MMMMMMM@= =,         ",
        "               =++%%%%+/:-.             ",
    ],
    // a2  
    &[
        "             =+$HM####@H%;,             ",
        "          /H###############M$,          ",
        "          ,@################+           ",
        "           .H##############+            ",
        "             X############/             ",
        "              $##########/              ",
        "               %########/               ",
        "                /X/;;+X/                ",
        "                 -XHHX-                 ",
        "                ,######,                ",
        "#############X  .M####M.  X#############",
        "##############-   -//-   -##############",
        "X##############%,      ,+##############X",
        "-##############X        X##############-",
        " %############%          %############% ",
        "  %##########;            ;##########%  ",
        "   ;#######M=              =M#######;   ",
        "    .+M###@,                ,@###M+.    ",
        "       :XH.                  .HX:       ",
        "                                        ",
    ],
    // Additional ASCII art patterns would go here...
    // For brevity, I'm including just the first two patterns
    // In the full implementation, all 10 patterns (a1-a10) should be included
];

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

pub fn start_credits() {
    // This will be implemented later with the full credits string and threading
    thread::spawn(|| {
        // Placeholder for credits scrolling
        println!("Credits started");
    });
}
