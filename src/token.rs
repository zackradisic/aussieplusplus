use std::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    kind: Kind,
    line: usize,
}

impl Token {
    pub fn new(kind: Kind, line: usize) -> Self {
        Self { kind, line }
    }

    pub fn kind(&self) -> Kind {
        self.kind.clone()
    }

    pub fn line(&self) -> usize {
        self.line
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Kind {
    Tilde,          // ~
    QuestionMark,   // ?
    LeftBoomerang,  // <
    RightBoomerang, // >
    LeftParen,      // (
    RightParen,     // )
    Assign,         // =
    Comma,          // ,
    Plus,           // +
    Minus,          // -
    Asterisk,       // *
    Slash,          // /
    Bang,           // !
    Semicolon,      // ;
    LTE,            // <=
    GTE,            // >=
    Equals,         // ==
    BangEqual,      // !=
    And,            // &&
    Or,             // ||
    Isa,            // (is a)
    BuggerAll,      // Bugger all (nil/null)
    ChookBickey,    // Chook bickey (end of program)
    Walkabout,      // Walkabout (for loop)
    BlimeyMate,     // Blimey mate (program start)
    IReckon,        // I reckon (var decl)
    YaReckon,       // Ya reckon (analogous to if)
    HardYakkaFor,   // Hard yakka for (function decl)
    Bail,           // bail (return)
    NahYeah,        // true
    YeahNah,        // false
    Ident(String),  // Identifier
    Number(f64),    // Number literal
    String(String), // String literal
    EOF,
}

impl Kind {
    pub fn literal(&self) -> String {
        match self {
            Kind::Isa => "is a",
            Kind::Tilde => "~",
            Kind::QuestionMark => "?",
            Kind::LeftBoomerang => "<",
            Kind::RightBoomerang => ">",
            Kind::LeftParen => "(",
            Kind::RightParen => ")",
            Kind::Assign => "=",
            Kind::Comma => ",",
            Kind::Plus => "+",
            Kind::Minus => "-",
            Kind::Asterisk => "*",
            Kind::Slash => "/",
            Kind::Bang => "!",
            Kind::Semicolon => ";",
            Kind::LTE => "<=",
            Kind::GTE => ">=",
            Kind::Equals => "==",
            Kind::BangEqual => "!=",
            Kind::And => "&&",
            Kind::Or => "||",
            Kind::BuggerAll => "bugger all",
            Kind::ChookBickey => "chook bickey", // Chook bickey (end of program)
            Kind::Walkabout => "walkabout",      // Walkabout (for loop)
            Kind::BlimeyMate => "blimey mate",   // Blimey mate (program start)
            Kind::IReckon => "i reckon",         // I reckon (var decl)
            Kind::YaReckon => "ya reckon",       // Ya reckon (analogous to if)
            Kind::HardYakkaFor => "hard yakka for", // Hard yakka for (function decl)
            Kind::Bail => "bail",                // bail (return)
            Kind::NahYeah => "nah, yeah",        // true
            Kind::YeahNah => "yeah, nah",        // false
            Kind::Ident(ref s) => s.as_str(),    // Identifier
            Kind::Number(n) => return format!("{}", n), // Number literal
            Kind::String(ref s) => s.as_str(),   // String literal
            Kind::EOF => "EOF",
        }
        .into()
    }
}

impl Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.literal())
    }
}
