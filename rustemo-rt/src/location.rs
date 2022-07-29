use std::fmt::Display;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct LineBased {
    pub line: usize,
    pub column: usize,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Position {
    Position(usize),
    LineBased(LineBased),
}

impl Position {
    pub fn from_lc(line: usize, column: usize) -> Self {
        Self::LineBased(LineBased { line, column })
    }

    pub fn from_pos(pos: usize) -> Self {
        Self::Position(pos)
    }

    #[inline]
    pub fn line(&self) -> usize {
        match self {
            Position::Position(pos) => *pos,
            Position::LineBased(lb) => lb.line,
        }
    }

    #[inline]
    pub fn column(&self) -> usize {
        match self {
            Position::Position(_) => 0,
            Position::LineBased(lb) => lb.column,
        }
    }

    #[inline]
    pub fn position(&self) -> usize {
        match self {
            Position::Position(pos) => *pos,
            Position::LineBased(lb) => lb.line,
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Position::Position(pos) => write!(f, "{pos}"),
            Position::LineBased(lb) => write!(f, "{}:{}", lb.line, lb.column),
        }
    }
}

/// `Location` describes a span from start till end in the parsed input.
///
/// Start is mandatory while the end is not.
///
/// The location doesn't keep the path of the parsed file on purpose as it will
/// be the same for the single parse tree so it would either unnccessary waste
/// memory, in case of owned strings, or propagate lifetime information throught
/// the API in case of borrowed string slice.
///
/// The path is kept on the parsing context and there is the method on the
/// context to produce the display of the location with the full file path.
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Location {
    /// The start position of the range.
    pub start: Position,
    /// The end position. Sometimes it is not known or relevant.
    pub end: Option<Position>,
}

impl Location {
    pub fn new(start: Position, end: Position) -> Self {
        Self {
            start,
            end: Some(end),
        }
    }
    pub fn from_start(start: Position) -> Self {
        Self { start, end: None }
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.end {
            Some(ref end) => write!(f, "{}-{}", self.start, end),
            None => write!(f, "{}", self.start),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Location, Position};

    #[test]
    pub fn test_position_linebased() {
        let p = Position::from_lc(2, 4);

        assert_eq!(p.line(), 2);
        assert_eq!(p.column(), 4);
        assert_eq!(format!("{}", p), "2:4");
    }

    #[test]
    pub fn test_position() {
        let p = Position::from_pos(5);

        assert_eq!(p.line(), 5);
        assert_eq!(p.column(), 0);
        assert_eq!(p.position(), 5);
        assert_eq!(format!("{}", p), "5");
    }

    #[test]
    pub fn test_location() {
        let r =
            Location::new(Position::from_lc(5, 15), Position::from_lc(13, 27));

        assert_eq!(format!("{}", r), "5:15-13:27");

        let r = Location::new(Position::from_lc(5, 15), Position::from_pos(49));

        assert_eq!(format!("{}", r), "5:15-49");
    }
}
