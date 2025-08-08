//! Basic parsing tests using snapshot testing

use simons_parser::*;

#[test]
fn test_item_parser() {
    let parser = item();
    let result = parser.parse("hello");
    insta::assert_debug_snapshot!(result, @r###"
    Ok(
        (
            'h',
            "ello",
        ),
    )
    "###);
}

#[test]
fn test_item_parser_empty() {
    let parser = item();
    let result = parser.parse("");
    insta::assert_debug_snapshot!(result, @r###"
    Err(
        UnexpectedEof,
    )
    "###);
}

#[test]
fn test_satisfy_success() {
    let parser = satisfy(|c: &char| c.is_alphabetic());
    let result = parser.parse("hello");
    insta::assert_debug_snapshot!(result, @r###"
    Ok(
        (
            'h',
            "ello",
        ),
    )
    "###);
}

#[test]
fn test_satisfy_failure() {
    let parser = satisfy(|c: &char| c.is_numeric());
    let result = parser.parse("hello");
    insta::assert_debug_snapshot!(result, @r#"
    Err(
        Expected {
            expected: "item satisfying predicate",
            found: Some(
                "different item",
            ),
            input: "hello",
        },
    )
    "#);
}

#[test]
fn test_token_success() {
    let parser = token('h');
    let result = parser.parse("hello");
    insta::assert_debug_snapshot!(result, @r###"
    Ok(
        (
            'h',
            "ello",
        ),
    )
    "###);
}

#[test]
fn test_token_failure() {
    let parser = token('x');
    let result = parser.parse("hello");
    insta::assert_debug_snapshot!(result, @r#"
    Err(
        Expected {
            expected: "'x'",
            found: Some(
                "'h'",
            ),
            input: "hello",
        },
    )
    "#);
}

#[test]
fn test_pure_parser() {
    let parser = pure(42);
    let result = parser.parse("hello");
    insta::assert_debug_snapshot!(result, @r###"
    Ok(
        (
            42,
            "hello",
        ),
    )
    "###);
}

#[test]
fn test_fail_parser() {
    let parser: Fail<&str, char> = fail("test error");
    let result = parser.parse("hello");
    insta::assert_debug_snapshot!(result, @r#"
    Err(
        Message {
            message: "test error",
            input: "hello",
        },
    )
    "#);
}
