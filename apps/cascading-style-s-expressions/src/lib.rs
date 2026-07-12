use std::{iter::Peekable, slice::Iter, str::Chars};
use wasm_bindgen::prelude::*;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    String(String),
    LParen,
    RParen,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Selector(String);

#[derive(Debug, PartialEq, Eq)]
pub struct Rule {
    property: String,
    value: Vec<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct SExpr {
    selector: Selector,
    rules: Vec<Rule>,
    children: Vec<SExpr>,
}

impl SExpr {
    pub fn to_stylesheet(&self, parent: &str) -> String {
        let selector = self.selector.0.split_inclusive(",").map(|s| format!("{} {}", parent, s)).collect::<Vec<String>>().join("\n");
        let rules: Vec<String> = self.rules.iter().map(|rule| format!("    {}: {};", rule.property, rule.value.join(" "))).collect();
        let children: Vec<String> = self.children.iter().map(|child| child.to_stylesheet(&selector)).collect();
        if self.rules.is_empty() {
            format!("{}", children.join(""))
        } else {
            format!("{} {{\n{}\n}}\n{}", selector, rules.join("\n"), children.join("\n"))
        }
    }
}

pub fn lex(input: String) -> Result<Vec<Token>, String> {
    let mut tokens = vec![];
    let mut chars = input.chars().peekable();
    while let Some(c) = chars.peek() {
        let token = match c {
            '(' => {
                chars.next();
                Token::LParen
            }
            ')' => {
                chars.next();
                Token::RParen
            }
            c if c.is_whitespace() => {
                chars.next();
                continue;
            }
            _ => Token::String(lex_string(&mut chars)),
        };
        tokens.push(token);
    }
    Ok(tokens)
}

fn lex_string(chars: &mut Peekable<Chars>) -> String {
    let mut string = String::new();
    let mut depth: u32 = 0;
    while let Some(c) = chars.peek() {
        match c {
            '(' => depth += 1,
            ')' if depth > 0 => depth -= 1,
            ')' => break,
            c if c.is_whitespace() && depth == 0 => break,
            _ => (),
        };
        string.push(*c);
        chars.next();
    }
    string
}

pub fn parse(tokens: Vec<Token>) -> Vec<SExpr> {
    let mut s_exprs = vec![];
    let mut left = 0;
    let mut depth: u32 = 0;
    for (right, token) in tokens.iter().enumerate() {
        match token {
            Token::LParen => depth += 1,
            Token::RParen => depth -= 1,
            _ => (),
        };
        if depth == 0 {
            if let Some(s_expr) = parse_s_expr(&mut tokens[left + 1..right].iter().peekable()) {
                s_exprs.push(s_expr);
            }
            left = right + 1;
        }
    }
    s_exprs
}

fn parse_s_expr(tokens: &mut Peekable<Iter<Token>>) -> Option<SExpr> {
    let selector = match tokens.next()? {
        Token::String(string) => Selector(string.to_owned()),
        _ => return None,
    };
    let mut rules = vec![];
    let mut children = vec![];
    while let Some(token) = tokens.peek() {
        match token {
            Token::String(_) => rules.push(parse_rule(tokens)?),
            Token::LParen => {
                tokens.next();
                children.push(parse_s_expr(tokens)?);
            }
            _ => {
                tokens.next();
                break;
            }
        };
    }
    Some(SExpr {
        selector,
        rules,
        children,
    })
}

fn parse_rule(tokens: &mut Peekable<Iter<Token>>) -> Option<Rule> {
    let property = match tokens.next()? {
        Token::String(string) => string.to_owned(),
        _ => return None,
    };
    let value = match tokens.next()? {
        Token::String(value) => vec![value.to_owned()],
        Token::LParen => tokens
            .map_while(|token| match token {
                Token::String(string) => Some(string.to_owned()),
                _ => None,
            })
            .collect::<Vec<String>>(),
        _ => return None,
    };
    let rule = Rule { property, value };
    Some(rule)
}

#[wasm_bindgen]
pub fn string_to_stylesheet(input: String) -> Result<String, JsValue> {
    let mut string = String::new();
    let tokens = lex(input)?;
    let s_exprs = parse(tokens);
    for s_expr in s_exprs {
        string += &s_expr.to_stylesheet("");
    }
    Ok(string)
}

#[cfg(test)]
mod tests {
    use crate::{lex, parse, Rule, SExpr, Selector, Token};

    #[test]
    fn lex_selector_empty() {
        let input = "(body)";
        let tokens = lex(input.into()).unwrap();
        let expected = vec![Token::LParen, Token::String("body".into()), Token::RParen];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn lex_selector_property_value() {
        let input = "(body color red)";
        let tokens = lex(input.into()).unwrap();
        let expected = vec![
            Token::LParen,
            Token::String("body".into()),
            Token::String("color".into()),
            Token::String("red".into()),
            Token::RParen,
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn parse_selector_property_value() {
        let input = "(body color red)";
        let tokens = lex(input.into()).unwrap();
        let s_exprs = parse(tokens);
        let expected = vec![SExpr {
            selector: Selector("body".into()),
            rules: vec![Rule {
                property: "color".into(),
                value: vec!["red".into()],
            }],
            children: vec![],
        }];
        assert_eq!(s_exprs, expected);
    }

    #[test]
    fn lex_selector_hyphenated_property_value() {
        let input = "(body background-color red)";
        let tokens = lex(input.into()).unwrap();
        let expected = vec![
            Token::LParen,
            Token::String("body".into()),
            Token::String("background-color".into()),
            Token::String("red".into()),
            Token::RParen,
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn lex_selector_property_list_value() {
        let input = "(body margin (0 8px 0 8px))";
        let tokens = lex(input.into()).unwrap();
        let expected = vec![
            Token::LParen,
            Token::String("body".into()),
            Token::String("margin".into()),
            Token::LParen,
            Token::String("0".into()),
            Token::String("8px".into()),
            Token::String("0".into()),
            Token::String("8px".into()),
            Token::RParen,
            Token::RParen,
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn parse_selector_property_list_value() {
        let input = "(body margin (0 8px 0 8px))";
        let tokens = lex(input.into()).unwrap();
        let s_exprs = parse(tokens);
        let expected = vec![SExpr {
            selector: Selector("body".into()),
            rules: vec![Rule {
                property: "margin".into(),
                value: vec!["0".into(), "8px".into(), "0".into(), "8px".into()],
            }],
            children: vec![],
        }];
        assert_eq!(s_exprs, expected);
    }

    #[test]
    fn lex_selector_property_alphanumeric_value() {
        let input = "(body font-size 14px)";
        let tokens = lex(input.into()).unwrap();
        let expected = vec![
            Token::LParen,
            Token::String("body".into()),
            Token::String("font-size".into()),
            Token::String("14px".into()),
            Token::RParen,
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn lex_selector_multiple_property_value() {
        let input = "(body background-color white color red)";
        let tokens = lex(input.into()).unwrap();
        let expected = vec![
            Token::LParen,
            Token::String("body".into()),
            Token::String("background-color".into()),
            Token::String("white".into()),
            Token::String("color".into()),
            Token::String("red".into()),
            Token::RParen,
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn parse_selector_multiple_property_value() {
        let input = "(body background-color white color red)";
        let tokens = lex(input.into()).unwrap();
        let s_exprs = parse(tokens);
        let expected = vec![SExpr {
            selector: Selector("body".into()),
            rules: vec![
                Rule {
                    property: "background-color".into(),
                    value: vec!["white".into()],
                },
                Rule {
                    property: "color".into(),
                    value: vec!["red".into()],
                },
            ],
            children: vec![],
        }];
        assert_eq!(s_exprs, expected);
    }

    #[test]
    fn parse_selector_property_value_interleave_nested() {
        let input = "(body background-color white (p color blue) color red)";
        let tokens = lex(input.into()).unwrap();
        let s_exprs = parse(tokens);
        let expected = vec![SExpr {
            selector: Selector("body".into()),
            rules: vec![
                Rule {
                    property: "background-color".into(),
                    value: vec!["white".into()],
                },
                Rule {
                    property: "color".into(),
                    value: vec!["red".into()],
                },
            ],
            children: vec![SExpr {
                selector: Selector("p".into()),
                rules: vec![Rule {
                    property: "color".into(),
                    value: vec!["blue".into()],
                }],
                children: vec![],
            }],
        }];
        assert_eq!(s_exprs, expected);
    }

    #[test]
    fn lex_selector_nested_selector_property_value() {
        let input = "(ul (li text-decoration none))";
        let tokens = lex(input.into()).unwrap();
        let expected = vec![
            Token::LParen,
            Token::String("ul".into()),
            Token::LParen,
            Token::String("li".into()),
            Token::String("text-decoration".into()),
            Token::String("none".into()),
            Token::RParen,
            Token::RParen,
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn parse_selector_nested_selector_property_value() {
        let input = "(ul (li text-decoration none))";
        let tokens = lex(input.into()).unwrap();
        let s_exprs = parse(tokens);
        let expected = vec![SExpr {
            selector: Selector("ul".into()),
            rules: vec![],
            children: vec![SExpr {
                selector: Selector("li".into()),
                rules: vec![Rule {
                    property: "text-decoration".into(),
                    value: vec!["none".into()],
                }],
                children: vec![],
            }],
        }];
        assert_eq!(s_exprs, expected);
    }

    #[test]
    fn parse_selector_property_value_nested_selector_property_value() {
        let input = "(ul padding 0 (li text-decoration none))";
        let tokens = lex(input.into()).unwrap();
        let s_exprs = parse(tokens);
        let expected = vec![SExpr {
            selector: Selector("ul".into()),
            rules: vec![Rule {
                property: "padding".into(),
                value: vec!["0".into()],
            }],
            children: vec![SExpr {
                selector: Selector("li".into()),
                rules: vec![Rule {
                    property: "text-decoration".into(),
                    value: vec!["none".into()],
                }],
                children: vec![],
            }],
        }];
        assert_eq!(s_exprs, expected);
    }

    #[test]
    fn parse_selector_multiple_property_value_nested_selector_property_value() {
        let input = "(ul padding 0 margin 0 (li padding-left 16px (a text-decoration none)))";
        let tokens = lex(input.into()).unwrap();
        let s_exprs = parse(tokens);
        let expected = vec![SExpr {
            selector: Selector("ul".into()),
            rules: vec![
                Rule {
                    property: "padding".into(),
                    value: vec!["0".into()],
                },
                Rule {
                    property: "margin".into(),
                    value: vec!["0".into()],
                },
            ],
            children: vec![SExpr {
                selector: Selector("li".into()),
                rules: vec![Rule {
                    property: "padding-left".into(),
                    value: vec!["16px".into()],
                }],
                children: vec![SExpr {
                    selector: Selector("a".into()),
                    rules: vec![Rule {
                        property: "text-decoration".into(),
                        value: vec!["none".into()],
                    }],
                    children: vec![],
                }],
            }],
        }];
        assert_eq!(s_exprs, expected);
    }

    #[test]
    fn lex_selector_property_value_nested_selector() {
        let input = "(body color red (table))";
        let tokens = lex(input.into()).unwrap();
        let expected = vec![
            Token::LParen,
            Token::String("body".into()),
            Token::String("color".into()),
            Token::String("red".into()),
            Token::LParen,
            Token::String("table".into()),
            Token::RParen,
            Token::RParen,
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn lex_selector_hyphenated_property_parentheses_value() {
        let input = "(body background-color var(--text-color, red))";
        let tokens = lex(input.into()).unwrap();
        let expected = vec![
            Token::LParen,
            Token::String("body".into()),
            Token::String("background-color".into()),
            Token::String("var(--text-color, red)".into()),
            Token::RParen,
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn lex_selector_hyphenated_property_parentheses_value_continuing_string() {
        let input = "(body background-color var(--text-color, red)def)";
        let tokens = lex(input.into()).unwrap();
        let expected = vec![
            Token::LParen,
            Token::String("body".into()),
            Token::String("background-color".into()),
            Token::String("var(--text-color, red)def".into()),
            Token::RParen,
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn lex_pseudo_selector_property_value() {
        let input = "(a:hover text-decoration underline)";
        let tokens = lex(input.into()).unwrap();
        let expected = vec![
            Token::LParen,
            Token::String("a:hover".into()),
            Token::String("text-decoration".into()),
            Token::String("underline".into()),
            Token::RParen,
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn lex_multiple_selector_property_value() {
        let input = "(body color red)\n(p color blue)";
        let tokens = lex(input.into()).unwrap();
        let expected = vec![
            Token::LParen,
            Token::String("body".into()),
            Token::String("color".into()),
            Token::String("red".into()),
            Token::RParen,
            Token::LParen,
            Token::String("p".into()),
            Token::String("color".into()),
            Token::String("blue".into()),
            Token::RParen,
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn parse_multiple_selector_property_value() {
        let input = "(body color red)\n(p color blue)";
        let tokens = lex(input.into()).unwrap();
        let s_exprs = parse(tokens);
        let expected = vec![
            SExpr {
                selector: Selector("body".into()),
                rules: vec![Rule {
                    property: "color".into(),
                    value: vec!["red".into()],
                }],
                children: vec![],
            },
            SExpr {
                selector: Selector("p".into()),
                rules: vec![Rule {
                    property: "color".into(),
                    value: vec!["blue".into()],
                }],
                children: vec![],
            },
        ];
        assert_eq!(s_exprs, expected);
    }
}
