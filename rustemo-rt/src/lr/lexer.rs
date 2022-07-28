use std::cmp::min;

use crate::debug::log;
use crate::grammar::TerminalInfo;
use crate::index::StateIndex;
use crate::lexer::{Context, Lexer, Token};
use crate::location::Location;

#[derive(Debug)]
pub struct LRContext<I> {
    file: String,
    input: I,
    position: usize,
    location: Option<Location>,
    token: Option<Token<I>>,
    layout: Option<I>,
    state: StateIndex,
}

impl<I> LRContext<I> {
    pub fn new(file: String, input: I) -> Self {
        Self {
            file,
            input,
            position: 0,
            token: None,
            location: None,
            layout: None,
            state: StateIndex(0),
        }
    }
}

impl<I> Context<I> for LRContext<I> {
    #[inline]
    fn file(&self) -> String {
        self.file.clone()
    }

    #[inline]
    fn location_str(&self) -> String {
        match self.location() {
            Some(location) => {
                format!("{}:{}", self.file(), location)
            }
            None => format!("{}:{}", self.file(), self.position()),
        }
    }

    #[inline]
    fn position(&self) -> usize {
        self.position
    }

    #[inline]
    fn set_position(&mut self, position: usize) {
        self.position = position
    }

    #[inline]
    fn token_ahead(&self) -> &Option<Token<I>> {
        &self.token
    }

    #[inline]
    fn set_token_ahead(&mut self, token: Option<Token<I>>) {
        self.token = token;
    }

    #[inline]
    fn input(&self) -> &I {
        &self.input
    }

    #[inline]
    fn location(&self) -> &Option<Location> {
        &self.location
    }

    #[inline]
    fn set_location(&mut self, location: Location) {
        self.location = Some(location);
    }

    #[inline]
    fn layout(&self) -> &Option<I> {
        &self.layout
    }

    #[inline]
    fn set_layout(&mut self, layout: I) {
        self.layout = Some(layout);
    }
}

impl<I> LRContext<I> {
    pub fn state(&self) -> StateIndex {
        self.state
    }

    pub fn set_state(&mut self, state: StateIndex) {
        self.state = state
    }
}

/// A lexer that operates over string inputs and uses generated string and regex
/// recognizers provided by the parser table.
pub struct LRStringLexer<D: 'static> {
    pub(crate) definition: &'static D,
}

impl<D> LRStringLexer<D>
where
    D: LexerDefinition<Recognizer = for<'a> fn(&'a str) -> Option<&'a str>>,
{
    pub fn new(definition: &'static D) -> Self {
        Self { definition }
    }

    fn skip<'i>(context: &mut impl Context<&'i str>) {
        let skipped = context.input()[context.position()..]
            .chars()
            .take_while(|x| x.is_whitespace())
            .collect::<String>();
        log!("Skipped ws: {}", skipped.len());
        context.set_layout(
            &context.input()
                [context.position()..context.position() + skipped.len()]);
        context.set_position(context.position() + skipped.len());
    }
}

impl<'i, D> Lexer<&'i str, LRContext<&'i str>> for LRStringLexer<D>
where
    D: LexerDefinition<Recognizer = for<'a> fn(&'a str) -> Option<&'a str>>,
{
    fn next_token(
        &self,
        context: &mut LRContext<&'i str>,
    ) -> Option<Token<&'i str>> {
        Self::skip(context);
        log!(
            "Context: {}",
            context.input()[context.position() - min(15, context.position())
                ..context.position()]
                .chars()
                .chain("-->".chars())
                .chain(context.input()[context.position()..].chars().take(15))
                .collect::<String>()
        );
        let token: Option<Token<&'i str>> = self
            .definition
            .recognizers(context.state())
            .map(|(recognizer, terminal_info)| {
                (
                    recognizer(&context.input()[context.position()..]),
                    terminal_info,
                )
            })
            // Skip unsuccesful recognition
            .skip_while(|(recognized, _)| recognized.is_none())
            // Create tokens
            .map(|(recognized, terminal_info)| Token {
                terminal: terminal_info,
                value: recognized.unwrap(),
                location: None,
                layout: None,
                layout_location: None,
            })
            // Take the first token or return None if no tokens are found.
            .next();

        match token {
            Some(t) => {
                let new_pos = context.position() + t.value.len();
                context.set_position(new_pos);
                context.set_token_ahead(Some(t.clone()));
                Some(t)
            }
            None => {
                context.set_token_ahead(None);
                None
            }
        }
    }
}

pub trait LexerDefinition {
    type Recognizer;
    /// For the given state, returns iterator of recognizers that should be
    /// tried in order.
    fn recognizers(
        &self,
        state_index: StateIndex,
    ) -> RecognizerIterator<Self::Recognizer>;
}

pub struct RecognizerIterator<R: 'static> {
    pub terminals: &'static [TerminalInfo],
    pub terminals_for_state: &'static [Option<usize>],
    pub recognizers: &'static [R],
    pub index: usize,
}

impl<R> Iterator for RecognizerIterator<R> {
    type Item = (&'static R, &'static TerminalInfo);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.terminals_for_state.len() {
            match self.terminals_for_state[self.index] {
                Some(term_idx) => {
                    self.index += 1;
                    Some((
                        &self.recognizers[term_idx],
                        &self.terminals[term_idx],
                    ))
                }
                None => None,
            }
        } else {
            None
        }
    }
}
