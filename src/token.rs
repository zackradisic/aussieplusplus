use std::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub kind: Kind,
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
    // Operators
    Modulo,         // %
    Tilde,          // ~
    QuestionMark,   // ?
    LeftBoomerang,  // <
    RightBoomerang, // >
    LeftBracket,    // [
    RightBracket,   // ]
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
    GoodOnYa,       // GOOD ON YA
    PullYaHeadIn,   // PullYaHeadIn

    // Keywords
    Import,       // IMPOHT ME FUNC
    FuckinPiker,  // FUCKINPIKER (early exit)
    MateFuckThis, // mate fuck this (break)
    Until,        // until
    From,         // from
    To,           // to
    Gimme,        // gimme
    Is,           // (is)
    Isa,          // (is a)
    BuggerAll,    // Bugger all (nil/null)
    Cheers,       // Cheers C***! (end of program)
    Whatabout,    // Whatabout (else)
    IllHaveA,     //  I'll Have a
    Walkabout,    // Walkabout (for loop)
    GdayMate,     // G'DAY MATE! (program start)
    IReckon,      // I reckon (var decl)
    IFullyReckon, // I fully reckon (constant var decl)
    YaReckon,     // Ya reckon (analogous to if)
    HardYakkaFor, // Hard yakka for (function decl)
    Bail,         // bail (return)
    True,         // true
    False,        // false

    // A sequence of Yeah/Nahs followed by a ! will be transformed
    // into one NahYeah or YeahNah. The parser will never see these tokens.
    Yeah,
    Nah,

    Ident(String),  // Identifier
    Number(f64),    // Number literal
    String(String), // String literal
    EOF,
}

impl Kind {
    pub fn literal(&self) -> String {
        match self {
            Kind::GoodOnYa => "good on ya",
            Kind::PullYaHeadIn => "pull ya head in",
            Kind::Import => "impoht me func",
            Kind::FuckinPiker => "fuckinpiker",
            Kind::Modulo => "%",
            Kind::MateFuckThis => "mate fuck this",
            Kind::LeftBracket => "[",
            Kind::RightBracket => "]",
            Kind::Until => "until",
            Kind::From => "from",
            Kind::To => "to",
            Kind::Is => "is",
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
            Kind::Gimme => "gimme",
            Kind::IllHaveA => "i'll have a",
            Kind::BuggerAll => "bugger all",
            Kind::Cheers => "cheers c***!", // Chook bickey (end of program)
            Kind::Whatabout => "whatabout", // Whatabout (else)
            Kind::Walkabout => "walkabout", // Walkabout (for loop)
            Kind::GdayMate => "g'day mate!", // G'DAY MATE! (program start)
            Kind::IReckon => "i reckon",    // I reckon (var decl)
            Kind::IFullyReckon => "i fully reckon", // I fully reckon (constant var decl)
            Kind::YaReckon => "ya reckon",  // Ya reckon (analogous to if)
            Kind::HardYakkaFor => "the hard yakka for", // Hard yakka for (function decl)
            Kind::Bail => "bail",           // bail (return)
            Kind::True => "nah, yeah!",     // true
            Kind::False => "yeah, nah!",    // false
            Kind::Nah => "nah",             // true
            Kind::Yeah => "yeah",           // false
            Kind::Ident(ref s) => s.as_str(), // Identifier
            Kind::Number(n) => return format!("{}", n), // Number literal
            Kind::String(ref s) => s.as_str(), // String literal
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
