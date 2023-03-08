use std::fmt::Display;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Segment {
    Horizontal,
    Vertical,
    UpAndLeft,
    UpAndRight,
    DownAndLeft,
    DownAndRight,
}

impl Segment {
    pub fn char(&self) -> char {
        match self {
            Self::Horizontal => '━',
            Self::Vertical => '┃',
            Self::UpAndLeft => '╯',
            Self::UpAndRight => '╰',
            Self::DownAndLeft => '╮',
            Self::DownAndRight => '╭',
        }
    }
}

impl Display for Segment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.char())
    }
}
