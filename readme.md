#Ascii picture creator

Creates an [ascii art](https://en.wikipedia.org/wiki/ASCII_art) (text-based) image from a provided image file.

##Usage

1. Install Rust
1. Run `cargo run`

![](img/pic-tiny.jpg)

```
xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx%xxxxxxxxxxxxxxxxxx%xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx%%%%%%%x%xxxxxxxxxxxx%xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx%#%%%%%%%%%xxxxx%xxxx%%xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
oooxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx%%%######%##xx%%%@%oxx%%%xx%%xoxoooooooooooooooooooooooooooooo
oooooooooooooooooooooooooooooooooooxxoxxx%#%##%%%%%oox%#%xx%xo%%%%%o;;:;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
oooooooooooooooooo;;;;;;;;;;o;;o;oo%%x%%%##%###%%%#%x%#@@##%%%@%%###%%xoo;;;;;;;;;;;;;;;;;;;;;;;;;;;
ooooooooooooooooooooooo;oo;oo;oooxx%%%%##########%###@@@##@##@#%@#x%@#@%%%xo;;;;;;;;;;;;;;;;;;;;;;;;
oooooooooooooooooooooooooo;;ooox%%%%#####%#@@@@@@#####@@@######@@#;x##@%ox%%xo;;;;;;;;;;;;;;;;;;;;;;
oooooooooooooooooooooooooo;oooox%%#%%###@##@@@@@@@@@@@@@@@##@@@#@@##@@@#;;;;;;;;;;;;;;;;;;;;;;;;;;;;
oooooooooooooooo;ooo;;oooooooooox%%##@@@@@@@@@@@@@@@@@@@@@##@@@@@@@@@##@oo;;;;;;;;;;;;;;;;;;;;;;;;;;
oooooooooooooooooooooooooooooooxxxo%#@@@@@@@@@@@@@@####@@@###@@#@@@@@@###x;;;;;;;;;;;;;;;;;;;;;;;;;;
ooooooooooooooooooo;;o;ooooo;oooxxo%###@@@@@@@@@@@@@####@@@@@@###@@@@@####x:;;;;;;;;;;;;;;;;;;;;;;;;
ooooooooooooooooo;oooo;;;oooxoox##%#@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@###@@#%#x:;;;;;;;;;;;;;;;;;;;;;;;
ooooooooooooooooo;;oo;;;;oxoxx%##@@@@@@@@@@@@@@@@@@@@@@@@@#@@@@@@@@@@##@#xx%o;;o;;;;;;;;;;;;;;;;;;;;
ooooooooooooooooo;ooo;ooo;oxx%#@#@@@@@@@@@@@@@@@@@@@@@@@@@#@@@@@@@@@@@####x%#xx%%xo;;;;;;;;;;;;;;;;;
oooooooooooooooooooo;;;;;;;o%#%%##@@@@@@@@@@@@@@@@@@@@@@@@#@@@@@@@@@@@@####x%%xxxx%o;;;;;;;;;;;;;;;;
oooooooooooooooooo;;;oo;;;;;%%x%#@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@#%#%x#%ooooo;;;;;;;;;;;;;;;;
oooooooooooooooo;;;oooxxooo;%%#@@@@@@@@@@@@@@@@@@@@@@@@@@@##@#@@@@@@@@@@@##%%##%%x%%o;;;;;;;;;;;;;;;
oooooooooooooooooo;;;ox%xxx%#@@@@@@@@@@@@@@@@@@@@@@@@@@@#@####@@@@@@@@@@@@##@@######%o;;;;;;;;;;;;;;
ooooooooooo;;;;;o;o;;;;x%%%#@#@@##@@@@@##@@@@@@@@@#@@@#########@@@@@@@@@@@@@@@@##@%x#x;;;;;;;;;;;;;;
xxxxoooooo;;;;;;;o;;;ooo#######@@@@@@@###@@@@@@@####@##########@@@@@@@@@@@@@@@##%%@#%#o;;;;;;;;;;;;;
xxxoooooo;;;;;;;;;;o;oxx%@@####@@@@@@###@@@@@@##################@##@@@@@@@@@@@@@#%%%##x;;;;;;;;;;;;;
xxxxxoooo;;;;;;;;;;;;oxx%#@@@@##@@@@@@@@@@@@####################@###@@@@@@@@@@@@@#%o%@x;;;;;;;;;;;;;
%xxxxoooo;;;;;;;;;;;;ox%###@@@@@@@@#############################@@@#@@@@@@@@@@#%#@@%o#x;;;;;;;;;;;;;
%%xxxxoo;;;;;;;;;;;;ooo%#####@@@@@##%%%%############%###%%%######@@@@@@@@@@@@@##%#@@xxo;;;;;;;;;;;;;
%%%xxxoo;;;;;;;;;;;ooxox###@#@@@###%%%%####%%%%%###%%%%#%%%%%%%####@@@@@@@@@@@@@#%@##xo:;;;;;;;;;;;;
%%%xxxxo;;;;;;;;;;;ox%xx%%###@@@###%%%#####%%%%%####%#######%%%####@@@@@@@@@@@@@@##@%%x;;;;;;;;;;;;;
xxxxxxooo;;;;;;;;;;;x#%%#####@@@##%%%x%#%#############@@@@@###%%###@@@@@@@@@#@@##@###%x;;;;;;;;;;;;;
xxxxxxooo;;;;;;;;;;;ox#@#@@@@@@##%x%%%####%########@@@@##############@@@@@@@@@####@%%%%o;;;;;;;;;;;;
xxxxxo;oo;;;;;;;;;;oo;%#@@@@@@@#%%%%##@@@##########@@################@@@@@@@@@@#%#%#%%xx;;;;;;;;;;;;
;;ooo;:ooo;;;;;;;;;oxxox%@@##@@#####@@@@@@############################@@@@@@@@@@%%%%#xo;;;;;;;;;;;;;
,,,,:;:;oo;;;;;;;;;;x%%%#@@##@@#%#####################@@@@@###########@@@@@@@@@@#%%%#%;:;;;;;;;;;;;;
;:,,:;;:oo;;;;;;;;;;ox#@@@###@@#%#@@@@@@@##@#########@@@@@@@##########@@@@@@@@@@@@%%%%;;;;;;;;;;;;;;
,,..,:;:;oo;;;;;;;;oox##%###@@@%#@@@@@@@@@@@#%%###@@@@@@@##@@##%######@@@@@@@@@@@@#%#x;;::;;::;;;;;;
,.,,.,;:;oo;;;;;;;;;ox#x#@@#@@#%@@@@##@@@@@@%%%#@@@@@@@@@#%#@@#######@@@@@@@@@@@#@####o:::;::::;::::
.....,:;:oo;;;;;;;;;x%x##@@@@@##@@@@###@@@@#x%##@@@@@@@@#####@@#######@@@@@@@@@@@###%x%o::::::::::::
......,::;oo;;;;:;:;x##@#@@@@@#########@@@#%x%%#@@#@@@@####%##########@@@@@@@@@@####%xo;::::::::::::
......,:::oo;;:::;:o%%##@@@@@@####%%###@@@#x%%%#@@@@@@#################@@@@@@@@@####%xo;::::::::::::
......,:;:;o;::::;:;x%%##@@@@@%#%######@@@%x%%%%##@@@########%%########@@@@@@@@@#####x;:::::::::::::
.,..,..,:::o;::::;::x%%##@@@@@%%%######@@#xx%%%####@@@######%%%%#######@@@@@@@@@@#%%%o;:::::::::::::
.,..,,..:::;o;:::::;x%x%#@@@@#%x%#####@##%xx%%%%#%###########%%%##%%###@@@@@@@@@@#x%xo::::::::::::::
........,::;o;::::::oxo%#@@@@%xx%%%#####%xx%%%%%%%########%%##%%%%%%###@@@@@@@@@@@#%xo::::::::::::::
.........:::oo:::::::o;%##@@@xxxx%#####%xx%%xx%%#############%%%%%%%####@@@@@@@@@@@#%o;:::::::::::::
..,..,.,.,::;o;:::::::;x#@@@#x%x%%%%%%x%%%%%%%%%############%%%%%%%%%###@@@@@@@@@@@#%o;:::::::::::::
.,...,,..,:::o;::::::::o%@@@%x%%%%%%xx%%########%##%%%%#####%%%%%%%%%%##@@@@@@@@@@@@%o::::::::::::::
.,...,,...,::;o:::::::::x#@@%x%%%%xxxx%########%%%#%%%%%%####%%%%%%%%%##@@@@@@@@@@#%x:,:::::::::::::
...,..,...,::;o;::::::::x#@@%%%%%xx%%%#@@###@@@#%###%%%%%####%##%%%%%%##@@@@@@@@@@#x::::::::::::::::
,..,..,,...:::;;::::::::;#@#%%%%%%x%%%#@@@##@@@#%###%%%%%%%#%%%%%%%%%%##@@@@@@@@@@@x;:::::::::::::::
:,.....,.,.,::;o:::::::::%@#%%%%%%%%%%%#@####@@#####%%%%%%%###%%%#%%%%##@@@##@@@@@@#;,,,,,,,,,,,,,,,
;:..,..,,..,::;oo;;;;;;;o#@#%%%%%%%%#%%#@@####@######%%%%%%#######%%%%%#@@@###@@##@%;;;:;;;;;;;;::::
::,.,..,,.,.,::x%xxxxxxx%%##%%%%%%%%#%x#@#####@##%%##%%%%%%%%%%###%%%%%###@@###@%%%xooooooooooooooxx
::,..,......,:::o:::::;:;:;#%%%%%%%#%xx%%######%%%%###%%%%%%#%%%#%%%%%%%###@@##o;;;;;;;;;;;;;;;;;;;;
:::,....,,,.,:::;:,,,,,,,,,o%%%%%%%%%x%%%####%#%%%####%%%%%%#####%%%%%%%###@@##;,.,,,,,,,,,,,,,,,,,,
,,::.....,,..,:,;;,,,,,,,:.:x%%%%%%%%%%%%#############%%%%%%#####%%%%%%%##@@#%@#xxx;,,,,,,,,,,,,,,,,
,,::,.....,,.,:::;:,,,,,,,,,x%%%%%%%%%xx%%%%%###%%###%%%%%%%%%%#%%%%%%%%@@####@@@@%:,,,,,,,,,,,,,,,,
,,,::......,..,::;;,,,,,,,,:x%%%%%%%%%%###%####%%#####%%%%%%%%%%%%%%%%%%@@%#%@@@@@#o,,,,,,,,,,,,,,,,
,,,,:,...,....,:::;,,,,,,,,,x%%%%%%%###@@@#@@###%%#####%%%#%%%%%%%%%%%%%##%#%@@###@%,,,,,,,,,,,,,,,,
,,,,,,,.......,,::;:,,,,,,.,%%%%%%##@@@@@@@@@@@@#######%%%##%%%%%#%%%x%####%%#@#%%%;,,,,,,,,,,,,,,,,
,,,,,:,...,....,:,:;,,,,,,.,x%%%%%#######@@@@@@@@@####%%%%######%#%%%x%#%#%%@#xxoo;:,,,,,,,,,,,,,,,,
,,,,,,,.........,::;,,.,,,,.x#%%%##%%xxx%%%##########%%%#%#####%##%%xx%@#%##@#xx;,.,.,,,.,,,,,,.,,,,
,.,,.,,,........,,,;:.,.,...o#%%##%%%%%%%%%%%%%%%######%#%#####%#%%%xx#@@@@@#x%@:..,.,,...,,,,......
,.....,,........,,,:;.,..,.,o#%###%%%##################%#%######%%%%xx@@@@@@@@@o,.,..,..............
......,,,........,,,;,,..,..:######%%############################%%xo%@@@@@##@%,,.,....,............
.......,,,......,.,,::.,....,%######%###@@@######################%%xo@@@@@@@@@o,,,.....,............
.....,..,,......,.,,:;,...,,.o@##%%%###%########################%%%ox@@@@@@@##o.....,...............
........,,........,,,:,..,,,.:###%%%%%%%x%%%%###################%%xo#@@@@@@#@%,.....,...............
...................,,,,..,,.,.%@#%%%%%%%%%%%%%%%##############%%%%ox#@@@@@@@@:...,..................
..............,,,,..,,.......,:@%%%%%%%%%%%%%%%%%#############%%%xox#@@@@@@@;.......................
....................,,,,......,x@%%%%%%#####%%###############%%%xoxx#@##@%o:..,.....................
............,,,,,,,,,....,.....:@#%#%%%#####################%%%%xo%x#@###o:..,.,....................
...........,,,,,::,,,,,......,..o@###%######################%%%xx%%x%xoo;:,.,,......................
................,,,..........,,.,x@########################%%%xxx%%xo;:,.,,...,.....................
...........................,..,..,%@@@#####################%%xxx%%%x::,,.......,....................
.............,,,.................,%#@@@#########@###########%x%%%%#o.,...,,.,,......................
................,...........,,...,%####@#######@############%%%#%%%;,...,.....,.....................
................................,,%#########################%%%%%%%;................................
,,,,,,,,.....................,,,.,%####@@####################%%%%%%:.,.,............................
::::::,,,,,,,,,,,,,,,,,,,,,,,,,,,:%%####@@#####@#############%%%%%%:.,.,............................
;;;::::::::::::::::::::::::::::::;%%########################%%%%%%x:,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;o%%######################%%%%%%%%x;:;;;;;;;;;;:::::::::::::::::::::
oooooooooooooooooooooooooooooo;;;o%%%####################%%%%%%%%%x;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;:o%%%#@@#################%%%%%%%%%x;:;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;:o%%%#####################%%%%%%%%x;::::::::::::::::::::::::::::::::
;;;;;;;;;;;;;;;;;;;;;;;;;;;;:;;::o%%%%#@#################%%%%%%%%%xo::::::::::::::::::::::::::::::::
;;;;;;;;;;;;;;;;;;;;;:::;::;;::;:o%%%%##################%%%%%%%%%%xo::::::::::::::::::::::::::::::::
;;;;;;;;;;;;;::::::::::::::::::::o%%%%%################%%%%%%%%%%%xx;,::::::::::::::::::::::::::::::
;;;;;;;;;;;::::::::::::::::::::::o#%%%%###############%%%%%%%%%%%%xxo:,:::::::::::::::::::::::::::::
;;;;;;;;;;:::::::::::::::::::::::x#%%%%##############%%%%%%%%%%%%%xxxo:,,:::::::::::::::::::::::::::
;;;;;;;:;::::::::::::::::::::::::x%%%%%%#############%%%%%%%%%%%%%xxxxxx;,::::::::::::::::::::::::::
;;;;;:::::::::::::::::::::::::::o%%%%%%%%#############%%%%%%%%%%%%%xxxx#@o,,,:::::::::::::::::::::::
;;;;;:::::::::::::::::::::;:,o%ox%%%%%%%%#############%%%%%%%%%%%%x%%xx%@@%x;,,:::::::::::::::::::::
;;;;;;:::::::::::;::::::::,,x@%ox%%%%%%%%%############%%%%%%%%%%%%%xx%x#@@@@@%;.,:::::::::::::::::::
;;;;;;;;;;;;;;;;;;:;;;;::,:x@%oxx%%%%%%%%%#############%%%%%%%%%%%%%%x%@@@@@@@@%;:,,::::::::::::::::
:::::::::::::::::::::::,:%@@@%xx%%%%%%%%%%##############%%%%%%%%%%%%%x#@@@@@@@@@@#o:,,::::::::::::::
::::,,,,,,,,,,,,,,,,:,,;#@@@@%xx%%%%%%%%%%%###############%%%%%%%%%%%%@@@@@@@@@@@@@@x:,,,,,,,,,,,,,,
```