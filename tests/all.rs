use std::fs;

use escoop::{Source, diag, lexer::Lexer};

#[test]
fn non_ascii() {
    let text = fs::read_to_string("tests/non_ascii.txt").unwrap();
    let src = Source::new("tests/non_ascii.txt", text.as_str());
    let lexer = Lexer::new(&src);
    lexer.for_each(|x| {
        x.span().apply();
    }); // Exhaust all tokens, if span gets out of sync, we'll know
    assert!(!diag::error());
}
