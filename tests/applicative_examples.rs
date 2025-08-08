//! Applicative style parsing examples that demonstrate ApplicativeDo-like syntax

use simons_parser::*;

#[derive(Debug, PartialEq, Clone)]
struct Person {
    name: String,
    age: u32,
}

// Simple applicative example - parsing two numbers with addition
#[test]
fn test_add_parser() {
    let parser = unsigned()
        .skip(spaces())
        .skip(char('+'))
        .skip(spaces())
        .and(unsigned())
        .map(|(a, b)| a + b);
    
    let result = parser.parse("42 + 13");
    
    insta::assert_debug_snapshot!(result, @r###"
    Ok(
        (
            55,
            "",
        ),
    )
    "###);
}

// Parse coordinate pair
#[test]  
fn test_coordinate_parsing() {
    let parser = integer()
        .preceded_by(char('('))
        .skip(spaces())
        .skip(char(','))
        .skip(spaces())
        .and(integer())
        .skip(char(')'))
        .map(|(x, y)| (x, y));
    
    let result = parser.parse("(-42, 13)");
    
    insta::assert_debug_snapshot!(result, @r###"
    Ok(
        (
            (
                -42,
                13,
            ),
            "",
        ),
    )
    "###);
}

// Parse RGB color values
#[test]
fn test_rgb_parsing() {
    let parser = unsigned() // Red
        .preceded_by(string("rgb("))
        .skip(char(','))
        .skip(spaces())
        .and(unsigned()) // Green  
        .skip(char(','))
        .skip(spaces())
        .and(unsigned()) // Blue
        .skip(char(')'))
        .map(|((r, g), b)| (r, g, b));
    
    let result = parser.parse("rgb(255, 128, 64)");
    
    insta::assert_debug_snapshot!(result, @r###"
    Ok(
        (
            (
                255,
                128,
                64,
            ),
            "",
        ),
    )
    "###);
}

// Parse list of numbers
#[test]
fn test_list_parsing() {
    let parser = sep_by(unsigned(), char(',').skip(spaces()))
        .preceded_by(char('['))
        .skip(char(']'));
    
    let result = parser.parse("[1, 2, 3, 4, 5]");
    
    insta::assert_debug_snapshot!(result, @r###"
    Ok(
        (
            [
                1,
                2,
                3,
                4,
                5,
            ],
            "",
        ),
    )
    "###);
}

// Simple person parsing with name and age
#[test]
fn test_simple_person_parsing() {
    let parser = alpha().many1()
        .map(|chars| chars.into_iter().collect::<String>())
        .skip(spaces())
        .and(unsigned())
        .map(|(name, age)| Person { name, age });
    
    let result = parser.parse("John 30");
    
    insta::assert_debug_snapshot!(result, @r###"
    Ok(
        (
            Person {
                name: "John",
                age: 30,
            },
            "",
        ),
    )
    "###);
}