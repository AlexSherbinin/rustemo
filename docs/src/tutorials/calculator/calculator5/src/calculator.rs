/// Generated by rustemo. Do not edit manually!
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;
use rustemo::{
    Result, Input as InputT, Lexer, Token, TokenRecognizer as TokenRecognizerT, Parser,
    ParserDefinition, State as StateT, Builder,
};
use regex::Regex;
use once_cell::sync::Lazy;
use rustemo::StringLexer;
use rustemo::LRBuilder;
use super::calculator_actions;
use rustemo::{LRParser, LRContext};
use rustemo::Action::{self, Shift, Reduce, Accept, Error};
#[allow(unused_imports)]
use rustemo::debug::{log, logn};
#[allow(unused_imports)]
#[cfg(debug_assertions)]
use colored::*;
pub type Input = str;
const TERMINAL_COUNT: usize = 6usize;
const NONTERMINAL_COUNT: usize = 3usize;
const STATE_COUNT: usize = 11usize;
#[allow(dead_code)]
const MAX_ACTIONS: usize = 1usize;
const MAX_RECOGNIZERS: usize = 5usize;
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TokenKind {
    #[default]
    STOP,
    Number,
    Plus,
    Minus,
    Mul,
    Div,
}
impl From<TokenKind> for usize {
    fn from(t: TokenKind) -> Self {
        t as usize
    }
}
#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, PartialEq)]
pub enum ProdKind {
    EAdd,
    ESub,
    EMul,
    EDiv,
    EP5,
}
impl std::fmt::Debug for ProdKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            ProdKind::EAdd => "E: E Plus E",
            ProdKind::ESub => "E: E Minus E",
            ProdKind::EMul => "E: E Mul E",
            ProdKind::EDiv => "E: E Div E",
            ProdKind::EP5 => "E: Number",
        };
        write!(f, "{}", name)
    }
}
#[allow(clippy::upper_case_acronyms)]
#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub enum NonTermKind {
    EMPTY,
    AUG,
    E,
}
impl From<ProdKind> for NonTermKind {
    fn from(prod: ProdKind) -> Self {
        match prod {
            ProdKind::EAdd => NonTermKind::E,
            ProdKind::ESub => NonTermKind::E,
            ProdKind::EMul => NonTermKind::E,
            ProdKind::EDiv => NonTermKind::E,
            ProdKind::EP5 => NonTermKind::E,
        }
    }
}
#[allow(clippy::enum_variant_names)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum State {
    #[default]
    AUGS0,
    NumberS1,
    ES2,
    PlusS3,
    MinusS4,
    MulS5,
    DivS6,
    ES7,
    ES8,
    ES9,
    ES10,
}
impl StateT for State {
    fn default_layout() -> Option<Self> {
        None
    }
}
impl From<State> for usize {
    fn from(s: State) -> Self {
        s as usize
    }
}
impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            State::AUGS0 => "0:AUG",
            State::NumberS1 => "1:Number",
            State::ES2 => "2:E",
            State::PlusS3 => "3:Plus",
            State::MinusS4 => "4:Minus",
            State::MulS5 => "5:Mul",
            State::DivS6 => "6:Div",
            State::ES7 => "7:E",
            State::ES8 => "8:E",
            State::ES9 => "9:E",
            State::ES10 => "10:E",
        };
        write!(f, "{name}")
    }
}
#[derive(Debug)]
pub enum Symbol {
    Terminal(Terminal),
    NonTerminal(NonTerminal),
}
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
pub enum Terminal {
    Number(calculator_actions::Number),
    Plus,
    Minus,
    Mul,
    Div,
}
#[derive(Debug)]
pub enum NonTerminal {
    E(calculator_actions::E),
}
pub struct CalculatorParserDefinition {
    actions: [[[Action<State, ProdKind>; MAX_ACTIONS]; TERMINAL_COUNT]; STATE_COUNT],
    gotos: [[Option<State>; NONTERMINAL_COUNT]; STATE_COUNT],
    token_kinds: [[Option<TokenKind>; MAX_RECOGNIZERS]; STATE_COUNT],
}
pub(crate) static PARSER_DEFINITION: CalculatorParserDefinition = CalculatorParserDefinition {
    actions: [
        [[Error], [Shift(State::NumberS1)], [Error], [Error], [Error], [Error]],
        [
            [Reduce(ProdKind::EP5, 1usize)],
            [Error],
            [Reduce(ProdKind::EP5, 1usize)],
            [Reduce(ProdKind::EP5, 1usize)],
            [Reduce(ProdKind::EP5, 1usize)],
            [Reduce(ProdKind::EP5, 1usize)],
        ],
        [
            [Accept],
            [Error],
            [Shift(State::PlusS3)],
            [Shift(State::MinusS4)],
            [Shift(State::MulS5)],
            [Shift(State::DivS6)],
        ],
        [[Error], [Shift(State::NumberS1)], [Error], [Error], [Error], [Error]],
        [[Error], [Shift(State::NumberS1)], [Error], [Error], [Error], [Error]],
        [[Error], [Shift(State::NumberS1)], [Error], [Error], [Error], [Error]],
        [[Error], [Shift(State::NumberS1)], [Error], [Error], [Error], [Error]],
        [
            [Reduce(ProdKind::EAdd, 3usize)],
            [Error],
            [Reduce(ProdKind::EAdd, 3usize)],
            [Reduce(ProdKind::EAdd, 3usize)],
            [Shift(State::MulS5)],
            [Shift(State::DivS6)],
        ],
        [
            [Reduce(ProdKind::ESub, 3usize)],
            [Error],
            [Reduce(ProdKind::ESub, 3usize)],
            [Reduce(ProdKind::ESub, 3usize)],
            [Shift(State::MulS5)],
            [Shift(State::DivS6)],
        ],
        [
            [Reduce(ProdKind::EMul, 3usize)],
            [Error],
            [Reduce(ProdKind::EMul, 3usize)],
            [Reduce(ProdKind::EMul, 3usize)],
            [Reduce(ProdKind::EMul, 3usize)],
            [Reduce(ProdKind::EMul, 3usize)],
        ],
        [
            [Reduce(ProdKind::EDiv, 3usize)],
            [Error],
            [Reduce(ProdKind::EDiv, 3usize)],
            [Reduce(ProdKind::EDiv, 3usize)],
            [Reduce(ProdKind::EDiv, 3usize)],
            [Reduce(ProdKind::EDiv, 3usize)],
        ],
    ],
    gotos: [
        [None, None, Some(State::ES2)],
        [None, None, None],
        [None, None, None],
        [None, None, Some(State::ES7)],
        [None, None, Some(State::ES8)],
        [None, None, Some(State::ES9)],
        [None, None, Some(State::ES10)],
        [None, None, None],
        [None, None, None],
        [None, None, None],
        [None, None, None],
    ],
    token_kinds: [
        [Some(TokenKind::Number), None, None, None, None],
        [
            Some(TokenKind::STOP),
            Some(TokenKind::Plus),
            Some(TokenKind::Minus),
            Some(TokenKind::Mul),
            Some(TokenKind::Div),
        ],
        [
            Some(TokenKind::STOP),
            Some(TokenKind::Plus),
            Some(TokenKind::Minus),
            Some(TokenKind::Mul),
            Some(TokenKind::Div),
        ],
        [Some(TokenKind::Number), None, None, None, None],
        [Some(TokenKind::Number), None, None, None, None],
        [Some(TokenKind::Number), None, None, None, None],
        [Some(TokenKind::Number), None, None, None, None],
        [
            Some(TokenKind::STOP),
            Some(TokenKind::Plus),
            Some(TokenKind::Minus),
            Some(TokenKind::Mul),
            Some(TokenKind::Div),
        ],
        [
            Some(TokenKind::STOP),
            Some(TokenKind::Plus),
            Some(TokenKind::Minus),
            Some(TokenKind::Mul),
            Some(TokenKind::Div),
        ],
        [
            Some(TokenKind::STOP),
            Some(TokenKind::Plus),
            Some(TokenKind::Minus),
            Some(TokenKind::Mul),
            Some(TokenKind::Div),
        ],
        [
            Some(TokenKind::STOP),
            Some(TokenKind::Plus),
            Some(TokenKind::Minus),
            Some(TokenKind::Mul),
            Some(TokenKind::Div),
        ],
    ],
};
impl ParserDefinition<State, ProdKind, TokenKind, NonTermKind>
for CalculatorParserDefinition {
    fn actions(
        &self,
        state: State,
        token: TokenKind,
    ) -> &'static [Action<State, ProdKind>] {
        &PARSER_DEFINITION.actions[state as usize][token as usize]
    }
    fn goto(&self, state: State, nonterm: NonTermKind) -> State {
        PARSER_DEFINITION.gotos[state as usize][nonterm as usize].unwrap()
    }
    fn expected_token_kinds(&self, state: State) -> &'static [Option<TokenKind>] {
        &PARSER_DEFINITION.token_kinds[state as usize]
    }
}
pub(crate) type Context<'i, I> = LRContext<'i, I, State, TokenKind>;
pub struct CalculatorParser<
    'i,
    I: InputT + ?Sized,
    L: Lexer<'i, Context<'i, I>, State, TokenKind, Input = I>,
    B,
>(
    LRParser<
        'i,
        Context<'i, I>,
        State,
        ProdKind,
        TokenKind,
        NonTermKind,
        CalculatorParserDefinition,
        L,
        B,
        I,
    >,
);
#[allow(dead_code)]
impl<
    'i,
> CalculatorParser<
    'i,
    Input,
    StringLexer<Context<'i, Input>, State, TokenKind, TokenRecognizer, TERMINAL_COUNT>,
    DefaultBuilder,
> {
    pub fn new() -> Self {
        Self(
            LRParser::new(
                &PARSER_DEFINITION,
                State::default(),
                false,
                false,
                Rc::new(StringLexer::new(true, &RECOGNIZERS)),
                DefaultBuilder::new(),
            ),
        )
    }
}
#[allow(dead_code)]
impl<'i, I, L, B> Parser<'i, I, Context<'i, I>, L, State, TokenKind>
for CalculatorParser<'i, I, L, B>
where
    I: InputT + ?Sized + Debug,
    L: Lexer<'i, Context<'i, I>, State, TokenKind, Input = I>,
    B: LRBuilder<'i, I, Context<'i, I>, State, ProdKind, TokenKind>,
{
    type Output = B::Output;
    fn parse(&self, input: &'i I) -> Result<Self::Output> {
        self.0.parse(input)
    }
    fn parse_with_context(
        &self,
        context: &mut Context<'i, I>,
        input: &'i I,
    ) -> Result<Self::Output> {
        self.0.parse_with_context(context, input)
    }
    fn parse_file<'a, F: AsRef<std::path::Path>>(
        &'a mut self,
        file: F,
    ) -> Result<Self::Output>
    where
        'a: 'i,
    {
        self.0.parse_file(file)
    }
}
#[allow(dead_code)]
#[derive(Debug)]
pub enum Recognizer {
    Stop,
    StrMatch(&'static str),
    RegexMatch(Lazy<Regex>),
}
#[allow(dead_code)]
#[derive(Debug)]
pub struct TokenRecognizer(TokenKind, Recognizer);
impl<'i> TokenRecognizerT<'i> for TokenRecognizer {
    fn recognize(&self, input: &'i str) -> Option<&'i str> {
        match &self {
            #[allow(unused_variables)]
            TokenRecognizer(token_kind, Recognizer::StrMatch(s)) => {
                logn!("{} {:?} -- ", "    Recognizing".green(), token_kind);
                if input.starts_with(s) {
                    log!("{}", "recognized".bold().green());
                    Some(s)
                } else {
                    log!("{}", "not recognized".red());
                    None
                }
            }
            #[allow(unused_variables)]
            TokenRecognizer(token_kind, Recognizer::RegexMatch(r)) => {
                logn!("{} {:?} -- ", "    Recognizing".green(), token_kind);
                let match_str = r.find(input);
                match match_str {
                    Some(x) => {
                        let x_str = x.as_str();
                        log!("{} '{}'", "recognized".bold().green(), x_str);
                        Some(x_str)
                    }
                    None => {
                        log!("{}", "not recognized".red());
                        None
                    }
                }
            }
            TokenRecognizer(_, Recognizer::Stop) => {
                logn!("{} STOP -- ", "    Recognizing".green());
                if input.is_empty() {
                    log!("{}", "recognized".bold().green());
                    Some("")
                } else {
                    log!("{}", "not recognized".red());
                    None
                }
            }
        }
    }
}
pub(crate) static RECOGNIZERS: [TokenRecognizer; TERMINAL_COUNT] = [
    TokenRecognizer(TokenKind::STOP, Recognizer::Stop),
    TokenRecognizer(
        TokenKind::Number,
        Recognizer::RegexMatch(
            Lazy::new(|| { Regex::new(concat!("^", "\\d+(\\.\\d+)?")).unwrap() }),
        ),
    ),
    TokenRecognizer(TokenKind::Plus, Recognizer::StrMatch("+")),
    TokenRecognizer(TokenKind::Minus, Recognizer::StrMatch("-")),
    TokenRecognizer(TokenKind::Mul, Recognizer::StrMatch("*")),
    TokenRecognizer(TokenKind::Div, Recognizer::StrMatch("/")),
];
pub struct DefaultBuilder {
    res_stack: Vec<Symbol>,
}
impl DefaultBuilder {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { res_stack: vec![] }
    }
}
impl Builder for DefaultBuilder {
    type Output = calculator_actions::E;
    fn get_result(&mut self) -> Self::Output {
        match self.res_stack.pop().unwrap() {
            Symbol::NonTerminal(NonTerminal::E(r)) => r,
            _ => panic!("Invalid result on the parse stack!"),
        }
    }
}
impl<'i> LRBuilder<'i, Input, Context<'i, Input>, State, ProdKind, TokenKind>
for DefaultBuilder {
    #![allow(unused_variables)]
    fn shift_action(
        &mut self,
        context: &mut Context<'i, Input>,
        token: Token<'i, Input, TokenKind>,
    ) {
        let val = match token.kind {
            TokenKind::STOP => panic!("Cannot shift STOP token!"),
            TokenKind::Number => {
                Terminal::Number(calculator_actions::number(&*context, token))
            }
            TokenKind::Plus => Terminal::Plus,
            TokenKind::Minus => Terminal::Minus,
            TokenKind::Mul => Terminal::Mul,
            TokenKind::Div => Terminal::Div,
        };
        self.res_stack.push(Symbol::Terminal(val));
    }
    fn reduce_action(
        &mut self,
        context: &mut Context<'i, Input>,
        prod: ProdKind,
        _prod_len: usize,
    ) {
        let prod = match prod {
            ProdKind::EAdd => {
                let mut i = self
                    .res_stack
                    .split_off(self.res_stack.len() - 3usize)
                    .into_iter();
                match (i.next().unwrap(), i.next().unwrap(), i.next().unwrap()) {
                    (
                        Symbol::NonTerminal(NonTerminal::E(p0)),
                        _,
                        Symbol::NonTerminal(NonTerminal::E(p1)),
                    ) => NonTerminal::E(calculator_actions::e_add(&*context, p0, p1)),
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::ESub => {
                let mut i = self
                    .res_stack
                    .split_off(self.res_stack.len() - 3usize)
                    .into_iter();
                match (i.next().unwrap(), i.next().unwrap(), i.next().unwrap()) {
                    (
                        Symbol::NonTerminal(NonTerminal::E(p0)),
                        _,
                        Symbol::NonTerminal(NonTerminal::E(p1)),
                    ) => NonTerminal::E(calculator_actions::e_sub(&*context, p0, p1)),
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::EMul => {
                let mut i = self
                    .res_stack
                    .split_off(self.res_stack.len() - 3usize)
                    .into_iter();
                match (i.next().unwrap(), i.next().unwrap(), i.next().unwrap()) {
                    (
                        Symbol::NonTerminal(NonTerminal::E(p0)),
                        _,
                        Symbol::NonTerminal(NonTerminal::E(p1)),
                    ) => NonTerminal::E(calculator_actions::e_mul(&*context, p0, p1)),
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::EDiv => {
                let mut i = self
                    .res_stack
                    .split_off(self.res_stack.len() - 3usize)
                    .into_iter();
                match (i.next().unwrap(), i.next().unwrap(), i.next().unwrap()) {
                    (
                        Symbol::NonTerminal(NonTerminal::E(p0)),
                        _,
                        Symbol::NonTerminal(NonTerminal::E(p1)),
                    ) => NonTerminal::E(calculator_actions::e_div(&*context, p0, p1)),
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::EP5 => {
                let mut i = self
                    .res_stack
                    .split_off(self.res_stack.len() - 1usize)
                    .into_iter();
                match i.next().unwrap() {
                    Symbol::Terminal(Terminal::Number(p0)) => {
                        NonTerminal::E(calculator_actions::e_number(&*context, p0))
                    }
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
        };
        self.res_stack.push(Symbol::NonTerminal(prod));
    }
}
