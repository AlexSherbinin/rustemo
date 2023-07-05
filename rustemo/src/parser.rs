use std::path::Path;

use crate::{context::Context, error::Result, input::Input, lexer::Lexer};

pub trait Parser<'i, I, C, L, S, TK>
where
    I: Input + ?Sized,
    C: Context<'i, I, S, TK>,
    L: Lexer<'i, C, S, TK, Input = I>,
    S: State,
{
    type Output;

    /// Parse the given input and produce the result. The output type is set by
    /// the parser implementers and it is usually defined by the builder if the
    /// building is done during the parse process.
    fn parse(&self, input: &'i L::Input) -> Result<Self::Output>;

    /// Parse with the given context which has information about the current
    /// parsing state (e.g. position, location). Used in situation when we need
    /// to continue parsing from a specific state, like in parsing the layout
    /// from the current location.
    fn parse_with_context(
        &self,
        context: &mut C,
        input: &'i L::Input,
    ) -> Result<Self::Output>;

    /// A convenience method for loading the content from the given file and
    /// calling `parse`. The parser will own the content being parsed and thus
    /// has to outlive the output if the output is borrowed from the content
    /// loaded from the file.
    fn parse_file<'a, F: AsRef<Path>>(
        &'a mut self,
        file: F,
    ) -> Result<Self::Output>
    where
        'a: 'i;
}

/// This trait must be implemented by the parser state type.
pub trait State: Default + Copy {
    /// Returns the default layout state.
    fn default_layout() -> Option<Self>;
}
