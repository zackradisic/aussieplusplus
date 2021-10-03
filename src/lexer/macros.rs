/// Advance if char matches input
#[macro_export]
macro_rules! peek_adv {
    ($self:ident, $($chars:expr),+) => {
        match $self.peek() {
            Some(ch) => {
                match ch {
                    $(
                        $chars => {
                            let _ = $self.next();
                            true
                        }
                    )*,
                    _ => false,
                }
            },
            None => false
        }
    };
    ($self:ident, $char:expr) => {
        match $self.peek() {
            Some(ch) => {
                if ch == $char {
                    let _ = $self.next();
                    true
                } else {
                    false
                }
            }
            None => false,
        }
    };
}

#[macro_export]
macro_rules! eat_keyword_or_ident {
    ($self:ident, $first_char:expr, $kind:path) => {{
        let res: Result<Kind> = match eat_keyword!($self, $kind) {
            Err(_) => $self.eat_identifier($first_char),
            Ok(kind) => Ok(kind),
        };
        res
    }};
}

#[macro_export]
macro_rules! eat_keyword {
    ($self:ident, $kind:path) => {{
        let s: String = $kind.literal().chars().skip(1).collect();
        let len = s.len();

        let mut ret: Option<Result<Kind>> = None;
        let mut expected: char;

        for i in 0..len {
            expected = s.chars().nth(i).unwrap();
            match $self.peek_multi() {
                None => {
                    $self.src.reset_peek();
                    ret = Some(Err(
                        LexError::ExpectedCharacter(expected, '\0', $self.line).new()
                    ));
                    break;
                }
                Some(c) => {
                    if c.to_ascii_lowercase().ne(&expected) {
                        $self.src.reset_peek();
                        ret = Some(Err(
                            LexError::ExpectedCharacter(expected, c, $self.line).new()
                        ));
                        break;
                    }
                }
            };
        }

        if let Some(e) = ret {
            e
        } else {
            // Space, new-line, or semi-colon must separate token
            $self.expect_separator()?;
            for _ in 0..len {
                let _ = $self.src.next();
            }
            Ok($kind)
        }
    }};
}
