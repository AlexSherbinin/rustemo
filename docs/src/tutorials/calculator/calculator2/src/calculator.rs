/// Generated by rustemo. Do not edit manually!
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use rustemo::Result;
use rustemo::lexer::{self, Token};
use rustemo::parser::Parser;
use rustemo::builder::Builder;
use rustemo::lr::builder::LRBuilder;
use rustemo::lr::parser::{LRParser, ParserDefinition};
use rustemo::lr::parser::Action::{self, Shift, Reduce, Accept, Error};
#[allow(unused_imports)]
use rustemo::debug::{log, logn};
#[allow(unused_imports)]
#[cfg(debug_assertions)]
use colored::*;
const TERMINAL_COUNT: usize = 6usize;
const NONTERMINAL_COUNT: usize = 3usize;
const STATE_COUNT: usize = 11usize;
#[allow(dead_code)]
const MAX_ACTIONS: usize = 5usize;
use regex::Regex;
use once_cell::sync::Lazy;
use rustemo::lexer::StringLexer;
use super::calculator_actions;
pub type Input = str;
pub type Context<'i> = lexer::Context<'i, Input>;
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenKind {
    #[default]
    STOP,
    Number,
    Plus,
    Minus,
    Mul,
    Div,
}
#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy)]
pub enum ProdKind {
    EP1,
    EP2,
    EP3,
    EP4,
    EP5,
}
impl std::fmt::Debug for ProdKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            ProdKind::EP1 => "E: E Plus E",
            ProdKind::EP2 => "E: E Minus E",
            ProdKind::EP3 => "E: E Mul E",
            ProdKind::EP4 => "E: E Div E",
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
            ProdKind::EP1 => NonTermKind::E,
            ProdKind::EP2 => NonTermKind::E,
            ProdKind::EP3 => NonTermKind::E,
            ProdKind::EP4 => NonTermKind::E,
            ProdKind::EP5 => NonTermKind::E,
        }
    }
}
#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy)]
pub enum State {
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
    actions: [[Action<State, ProdKind>; TERMINAL_COUNT]; STATE_COUNT],
    gotos: [[Option<State>; NONTERMINAL_COUNT]; STATE_COUNT],
    token_recognizers: [[Option<TokenRecognizer>; 5usize]; STATE_COUNT],
}
pub(crate) static PARSER_DEFINITION: CalculatorParserDefinition = CalculatorParserDefinition {
    actions: [
        [Error, Shift(State::NumberS1), Error, Error, Error, Error],
        [
            Reduce(ProdKind::EP5, 1usize),
            Error,
            Reduce(ProdKind::EP5, 1usize),
            Reduce(ProdKind::EP5, 1usize),
            Reduce(ProdKind::EP5, 1usize),
            Reduce(ProdKind::EP5, 1usize),
        ],
        [
            Accept,
            Error,
            Shift(State::PlusS3),
            Shift(State::MinusS4),
            Shift(State::MulS5),
            Shift(State::DivS6),
        ],
        [Error, Shift(State::NumberS1), Error, Error, Error, Error],
        [Error, Shift(State::NumberS1), Error, Error, Error, Error],
        [Error, Shift(State::NumberS1), Error, Error, Error, Error],
        [Error, Shift(State::NumberS1), Error, Error, Error, Error],
        [
            Reduce(ProdKind::EP1, 3usize),
            Error,
            Reduce(ProdKind::EP1, 3usize),
            Reduce(ProdKind::EP1, 3usize),
            Shift(State::MulS5),
            Shift(State::DivS6),
        ],
        [
            Reduce(ProdKind::EP2, 3usize),
            Error,
            Reduce(ProdKind::EP2, 3usize),
            Reduce(ProdKind::EP2, 3usize),
            Shift(State::MulS5),
            Shift(State::DivS6),
        ],
        [
            Reduce(ProdKind::EP3, 3usize),
            Error,
            Reduce(ProdKind::EP3, 3usize),
            Reduce(ProdKind::EP3, 3usize),
            Reduce(ProdKind::EP3, 3usize),
            Reduce(ProdKind::EP3, 3usize),
        ],
        [
            Reduce(ProdKind::EP4, 3usize),
            Error,
            Reduce(ProdKind::EP4, 3usize),
            Reduce(ProdKind::EP4, 3usize),
            Reduce(ProdKind::EP4, 3usize),
            Reduce(ProdKind::EP4, 3usize),
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
    token_recognizers: [
        [
            Some(TokenRecognizer {
                token_kind: TokenKind::Number,
                recognizer: Recognizer::RegexMatch(1usize),
                finish: true,
            }),
            None,
            None,
            None,
            None,
        ],
        [
            Some(TokenRecognizer {
                token_kind: TokenKind::STOP,
                recognizer: Recognizer::Stop,
                finish: true,
            }),
            Some(TokenRecognizer {
                token_kind: TokenKind::Plus,
                recognizer: Recognizer::StrMatch("+"),
                finish: true,
            }),
            Some(TokenRecognizer {
                token_kind: TokenKind::Minus,
                recognizer: Recognizer::StrMatch("-"),
                finish: true,
            }),
            Some(TokenRecognizer {
                token_kind: TokenKind::Mul,
                recognizer: Recognizer::StrMatch("*"),
                finish: true,
            }),
            Some(TokenRecognizer {
                token_kind: TokenKind::Div,
                recognizer: Recognizer::StrMatch("/"),
                finish: true,
            }),
        ],
        [
            Some(TokenRecognizer {
                token_kind: TokenKind::STOP,
                recognizer: Recognizer::Stop,
                finish: true,
            }),
            Some(TokenRecognizer {
                token_kind: TokenKind::Plus,
                recognizer: Recognizer::StrMatch("+"),
                finish: true,
            }),
            Some(TokenRecognizer {
                token_kind: TokenKind::Minus,
                recognizer: Recognizer::StrMatch("-"),
                finish: true,
            }),
            Some(TokenRecognizer {
                token_kind: TokenKind::Mul,
                recognizer: Recognizer::StrMatch("*"),
                finish: true,
            }),
            Some(TokenRecognizer {
                token_kind: TokenKind::Div,
                recognizer: Recognizer::StrMatch("/"),
                finish: true,
            }),
        ],
        [
            Some(TokenRecognizer {
                token_kind: TokenKind::Number,
                recognizer: Recognizer::RegexMatch(1usize),
                finish: true,
            }),
            None,
            None,
            None,
            None,
        ],
        [
            Some(TokenRecognizer {
                token_kind: TokenKind::Number,
                recognizer: Recognizer::RegexMatch(1usize),
                finish: true,
            }),
            None,
            None,
            None,
            None,
        ],
        [
            Some(TokenRecognizer {
                token_kind: TokenKind::Number,
                recognizer: Recognizer::RegexMatch(1usize),
                finish: true,
            }),
            None,
            None,
            None,
            None,
        ],
        [
            Some(TokenRecognizer {
                token_kind: TokenKind::Number,
                recognizer: Recognizer::RegexMatch(1usize),
                finish: true,
            }),
            None,
            None,
            None,
            None,
        ],
        [
            Some(TokenRecognizer {
                token_kind: TokenKind::STOP,
                recognizer: Recognizer::Stop,
                finish: true,
            }),
            Some(TokenRecognizer {
                token_kind: TokenKind::Plus,
                recognizer: Recognizer::StrMatch("+"),
                finish: true,
            }),
            Some(TokenRecognizer {
                token_kind: TokenKind::Minus,
                recognizer: Recognizer::StrMatch("-"),
                finish: true,
            }),
            Some(TokenRecognizer {
                token_kind: TokenKind::Mul,
                recognizer: Recognizer::StrMatch("*"),
                finish: true,
            }),
            Some(TokenRecognizer {
                token_kind: TokenKind::Div,
                recognizer: Recognizer::StrMatch("/"),
                finish: true,
            }),
        ],
        [
            Some(TokenRecognizer {
                token_kind: TokenKind::STOP,
                recognizer: Recognizer::Stop,
                finish: true,
            }),
            Some(TokenRecognizer {
                token_kind: TokenKind::Plus,
                recognizer: Recognizer::StrMatch("+"),
                finish: true,
            }),
            Some(TokenRecognizer {
                token_kind: TokenKind::Minus,
                recognizer: Recognizer::StrMatch("-"),
                finish: true,
            }),
            Some(TokenRecognizer {
                token_kind: TokenKind::Mul,
                recognizer: Recognizer::StrMatch("*"),
                finish: true,
            }),
            Some(TokenRecognizer {
                token_kind: TokenKind::Div,
                recognizer: Recognizer::StrMatch("/"),
                finish: true,
            }),
        ],
        [
            Some(TokenRecognizer {
                token_kind: TokenKind::STOP,
                recognizer: Recognizer::Stop,
                finish: true,
            }),
            Some(TokenRecognizer {
                token_kind: TokenKind::Plus,
                recognizer: Recognizer::StrMatch("+"),
                finish: true,
            }),
            Some(TokenRecognizer {
                token_kind: TokenKind::Minus,
                recognizer: Recognizer::StrMatch("-"),
                finish: true,
            }),
            Some(TokenRecognizer {
                token_kind: TokenKind::Mul,
                recognizer: Recognizer::StrMatch("*"),
                finish: true,
            }),
            Some(TokenRecognizer {
                token_kind: TokenKind::Div,
                recognizer: Recognizer::StrMatch("/"),
                finish: true,
            }),
        ],
        [
            Some(TokenRecognizer {
                token_kind: TokenKind::STOP,
                recognizer: Recognizer::Stop,
                finish: true,
            }),
            Some(TokenRecognizer {
                token_kind: TokenKind::Plus,
                recognizer: Recognizer::StrMatch("+"),
                finish: true,
            }),
            Some(TokenRecognizer {
                token_kind: TokenKind::Minus,
                recognizer: Recognizer::StrMatch("-"),
                finish: true,
            }),
            Some(TokenRecognizer {
                token_kind: TokenKind::Mul,
                recognizer: Recognizer::StrMatch("*"),
                finish: true,
            }),
            Some(TokenRecognizer {
                token_kind: TokenKind::Div,
                recognizer: Recognizer::StrMatch("/"),
                finish: true,
            }),
        ],
    ],
};
impl ParserDefinition<TokenRecognizer, State, ProdKind, TokenKind, NonTermKind>
for CalculatorParserDefinition {
    fn action(&self, state: State, token: TokenKind) -> Action<State, ProdKind> {
        PARSER_DEFINITION.actions[state as usize][token as usize]
    }
    fn goto(&self, state: State, nonterm: NonTermKind) -> State {
        PARSER_DEFINITION.gotos[state as usize][nonterm as usize].unwrap()
    }
    fn recognizers(&self, state: State) -> Vec<&TokenRecognizer> {
        PARSER_DEFINITION
            .token_recognizers[state as usize]
            .iter()
            .map_while(|tr| tr.as_ref())
            .collect()
    }
}
#[derive(Default)]
pub struct CalculatorParser {
    content: Option<<Input as ToOwned>::Owned>,
}
#[allow(dead_code)]
impl<'i> CalculatorParser {
    pub fn new() -> Self {
        Self { content: None }
    }
    #[allow(clippy::needless_lifetimes)]
    pub fn parse_file<P: AsRef<std::path::Path>>(
        &'i mut self,
        file: P,
    ) -> Result<<DefaultBuilder as Builder>::Output> {
        self.content = Some(<Input as rustemo::lexer::Input>::read_file(&file)?);
        let mut context = Context::new(
            file.as_ref().to_string_lossy().to_string(),
            self.content.as_ref().unwrap(),
        );
        self.inner_parse(&mut context)
    }
    #[allow(clippy::needless_lifetimes)]
    pub fn parse(
        &self,
        input: &'i Input,
    ) -> Result<<DefaultBuilder as Builder>::Output> {
        let mut context = Context::new("<str>".to_string(), input);
        self.inner_parse(&mut context)
    }
    #[allow(clippy::needless_lifetimes)]
    fn inner_parse(
        &self,
        context: &mut Context<'i>,
    ) -> Result<<DefaultBuilder as Builder>::Output> {
        let local_lexer = StringLexer::new(true);
        let lexer = &local_lexer;
        let mut local_builder = DefaultBuilder::new();
        let builder = &mut local_builder;
        let mut parser = LRParser::new(&PARSER_DEFINITION, State::AUGS0, false);
        parser.parse(context, lexer, builder)
    }
}
pub(crate) static RECOGNIZERS: [Option<Lazy<Regex>>; TERMINAL_COUNT] = [
    None,
    Some(Lazy::new(|| { Regex::new(concat!("^", "\\d+(\\.\\d+)?")).unwrap() })),
    None,
    None,
    None,
    None,
];
#[allow(dead_code)]
#[derive(Debug)]
pub enum Recognizer {
    Stop,
    StrMatch(&'static str),
    RegexMatch(usize),
}
#[derive(Debug)]
pub struct TokenRecognizer {
    token_kind: TokenKind,
    recognizer: Recognizer,
    finish: bool,
}
impl lexer::TokenRecognizer for TokenRecognizer {
    type TokenKind = TokenKind;
    type Input = str;
    fn recognize<'i>(&self, input: &'i str) -> Option<&'i str> {
        match &self.recognizer {
            Recognizer::StrMatch(s) => {
                logn!("{} {:?} -- ", "\tRecognizing".green(), self.token_kind());
                if input.starts_with(s) {
                    log!("{}", "recognized".bold().green());
                    Some(s)
                } else {
                    log!("{}", "not recognized".red());
                    None
                }
            }
            Recognizer::RegexMatch(r) => {
                logn!("{} {:?} -- ", "\tRecognizing".green(), self.token_kind());
                let match_str = RECOGNIZERS[*r].as_ref().unwrap().find(input);
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
            Recognizer::Stop => {
                logn!("{} STOP -- ", "\tRecognizing".green());
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
    #[inline]
    fn token_kind(&self) -> TokenKind {
        self.token_kind
    }
    #[inline]
    fn finish(&self) -> bool {
        self.finish
    }
}
impl PartialEq for TokenRecognizer {
    fn eq(&self, other: &Self) -> bool {
        self.token_kind == other.token_kind
    }
}
impl Eq for TokenRecognizer {}
impl Hash for TokenRecognizer {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.token_kind.hash(state);
    }
}
pub struct DefaultBuilder {
    res_stack: Vec<Symbol>,
}
impl Builder for DefaultBuilder {
    type Output = calculator_actions::E;
    fn new() -> Self {
        Self { res_stack: vec![] }
    }
    fn get_result(&mut self) -> Self::Output {
        match self.res_stack.pop().unwrap() {
            Symbol::NonTerminal(NonTerminal::E(r)) => r,
            _ => panic!("Invalid result on the parse stack!"),
        }
    }
}
impl<'i> LRBuilder<'i, Input, ProdKind, TokenKind> for DefaultBuilder {
    #![allow(unused_variables)]
    fn shift_action(
        &mut self,
        context: &mut Context<'i>,
        token: Token<'i, Input, TokenKind>,
    ) {
        let val = match token.kind {
            TokenKind::STOP => panic!("Cannot shift STOP token!"),
            TokenKind::Number => {
                Terminal::Number(calculator_actions::number(context, token))
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
        context: &mut Context<'i>,
        prod: ProdKind,
        _prod_len: usize,
    ) {
        let prod = match prod {
            ProdKind::EP1 => {
                let mut i = self
                    .res_stack
                    .split_off(self.res_stack.len() - 3usize)
                    .into_iter();
                match (i.next().unwrap(), i.next().unwrap(), i.next().unwrap()) {
                    (
                        Symbol::NonTerminal(NonTerminal::E(p0)),
                        _,
                        Symbol::NonTerminal(NonTerminal::E(p1)),
                    ) => NonTerminal::E(calculator_actions::e_c1(context, p0, p1)),
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::EP2 => {
                let mut i = self
                    .res_stack
                    .split_off(self.res_stack.len() - 3usize)
                    .into_iter();
                match (i.next().unwrap(), i.next().unwrap(), i.next().unwrap()) {
                    (
                        Symbol::NonTerminal(NonTerminal::E(p0)),
                        _,
                        Symbol::NonTerminal(NonTerminal::E(p1)),
                    ) => NonTerminal::E(calculator_actions::e_c2(context, p0, p1)),
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::EP3 => {
                let mut i = self
                    .res_stack
                    .split_off(self.res_stack.len() - 3usize)
                    .into_iter();
                match (i.next().unwrap(), i.next().unwrap(), i.next().unwrap()) {
                    (
                        Symbol::NonTerminal(NonTerminal::E(p0)),
                        _,
                        Symbol::NonTerminal(NonTerminal::E(p1)),
                    ) => NonTerminal::E(calculator_actions::e_c3(context, p0, p1)),
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::EP4 => {
                let mut i = self
                    .res_stack
                    .split_off(self.res_stack.len() - 3usize)
                    .into_iter();
                match (i.next().unwrap(), i.next().unwrap(), i.next().unwrap()) {
                    (
                        Symbol::NonTerminal(NonTerminal::E(p0)),
                        _,
                        Symbol::NonTerminal(NonTerminal::E(p1)),
                    ) => NonTerminal::E(calculator_actions::e_c4(context, p0, p1)),
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
                        NonTerminal::E(calculator_actions::e_number(context, p0))
                    }
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
        };
        self.res_stack.push(Symbol::NonTerminal(prod));
    }
}
