///! This file is maintained by rustemo but can be modified manually.
///! All manual changes will be preserved except non-doc comments.
use rustemo::lexer;
use super::calculator::TokenKind;
pub type Input = str;
#[allow(dead_code)]
pub type Token<'i> = lexer::Token<'i, Input, TokenKind>;
pub type Number = String;
pub fn number(token: Token) -> Number {
    token.value.into()
}
/// ANCHOR: structs
#[derive(Debug, Clone)]
pub struct EC1 {
    pub e_1: Box<E>,
    pub e_3: Box<E>,
}
#[derive(Debug, Clone)]
pub struct EC2 {
    pub e_1: Box<E>,
    pub e_3: Box<E>,
}
/// ANCHOR_END: structs
#[derive(Debug, Clone)]
pub struct EC3 {
    pub e_1: Box<E>,
    pub e_3: Box<E>,
}
#[derive(Debug, Clone)]
pub struct EC4 {
    pub e_1: Box<E>,
    pub e_3: Box<E>,
}
/// ANCHOR: enum
#[derive(Debug, Clone)]
pub enum E {
    C1(EC1),
    C2(EC2),
    C3(EC3),
    C4(EC4),
    C5(Number),
}
/// ANCHOR_END: enum
pub fn e_c1(e_1: E, e_3: E) -> E {
    E::C1(EC1 {
        e_1: Box::new(e_1),
        e_3: Box::new(e_3),
    })
}
pub fn e_c2(e_1: E, e_3: E) -> E {
    E::C2(EC2 {
        e_1: Box::new(e_1),
        e_3: Box::new(e_3),
    })
}
pub fn e_c3(e_1: E, e_3: E) -> E {
    E::C3(EC3 {
        e_1: Box::new(e_1),
        e_3: Box::new(e_3),
    })
}
pub fn e_c4(e_1: E, e_3: E) -> E {
    E::C4(EC4 {
        e_1: Box::new(e_1),
        e_3: Box::new(e_3),
    })
}
pub fn e_c5(number: Number) -> E {
    E::C5(number)
}