//! Combinator tests using snapshot testing

use simons_parser::*;

#[test]
fn test_map_combinator() {
    let parser = item().map(|c: char| c.to_ascii_uppercase());
    let result = parser.parse("hello");
    insta::assert_debug_snapshot!(result, @r###"
    Ok(
        (
            'H',
            "ello",
        ),
    )
    "###);
}

#[test]
fn test_and_combinator() {
    let parser = item().and(item());
    let result = parser.parse("hello");
    insta::assert_debug_snapshot!(result, @r###"
    Ok(
        (
            (
                'h',
                'e',
            ),
            "llo",
        ),
    )
    "###);
}

#[test]
fn test_skip_combinator() {
    let parser = item().skip(item());
    let result = parser.parse("hello");
    insta::assert_debug_snapshot!(result, @r###"
    Ok(
        (
            'h',
            "llo",
        ),
    )
    "###);
}

#[test]
fn test_preceded_by_combinator() {
    let parser = item().preceded_by(item());
    let result = parser.parse("hello");
    insta::assert_debug_snapshot!(result, @r###"
    Ok(
        (
            'e',
            "llo",
        ),
    )
    "###);
}

#[test]
fn test_or_combinator_first_succeeds() {
    let parser = token('h').or(token('x'));
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
fn test_or_combinator_second_succeeds() {
    let parser = token('x').or(token('h'));
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
fn test_or_combinator_both_fail() {
    let parser = token('x').or(token('y'));
    let result = parser.parse("hello");
    insta::assert_debug_snapshot!(result, @r#"
    Err(
        Many(
            [
                Expected {
                    expected: "'x'",
                    found: Some(
                        "'h'",
                    ),
                    input: "hello",
                },
                Expected {
                    expected: "'y'",
                    found: Some(
                        "'h'",
                    ),
                    input: "hello",
                },
            ],
        ),
    )
    "#);
}

#[test]
fn test_optional_success() {
    let parser = token('h').optional();
    let result = parser.parse("hello");
    insta::assert_debug_snapshot!(result, @r###"
    Ok(
        (
            Some(
                'h',
            ),
            "ello",
        ),
    )
    "###);
}

#[test]
fn test_optional_failure() {
    let parser = token('x').optional();
    let result = parser.parse("hello");
    insta::assert_debug_snapshot!(result, @r###"
    Ok(
        (
            None,
            "hello",
        ),
    )
    "###);
}

#[test]
fn test_many_success() {
    let parser = token('l').many();
    let result = parser.parse("lllhello");
    insta::assert_debug_snapshot!(result, @r###"
    Ok(
        (
            [
                'l',
                'l',
                'l',
            ],
            "hello",
        ),
    )
    "###);
}

#[test]
fn test_many_zero_matches() {
    let parser = token('x').many();
    let result = parser.parse("hello");
    insta::assert_debug_snapshot!(result, @r###"
    Ok(
        (
            [],
            "hello",
        ),
    )
    "###);
}

#[test]
fn test_many1_success() {
    let parser = token('l').many1();
    let result = parser.parse("lllhello");
    insta::assert_debug_snapshot!(result, @r###"
    Ok(
        (
            [
                'l',
                'l',
                'l',
            ],
            "hello",
        ),
    )
    "###);
}

#[test]
fn test_many1_failure() {
    let parser = token('x').many1();
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
fn test_between() {
    let parser = between(token('('), item(), token(')'));
    let result = parser.parse("(x)");
    insta::assert_debug_snapshot!(result, @r###"
    Ok(
        (
            'x',
            "",
        ),
    )
    "###);
}

#[test]
fn test_choice() {
    let parsers = vec![token('a'), token('b'), token('c')];
    let parser = choice(parsers);
    let result = parser.parse("b");
    insta::assert_debug_snapshot!(result, @r###"
    Ok(
        (
            'b',
            "",
        ),
    )
    "###);
}

#[test]
fn test_sep_by_empty() {
    let parser = sep_by(item(), token(','));
    let result = parser.parse("");
    insta::assert_debug_snapshot!(result, @r###"
    Ok(
        (
            [],
            "",
        ),
    )
    "###);
}

#[test]
fn test_sep_by_single() {
    let parser = sep_by(item(), token(','));
    let result = parser.parse("a");
    insta::assert_debug_snapshot!(result, @r###"
    Ok(
        (
            [
                'a',
            ],
            "",
        ),
    )
    "###);
}

#[test]
fn test_sep_by_multiple() {
    let parser = sep_by(item(), token(','));
    let result = parser.parse("a,b,c");
    insta::assert_debug_snapshot!(result, @r###"
    Ok(
        (
            [
                'a',
                'b',
                'c',
            ],
            "",
        ),
    )
    "###);
}

#[test]
fn test_sep_by1_success() {
    let parser = sep_by1(item(), token(','));
    let result = parser.parse("a,b,c");
    insta::assert_debug_snapshot!(result, @r###"
    Ok(
        (
            [
                'a',
                'b',
                'c',
            ],
            "",
        ),
    )
    "###);
}

#[test]
fn test_sep_by1_failure() {
    let parser = sep_by1(item(), token(','));
    let result = parser.parse("");
    insta::assert_debug_snapshot!(result, @r"
    Err(
        UnexpectedEof,
    )
    ");
}
