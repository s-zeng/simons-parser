//! Text parsing tests using snapshot testing

use simons_parser::*;

#[test]
fn test_char_parser() {
    let parser = char('h');
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
fn test_string_parser() {
    let parser = string("hello");
    let result = parser.parse("hello world");
    insta::assert_debug_snapshot!(result, @r###"
    Ok(
        (
            "hello",
            " world",
        ),
    )
    "###);
}

#[test]
fn test_string_parser_failure() {
    let parser = string("goodbye");
    let result = parser.parse("hello world");
    insta::assert_debug_snapshot!(result, @r#"
    Err(
        Expected {
            expected: "string 'goodbye'",
            found: Some(
                "character 'h'",
            ),
            input: "hello world",
        },
    )
    "#);
}

#[test]
fn test_alpha_parser() {
    let parser = alpha();
    let result = parser.parse("hello123");
    insta::assert_debug_snapshot!(result, @r###"
    Ok(
        (
            'h',
            "ello123",
        ),
    )
    "###);
}

#[test]
fn test_digit_parser() {
    let parser = digit();
    let result = parser.parse("123abc");
    insta::assert_debug_snapshot!(result, @r###"
    Ok(
        (
            '1',
            "23abc",
        ),
    )
    "###);
}

#[test]
fn test_alphanumeric_parser() {
    let parser = alphanumeric();
    let result = parser.parse("a1b2c3");
    insta::assert_debug_snapshot!(result, @r###"
    Ok(
        (
            'a',
            "1b2c3",
        ),
    )
    "###);
}

#[test]
fn test_space_parser() {
    let parser = space();
    let result = parser.parse(" hello");
    insta::assert_debug_snapshot!(result, @r###"
    Ok(
        (
            ' ',
            "hello",
        ),
    )
    "###);
}

#[test]
fn test_spaces_parser() {
    let parser = spaces();
    let result = parser.parse("   hello");
    insta::assert_debug_snapshot!(result, @r###"
    Ok(
        (
            "   ",
            "hello",
        ),
    )
    "###);
}

#[test]
fn test_spaces1_parser() {
    let parser = spaces1();
    let result = parser.parse("   hello");
    insta::assert_debug_snapshot!(result, @r###"
    Ok(
        (
            "   ",
            "hello",
        ),
    )
    "###);
}

#[test]
fn test_spaces1_failure() {
    let parser = spaces1();
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
fn test_newline_parser() {
    let parser = newline();
    let result = parser.parse("\nhello");
    insta::assert_debug_snapshot!(result, @r###"
    Ok(
        (
            '\n',
            "hello",
        ),
    )
    "###);
}

#[test]
fn test_tab_parser() {
    let parser = tab();
    let result = parser.parse("\thello");
    insta::assert_debug_snapshot!(result, @r###"
    Ok(
        (
            '\t',
            "hello",
        ),
    )
    "###);
}

#[test]
fn test_unsigned_parser() {
    let parser = unsigned();
    let result = parser.parse("12345abc");
    insta::assert_debug_snapshot!(result, @r###"
    Ok(
        (
            12345,
            "abc",
        ),
    )
    "###);
}

#[test]
fn test_integer_positive() {
    let parser = integer();
    let result = parser.parse("12345abc");
    insta::assert_debug_snapshot!(result, @r###"
    Ok(
        (
            12345,
            "abc",
        ),
    )
    "###);
}

#[test]
fn test_integer_negative() {
    let parser = integer();
    let result = parser.parse("-12345abc");
    insta::assert_debug_snapshot!(result, @r###"
    Ok(
        (
            -12345,
            "abc",
        ),
    )
    "###);
}

#[test]
fn test_one_of_parser() {
    let parser = one_of("aeiou");
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

    let result2 = parser.parse("apple");
    insta::assert_debug_snapshot!(result2, @r###"
    Ok(
        (
            'a',
            "pple",
        ),
    )
    "###);
}

#[test]
fn test_none_of_parser() {
    let parser = none_of("aeiou");
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
fn test_not_char_parser() {
    let parser = not_char('x');
    let result = parser.parse("hello");
    insta::assert_debug_snapshot!(result, @r###"
    Ok(
        (
            'h',
            "ello",
        ),
    )
    "###);

    let result2 = parser.parse("xhello");
    insta::assert_debug_snapshot!(result2, @r#"
    Err(
        Expected {
            expected: "item satisfying predicate",
            found: Some(
                "different item",
            ),
            input: "xhello",
        },
    )
    "#);
}
