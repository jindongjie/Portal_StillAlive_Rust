// data.rs
// Contains all the lyrics and ASCII art data from the Python script

use crate::tui_draw::Lyric;

// ASCII art patterns (all 10 from the Python script)
pub const ASCII_ART: &[&[&str]] = &[
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
    // a3
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
    // a4
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
    // a5
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
    // a6
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
    // a7
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
    // a8
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
    // a9
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
    // a10
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

// Lyrics data converted from Python
pub fn get_lyrics() -> Vec<Lyric> {
    vec![
        // Page 1
        Lyric::new("Forms FORM-29827281-12:", 0, -1.0, 0),
        Lyric::new("Test Assessment Report", 200, -1.0, 0),
        Lyric::new("\u{00}\u{00}\u{00}\u{00}\u{00}\u{00}\u{00}", 400, -1.0, 0),
        Lyric::new("", 710, 0.0, 4), // Music start
        Lyric::new("This was a triumph.", 730, 2.0, 0),
        Lyric::new("", 930, 0.0, 5), // Credits start
        Lyric::new("I'm making a note here:", 1123, 2.0, 0),
        Lyric::new("HUGE SUCCESS.", 1347, 1.7, 0),
        Lyric::new("It's hard to overstate", 1627, -1.0, 0),
        Lyric::new("my satisfaction.", 1873, 2.6, 0),
        Lyric::new("Aperture Science", 2350, 1.8, 0),
        Lyric::new("0", 2350, 0.0, 2), // ASCII 1
        Lyric::new("We do what we must", 2733, 1.6, 0),
        Lyric::new("because we can.", 2910, 1.5, 0),
        Lyric::new("For the good of all of us.", 3237, -1.0, 0),
        Lyric::new("1", 3500, 0.0, 2), // ASCII 2
        Lyric::new("Except the ones who are dead.", 3567, -1.0, 0),
        Lyric::new("", 3717, 0.05, 0),
        Lyric::new("0", 3717, 0.0, 2), // ASCII 1
        Lyric::new("But there's no sense crying", 3787, -1.0, 0),
        Lyric::new("over every mistake.", 3973, 1.77, 0),
        Lyric::new("You just keep on trying", 4170, -1.0, 0),
        Lyric::new("till you run out of cake.", 4370, -1.0, 0),
        Lyric::new("2", 4500, 0.0, 2), // ASCII 3
        Lyric::new("And the Science gets done.", 4570, -1.0, 0),
        Lyric::new("And you make a neat gun.", 4767, -1.0, 0),
        Lyric::new("0", 4903, 0.0, 2), // ASCII 1
        Lyric::new("For the people who are", 4973, -1.0, 0),
        Lyric::new("still alive.", 5110, 1.6, 1),
        // Page 2
        Lyric::new("", 5353, 0.0, 3), // Clear lyrics
        Lyric::new("Forms FORM-55551-5:", 5413, -1.0, 0),
        Lyric::new("Personnel File Addendum:", 5477, 1.13, 0),
        Lyric::new("", 5650, 0.05, 0),
        Lyric::new("Dear <<Subject Name Here>>,", 5650, -1.0, 0),
        Lyric::new("", 5900, -1.0, 0),
        Lyric::new("I'm not even angry.", 5900, 1.86, 0),
        Lyric::new("I'm being ", 6320, -1.0, 1),
        Lyric::new("so ", 6413, -1.0, 1),
        Lyric::new("sincere right now.", 6470, 1.9, 0),
        Lyric::new("Even though you broke ", 6827, -1.0, 1),
        Lyric::new("3", 7020, 0.0, 2), // ASCII 4
        Lyric::new("my heart.", 7090, -1.0, 0),
        Lyric::new("And killed me.", 7170, 1.43, 0),
        Lyric::new("4", 7300, 0.0, 2), // ASCII 5
        Lyric::new("And tore me to pieces.", 7500, 1.83, 0),
        Lyric::new("And threw every piece ", 7900, -1.0, 1),
        Lyric::new("into a fire.", 8080, 1.8, 0),
        Lyric::new("5", 8080, 0.0, 2), // ASCII 6
        Lyric::new("As they burned it hurt because", 8430, -1.0, 0),
        Lyric::new("6", 8690, 0.0, 2), // ASCII 7
        Lyric::new("I was so happy for you!", 8760, 1.67, 0),
        Lyric::new("Now, these points of data", 8960, -1.0, 0),
        Lyric::new("make a beautiful line.", 9167, -1.0, 0),
        Lyric::new("And we're out of beta.", 9357, -1.0, 0),
        Lyric::new("We're releasing on time.", 9560, -1.0, 0),
        Lyric::new("4", 9700, 0.0, 2), // ASCII 5
        Lyric::new("So I'm GLaD I got burned.", 9770, -1.0, 0),
        Lyric::new("2", 9913, 0.0, 2), // ASCII 3
        Lyric::new("Think of all the things we learned", 9983, -1.0, 0),
        Lyric::new("0", 10120, 0.0, 2), // ASCII 1
        Lyric::new("For the people who are", 10190, -1.0, 0),
        Lyric::new("Still alive.", 10327, 1.8, 0),
        // Page 3
        Lyric::new("", 10603, 0.0, 3), // Clear lyrics
        Lyric::new("Forms FORM-55551-6:", 10663, -1.0, 0),
        Lyric::new("Personnel File Addendum Addendum:", 10710, 1.36, 0),
        Lyric::new("", 10710, 0.05, 0),
        Lyric::new("One last thing:", 10910, -1.0, 0),
        Lyric::new("", 11130, 0.05, 0),
        Lyric::new("Go ahead and leave ", 11130, -1.0, 1),
        Lyric::new("me.", 11280, 0.5, 0),
        Lyric::new("I think I'd prefer to stay ", 11507, -1.0, 1),
        Lyric::new("inside.", 11787, 1.13, 0),
        Lyric::new("Maybe you'll find someone else", 12037, -1.0, 0),
        Lyric::new("To help you.", 12390, 1.23, 0),
        Lyric::new("Maybe Black ", 12737, -1.0, 1),
        Lyric::new("7", 12787, 0.0, 2), // ASCII 8
        Lyric::new("Mesa...", 12857, 2.7, 0),
        Lyric::new("THAT WAS A JOKE.", 13137, 1.46, 1),
        Lyric::new(" FAT CHANCE.", 13387, 1.1, 0),
        Lyric::new("Anyway, ", 13620, -1.0, 1),
        Lyric::new("8", 13670, 0.0, 2), // ASCII 9
        Lyric::new("this cake is great.", 13740, -1.0, 0),
        Lyric::new("It's so delicious and moist.", 13963, -1.0, 0),
        Lyric::new("9", 14123, 0.0, 2), // ASCII 10
        Lyric::new("Look at me still talking", 14193, -1.0, 0),
        Lyric::new("1", 14320, 0.0, 2), // ASCII 2
        Lyric::new("when there's science to do.", 14390, -1.0, 0),
        Lyric::new("0", 14527, 0.0, 2), // ASCII 1
        Lyric::new("When I look out there,", 14597, -1.0, 0),
        Lyric::new("It makes me GLaD I'm not you.", 14767, -1.0, 0),
        Lyric::new("2", 14913, 0.0, 2), // ASCII 3
        Lyric::new("I've experiments to run.", 14983, -1.0, 0),
        Lyric::new("4", 15120, 0.0, 2), // ASCII 5
        Lyric::new("There is research to be done.", 15190, -1.0, 0),
        Lyric::new("0", 15320, 0.0, 2), // ASCII 1
        Lyric::new("On the people who are", 15390, -1.0, 0),
        Lyric::new("still alive", 15553, 2.0, 1),
        // Page 4
        Lyric::new("", 15697, 0.0, 3), // Clear lyrics
        Lyric::new("", 15757, 0.05, 0),
        Lyric::new("", 15757, 0.05, 0),
        Lyric::new("", 15757, 0.05, 0),
        Lyric::new("PS: And believe me I am", 15757, -1.0, 0),
        Lyric::new("still alive.", 15960, 1.13, 0),
        Lyric::new("PPS: I'm doing Science and I'm", 16150, -1.0, 0),
        Lyric::new("still alive.", 16363, 1.13, 0),
        Lyric::new("PPPS: I feel FANTASTIC and I'm", 16550, -1.0, 0),
        Lyric::new("still alive.", 16760, -1.0, 0),
        Lyric::new("", 16860, -1.0, 0),
        Lyric::new("FINAL THOUGH:", 16860, -1.0, 0),
        Lyric::new("While you're dying I'll be", 16993, -1.0, 0),
        Lyric::new("still alive.", 17157, -1.0, 0),
        Lyric::new("", 17277, -1.0, 0),
        Lyric::new("FINAL THOUGH PS:", 17277, -1.0, 0),
        Lyric::new("And when you're dead I will be", 17367, -1.0, 0),
        Lyric::new("still alive.", 17550, 1.13, 0),
        Lyric::new("", 17550, -1.0, 0),
        Lyric::new("", 17550, 0.05, 0),
        Lyric::new("STILL ALIVE", 17760, 1.13, 0),
        Lyric::new("", 17900, 0.0, 3), // Clear lyrics
        Lyric::new("", 18500, 0.0, 3), // Clear lyrics
        Lyric::new("ENDENDENDENDENDENDENDEND", 18500, 0.05, 9), // END
    ]
}

pub const CREDITS: &str = r">LIST PERSONNEL

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
ENRICHMENT CENTER ACTIVITY!!J";
