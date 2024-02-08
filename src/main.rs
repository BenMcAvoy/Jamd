mod ast;

fn main() {
    let input = "721 - (332 + 753) * 82 / 21";

    // Get tokens from the lexer iteratively and collect them into a vector.
    let tokens: Vec<_> = ast::Lexer::new(input)
        .filter(|t| t.kind != ast::TokenKind::Whitespace)
        .collect();

    print_tokens(tokens);
}

/// A function to print tokens with their spans and kinds. Also has padding so
/// that the output is aligned.
fn print_tokens(tokens: Vec<ast::Token>) {
    let kind_width = tokens
        .iter()
        .map(|token| format!("{:?}", token.kind).len())
        .max()
        .unwrap_or(0);

    let span_width = tokens
        .iter()
        .map(|token| format!("{}..{} ", token.span.start, token.span.end).len())
        .max()
        .unwrap_or(0);

    // Do this in less lines.
    for token in tokens {
        let span = format!("{}..{} ", token.span.start, token.span.end);
        let kind = format!("{:?}", token.kind);
        let literal = token.span.literal;

        println!("{span:<span_width$}│ {kind:<kind_width$} ({literal:?})");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_lexing() {
        //           1   2 34   5 6  7 8 9 10 11 ... 12 (EOF)
        let input = "721 - (332 + 753) * 82 / 21";
        let tokens: Vec<_> = ast::Lexer::new(input)
            .filter(|t| t.kind != ast::TokenKind::Whitespace)
            .collect();

        assert_eq!(tokens.len(), 12);
        assert_eq!(tokens[0].kind, ast::TokenKind::Number(721));
        assert_eq!(tokens[1].kind, ast::TokenKind::Minus);
        assert_eq!(tokens[2].kind, ast::TokenKind::LeftParen);
        assert_eq!(tokens[3].kind, ast::TokenKind::Number(332));
        assert_eq!(tokens[4].kind, ast::TokenKind::Plus);
        assert_eq!(tokens[5].kind, ast::TokenKind::Number(753));
        assert_eq!(tokens[6].kind, ast::TokenKind::RightParen);
        assert_eq!(tokens[7].kind, ast::TokenKind::Asterisk);
        assert_eq!(tokens[8].kind, ast::TokenKind::Number(82));
        assert_eq!(tokens[9].kind, ast::TokenKind::Slash);
        assert_eq!(tokens[10].kind, ast::TokenKind::Number(21));
        assert_eq!(tokens[11].kind, ast::TokenKind::Eof);
    }
}
