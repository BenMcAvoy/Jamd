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

