use core::panic;
use std::fs;

use escoop::{diag, lexer::Lexer, Source};

#[test]
fn non_ascii() {
    let text = fs::read_to_string("tests/non_ascii.txt").unwrap();
    let src = Source::new("tests/non_ascii.txt", text.as_str());
    let lexer = Lexer::new(&src);
    lexer.for_each(|x| {
        //dbg!(x);
        drop(x);
    }); // Exhaust all tokens, we've reached the semicolon, which will cause a diagnostic to be outputted
    assert!(!diag::bug());
    panic!();
}