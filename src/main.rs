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

use rodio::{Decoder, OutputStream, Source};
use std::fs::File;
use std::io::BufReader;
use std::thread;
use std::time::Duration;

struct Lyric {
    words: &str,
    time: u32,
    interval: f32,
    mode: u8,
}

impl Lyric {
    fn new(words: &str, time: u32, interval: f32, mode: u8) -> Self {
        Lyric {
            words,
            time,
            interval,
            mode,
        }
    }
}

#[allow(dead_code)]
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



fn main() {
    //Play song!
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = File::open("sa1.mp3").unwrap();
    let source = Decoder::new(BufReader::new(file)).unwrap();
    stream_handle.play_raw(source.convert_samples()).unwrap();

    thread::sleep(Duration::from_secs(30));
    }

const LYRICS: &[&Lyric] = &[
    //Page 1
    Lyric::new("Forms FORM-29827281-12:", 0, -1.0, 0),
    &Lyric::new("Test Assessment Report".to_string(), 200, -1.0, 0),
    &Lyric::new("\0\0\0\0\0\0\0".to_string(), 400, -1.0, 0),  // Keep flushing the buffer
    &Lyric::new("".to_string(), 710, 0.0, 4),  // Music start
    &Lyric::new("This was a triumph.".to_string(), 730, 2.0, 0),
    &Lyric::new("".to_string(), 930, 0.0, 5),  // Credits start
    &Lyric::new("I'm making a note here:".to_string(), 1123, 2.0, 0),
    &Lyric::new("HUGE SUCCESS.".to_string(), 1347, 1.7, 0),
    &Lyric::new("It's hard to overstate".to_string(), 1627, -1.0, 0),
    &Lyric::new("my satisfaction.".to_string(), 1873, 2.6, 0),
    &Lyric::new("Aperture Science".to_string(), 2350, 1.8, 0),
    &Lyric::new("".to_string(), 2350, 0.0, 2),  // ASCII 1
    &Lyric::new("We do what we must".to_string(), 2733, 1.6, 0),
    &Lyric::new("because we can.".to_string(), 2910, 1.5, 0),
    &Lyric::new("For the good of all of us.".to_string(), 3237, -1.0, 0),
    &Lyric::new("".to_string(), 3500, 0.0, 2),  // ASCII 2
    &Lyric::new("Except the ones who are dead.".to_string(), 3567, -1.0, 0),
    &Lyric::new("".to_string(), 3717, 0.05, 0),
    &Lyric::new("".to_string(), 3717, 0.0, 2),  // ASCII 1
    &Lyric::new("But there's no sense crying".to_string(), 3787, -1.0, 0),
    &Lyric::new("over every mistake.".to_string(), 3973, 1.77, 0),
    &Lyric::new("You just keep on trying".to_string(), 4170, -1.0, 0),
    &Lyric::new("till you run out of cake.".to_string(), 4370, -1.0, 0),
    &Lyric::new("".to_string(), 4500, 0.0, 2),  // ASCII 3
    &Lyric::new("And the Science gets done.".to_string(), 4570, -1.0, 0),
    &Lyric::new("And you make a neat gun.".to_string(), 4767, -1.0, 0),
    &Lyric::new("".to_string(), 4903, 0.0, 2),  // ASCII 1
    &Lyric::new("For the people who are".to_string(), 4973, -1.0, 0),
    &Lyric::new("still alive.".to_string(), 5110, 1.6, 1),

    // PAGE 2
    &Lyric::new("".to_string(), 5353, 0.0, 3),  // Clear LYRICS
    &Lyric::new("Forms FORM-55551-5:".to_string(), 5413, -1.0, 0),
    &Lyric::new("Personnel File Addendum:".to_string(), 5477, 1.13, 0),
    &Lyric::new("".to_string(), 5650, 0.05, 0),
    &Lyric::new("Dear <<Subject Name Here>>,".to_string(), 5650, -1.0, 0),
    &Lyric::new("".to_string(), 5900, -1.0, 0),
    &Lyric::new("I'm not even angry.".to_string(), 5900, 1.86, 0),
    &Lyric::new("I'm being ".to_string(), 6320, -1.0, 1),
    &Lyric::new("so ".to_string(), 6413, -1.0, 1),
    &Lyric::new("sincere right now.".to_string(), 6470, 1.9, 0),
    &Lyric::new("Even though you broke ".to_string(), 6827, -1.0, 1),
    &Lyric::new("".to_string(), 7020, 0.0, 2),  // ASCII 4
    &Lyric::new("my heart.".to_string(), 7090, -1.0, 0),
    &Lyric::new("And killed me.".to_string(), 7170, 1.43, 0),
    &Lyric::new("".to_string(), 7300, 0.0, 2),  // ASCII 5
    &Lyric::new("And tore me to pieces.".to_string(), 7500, 1.83, 0),
    &Lyric::new("And threw every piece ".to_string(), 7900, -1.0, 1),
    &Lyric::new("into a fire.".to_string(), 8080, 1.8, 0),
    &Lyric::new("".to_string(), 8080, 0.0, 2),  // ASCII 6
    &Lyric::new("As they burned it hurt because".to_string(), 8430, -1.0, 0),
    &Lyric::new("".to_string(), 8690, 0.0, 2),  // ASCII 7
    &Lyric::new("I was so happy for you!".to_string(), 8760, 1.67, 0),
    &Lyric::new("Now, these points of data".to_string(), 8960, -1.0, 0),
    &Lyric::new("make a beautiful line.".to_string(), 9167, -1.0, 0),
    &Lyric::new("And we're out of beta.".to_string(), 9357, -1.0, 0),
    &Lyric::new("We're releasing on time.".to_string(), 9560, -1.0, 0),
    &Lyric::new("".to_string(), 9700, 0.0, 2),  // ASCII 5
    &Lyric::new("So I'm GLaD I got burned.".to_string(), 9770, -1.0, 0),
    &Lyric::new("".to_string(), 9913, 0.0, 2),  // ASCII 3
    &Lyric::new("Think of all the things we learned".to_string(), 9983, -1.0, 0),
    &Lyric::new("".to_string(), 10120, 0.0, 2),  // ASCII 1
    &Lyric::new("For the people who are".to_string(), 10190, -1.0, 0),
    &Lyric::new("Still alive.".to_string(), 10327, 1.8, 0),

    // PAGE 3
    &Lyric::new("".to_string(), 10603, 0.0, 3),  // Clear LYRICS
    &Lyric::new("Forms FORM-55551-6:".to_string(), 10663, -1.0, 0),
    &Lyric::new("Personnel File Addendum Addendum:".to_string(), 10710, 1.36, 0),
    &Lyric::new("".to_string(), 10710, 0.05, 0),
    &Lyric::new("One last thing:".to_string(), 10910, -1.0, 0),
    &Lyric::new("".to_string(), 11130, 0.05, 0),
    &Lyric::new("Go ahead and leave ".to_string(), 11130, -1.0, 1),
    &Lyric::new("me.".to_string(), 11280, 0.5, 0),
    &Lyric::new("I think I'd prefer to stay ".to_string(), 11507, -1.0, 1),
    &Lyric::new("inside.".to_string(), 11787, 1.13, 0),
    &Lyric::new("Maybe you'll find someone else".to_string(), 12037, -1.0, 0),
    &Lyric::new("To help you.".to_string(), 12390, 1.23, 0),
    &Lyric::new("Maybe Black ".to_string(), 12737, -1.0, 1),
    &Lyric::new("".to_string(), 12787, 0.0, 2),  // ASCII 8
    &Lyric::new("Mesa...".to_string(), 12857, 2.7, 0),
    &Lyric::new("THAT WAS A JOKE.".to_string(), 13137, 1.46, 1),
    &Lyric::new(" FAT CHANCE.".to_string(), 13387, 1.1, 0),
    &Lyric::new("Anyway, ".to_string(), 13620, -1.0, 1),
    &Lyric::new("".to_string(), 13670, 0.0, 2),  // ASCII 9
    &Lyric::new("this cake is great.".to_string(), 13740, -1.0, 0),
    &Lyric::new("It's so delicious and moist.".to_string(), 13963, -1.0, 0),
    &Lyric::new("".to_string(), 14123, 0.0, 2),  // ASCII 10
    &Lyric::new("Look at me still talking".to_string(), 14193, -1.0, 0),
    &Lyric::new("".to_string(), 14320, 0.0, 2),  // ASCII 2
    &Lyric::new("when there's science to do.".to_string(), 14390, -1.0, 0),
    &Lyric::new("".to_string(), 14527, 0.0, 2),  // ASCII 1
    &Lyric::new("When I look out there,".to_string(), 14597, -1.0, 0),
    &Lyric::new("It makes me GLaD I'm not you.".to_string(), 14767, -1.0, 0),
    &Lyric::new("".to_string(), 14913, 0.0, 2),  // ASCII 3
    &Lyric::new("I've experiments to run.".to_string(), 14983, -1.0, 0),
    &Lyric::new("".to_string(), 15120, 0.0, 2),  // ASCII 5
    &Lyric::new("There is research to be done.".to_string(), 15190, -1.0, 0),
    &Lyric::new("".to_string(), 15320, 0.0, 2),  // ASCII 1
    &Lyric::new("On the people who are".to_string(), 15390, -1.0, 0),
    &Lyric::new("still alive".to_string(), 15553, 2.0, 1),

    // PAGE 4
    &Lyric::new("".to_string(), 15697, 0.0, 3),  // Clear LYRICS
    &Lyric::new("".to_string(), 15757, 0.05, 0),
    &Lyric::new("".to_string(), 15757, 0.05, 0),
    &Lyric::new("".to_string(), 15757, 0.05, 0),
    &Lyric::new("PS: And believe me I am".to_string(), 15757, -1.0, 0),
    &Lyric::new("still alive.".to_string(), 15960, 1.13, 0),
    &Lyric::new("PPS: I'm doing Science and I'm".to_string(), 16150, -1.0, 0),
    &Lyric::new("still alive.".to_string(), 16363, 1.13, 0),
    &Lyric::new("PPPS: I feel FANTASTIC and I'm".to_string(), 16550, -1.0, 0),
    &Lyric::new("still alive.".to_string(), 16760, -1.0, 0),
    &Lyric::new("".to_string(), 16860, -1.0, 0),
    &Lyric::new("FINAL THOUGH:".to_string(), 16860, -1.0, 0),
    &Lyric::new("While you're dying I'll be".to_string(), 16993, -1.0, 0),
    &Lyric::new("still alive.".to_string(), 17157, -1.0, 0),
    &Lyric::new("".to_string(), 17277, -1.0, 0),
    &Lyric::new("FINAL THOUGH PS:".to_string(), 17277, -1.0, 0),
    &Lyric::new("And when you're dead I will be".to_string(), 17367, -1.0, 0),
    &Lyric::new("still alive.".to_string(), 17550, 1.13, 0),
    &Lyric::new("".to_string(), 17550, -1.0, 0),
    &Lyric::new("".to_string(), 17550, 0.05, 0),
    &Lyric::new("STILL ALIVE".to_string(), 17760, 1.13, 0),
    &Lyric::new("".to_string(), 17900, 0.0, 3),  // Clear LYRICS
    &Lyric::new("".to_string(), 18500, 0.0, 3),  // Clear LYRICS
    &Lyric::new("ENDENDENDENDENDENDENDEND".to_string(), 18500, 0.05, 9),
];
