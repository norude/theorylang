use lexgen::lexer;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token<'input> {
    // An identifier
    Ident(&'input str),
    Number(&'input str),
    Plus,
    Star,
    Equals,
    OpenParen,
    CloseParen,
}

macro_rules! wrapped {
    ($t:path) => {
        |lexer| lexer.return_($t(lexer.match_()))
    };
}

lexer! {
    pub Lexer -> Token<'input>;

    let identInitial = ['a'-'z' 'A'-'Z' '_'];
    let numbers = ['0'-'9'];
    let identSubsequent = $identInitial | $numbers;

    rule Init {
        $$whitespace+,
        "//" (_ # '\n')* > ('\n' | $),

        '+' = Token::Plus,
        '*' = Token::Star,
        '=' = Token::Equals,
        '(' = Token::OpenParen,
        ')' = Token::CloseParen,
        $identInitial $identSubsequent* => wrapped!(Token::Ident),
        $numbers* => wrapped!(Token::Number),
    }
}
