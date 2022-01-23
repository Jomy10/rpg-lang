use std::fmt::{Debug, Formatter};
use lazy_static::lazy_static;
use regex::Regex;
use TokenType::*;
use crate::compile_error;

lazy_static! {
    /// All token types and their regexes
    static ref TOKEN_TYPES: [TokenRegex; 26] = [
        TokenRegex { ttype: Char, regex: Regex::new(r"\A\bchar\b").unwrap() },
        TokenRegex { ttype: Zombie, regex: Regex::new(r"\A\bzombie\b").unwrap() },
        TokenRegex { ttype: Merchant, regex: Regex::new(r"\A\bmerchant\b").unwrap() },
        TokenRegex { ttype: Potion, regex: Regex::new(r"\A\bpotion\b").unwrap() },
        TokenRegex { ttype: SpellBook, regex: Regex::new(r"\A\bspellbook\b").unwrap() },
        TokenRegex { ttype: End, regex: Regex::new(r"\A\bend\b").unwrap() },
        TokenRegex { ttype: FnAttacks, regex: Regex::new(r"\A\battacks\b").unwrap() },
        TokenRegex { ttype: FnShouts, regex: Regex::new(r"\A\bshouts\b").unwrap() },
        TokenRegex { ttype: FnWhispers, regex: Regex::new(r"\A\bwhispers\b").unwrap() },
        TokenRegex { ttype: FnBuys, regex: Regex::new(r"\A\bbuys\b").unwrap() },
        TokenRegex { ttype: FnUses, regex: Regex::new(r"\A\buses\b").unwrap() },
        TokenRegex { ttype: FnCasting, regex: Regex::new(r"\A\bcasting\b").unwrap() },
        TokenRegex { ttype: SbFnSpeak, regex: Regex::new(r"\A\bspeak\b").unwrap() },
        TokenRegex { ttype: SbFnUnZombify, regex: Regex::new(r"\A\bun_zombify\b").unwrap() },
        TokenRegex { ttype: SbFnConfuse, regex: Regex::new(r"\A\bconfuse\b").unwrap() },
        TokenRegex { ttype: SbFnGodSpeech, regex: Regex::new(r"\A\bgod_speech\b").unwrap() },
        TokenRegex { ttype: SbFnTimeWarp, regex: Regex::new(r"\A\btime_warp\b").unwrap() },
        TokenRegex { ttype: SbFnShift, regex: Regex::new(r"\A\bshift\b").unwrap() },
        TokenRegex { ttype: SbFnCreatePotion, regex: Regex::new(r"\A\bcreate_potion\b").unwrap() },
        TokenRegex { ttype: From, regex: Regex::new(r"\A\bfrom\b").unwrap() },
        // Identifier also matches all of the above, which is why it should be below all of them
        // This means that all of the above are reserved words
        TokenRegex { ttype: Identifier, regex: Regex::new(r"\A\b[a-zA-Z_]\w*\b").unwrap() },
        TokenRegex { ttype: Integer, regex: Regex::new(r"\A-?[0-9]+").unwrap() },
        TokenRegex { ttype: Equals, regex: Regex::new(r"\A=").unwrap() },
        TokenRegex { ttype: OParen, regex: Regex::new(r"\A\(").unwrap() },
        TokenRegex { ttype: CParen, regex: Regex::new(r"\A\)").unwrap() },
        TokenRegex { ttype: Comma, regex: Regex::new(r"\A,").unwrap() },
    ];
}

/// Tokenizes an input string
pub struct Tokenizer<'a> {
    code: &'a str
}

impl<'a> Tokenizer<'a> {
    pub fn new(code: &'a str) -> Tokenizer<'a> {
        Self { code }
    }
    
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        while !self.code.is_empty() {
            tokens.push(self.tokenize_next());
            self.code = self.code.trim();
        }
        tokens
    }
    
    fn tokenize_next(&mut self) -> Token {
        for token_type in TOKEN_TYPES.iter() {
            // m = match
            for m in token_type.regex.captures_iter(&self.code) {
                // We will only have 1 match because of \A
                if let Some(_match) = m.get(0) {
                    let m = _match.as_str().to_string();
                    self.code = self.code.strip_prefix(m.as_str()).expect("Unexpected error: could not strip match from code");
                    return Token::new(token_type.ttype, m);
                }
            }
        }
        
        // Have no matches
        let first_token = self.code.split(|c| c == ' ').collect::<Vec<&str>>();
        if let Some(first_token) = first_token.get(0) {
            compile_error!("Unexpected token: found {}", first_token)
        } else {
            compile_error!("Expected token but got None")
        }
    }
}

/// Contains the regex for a token and its type
#[derive(Debug)]
pub struct TokenRegex {
    /// Token type
    pub ttype: TokenType,
    /// A regex in perl-compatible syntax
    pub regex: Regex
}

/// Represents a single token
#[derive(Debug, Clone)]
pub struct Token {
    /// Token type
    pub ttype: TokenType,
    /// token value
    pub value: String
}

impl Token {
    pub fn new(t: TokenType, v: String) -> Self {
        Self { ttype: t, value: v }
    }
}

#[derive(Copy, Clone, PartialEq)]
/// The token types available in the RPG language
pub enum TokenType {
    // types
    Char,
    Zombie,
    Merchant,
    Potion,
    SpellBook,
    End,
    // functions
    FnBuys,
    FnAttacks,
    FnShouts,
    FnWhispers,
    FnUses,
    FnCasting,
    // SpellBookFunctions
    SbFnSpeak,
    SbFnUnZombify,
    SbFnConfuse,
    SbFnGodSpeech,
    SbFnTimeWarp,
    SbFnShift,
    SbFnCreatePotion,
    // Other
    From,
    // Names
    Identifier,
    // Implicit types
    /// A signed integer
    Integer,
    // Punctuation
    /// =
    Equals,
    /// )
    OParen,
    /// (
    CParen,
    Comma
}

impl Debug for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl ToString for TokenType {
    fn to_string(&self) -> String {
        match self {
            Self::Char => "char".to_string(),
            Self::Zombie => "zombie".to_string(),
            Self::Merchant => "merchant".to_string(),
            Self::Potion => "potion".to_string(),
            Self::SpellBook => "spellbook".to_string(),
            Self::End => "end".to_string(),
            Self::FnAttacks => "attacks".to_string(),
            Self::FnShouts => "shouts".to_string(),
            Self::FnWhispers => "whispers".to_string(),
            Self::FnBuys => "buys".to_string(),
            Self::FnUses => "uses".to_string(),
            Self::FnCasting => "casting".to_string(),
            Self::SbFnSpeak => "speak()".to_string(),
            Self::SbFnUnZombify => "un_zombify()".to_string(),
            Self::SbFnConfuse => "confuse()".to_string(),
            Self::SbFnGodSpeech => "god_speech()".to_string(),
            Self::SbFnTimeWarp => "time_warp()".to_string(),
            Self::SbFnShift => "shift()".to_string(),
            Self::SbFnCreatePotion => "create_potion()".to_string(),
            Self::From => "from".to_string(),
            Self::Identifier => "identifier".to_string(),
            Self::Integer => "integer".to_string(),
            Self::Equals => "'='".to_string(),
            Self::OParen => "'('".to_string(),
            Self::CParen => "')'".to_string(),
            Self::Comma => "','".to_string(),
        }
    }
}

#[allow(unused)]
impl TokenType {
    fn formatted(&self) -> String {
        match self {
            Self::Identifier => { format!("an {}", self.to_string()) }
            Self::Integer => { format!("an {}", self.to_string()) }
            _ => self.to_string()
        }
    }
}