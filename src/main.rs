mod ast;

fn main() {
    let input = "7 - (3 + 7) * 8 / 2";

    let mut lexer = ast::Lexer::new(input);
    let mut tokens = Vec::new();

    while let Some(token) = lexer.next_token() {
        tokens.push(token);
    }

    let max_length = tokens
        .iter()
        .map(|token| format!("{:?}", token.kind).len())
        .max()
        .unwrap_or(0);

    let max_span_length = tokens
        .iter()
        .map(|token| format!("{}..{} ", token.span.start, token.span.end).len())
        .max()
        .unwrap_or(0);

    for token in tokens {
        println!(
            "{:<span_width$} {:<kind_width$} ({:?})",
            format!("{}..{} ", token.span.start, token.span.end),
            format!("{:?}", token.kind),
            token.span.literal,
            span_width = max_span_length,
            kind_width = max_length
        );
    }
}
