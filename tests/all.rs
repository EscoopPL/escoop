use core::panic;
use std::fs;

use escoop::{diag::{self, Diag}, lexer::Lexer};

#[test]
fn non_ascii() {
    let text = fs::read_to_string("tests/non_ascii.txt").unwrap();
    let lexer = Lexer::new_with_path(
        text.as_str(),
        "tests/non_ascii.txt"
    );
    lexer.for_each(|x| {
        //dbg!(x);
        drop(x);
    }); // Exhaust all tokens, we've reached the semicolon, which will cause a diagnostic to be outputted
    Diag::flush(); // Flush in order to trigger the bug
    assert!(!diag::bug());
    panic!();
}