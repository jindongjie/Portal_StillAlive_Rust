// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
//
// The copyright for song <Still Alive> belongs to Jonathan Coulton and Valve Software
// Contact me if there's copyright violation and if deletion of the content is needed.
//
// PORTAL--STILL ALIVE demo by LHF (BD4SUP) - rewrite in Rust by ar0m
// October 2024
// Runs with Rust on most terminals with an 80x24 characters area.
// This program was adjusted specifically for serial terminal under Linux
// (in this case, an DEC VT220 @ 19200bps and Arch Linux)
// Feel free to do whatever you like with this code.

use crossterm::{
    cursor,
    execute,
    style::ResetColor,
    style::SetForegroundColor,
    terminal,
    terminal::{Clear, ClearType}
};
use rodio::{Decoder, OutputStream, Source};
use std::io::{self, Write};
use std::io::Cursor;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{env, thread, time};

const MP3_DATA: &[u8] = include_bytes!("../sa1.mp3");
static PRINT_LOCK: once_cell::sync::Lazy<Arc<Mutex<()>>> = once_cell::sync::Lazy::new(|| Arc::new(Mutex::new(())));

static CURSOR_POSITION: once_cell::sync::Lazy<Mutex<(i32, i32)>> = once_cell::sync::Lazy::new(|| Mutex::new((1, 1)));

static IS_DRAW_END: once_cell::sync::Lazy<Mutex<bool>> = once_cell::sync::Lazy::new(|| Mutex::new(false));
fn get_term() -> String {
    env::var("TERM").unwrap_or_else(|_| "vt220".to_string())
}

fn is_vt(term: &str) -> bool {
    term.contains("vt")
}

fn enable_screen_buffer(term: &str) -> bool {
    !is_vt(term) && term != "linux"
}

fn enable_color(term: &str) -> bool {
    !is_vt(term) || term[2..].parse::<i32>().unwrap_or(0) >= 241
}

fn get_terminal_size() -> (usize, usize) {
    if is_vt(&get_term()) {
        (80, 24)
    } else {
        terminal::size().map(|(w, h)| (w as usize, h as usize)).unwrap_or((80, 24))
    }
}



fn sigint_handler() {
    end_draw();
    println!("Interrupt by user");
    std::process::exit(0);
}

 fn begin_draw() {
    let term = get_term();
    if enable_screen_buffer(&term) {
        let _lock = PRINT_LOCK.lock().unwrap();
        execute!(io::stdout(), terminal::EnterAlternateScreen).unwrap();
    }
    if enable_color(&term) {
        let _lock = PRINT_LOCK.lock().unwrap();
        execute!(io::stdout(), SetForegroundColor(crossterm::style::Color::Yellow)).unwrap();
    }
}

fn end_draw() {
        let _lock = PRINT_LOCK.lock().unwrap();
        if enable_color(&get_term()) {
            execute!(io::stdout(), ResetColor).unwrap();
        }
        if enable_screen_buffer(&get_term()) {
            execute!(io::stdout(), terminal::LeaveAlternateScreen).unwrap();
        } else {
            clear(false);
            move_cursor(1, 1, false, false);
        }
}

fn move_cursor(x: i32, y: i32, update_cursor: bool, mutex: bool) {
    if mutex {
        let _lock = PRINT_LOCK.lock().unwrap();
    }
    execute!(io::stdout(), cursor::MoveTo(x as u16, y as u16)).unwrap();
    if update_cursor {
            let mut cursor_position = CURSOR_POSITION.lock().unwrap();
        cursor_position.0 = x;
        cursor_position.1 = y;
        }
}

fn clear(mutex: bool) {
        if mutex {
            let _lock = PRINT_LOCK.lock().unwrap();
        }
        execute!(io::stdout(), Clear(ClearType::All)).unwrap();
        let mut cursor_position = CURSOR_POSITION.lock().unwrap();
        cursor_position.0 = 1;
        cursor_position.1 = 1;
}

fn _print(s: &str, newline: bool) {
        let _lock = PRINT_LOCK.lock().unwrap();
        let mut cursor_position = CURSOR_POSITION.lock().unwrap();
        if newline {
            println!("{}", s);
            cursor_position.0 = 1;
            cursor_position.1 += 1;
        } else {
            print!("{}", s);
            cursor_position.0 += s.len() as i32;
        }
        io::stdout().flush().unwrap();
}

const ASCII_ART_WIDTH: usize = 40;
const ASCII_ART_HEIGHT: usize = 20;

fn get_credits_width() -> usize {
    std::cmp::min(((get_terminal_size().0 - 4) / 2), 56)
}

fn get_credits_height() -> usize {
    (get_terminal_size().1 - ASCII_ART_HEIGHT - 2)
}

fn get_lyric_width() -> usize {
    (get_terminal_size().0 - 4 - get_credits_width())
}

fn get_lyric_height() -> usize {
    (get_terminal_size().1 - 2) as usize
}

fn get_credits_pos_x() -> usize {
    get_lyric_width() + 4
}

fn get_ascii_art_x() -> usize {
    get_lyric_width() + 4 + (get_credits_width() - ASCII_ART_WIDTH) / 2
}

fn get_ascii_art_y() -> usize {
    get_credits_height() + 3
}

fn draw_aa(x: usize, y: usize, ch: usize, ascii_art: &[&[&str]]) {
    for dy in 0..ASCII_ART_HEIGHT {
        move_cursor(x as i32, (y + dy) as i32, false, false);
        print!("{}", ascii_art[ch][dy]);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(10));
    }
}

fn draw_frame() {
    let lyric_width = get_lyric_width();
    let credits_width = get_credits_width();
    let credits_height = get_credits_height();
    let lyric_height = get_lyric_height();

    move_cursor(1, 1, false, false);
    println!(" {}  {}", "-".repeat(lyric_width), "-".repeat(credits_width));
    for _ in 0..credits_height {
        println!("|{}||{}|", " ".repeat(lyric_width), " ".repeat(credits_width));
    }
    println!("|{}| {}", " ".repeat(lyric_width), "-".repeat(credits_width));
    for _ in 0..(lyric_height - 1 - credits_height) {
        println!("|{}|", " ".repeat(lyric_width));
    }
    println!(" {}", "-".repeat(lyric_width));
    move_cursor(2, 2, false, false);
    io::stdout().flush().unwrap();
    thread::sleep(Duration::from_secs(1));
}

fn clear_lyrics() {
    let lyric_width = get_lyric_width();
    let lyric_height = get_lyric_height();

    move_cursor(1, 2, false, false);
    for _ in 0..lyric_height {
        println!("|{}|", " ".repeat(lyric_width));
    }
    move_cursor(2, 2, false, false);
}

fn draw_lyrics(text: &str, x: usize, mut y: usize, interval: f64, newline: bool) -> usize {
    let mut local_i: usize = x;
    move_cursor((local_i + 2) as i32, (y + 2) as i32, false, false);
    for ch in text.chars() {
        print!("{}", ch);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs_f64(interval));
        local_i += 1;
    }
    if newline {
        local_i = 0;
        y += 1;
        move_cursor((local_i + 2) as i32, (y + 2) as i32, false, false);
    }
    local_i
}


struct ThreadCredits {
    current_time: time::Instant,
}

impl ThreadCredits {
    fn run(&self) {
        let mut credit_x = 0;
        let mut i = 0;
        let length = CREDITS.len();
        let mut last_credits = vec![String::new()];
        let start_time = std::time::Instant::now();

        for ch in CREDITS.chars() {
            let _current_time = start_time + Duration::from_secs_f64(174.0 / length as f64 * i as f64);
            i += 1;
            if ch == '\n' {
                credit_x = 0;
                last_credits.push(String::new());
                if last_credits.len() > get_credits_height() {
                    last_credits = last_credits.split_off(last_credits.len() - get_credits_height());
                }
                }
                let _lock = PRINT_LOCK.lock().unwrap();
                if *IS_DRAW_END.lock().unwrap() {
                    break;
                }
                for y in 2..(2 + get_credits_height() - last_credits.len()) {
                    move_cursor(get_credits_pos_x() as i32, y as i32, false, false);
                    print!("{}{}", " ".repeat(get_credits_width()), "");
                }
                let last_credits_copy: Vec<_> = last_credits.iter().enumerate().collect();
                for (_k, _line) in last_credits_copy {
                    for (k, line) in last_credits.iter().enumerate() {
                        move_cursor(get_credits_pos_x() as i32, (2 + k) as i32, false, false);
                    print!("{}{}", line, " ".repeat(get_credits_width() - line.len()));
                    }
                }
                last_credits.last_mut().unwrap().push(ch);
                if *IS_DRAW_END.lock().unwrap() {
                    break;
                }
                    }
                move_cursor(CURSOR_POSITION.lock().unwrap().0, CURSOR_POSITION.lock().unwrap().1, false, false);
                credit_x += 1;
            while std::time::Instant::now() < self.current_time {
                thread::sleep(Duration::from_millis(10));
            }
        }
    }


const ASCII_ART: [&[&str]; 10] = [
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
    &[
        "                 =/;;/-                 ",
        "                +:    //                ",
        "               /;      /;               ",
        "              -X        H.              ",
        ".//;;;:;;-,   X=        :+   .-;:=;:;%;.",
        "M-       ,=;;;#:,      ,:#;;:=,       ,@",
        ":%           :%.=/++++/=.$=           %=",
        " ,%;         %/:+/;,,/++:+/         ;+. ",
        "   ,+/.    ,;@+,        ,%H;,    ,/+,   ",
        "      ;+;;/= @.  .H##X   -X :///+;      ",
        "      ;+=;;;.@,  .XM@$.  =X.//;=%/.     ",
        "   ,;:      :@%=        =$H:     .+%-   ",
        " ,%=         %;-///==///-//         =%, ",
        ";+           :%-;;;:;;;;-X-           +:",
        "@-      .-;;;;M-        =M/;;;-.      -X",
        " :;;::;;-.    %-        :+    ,-;;-;:== ",
        "              ,X        H.              ",
        "               ;/      %=               ",
        "                //    +;                ",
        "                 ,////,                 ",
    ],
    &[
        "                          .,---.        ",
        "                        ,/XM#MMMX;,     ",
        "                      -%##########M%,   ",
        "                     -@######%  $###@=  ",
        "      .,--,         -H#######$   $###M: ",
        "   ,;$M###MMX;     .;##########$;HM###X=",
        " ,/@##########H=      ;################+",
        "-+#############M/,      %##############+",
        "%M###############=      /##############:",
        "H################      .M#############;.",
        "@###############M      ,@###########M:. ",
        "X################,      -$=X#######@:   ",
        "/@##################%-     +######$-    ",
        ".;##################X     .X#####+,     ",
        " .;H################/     -X####+.      ",
        "   ,;X##############,       .MM/        ",
        "      ,:+$H@M#######M#$-    .$$=        ",
        "           .,-=;+$@###X:    ;/=.        ",
        "                  .,/X$;   .::,         ",
        "                      .,    ..          ",
    ],
    &[
        "            .+                          ",
        "             /M;                        ",
        "              H#@:              ;,      ",
        "              -###H-          -@/       ",
        "               %####$.  -;  .%#X        ",
        "                M#####+;#H :M#M.        ",
        "..          .+/;%#########X###-         ",
        " -/%H%+;-,    +##############/          ",
        "    .:$M###MH$%+############X  ,--=;-   ",
        "        -/H#####################H+=.    ",
        "           .+#################X.        ",
        "         =%M####################H;.     ",
        "            /@###############+;;/%%;,   ",
        "         -%###################$.        ",
        "       ;H######################M=       ",
        "    ,%#####MH$%;+#####M###-/@####%      ",
        "  :$H%+;=-      -####X.,H#   -+M##@-    ",
        " .              ,###;    ;      =$##+   ",
        "                .#H,               :XH, ",
        "                 +                   .;-",
    ],
    &[
        "                     -$-                ",
        "                    .H##H,              ",
        "                   +######+             ",
        "                .+#########H.           ",
        "              -$############@.          ",
        "            =H###############@  -X:     ",
        "          .$##################:  @#@-   ",
        "     ,;  .M###################;  H###;  ",
        "   ;@#:  @###################@  ,#####: ",
        " -M###.  M#################@.  ;######H ",
        " M####-  +###############$   =@#######X ",
        " H####$   -M###########+   :#########M, ",
        "  /####X-   =########%   :M########@/.  ",
        "    ,;%H@X;   .$###X   :##MM@%+;:-      ",
        "                 ..                     ",
        "  -/;:-,.              ,,-==+M########H ",
        " -##################@HX%%+%%$%%%+:,,    ",
        "   .-/H%%%+%%$H@###############M@+=:/+: ",
        "/XHX%:#####MH%=    ,---:;;;;/%%XHM,:###$",
        "$@#MX %+;-                           .  ",
    ],
    &[
        "                                     :X-",
        "                                  :X### ",
        "                                ;@####@ ",
        "                              ;M######X ",
        "                            -@########$ ",
        "                          .$##########@ ",
        "                         =M############-",
        "                        +##############$",
        "                      .H############$=. ",
        "         ,/:         ,M##########M;.    ",
        "      -+@###;       =##########M;       ",
        "   =%M#######;     :#########M/         ",
        "-$M###########;   :#########/           ",
        " ,;X###########; =########$.            ",
        "     ;H#########+#######M=              ",
        "       ,+##############+                ",
        "          /M#########@-                 ",
        "            ;M######%                   ",
        "              +####:                    ",
        "               ,$M-                     ",
    ],
    &[
        "           .-;+$XHHHHHHX$+;-.           ",
        "        ,;X@@X%/;=----=:/%X@@X/,        ",
        "      =$@@%=.              .=+H@X:      ",
        "    -XMX:                      =XMX=    ",
        "   /@@:                          =H@+   ",
        "  %@X,                            .$@$  ",
        " +@X.                               $@% ",
        "-@@,                                .@@=",
        "%@%                                  +@$",
        "H@:                                  :@H",
        "H@:         :HHHHHHHHHHHHHHHHHHX,    =@H",
        "%@%         ;@M@@@@@@@@@@@@@@@@@H-   +@$",
        "=@@,        :@@@@@@@@@@@@@@@@@@@@@= .@@:",
        " +@X        :@@@@@@@@@@@@@@@M@@@@@@:%@% ",
        "  $@$,      ;@@@@@@@@@@@@@@@@@M@@@@@@$. ",
        "   +@@HHHHHHH@@@@@@@@@@@@@@@@@@@@@@@+   ",
        "    =X@@@@@@@@@@@@@@@@@@@@@@@@@@@@X=    ",
        "      :$@@@@@@@@@@@@@@@@@@@M@@@@$:      ",
        "        ,;$@@@@@@@@@@@@@@@@@@X/-        ",
        "           .-;+$XXHHHHHX$+;-.           ",
    ],
    &[
        "            ,:/+/-                      ",
        "            /M/              .,-=;//;-  ",
        "       .:/= ;MH/,    ,=/+%$XH@MM#@:     ",
        "      -$##@+$###@H@MMM#######H:.    -/H#",
        " .,H@H@ X######@ -H#####@+-     -+H###@ ",
        "  .,@##H;      +XM##M/,     =%@###@X;-  ",
        "X%-  :M##########$.    .:%M###@%:       ",
        "M##H,   +H@@@$/-.  ,;$M###@%,          -",
        "M####M=,,---,.-%%H####M$:          ,+@##",
        "@##################@/.         :%H##@$- ",
        "M###############H,         ;HM##M$=     ",
        "#################.    .=$M##M$=         ",
        "################H..;XM##M$=          .:+",
        "M###################@%=           =+@MH%",
        "@################M/.          =+H#X%=   ",
        "=+M##############M,       -/X#X+;.      ",
        "  .;XM##########H=    ,/X#H+:,          ",
        "     .=+HM######M+/+HM@+=.              ",
        "         ,:/%XM####H/.                  ",
        "              ,.:=-.                    ",
    ],
    &[
        "       #+ @      # #              M#@   ",
        " .    .X  X.%##@;# #   +@#######X. @#%  ",
        "   ,==.   ,######M+  -#####%M####M-    #",
        "  :H##M%:=##+ .M##M,;#####/+#######% ,M#",
        " .M########=  =@#@.=#####M=M#######=  X#",
        " :@@MMM##M.  -##M.,#######M#######. =  M",
        "             @##..###:.    .H####. @@ X,",
        "   ############: ###,/####;  /##= @#. M ",
        "           ,M## ;##,@#M;/M#M  @# X#% X# ",
        ".%=   ######M## ##.M#:   ./#M ,M #M ,#$ ",
        "##/         $## #+;#: #### ;#/ M M- @# :",
        "#+ #M@MM###M-;M #:$#-##$H# .#X @ + $#. #",
        "      ######/.: #%=# M#:MM./#.-#  @#: H#",
        "+,.=   @###: /@ %#,@  ##@X #,-#@.##% .@#",
        "#####+;/##/ @##  @#,+       /#M    . X, ",
        "   ;###M#@ M###H .#M-     ,##M  ;@@; ###",
        "   .M#M##H ;####X ,@#######M/ -M###$  -H",
        "    .M###%  X####H  .@@MM@;  ;@#M@      ",
        "      H#M    /@####/      ,++.  / ==-,  ",
        "               ,=/:, .+X@MMH@#H  #####$=",
    ],
];

const CREDITS: &str = r#"
>LIST PERSONNEL

Gautam Babbar
Ted Backman
Kelly Bailey
Jeff Ballinger
Aaron Barber
Jeep Barnett
Jeremy Bennett
Dan Berger
Yahn Bernier
Ken Birdwell
Derrick BirumMike Blaszczak
Iestyn Bleasdale-Shepherd
Chris Bokitch
Steve Bond
Matt Boone
Antoine Bourdon
Jamaal Bradley
Jason Brashill
Charlie Brown
Charlie Burgin
Andrew Burke
Augusta Butlin
Julie Caldwell
Dario Casali
Chris Chin
Jess Cliffe
Phil Co
John Cook
Christen Coomer
Greg Coomer
Scott Dalton
Kerry Davis
Jason Deakins
Joe Demers
Ariel Diaz
Quintin Doroquez
Jim Dose
Chris Douglass
Laura Dubuk
Mike Dunkle
Mike Durand
Mike Dussault
Dhabih Eng
Katie Engel
Chet Faliszek
Adrian Finol
Bill Fletcher
Moby Francke
Stephane Gaudette
Kathy Gehrig
Vitaliy Genkin
Paul Graham
Chris Green
Chris Grinstead
John Guthrie
Aaron Halifax
Reagan Halifax
Leslie Hall
Jeff Hameluck
Joe Han
Don Holden
Jason Holtman
Gray Horsfield
Keith Huggins
Jim Hughes
Jon Huisingh
Brian Jacobson
Lars Jensvold
Erik Johnson
Jakob Jungels
Rich Kaethler
Steve Kalning
Aaron Kearly
Iikka Keranen
David Kircher
Eric Kirchmer
Scott Klintworth
Alden Kroll
Marc Laidlaw
Jeff Lane
Tim Larkin
Dan LeFree
Isabelle LeMay
Tom Leonard
Jeff Lind
Doug Lombardi
Bianca Loomis
Richard Lord
Realm Lovejoy
Randy Lundeen
Scott Lynch
Ido Magal
Nick Maggiore
John McCaskey
Patrick McClard
Steve McClure
Hamish McKenzie
Gary McTaggart
Jason Mitchell
Mike Morasky
John Morello II
Bryn Moslow
Arsenio Navarro
Gabe Newell
Milton Ngan
Jake Nicholson
Martin Otten
Nick Papineau
Karen Prell
Bay Raitt
Tristan Reidford
Alfred Reynolds
Matt Rhoten
Garret Rickey
Dave Riller
Elan Ruskin
Matthew Russell
Jason Ruymen
David Sawyer
Marc Scaparro
Wade Schin
Matthew Scott
Aaron Seeler
Jennifer Seeley
Taylor Sherman
Eric Smith
Jeff Sorensen
David Speyrer
Jay Stelly
Jeremy Stone
Eric Strand
Kim Swift
Kelly Thornton
Eric Twelker
Carl Uhlman
Doug Valente
Bill Van Buren
Gabe Van Engel
Alex Vlachos
Robin Walker
Joshua Weier
Andrea Wicklund
Greg Winkler
Erik Wolpaw
Doug Wood
Matt T. Wood
Danika Wright
Matt Wright
Shawn Zabecki
Torsten Zabka


'Still Alive' by:
Jonathan Coulton

Voices:
Ellen McLain - GlaDOS, Turrets
Mike Patton - THE ANGER SPHERE

Voice Casting:
Shana Landsburg\Teri Fiddleman

Voice Recording:
Pure Audio, Seattle, WA

Voice recording
scheduling and logistics:
Pat Cockburn, Pure Audio

Translations:
SDL

Crack Legal Team:
Liam Lavery
Karl Quackenbush
Kristen Boraas
Kevin Rosenfield
Alan Bruggeman
Dennis Tessier

Thanks for the use of their face:
Alesia Glidewell - Chell

Special thanks to everyone at:
Alienware
ATI
Dell
Falcon Northwest
Havok
SOFTIMAGE
and Don Kemmis, SLK Technologies


THANK YOU FOR PARTICIPATING
IN THIS
ENRICHMENT CENTER ACTIVITY!!
"#;


struct Lyric<'a> {
    words: &'a str,
    time: u32,
    interval: f32,
    mode: u8,
}

impl<'a> Lyric<'a> {
    const fn new(words: &'a str, time: u32, interval: f32, mode: u8) -> Self {
        Lyric {
            words,
            time,
            interval,
            mode,
        }
    }
}

const LYRICS: &[&Lyric] = &[
    //Page 1
    &Lyric::new("Forms FORM-29827281-12:", 0, -1.0, 0),
    &Lyric::new("Test Assessment Report", 200, -1.0, 0),
    &Lyric::new("\0\0\0\0\0\0\0", 400, -1.0, 0),  // Keep flushing the buffer
    &Lyric::new("", 710, 0.0, 4),  // Music start
    &Lyric::new("This was a triumph.", 730, 2.0, 0),
    &Lyric::new("", 930, 0.0, 5),  // Credits start
    &Lyric::new("I'm making a note here:", 1123, 2.0, 0),
    &Lyric::new("HUGE SUCCESS.", 1347, 1.7, 0),
    &Lyric::new("It's hard to overstate", 1627, -1.0, 0),
    &Lyric::new("my satisfaction.", 1873, 2.6, 0),
    &Lyric::new("Aperture Science", 2350, 1.8, 0),
    &Lyric::new("", 2350, 0.0, 2),  // ASCII 1
    &Lyric::new("We do what we must", 2733, 1.6, 0),
    &Lyric::new("because we can.", 2910, 1.5, 0),
    &Lyric::new("For the good of all of us.", 3237, -1.0, 0),
    &Lyric::new("", 3500, 0.0, 2),  // ASCII 2
    &Lyric::new("Except the ones who are dead.", 3567, -1.0, 0),
    &Lyric::new("", 3717, 0.05, 0),
    &Lyric::new("", 3717, 0.0, 2),  // ASCII 1
    &Lyric::new("But there's no sense crying", 3787, -1.0, 0),
    &Lyric::new("over every mistake.", 3973, 1.77, 0),
    &Lyric::new("You just keep on trying", 4170, -1.0, 0),
    &Lyric::new("till you run out of cake.", 4370, -1.0, 0),
    &Lyric::new("", 4500, 0.0, 2),  // ASCII 3
    &Lyric::new("And the Science gets done.", 4570, -1.0, 0),
    &Lyric::new("And you make a neat gun.", 4767, -1.0, 0),
    &Lyric::new("", 4903, 0.0, 2),  // ASCII 1
    &Lyric::new("For the people who are", 4973, -1.0, 0),
    &Lyric::new("still alive.", 5110, 1.6, 1),

    // PAGE 2
    &Lyric::new("", 5353, 0.0, 3),  // Clear LYRICS
    &Lyric::new("Forms FORM-55551-5:", 5413, -1.0, 0),
    &Lyric::new("Personnel File Addendum:", 5477, 1.13, 0),
    &Lyric::new("", 5650, 0.05, 0),
    &Lyric::new("Dear <<Subject Name Here>>,", 5650, -1.0, 0),
    &Lyric::new("", 5900, -1.0, 0),
    &Lyric::new("I'm not even angry.", 5900, 1.86, 0),
    &Lyric::new("I'm being ", 6320, -1.0, 1),
    &Lyric::new("so ", 6413, -1.0, 1),
    &Lyric::new("sincere right now.", 6470, 1.9, 0),
    &Lyric::new("Even though you broke ", 6827, -1.0, 1),
    &Lyric::new("", 7020, 0.0, 2),  // ASCII 4
    &Lyric::new("my heart.", 7090, -1.0, 0),
    &Lyric::new("And killed me.", 7170, 1.43, 0),
    &Lyric::new("", 7300, 0.0, 2),  // ASCII 5
    &Lyric::new("And tore me to pieces.", 7500, 1.83, 0),
    &Lyric::new("And threw every piece ", 7900, -1.0, 1),
    &Lyric::new("into a fire.", 8080, 1.8, 0),
    &Lyric::new("", 8080, 0.0, 2),  // ASCII 6
    &Lyric::new("As they burned it hurt because", 8430, -1.0, 0),
    &Lyric::new("", 8690, 0.0, 2),  // ASCII 7
    &Lyric::new("I was so happy for you!", 8760, 1.67, 0),
    &Lyric::new("Now, these points of data", 8960, -1.0, 0),
    &Lyric::new("make a beautiful line.", 9167, -1.0, 0),
    &Lyric::new("And we're out of beta.", 9357, -1.0, 0),
    &Lyric::new("We're releasing on time.", 9560, -1.0, 0),
    &Lyric::new("", 9700, 0.0, 2),  // ASCII 5
    &Lyric::new("So I'm GLaD I got burned.", 9770, -1.0, 0),
    &Lyric::new("", 9913, 0.0, 2),  // ASCII 3
    &Lyric::new("Think of all the things we learned", 9983, -1.0, 0),
    &Lyric::new("", 10120, 0.0, 2),  // ASCII 1
    &Lyric::new("For the people who are", 10190, -1.0, 0),
    &Lyric::new("Still alive.", 10327, 1.8, 0),

    // PAGE 3
    &Lyric::new("", 10603, 0.0, 3),  // Clear LYRICS
    &Lyric::new("Forms FORM-55551-6:", 10663, -1.0, 0),
    &Lyric::new("Personnel File Addendum Addendum:", 10710, 1.36, 0),
    &Lyric::new("", 10710, 0.05, 0),
    &Lyric::new("One last thing:", 10910, -1.0, 0),
    &Lyric::new("", 11130, 0.05, 0),
    &Lyric::new("Go ahead and leave ", 11130, -1.0, 1),
    &Lyric::new("me.", 11280, 0.5, 0),
    &Lyric::new("I think I'd prefer to stay ", 11507, -1.0, 1),
    &Lyric::new("inside.", 11787, 1.13, 0),
    &Lyric::new("Maybe you'll find someone else", 12037, -1.0, 0),
    &Lyric::new("To help you.", 12390, 1.23, 0),
    &Lyric::new("Maybe Black ", 12737, -1.0, 1),
    &Lyric::new("", 12787, 0.0, 2),  // ASCII 8
    &Lyric::new("Mesa...", 12857, 2.7, 0),
    &Lyric::new("THAT WAS A JOKE.", 13137, 1.46, 1),
    &Lyric::new(" FAT CHANCE.", 13387, 1.1, 0),
    &Lyric::new("Anyway, ", 13620, -1.0, 1),
    &Lyric::new("", 13670, 0.0, 2),  // ASCII 9
    &Lyric::new("this cake is great.", 13740, -1.0, 0),
    &Lyric::new("It's so delicious and moist.", 13963, -1.0, 0),
    &Lyric::new("", 14123, 0.0, 2),  // ASCII 10
    &Lyric::new("Look at me still talking", 14193, -1.0, 0),
    &Lyric::new("", 14320, 0.0, 2),  // ASCII 2
    &Lyric::new("when there's science to do.", 14390, -1.0, 0),
    &Lyric::new("", 14527, 0.0, 2),  // ASCII 1
    &Lyric::new("When I look out there,", 14597, -1.0, 0),
    &Lyric::new("It makes me GLaD I'm not you.", 14767, -1.0, 0),
    &Lyric::new("", 14913, 0.0, 2),  // ASCII 3
    &Lyric::new("I've experiments to run.", 14983, -1.0, 0),
    &Lyric::new("", 15120, 0.0, 2),  // ASCII 5
    &Lyric::new("There is research to be done.", 15190, -1.0, 0),
    &Lyric::new("", 15320, 0.0, 2),  // ASCII 1
    &Lyric::new("On the people who are", 15390, -1.0, 0),
    &Lyric::new("still alive", 15553, 2.0, 1),

    // PAGE 4
    &Lyric::new("", 15697, 0.0, 3),  // Clear LYRICS
    &Lyric::new("", 15757, 0.05, 0),
    &Lyric::new("", 15757, 0.05, 0),
    &Lyric::new("", 15757, 0.05, 0),
    &Lyric::new("PS: And believe me I am", 15757, -1.0, 0),
    &Lyric::new("still alive.", 15960, 1.13, 0),
    &Lyric::new("PPS: I'm doing Science and I'm", 16150, -1.0, 0),
    &Lyric::new("still alive.", 16363, 1.13, 0),
    &Lyric::new("PPPS: I feel FANTASTIC and I'm", 16550, -1.0, 0),
    &Lyric::new("still alive.", 16760, -1.0, 0),
    &Lyric::new("", 16860, -1.0, 0),
    &Lyric::new("FINAL THOUGH:", 16860, -1.0, 0),
    &Lyric::new("While you're dying I'll be", 16993, -1.0, 0),
    &Lyric::new("still alive.", 17157, -1.0, 0),
    &Lyric::new("", 17277, -1.0, 0),
    &Lyric::new("FINAL THOUGH PS:", 17277, -1.0, 0),
    &Lyric::new("And when you're dead I will be", 17367, -1.0, 0),
    &Lyric::new("still alive.", 17550, 1.13, 0),
    &Lyric::new("", 17550, -1.0, 0),
    &Lyric::new("", 17550, 0.05, 0),
    &Lyric::new("STILL ALIVE", 17760, 1.13, 0),
    &Lyric::new("", 17900, 0.0, 3),  // Clear LYRICS
    &Lyric::new("", 18500, 0.0, 3),  // Clear LYRICS
    &Lyric::new("ENDENDENDENDENDENDENDEND", 18500, 0.05, 9),
];

fn main() {
    let enable_sound: bool = !env::args().any(|arg| arg == "--no-sound");
    begin_draw();
    clear(false);
    draw_frame();
    move_cursor(2, 2, false, false);
    thread::sleep(Duration::from_secs(1));

    let start_time = chrono::Local::now();
    let mut current_time;
    let mut current_lyric = 0;
    let mut current_credit = 0;
    let mut x = 0;
    let mut y = 0;

    while LYRICS[current_lyric].mode != 9 {
        current_time = (chrono::Local::now() - start_time).num_milliseconds() as f32;
        if current_time > LYRICS[current_lyric].time as f32 {
            let word_count: f32 = if LYRICS[current_lyric].mode <= 1 || LYRICS[current_lyric].mode >= 5 {
                LYRICS[current_lyric].words.len() as f32
            } else {
                1f32
            };

            let interval = if LYRICS[current_lyric].interval < 0.0 {
                (LYRICS[current_lyric + 1].time - LYRICS[current_lyric].time) as f32/ 100f32 / word_count
            } else {
                LYRICS[current_lyric].interval / word_count
            };

            match LYRICS[current_lyric].mode {
                0 => {
                    x = draw_lyrics(&LYRICS[current_lyric].words, x, y, interval.into(), true);
                    y += 1;
                }
                1 => {
                    x = draw_lyrics(&LYRICS[current_lyric].words, x, y, interval.into(), false);
                }
                2 => {
                    draw_aa(ASCII_ART_HEIGHT, ASCII_ART_WIDTH, (&LYRICS[current_lyric].words).parse().unwrap(), &[]);
                    move_cursor((x + 2) as i32, (y + 2) as i32, false, false);
                }
                3 => {
                    clear_lyrics();
                    x = 0;
                    y = 0;
                }
                4 => {
                    if enable_sound {
                        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
                        let cursor = Cursor::new(MP3_DATA);
                        let source = Decoder::new(cursor).unwrap();
                        stream_handle.play_raw(source.convert_samples()).unwrap();
                    }
                }
                5 => {
                let the_credit = ThreadCredits { current_time: std::time::Instant::now() };
                the_credit.run();
                }
                _ => {}
            }
            current_lyric += 1;
        }

        thread::sleep(Duration::from_millis(10));
    }

    end_draw();
}