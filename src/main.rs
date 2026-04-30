use std::collections::HashSet;
use std::io;
use std::panic;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use chrono::{Local, Timelike};
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use rand::Rng;
use rand::seq::SliceRandom;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Alignment, Constraint, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Terminal;

#[derive(Copy, Clone)]
enum Theme {
    Sun,
    Shadow,
    Time,
    Mortality,
    Light,
    Wisdom,
    Work,
    Nature,
}

impl Theme {
    fn raw_arts(&self) -> &'static [&'static str] {
        match self {
            Theme::Sun => &[
                concat!(
                    "        \\     (      /\n",
                    "   `.    \\     )    /    .'\n",
                    "     `.   \\   (    /   .'\n",
                    "       `.  .-''''-.  .'\n",
                    " `~._    .'/_    _\\`.    _.~'\n",
                    "     `~ /  / \\  / \\  \\ ~'\n",
                    "_ _ _ _|  _\\O/  \\O/_  |_ _ _ _\n",
                    "       | (_)  /\\  (_) |\n",
                    "    _.~ \\  \\      /  / ~._\n",
                    " .~'     `. `.__.' .'     `~.\n",
                    "       .'  `-,,,,-'  `.\n",
                    "     .'   /    )   \\   `.\n",
                    "   .'    /    (     \\    `.\n",
                    "        /      )     \\\n",
                    "              (\n",
                ),
                concat!(
                    "             .\n",
                    "         .   :   .\n",
                    "     '.   .  :  .   .'\n",
                    "  ._   '._.-'''-._.'   _.\n",
                    "    '-..'         '..-'\n",
                    " --._ /.==.     .==.\\ _.--\n",
                    "     ;/_o__\\   /_o__\\;\n",
                    "-----|`     ) (     `|-----\n",
                    "    _: \\_) (\\_/) (_/ ;_\n",
                    " --'  \\  '._.=._.'  /  '--\n",
                    "   _.-''.  '._.'  .''-._\n",
                    "  '    .''-.(_).-''.    '\n",
                    "     .'   '  :  '   '.\n",
                    "        '    :   '\n",
                    "             '\n",
                ),
                concat!(
                    "                        |\n",
                    "                    .   |\n",
                    "                        |\n",
                    "          \\    *        |     *    .  /\n",
                    "            \\        *  |  .        /\n",
                    "         .    \\     ___---___     /    .\n",
                    "                \\.--         --./\n",
                    "     ~-_    *  ./               \\.   *   _-~\n",
                    "        ~-_   /    ^         ^    \\   _-~     *\n",
                    "   *       ~-/    ___       ___    \\-~\n",
                    "     .      |    (_O_)     (_O_)    |      .\n",
                    "         * |                         | *\n",
                    "-----------|                         |-----------\n",
                    "  .        |    <               >    |        .\n",
                    "        *   |    \\             /    | *\n",
                    "           _-\\    `.         .'    /-_    *\n",
                    "     .  _-~ . \\     `-.___.-'     /   ~-_\n",
                    "     _-~       `\\               /'*      ~-_\n",
                    "    ~           /`--___   ___--'\\           ~\n",
                    "           *  /        ---     .  \\      .\n",
                    "            /     *     |           \\\n",
                    "          /             |   *         \\\n",
                    "                     .  |        .\n",
                    "                        |\n",
                    "                        |\n",
                ),
            ],
            Theme::Shadow => &[
                concat!(
                    "         __\n",
                    "        / /\\\n",
                    "       / /  \\\n",
                    "      / /    \\__________\n",
                    "     / /      \\        /\\\n",
                    "    /_/        \\      / /\n",
                    " ___\\ \\      ___\\____/_/_\n",
                    "/____\\ \\    /___________/\\\n",
                    "\\     \\ \\   \\           \\ \\\n",
                    " \\     \\ \\   \\____       \\ \\\n",
                    "  \\     \\ \\  /   /\\       \\ \\\n",
                    "   \\   / \\_\\/   / /        \\ \\\n",
                    "    \\ /        / /__________\\/\n",
                    "     /        / /     /\n",
                    "    /        / /     /\n",
                    "   /________/ /\\    /\n",
                    "   \\________\\/\\ \\  /\n",
                    "               \\_\\/\n",
                ),
                concat!(
                    "      ____________\n",
                    "     /\\  ________ \\\n",
                    "    /  \\ \\______/\\ \\\n",
                    "   / /\\ \\ \\  / /\\ \\ \\\n",
                    "  / / /\\ \\ \\/ / /\\ \\ \\\n",
                    " / / /__\\_\\/ / /__\\_\\ \\\n",
                    "/ /_/_______/ /________\\\n",
                    "\\ \\ \\______ \\ \\______  /\n",
                    " \\ \\ \\  / /\\ \\ \\  / / /\n",
                    "  \\ \\ \\/ / /\\ \\ \\/ / /\n",
                    "   \\ \\/ / /__\\_\\/ / /\n",
                    "    \\  / /______\\/ /\n",
                    "     \\/___________/\n",
                ),
                concat!(
                    "          ________\n",
                    "         /\\       \\\n",
                    "        /  \\       \\\n",
                    "       /    \\       \\\n",
                    "      /      \\_______\\\n",
                    "      \\      /       /\n",
                    "    ___\\    /   ____/___\n",
                    "   /\\   \\  /   /\\       \\\n",
                    "  /  \\   \\/___/  \\       \\\n",
                    " /    \\       \\   \\       \\\n",
                    "/      \\_______\\   \\_______\\\n",
                    "\\      /       /   /       /\n",
                    " \\    /       /   /       /\n",
                    "  \\  /       /\\  /       /\n",
                    "   \\/_______/  \\/_______/\n",
                ),
                concat!(
                    "                        .\n",
                    "                  .     :     .\n",
                    "       _           '. _.:._ .'\n",
                    "      (_)       '-. .'     '. .-'\n",
                    "      _;_     ' - ./         \\. - '\n",
                    "     / | \\    - - |           | - -\n",
                    "\"\"\"\" \\ |  \\ \"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\n",
                    "      `|\\  `\n",
                    "       | \\\n",
                    "      /  /\n",
                    "     /  /_\n",
                    "     `\n",
                ),
            ],
            Theme::Time => &[
                concat!(
                    "       ,--.-----.--.\n",
                    "       |  |-----|  |\n",
                    "     __|--|     |--|__\n",
                    "    /  |  |-----|  |  \\\n",
                    "   /   \\__|-----|__/   \\\n",
                    "  /   ______---______   \\/\\\n",
                    " /   /               \\   \\/\n",
                    "{   /    _     _   _  \\   }\n",
                    "|  {    | | . | | | |  }  |-,\n",
                    "|  |    |_| . |_| |_|  |  | |\n",
                    "|  {                   }  |-'\n",
                    "{   \\                 /   }\n",
                    " \\   `------___------'   /\\\n",
                    "  \\     __|-----|__     /\\/\n",
                    "   \\   /  |-----|  \\   /\n",
                    "    \\  |--|     |--|  /\n",
                    "     --|  |-----|  |--\n",
                    "       |--|-----|--|\n",
                    "       `--'-----`--'\n",
                ),
                concat!(
                    "          _ (_) _\n",
                    "        /`_) H (_`\\\n",
                    "      .' (  { }  ) '.\n",
                    "    _/ /` '-'='-' `\\ \\_\n",
                    "   [_.'   _,...,_   '._]\n",
                    "    |   .:\"`````\":.   |\n",
                    "    |__//_________\\\\__|\n",
                    "     | .-----------. |\n",
                    "     | |  .-\"\"\"--.  | |\n",
                    "     | | /    /  \\ | |\n",
                    "     | ||-   <   -|| |\n",
                    "     | | \\    \\  / | |\n",
                    "     | |[`'-...-'`]| |\n",
                    "     | | ;-.___.-; | |\n",
                    "     | | |  |||  | | |\n",
                    "     | | |  |||  | | |\n",
                    "     | | |  |||  | | |\n",
                    "     | | |  |||  | | |\n",
                    "     | | | _|||_ | | |\n",
                    "     | | | >===< | | |\n",
                    "     | | | |___| | | |\n",
                    "     | | '-------' | |\n",
                    "    _| '-----------' |_\n",
                    "   [= === === ==== == =]\n",
                    " `\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"`\n",
                ),
                concat!(
                    "          ______          \\'/\n",
                    "      .-'` .    `'-.    -= * =-\n",
                    "    .'  '    .---.  '.    /.\\\n",
                    "   /  '    .'     `'. \\\n",
                    "  ;  '    /          \\|\n",
                    " :  '  _ ;            `\n",
                    ";  :  /(\\ \\\n",
                    "|  .       '.\n",
                    "|  ' /     --'\n",
                    "|  .   '.__\\\n",
                    ";  :       /\n",
                    " ;  .     |            ,\n",
                    "  ;  .    \\           /|\n",
                    "   \\  .    '.       .'/\n",
                    "    '.  '  . `'---'`.' \n",
                    "      `'-..._____.-`\n",
                ),
            ],
            Theme::Mortality => &[
                concat!(
                    "             ___\n",
                    "            /   \\\\\n",
                    "       /\\\\ | . . \\\\\n",
                    "     ////\\\\|     ||\n",
                    "   ////   \\\\ ___//\\\n",
                    "  ///      \\\\      \\\n",
                    " ///       |\\\\      |\n",
                    "//         | \\\\  \\   \\\n",
                    "/          |  \\\\  \\   \\\n",
                    "           |   \\\\ /   /\n",
                    "           |    \\\\/   /\n",
                    "           |     \\\\/|\n",
                    "           |      \\\\|\n",
                    "           |       \\\\\n",
                    "           |        |\n",
                    "           |_________\\\n",
                ),
                concat!(
                    "      .-.\n",
                    "     (o.o)\n",
                    "      |=|\n",
                    "     __|__\n",
                    "   //.=|=.\\\\\n",
                    "  // .=|=. \\\\\n",
                    "  \\\\ .=|=. //\n",
                    "   \\\\(_=_)//\n",
                    "    (:| |:)\n",
                    "     || ||\n",
                    "     () ()\n",
                    "     || ||\n",
                    "     || ||\n",
                    "    ==' '==\n",
                ),
                concat!(
                    ".                                       ,\n",
                    ")).               -===-               ,((\n",
                    "))).                                 ,(((\n",
                    "))))).            .:::.           ,((((((\n",
                    "))))))))).        :. .:        ,(((((((('\n",
                    "`))))))))))).     : - :    ,((((((((((((\n",
                    " ))))))))))))))))_:' ':_((((((((((((((('\n",
                    " `)))))))))))).-' \\___/ '-._((((((((((\n",
                    "  `))))_._.-' __)(     )(_  '-._._(((('\n",
                    "   `))'---)___)))'\\_  _/'((((__(---'(('\n",
                    "     `))))))))))))|' '|(((((((((((('\n",
                    "       `)))))))))/'   '\\((((((((('\n",
                    "         `)))))))|     |((((((('\n",
                    "          `))))))|     |(((((('\n",
                    "                /'     '\\\n",
                    "               /'       '\\\n",
                    "              /'         '\\\n",
                    "             /'           '\\\n",
                    "             '---..___..---'\n",
                ),
            ],
            Theme::Light => &[
                concat!(
                    "   ,,;;;;;,,\n",
                    " ,;;:::::::;;,\n",
                    ",;;::' , ':::;,\n",
                    ";;::  /(   ::;;\n",
                    ";;:: |  \\  ::;;\n",
                    "';;::.\\c/.::;;'\n",
                    " ';:::'-,:::;'\n",
                    "   '';| |;''\n",
                    "      '-,\n",
                    "      | |\n",
                    "      '-,\n",
                    "      | |\n",
                    "      '-,\n",
                    "      | |\n",
                    "      '-,\n",
                    "      | |\n",
                    "     /`\"`\\\n",
                    "  .-'.  _.'-.\n",
                    " `._  `    _.'\n",
                    "    `\"---\"`\n",
                ),
                concat!(
                    "         ,,;;;,,\n",
                    "       ,;;:::::;;,\n",
                    "      ;;::  )  ::;;\n",
                    "      ;;:: (_) ::;;\n",
                    "      ';;:.-'-.;;'\n",
                    "        ';|   |;'\n",
                    "          |   |\n",
                    "          |   |\n",
                    "          |   |\n",
                    "          |   |\n",
                    "        __|   |__   .-.\n",
                    "     .-'  |   |  `-:   :\n",
                    "    :     `---'     :-'\n",
                    "     `-._       _.-'\n",
                    "         '\"\"\"\"\"\"'\n",
                ),
                concat!(
                    "           _ _ _\n",
                    "          '.` `.' \n",
                    "          _ >_< _\n",
                    "     _.-'`-.-=-.-`'-._\n",
                    "   .;'.--./     \\.--.';.\n",
                    " .'  >    \\     /    <  '.\n",
                    "/.__./\\__.';-=-;'.__/\\.__.\\\n",
                    "|  \\ /   \\/     \\/   \\ /  |\n",
                    ";   |    |       |    |   |\n",
                    " '._/\\   /\\     /\\   /\\_.' \n",
                    "      `-`  ;-.-;  `-`\n",
                    "            > <\n",
                    "        .-'{___}'-.\n",
                    "       '-.._____..-'\n",
                    "            |||\n",
                    "            |||\n",
                    "           /'-'\\\n",
                    "        .-',_ _,'-.\n",
                    "       /'._ .`. _.'\\  \n",
                    "  .='  . `. .` .  '=.\n",
                    "     '-'`-./   \\.-`'-'\n",
                    "           '._.' \n",
                ),
                concat!(
                    "                           (    )\n",
                    "                          (    )\n",
                    "                            )  )\n",
                    "                           (  (\n",
                    "                            (_)\n",
                    "                    ________[_]________\n",
                    "           /\\      /\\        ______    \\\n",
                    "          /  \\    //_\\       \\    /\\    \\\n",
                    "   /\\    / /\\/\\  //___\\       \\__/  \\    \\\n",
                    "  /  \\  /\\/    \\//_____\\       \\ |[]|     \\\n",
                    " /\\/\\/\\/       //_______\\       \\|__|      \\\n",
                    "/      \\      /XXXXXXXXXX\\                  \\\n",
                    "        \\    /_I_II  I__I_\\__________________\\\n",
                    "               I_I|  I__I_____[]_|_[]_____I\n",
                    "               I_II  I__I_____[]_|_[]_____I\n",
                    "               I II__I  I     XXXXXXX     I\n",
                    "            ~~~~~\"   \"~~~~~~~~~~~~~~~~~~~~~~~~\n",
                ),
            ],
            Theme::Wisdom => &[
                concat!(
                    "   ...    *    .   _  .\n",
                    "*  .  *     .   * (_)   *\n",
                    "  .      |*  ..   *   ..\n",
                    "   .  * \\|  *  ___  . . *\n",
                    "*   \\/   |/ \\/{o,o}     .\n",
                    "  _\\_\\   |  / /)  )* _/_ *\n",
                    "      \\ \\| /,--\"-\"---  ..\n",
                    "_-----`  |(,__,__/__/_ .\n",
                    "       \\ ||      ..\n",
                    "        ||| .            *\n",
                    "        |||\n",
                    "        |||\n",
                    "  , -=-~' .-^- _\n",
                ),
                concat!(
                    "                             /\\\n",
                    "                            /  \\\n",
                    "                           |    |\n",
                    "                         --:'''':--\n",
                    "                           :'_' :\n",
                    "                           _:\"\":\\___\n",
                    "            ' '      ____.' :::     '._\n",
                    "           . *=====<<=)           \\    :\n",
                    "            .  '      '-'-'\\_      /'._.' \n",
                    "                             \\====:_ \"\"\n",
                    "                            .'     \\\\\n",
                    "                           :       :\n",
                    "                          /   :    \\\n",
                    "                         :   .      '.\n",
                    "         ,.  _           :  : :      :\n",
                    "      '-'    ).          :__:-:__.;--'\n",
                    "    (        '  )        '-'   '-'\n",
                    " ( -   .00.   - _\n",
                    "(    .'  _ )     )\n",
                    "'-  ()_.\\,\\,   -\n",
                ),
                concat!(
                    "          _\n",
                    "         (_) -\n",
                    "               '\n",
                    "       @_  _    '\n",
                    "        )\\/(@    '\n",
                    "      __(/ \\--._\n",
                    "     (,-.---'--'@\n",
                    "      @ )0_0(     _\n",
                    "        ('-')    (_)\n",
                    "   '    _\\Y/_\n",
                    "   ' .-'-\\-/-'-._  '\n",
                    "   _ /    '*     \\ '\n",
                    "  (_)  /)  *    .-.))>'\n",
                    "    ._/  \\__*_ /\\__'.\n",
                    "'<((_'    |__H/  \\__\\\n",
                    "          /   ,_/ |_|\n",
                    "          )-- /   |x|\n",
                    "          \\ _/    (_ x\n",
                    "          /_/       \\_\\@\n",
                    "         /_/\n",
                    "        /x/\n",
                    "       (_ x\n",
                    "         \\_\\@\n",
                ),
                concat!(
                    "         ,-.\n",
                    "          ) \\\n",
                    "      .--'   |\n",
                    "     /       /\n",
                    "     |_______|\n",
                    "    (  O   O  )\n",
                    "     {'-(_)-'}\n",
                    "   .-{   ^   }-.\n",
                    "  /   '.___.'   \\\n",
                    " /  |    o    |  \\\n",
                    " |__|    o    |__|\n",
                    " (((\\_________/)))\n",
                    "     \\___|___/\n",
                    " .--' | | '--.\n",
                    "  \\__._| |_.__/\n",
                ),
            ],
            Theme::Work => &[
                concat!(
                    "                __...__\n",
                    "        .--\"\"```       ```\"\"--..\n",
                    "         ':--..___   ___..--:'\n",
                    "           \\      ```      /\n",
                    "         .-`  ___.....___  '-.\n",
                    "       .:-\"\"``   ~   ~   ``\"\":-.\n",
                    "      /`-..___ ~    ~   ~___..-'\\\n",
                    "     /  ~    '`\"\"-----\"\"`        \\\n",
                    "    ;                             ;\n",
                    "   ; '::.   '        .:'    _.     ;\n",
                    "   |~  .:' .     _   ':.           |\n",
                    "   |  ':. .  ~     .    _   .:     |\n",
                    "   ; '::.        _     /|| .;'     ;\n",
                    "    ;   ':      ( }    \\||D       ;\n",
                    "     \\.'.:':. | /\\__,=_[_]       /\n",
                    "      \\ ':. ~ |_\\__ |----|  `   /\n",
                    "       '.'::._|  |/ |--. |   ~.'\n",
                    "         '.-' |  /_ |    |`'.'\n",
                    "          (`'--..._____...--'`)\n",
                    "           `\"--...__ __...--\"`\n",
                ),
                concat!(
                    "       .--.                   .---.\n",
                    "   .---|__|           .-.     |~~~|\n",
                    ".--|===|--|_          |_|     |~~~|--.\n",
                    "|  |===|  |'\\     .---!~|  .--|   |--|\n",
                    "|%%|   |  |.'\\    |===| |--|%%|   |  |\n",
                    "|%%|   |  |\\.'\\   |   | |__|  |   |  |\n",
                    "|  |   |  | \\  \\  |===| |==|  |   |  |\n",
                    "|  |   |__|  \\.'\\ |   |_|__|  |~~~|__|\n",
                    "|  |===|--|   \\.'\\|===|~|--|%%|~~~|--|\n",
                    "^--^---'--^    `-'`---^-^--^--^---'--'\n",
                ),
                concat!(
                    "            ___\n",
                    "          .;___`'.\n",
                    "         /_  _ \\  |\n",
                    "         |a  a  \\_/     ,__\n",
                    "         | <     _)    _)(_)\n",
                    "  _,_     \\_--' /____.(/   \\\n",
                    " (_/(/'---/`\\_/`\\     |  $  |\n",
                    " /   \\.___\\_/`\\_| .---'.___.' \n",
                    "|  $  |   \\\\ '   \\ \\\n",
                    "'.___.'    \\\\_'___\\ \\\n",
                    "          .'   _   \\/\n",
                    "         /   .' \\   \\\n",
                    "        /   /    \\   \\\n",
                    "       |    |     |   |\n",
                    "       \\____\\    /____/\n",
                    "       (__,_|    |_,__)\n",
                ),
            ],
            Theme::Nature => &[
                concat!(
                    "             .-.'  '.-.\n",
                    "          .-(   \\  /   )-.\n",
                    "         /   '..oOOo..'   \\\n",
                    " ,       \\.--. oOOOOOOo.--./\n",
                    " |\\  ,   (   :oOOOOOOo:   )\n",
                    "_\\.\\/|   /'--'oOOOOOOo'--'\\\n",
                    "'-.. ;/| \\   .''oOOo''.   /\n",
                    ".--`'. :/|'-(   /  \\   )-'\n",
                    " '--. `. / //'-'.__.'-;\n",
                    "   `'-,_';//      ,  /|\n",
                    "        '((       |\\./_\n",
                    "          \\\\  . |\\; ..-'\n",
                    "           \\\\ |\\: .'`--.\n",
                    "            \\\\, .' .--'\n",
                    "             ))'_,-'`\n",
                    "            //-'\n",
                    "           //\n",
                    "          //\n",
                    "         |/\n",
                ),
                concat!(
                    "         .-==-.\n",
                    "        /{.=-.}\\\n",
                    "       | / .  \\ |\n",
                    "       |;   :  :|\n",
                    "       \\(   :  )/\n",
                    "        `._'__.'\n",
                    "      |\\   ||\n",
                    "      \\ \\  ||\n",
                    "       | | ||\n",
                    "       | | ||   /|\n",
                    "       \\  \\||  / /\n",
                    "        \\ ||| | |\n",
                    "         | || | |\n",
                    "          \\||/  /\n",
                    "           ||| /\n",
                    "           || |\n",
                    "           ||/\n",
                    "           ||\n",
                ),
            ],
        }
    }

    /// Returns a random art piece with all lines padded to equal width.
    /// If `prev` is provided and the theme has multiple arts, avoids picking
    /// art that produces the same padded output as `prev`.
    fn art(&self, rng: &mut impl Rng, prev: Option<&str>) -> String {
        let arts = self.raw_arts();
        let mut padded = String::new();
        for _ in 0..arts.len() * 2 {
            let raw = arts.choose(rng).unwrap();
            let max_w = raw.lines().map(|l| l.len()).max().unwrap_or(0);
            padded = raw
                .lines()
                .map(|l| format!("{:width$}", l, width = max_w))
                .collect::<Vec<_>>()
                .join("\n");
            if arts.len() <= 1 || prev != Some(padded.as_str()) {
                return padded;
            }
        }
        padded // fallback: accept the duplicate rather than hang
    }
}

use Theme::*;

enum TimeOfDay {
    Dawn,
    Morning,
    Afternoon,
    Evening,
    Night,
}

impl TimeOfDay {
    fn from_hour(hour: u32) -> Self {
        match hour {
            5..=7 => TimeOfDay::Dawn,
            8..=11 => TimeOfDay::Morning,
            12..=16 => TimeOfDay::Afternoon,
            17..=19 => TimeOfDay::Evening,
            _ => TimeOfDay::Night,
        }
    }

    fn accent(&self) -> Color {
        match self {
            TimeOfDay::Dawn => Color::Rgb(200, 170, 130),
            TimeOfDay::Morning => Color::Rgb(240, 200, 85),
            TimeOfDay::Afternoon => Color::Rgb(230, 200, 60),
            TimeOfDay::Evening => Color::Rgb(230, 140, 100),
            TimeOfDay::Night => Color::Rgb(140, 160, 200),
        }
    }

    const ALL: [TimeOfDay; 5] = [
        TimeOfDay::Dawn,
        TimeOfDay::Morning,
        TimeOfDay::Afternoon,
        TimeOfDay::Evening,
        TimeOfDay::Night,
    ];
}

// Sundial mottoes, many sourced from Alfred H. Hyatt's
// "Though Silent, I Speak: A Book of Sundial Mottoes" (1903).
const MOTTOS: &[(Theme, &str, &str)] = &[
    // --- Original selection ---
    (Sun, "Horas non numero nisi serenas", "I count only the sunny hours"),
    (Time, "Tempus fugit", "Time flies"),
    (Wisdom, "Carpe diem", "Seize the day"),
    (Light, "Lux et umbra vicissim", "Light and shadow by turns"),
    (Sun, "Sol omnibus lucet", "The sun shines for everyone"),
    (Shadow, "Umbra sumus", "We are shadows"),
    (Time, "Sic transit gloria mundi", "So passes the glory of the world"),
    (Time, "Tempus edax rerum", "Time, devourer of all things"),
    (Light, "Post tenebras lux", "After darkness, light"),
    (Shadow, "Lucem demonstrat umbra", "The shadow proves the sunshine"),
    (Sun, "Sine sole sileo", "Without the sun I am silent"),
    (Wisdom, "Vita in motu", "Life is in motion"),
    (Time, "Pereunt et imputantur", "They pass and are charged to our account"),
    (Wisdom, "Festina lente", "Make haste slowly"),
    (Mortality, "Ultima forsan", "Perhaps the last"),
    (Mortality, "Omnes vulnerant, ultima necat", "All hours wound, the last one kills"),
    (Sun, "Sol me, vos umbra regit", "The sun guides me, the shadow guides you"),
    (Wisdom, "Amicum secreto admone", "Advise a friend in secret"),
    (Light, "Ab hoc momento pendet aeternitas", "Eternity hangs from this moment"),
    (Work, "Transit hora, manent opera", "The hour passes, the works remain"),
    // --- From "Though Silent, I Speak" (1903) - Latin ---
    (Wisdom, "Silens loquor", "Though silent, I speak"),
    (Light, "Aliis inserviendo consumor", "My loss is your gain"),
    (Mortality, "Disce dies numerare tuos", "Learn to number thy days"),
    (Time, "Dum spectas fugio", "While you watch, I flee"),
    (Shadow, "Homo quasi umbra", "Man is a shade of a shadow"),
    (Mortality, "Hora pars vitae", "Every hour shortens life"),
    (Time, "Mora trahit periculum", "Delay drags danger"),
    (Light, "Nemo sine crimine vivit", "The brightest day has its shades"),
    (Wisdom, "Non nobis nati sumus", "We are not made for ourselves"),
    (Wisdom, "Nosce teipsum", "Look into thyself"),
    (Light, "Tempus ad lucem ducit veritatem", "Time brings truth to light"),
    (Time, "Tempus vitae monitor", "Time is life's remembrancer"),
    (Shadow, "Umbra Dei", "The workmanship of the great architect"),
    (Shadow, "Ut umbra sic vita", "Shade is life's pattern"),
    (Time, "Vigilate et orate, tempus fugit", "Watch and pray, time hastes away"),
    (Wisdom, "Vita nostra est instar comoediae", "Life often changes scenes"),
    (Time, "Veritas temporis filia", "Time, the father of truth"),
    (Time, "Transeunt dies tui", "Thy days are passing"),
    (Mortality, "Aspice in horam, et memento mori", "Look on the hour and remember death"),
    (Time, "Aetas cito pede praeterit", "Age passes with swift foot"),
    (Shadow, "Omnia sub vanitas", "Under the sun, vanity"),
    (Light, "Transit umbra, lux permanet", "Shadow passes, light stays"),
    (Sun, "Sol regit omnia", "The sun rules all"),
    (Mortality, "Vive memor lethi, fugit hora", "Live remembering death, the hour flies"),
    (Time, "Sic labitur aetas", "Thus life slips away"),
    (Time, "Volat irrevocabilis hora", "Flies the hour, never to be recalled"),
    (Time, "Fugit irreparabile tempus", "Irrecoverable time is flying away"),
    (Mortality, "Veni, vide, vale", "Come, see, farewell"),
    (Time, "Vestigia nulla retrorsum", "There are no steps backward"),
    (Light, "Via crucis via lucis", "The way of the cross is the way of light"),
    (Time, "Maneo nemini", "I wait for none"),
    (Mortality, "Memor esto brevis aevi", "Remember how brief is life"),
    (Work, "Utere non numera", "Employ them, count them not"),
    (Work, "Age quod agis", "Do what you are doing"),
    (Wisdom, "Dum vivimus vivamus", "Live while you live"),
    (Time, "Aspice et abi", "Gaze and pass on"),
    (Sun, "Lux tua vita mea", "Thy light, my life"),
    (Time, "Irrevocabile", "Never to be recalled"),
    (Mortality, "Non redibo", "I shall not return"),
    (Time, "Est hora", "It is the hour"),
    (Time, "Vivite, ait, fugio", "Live, it says, I fly"),
    (Time, "Quid celerius tempore?", "What is swifter than time?"),
    (Shadow, "Circulus et umbra", "A circle and a shadow"),
    (Light, "A lumine motus", "Moved by light"),
    (Light, "In coelo quies", "In heaven is rest"),
    (Mortality, "Unam time", "Fear one hour"),
    (Mortality, "Vix orimur et occidimus", "Scarcely arisen and we have set"),
    (Light, "Lux et umbra vicissim sed semper amor", "Light and shadow by turns, but always love"),
    (Mortality, "Ultima latet et observantur omnes", "The last hour is hidden and all are watched"),
    (Wisdom, "Hora, dies, et vita fugiunt; manet unica virtus", "The hour, day, and life all fly; only virtue remains"),
    (Wisdom, "Una dabit quod altera negat", "One hour will give what the next denies"),
    // --- From "Though Silent, I Speak" (1903) - English ---
    (Time, "It is later than you think", "It is later than you think"),
    (Nature, "Hours fly, flowers die", "Hours fly, flowers die"),
    (Work, "Time wasted is existence, used is life", "Time wasted is existence, used is life"),
    (Sun, "I mark not the hours unless they be bright", "I mark not the hours unless they be bright"),
    (Shadow, "Shadows are we and like shadows depart", "Shadows are we and like shadows depart"),
    (Time, "Time is the chrysalis of eternity", "Time is the chrysalis of eternity"),
    (Sun, "'Tis always morning somewhere in the world", "'Tis always morning somewhere in the world"),
    (Work, "Do today's work today", "Do today's work today"),
    (Mortality, "The time thou killest will in time kill thee", "The time thou killest will in time kill thee"),
    (Wisdom, "I have no voice and yet I speak", "I have no voice and yet I speak"),
    (Wisdom, "Stay and think", "Stay and think"),
    (Wisdom, "Live today", "Live today"),
    // --- From "Though Silent, I Speak" (1903) - French ---
    (Light, "L'heure passe, l'amitie reste", "The hour passes, friendship stays"),
    // --- From "Though Silent, I Speak" (1903) - Italian ---
    (Sun, "Mia vita e il sole", "My life is the sun"),
    (Mortality, "Io vado e vengo ogni giorno, ma tu andrai senza ritorno", "I go and come every day, but thou shalt go without returning"),
];

fn fetch_weather() -> Option<String> {
    let agent = ureq::AgentBuilder::new()
        .timeout(Duration::from_secs(10))
        .build();
    let body = agent
        .get("https://wttr.in/?format=%C+%t")
        .set("User-Agent", "curl/7.0")
        .call()
        .ok()?
        .into_string()
        .ok()?;
    let cleaned: String = body
        .trim()
        .replace("+", "")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase();
    if cleaned.is_empty() || cleaned.contains("unknown") {
        None
    } else {
        Some(cleaned)
    }
}

fn footer_widget(accent: Color) -> Paragraph<'static> {
    Paragraph::new(Line::from(vec![
        Span::styled(
            " p ",
            Style::default()
                .fg(accent)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("prev  "),
        Span::styled(
            " n ",
            Style::default()
                .fg(accent)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("next  "),
        Span::styled(
            " q ",
            Style::default()
                .fg(accent)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("quit"),
    ]))
    .alignment(Alignment::Center)
}

fn run(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    quit_flag: &AtomicBool,
) -> io::Result<()> {
    let mut rng = rand::thread_rng();
    let mut bag: Vec<usize> = (0..MOTTOS.len()).collect();
    bag.shuffle(&mut rng);
    let mut bag_idx = 0;

    let initial_idx = bag[bag_idx];
    let initial_art = MOTTOS[initial_idx].0.art(&mut rng, None);
    let mut history: Vec<(usize, String)> = vec![(initial_idx, initial_art)];
    let mut hist_pos: usize = 0;
    let mut seen: HashSet<usize> = HashSet::from([initial_idx]);
    let mut debug_tod: Option<usize> = None;
    let mut last_interaction = Instant::now();
    let auto_advance = Duration::from_secs(15 * 60);

    let weather: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
    let weather_bg = Arc::clone(&weather);
    thread::spawn(move || {
        let mut fail_streak: u32 = 0;
        loop {
            if let Some(w) = fetch_weather() {
                if let Ok(mut guard) = weather_bg.lock() {
                    *guard = Some(w);
                }
                fail_streak = 0;
                thread::sleep(Duration::from_secs(30 * 60));
            } else {
                fail_streak = fail_streak.saturating_add(1);
                // 30s, 60s, 120s, 240s, 480s, then cap at 10 min
                let delay = (30u64 << (fail_streak - 1).min(4)).min(10 * 60);
                thread::sleep(Duration::from_secs(delay));
            }
        }
    });

    loop {
        if quit_flag.load(Ordering::Relaxed) {
            break;
        }

        let motto = &MOTTOS[history[hist_pos].0];
        let art_text = &history[hist_pos].1;

        terminal.draw(|frame| {
            let area = frame.area();

            let now = Local::now();
            let tod = match debug_tod {
                Some(i) => &TimeOfDay::ALL[i],
                None => &TimeOfDay::from_hour(now.hour()),
            };
            let accent = tod.accent();

            // Too small to render anything meaningful
            if area.width < 20 || area.height < 3 {
                let msg = Paragraph::new("Terminal too small")
                    .alignment(Alignment::Center)
                    .style(Style::default().fg(accent));
                frame.render_widget(msg, area);
                return;
            }

            let time_str = format!(" {} ", now.format("%H:%M"));
            let weather_str = weather.lock().ok().and_then(|g| g.clone());
            let mut outer = Block::default()
                .title(Line::from(" * sunshine * ").centered())
                .title_bottom(
                    Line::from(Span::styled(
                        time_str,
                        Style::default().fg(accent),
                    ))
                    .centered(),
                );
            if seen.len() < MOTTOS.len() {
                outer = outer.title_bottom(
                    Line::from(Span::styled(
                        format!(" {}/{} ", seen.len(), MOTTOS.len()),
                        Style::default().fg(Color::DarkGray),
                    ))
                    .right_aligned(),
                );
            }
            let mut outer = outer
                .borders(Borders::ALL)
                .border_style(Style::default().fg(accent));
            if let Some(ref w) = weather_str {
                outer = outer.title(
                    Line::from(Span::styled(
                        format!(" {} ", w),
                        Style::default().fg(Color::DarkGray),
                    ))
                    .left_aligned(),
                );
            }
            let inner_area = outer.inner(area);
            frame.render_widget(outer, area);

            let art_height = art_text.lines().count() as u16;

            // Minimal: just the English translation, centered
            if area.height < 7 {
                let layout = Layout::vertical([
                    Constraint::Fill(1),
                    Constraint::Length(1),
                    Constraint::Fill(1),
                ])
                .split(inner_area);

                let english = Paragraph::new(Line::from(Span::styled(
                    motto.2,
                    Style::default()
                        .fg(accent)
                        .add_modifier(Modifier::BOLD),
                )))
                .alignment(Alignment::Center);
                frame.render_widget(english, layout[1]);
                return;
            }

            // Compact: motto text + footer, no art
            if area.height < art_height + 9 {
                let is_english_only = motto.1 == motto.2;

                if is_english_only {
                    let layout = Layout::vertical([
                        Constraint::Fill(1),
                        Constraint::Length(1),
                        Constraint::Fill(1),
                        Constraint::Length(1),
                    ])
                    .split(inner_area);

                    let english = Paragraph::new(Line::from(Span::styled(
                        motto.2,
                        Style::default()
                            .fg(accent)
                            .add_modifier(Modifier::BOLD),
                    )))
                    .alignment(Alignment::Center);
                    frame.render_widget(english, layout[1]);
                    frame.render_widget(footer_widget(accent), layout[3]);
                } else {
                    let layout = Layout::vertical([
                        Constraint::Fill(1),
                        Constraint::Length(1),
                        Constraint::Length(1),
                        Constraint::Fill(1),
                        Constraint::Length(1),
                    ])
                    .split(inner_area);

                    let latin = Paragraph::new(Line::from(Span::styled(
                        motto.1,
                        Style::default()
                            .fg(accent)
                            .add_modifier(Modifier::BOLD),
                    )))
                    .alignment(Alignment::Center);
                    frame.render_widget(latin, layout[1]);

                    let english = Paragraph::new(Line::from(Span::styled(
                        format!("— {} —", motto.2),
                        Style::default()
                            .fg(Color::White)
                            .add_modifier(Modifier::ITALIC),
                    )))
                    .alignment(Alignment::Center);
                    frame.render_widget(english, layout[2]);
                    frame.render_widget(footer_widget(accent), layout[4]);
                }
                return;
            }

            // Full layout: art + motto + footer
            let layout = Layout::vertical([
                Constraint::Fill(1),
                Constraint::Length(art_height),
                Constraint::Length(2),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Fill(1),
                Constraint::Length(1),
            ])
            .split(inner_area);

            let art = Paragraph::new(art_text.as_str())
                .alignment(Alignment::Center)
                .style(Style::default().fg(Color::Rgb(140, 140, 140)));
            frame.render_widget(art, layout[1]);

            let is_english_only = motto.1 == motto.2;

            if !is_english_only {
                let latin = Paragraph::new(Line::from(vec![Span::styled(
                    motto.1,
                    Style::default()
                        .fg(accent)
                        .add_modifier(Modifier::BOLD),
                )]))
                .alignment(Alignment::Center);
                frame.render_widget(latin, layout[3]);
            }

            let english = Paragraph::new(Line::from(vec![Span::styled(
                if is_english_only {
                    motto.2.to_string()
                } else {
                    format!("— {} —", motto.2)
                },
                Style::default()
                    .fg(if is_english_only {
                        accent
                    } else {
                        Color::White
                    })
                    .add_modifier(if is_english_only {
                        Modifier::BOLD
                    } else {
                        Modifier::ITALIC
                    }),
            )]))
            .alignment(Alignment::Center);
            frame.render_widget(english, layout[4]);

            frame.render_widget(footer_widget(accent), layout[6]);
        })?;

        let mut advance = false;

        if event::poll(Duration::from_secs(1))? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }
                last_interaction = Instant::now();
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Char('c')
                        if key.modifiers.contains(KeyModifiers::CONTROL) =>
                    {
                        break
                    }
                    KeyCode::Char('n') | KeyCode::Char(' ') | KeyCode::Right => {
                        advance = true;
                    }
                    KeyCode::Char('p') | KeyCode::Backspace | KeyCode::Left => {
                        hist_pos = hist_pos.saturating_sub(1);
                    }
                    KeyCode::Char('d') => {
                        debug_tod = match debug_tod {
                            None => Some(0),
                            Some(i) if i + 1 < TimeOfDay::ALL.len() => Some(i + 1),
                            Some(_) => None,
                        };
                    }
                    _ => {}
                }
            }
        } else if last_interaction.elapsed() >= auto_advance {
            advance = true;
            last_interaction = Instant::now();
        }

        if advance {
            if hist_pos < history.len() - 1 {
                hist_pos += 1;
            } else {
                let last_idx = bag[bag_idx];
                bag_idx += 1;
                if bag_idx >= bag.len() {
                    bag.shuffle(&mut rng);
                    // Avoid back-to-back repeat at the cycle boundary.
                    if bag.len() > 1 && bag[0] == last_idx {
                        bag.swap(0, 1);
                    }
                    bag_idx = 0;
                }
                let idx = bag[bag_idx];
                let prev_art = &history[hist_pos].1;
                let art = MOTTOS[idx].0.art(&mut rng, Some(prev_art));
                history.push((idx, art));
                hist_pos = history.len() - 1;
                seen.insert(idx);
                // Cap history to prevent unbounded growth
                if history.len() > 200 {
                    history.drain(..history.len() - 200);
                    hist_pos = history.len() - 1;
                }
            }
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    // Install panic hook that restores the terminal before printing the panic.
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
        original_hook(info);
    }));

    // Register SIGINT handler so external signals trigger a clean exit.
    let quit_flag = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::SIGINT, Arc::clone(&quit_flag))
        .expect("failed to register SIGINT handler");

    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;

    let result = run(&mut terminal, &quit_flag);

    // Always restore the terminal, even if run() returned an error.
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    result
}
