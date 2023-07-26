#![allow(dead_code, unused)]
use std::collections::HashMap;

use once_cell::sync::Lazy;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::Parser;
use regex::Regex;

fn main() {
    println!("Implement me!");
}

fn parse_regex(input: &str) -> Output {
    const REGEX_STR: &str = r".*(\+|\-).*(\d+)\.(\d+|\*).*";
    static REGEX: Lazy<Regex> =
        Lazy::new(|| Regex::new(REGEX_STR).expect("Couldn't parse regex string"));

    todo!();
}

#[derive(Parser)]
#[grammar = "simplified.pest"]
struct TokenParser;
fn parse_token(input: &str) -> Option<Output> {
    let mut pairs = TokenParser::parse(Rule::value, input).ok()?;
    let input_value = pairs.next().unwrap().into_inner();
    let mut output = Output::default();

    for x in input_value {
        match x.as_rule() {
            Rule::sign => {
                let sign_str = x.as_str();
                let sign = match sign_str {
                    "+" => Sign::Plus,
                    "-" => Sign::Minus,
                    _ => panic!("Impossible sign"),
                };

                output.sign = Some(sign);
            }
            Rule::width => {
                let width: Option<usize> = x
                    .into_inner()
                    .next()
                    .map(|v| v.as_str())
                    .and_then(|v| v.parse().ok());

                if let Some(width) = width {
                    output.width = Some(width);
                }
            }
            Rule::precision => {
                let str_opt = x.as_str();
                let precision = match x.as_str() {
                    "*" => Some(Precision::Asterisk),
                    maybe_int => maybe_int.parse().ok().map(Precision::Integer),
                };

                if let Some(precision) = precision {
                    output.precision = Some(precision);
                }
            }

            Rule::EOI => return Some(output),

            Rule::WHITESPACE
            | Rule::COMMENT
            | Rule::not_valid
            | Rule::integer
            | Rule::separator
            | Rule::value => unreachable!("Reached with {}", x),
        }
    }

    // No loops = no output
    None
}

fn parse(input: &str) -> Output {
    parse_token(input).unwrap_or_default()
}

#[derive(Default)]
struct Output {
    sign: Option<Sign>,
    width: Option<usize>,
    precision: Option<Precision>,
}

#[derive(Debug, PartialEq)]
enum Sign {
    Plus,
    Minus,
}

#[derive(Debug, PartialEq)]
enum Precision {
    Integer(usize),
    Argument(usize),
    Asterisk,
}

#[cfg(test)]
mod spec {
    use super::*;

    #[test]
    fn parses_sign() {
        for (input, expected) in vec![
            ("", None),
            (">8.*", None),
            (">+8.*", Some(Sign::Plus)),
            ("-.1$x", Some(Sign::Minus)),
            ("a^#043.8?", None),
        ] {
            let Output { sign, .. } = parse(input);
            assert_eq!(sign, expected);
        }
    }

    #[test]
    fn parses_width() {
        for (input, expected) in vec![
            ("", None),
            (">8.*", Some(8)),
            (">+8.*", Some(8)),
            ("-.1$x", None),
            ("a^#043.8?", Some(43)),
        ] {
            let Output { width, .. } = parse(input);
            assert_eq!(width, expected);
        }
    }

    #[test]
    fn parses_precision() {
        for (input, expected) in vec![
            ("", None),
            (">8.*", Some(Precision::Asterisk)),
            (">+8.*", Some(Precision::Asterisk)),
            // Is this supposed to be length of the argument or precision before
            // the argument? I don't even know anymore
            // ("-.1$x", Some(Precision::Argument(1))),
            ("a^#043.8?", Some(Precision::Integer(8))),
        ] {
            let Output { precision, .. } = parse(input);
            assert_eq!(precision, expected);
        }
    }
}
