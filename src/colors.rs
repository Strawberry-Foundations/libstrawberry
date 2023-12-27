pub const BOLD: &str = "\x1b[1m";
pub const UNDERLINE: &str = "\x1b[4m";
pub const C_RESET: &str = "\x1b[0m";

// Basic Colors
pub const BLACK: &str = "\x1b[30m";
pub const RED: &str = "\x1b[31m";
pub const GREEN: &str = "\x1b[32m";
pub const YELLOW: &str = "\x1b[33m";
pub const BLUE: &str = "\x1b[34m";
pub const MAGENTA: &str = "\x1b[35m";
pub const CYAN: &str = "\x1b[36m";
pub const WHITE: &str = "\x1b[37m";
pub const RESET: &str = "\x1b[39m";
pub const GRAY: &str =  "\x1b[90m";

// Light Colors
pub const LIGHT_RED: &str = "\x1b[91m";
pub const LIGHT_GREEN: &str = "\x1b[92m";
pub const LIGHT_YELLOW: &str = "\x1b[93m";
pub const LIGHT_BLUE: &str = "\x1b[94m";
pub const LIGHT_MAGENTA: &str = "\x1b[95m";
pub const LIGHT_CYAN: &str = "\x1b[96m";
pub const LIGHT_WHITE: &str = "\x1b[97m";

// Background Colors
pub const BACK_BLACK: &str = "\x1b[40m";
pub const BACK_RED: &str = "\x1b[41m";
pub const BACK_GREEN: &str = "\x1b[42m";
pub const BACK_YELLOW: &str = "\x1b[43m";
pub const BACK_BLUE: &str = "\x1b[44m";
pub const BACK_MAGENTA: &str = "\x1b[45m";
pub const BACK_CYAN: &str = "\x1b[46m";
pub const BACK_WHITE: &str = "\x1b[47m";
pub const BACK_RESET: &str = "\x1b[49m";

// Light Background Colors
pub const BACK_LIGHT_RED: &str = "\x1b[101m";
pub const BACK_LIGHT_GREEN: &str = "\x1b[102m";
pub const BACK_LIGHT_YELLOW: &str = "\x1b[103m";
pub const BACK_LIGHT_BLUE: &str = "\x1b[104m";
pub const BACK_LIGHT_MAGENTA: &str = "\x1b[105m";
pub const BACK_LIGHT_CYAN: &str = "\x1b[106m";
pub const BACK_LIGHT_WHITE: &str = "\x1b[107m";

// Styles
pub const STYLE_BRIGHT: &str = "\x1b[1m";
pub const STYLE_DIM: &str = "\x1b[2m";
pub const STYLE_NORMAL: &str = "\x1b[22m";

// Aliases
pub const LIGHT_BLACK: &str = "\x1b[90m"; // Alias of GRAY
pub const PURPLE: &str = "\x1b[35m"; // Alias of MAGENTA
pub const STYLE_RESET: &str = "\x1b[0m"; // Alias of C_RESET