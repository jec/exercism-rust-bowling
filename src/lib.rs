#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

trait Frame {
    fn roll(&mut self, pins: u16) -> Result<(), Error>;
    fn score(&self) -> Option<u16>;
}

mod regular {
    #[derive(Clone, Debug, Eq, PartialEq)]
    enum Status {
        Pending,
        Open,
        Strike,
        Spare,
        Closed,
    }

    use Status::*;
    use crate::{Error, Frame};

    pub struct RegularFrame<'a> {
        status: Status,
        frame_number: u16,
        previous_frame: Option<&'a dyn Frame>,
    }

    impl RegularFrame<'_> {
        pub fn new() -> Self {
            RegularFrame { status: Pending, frame_number: 1, previous_frame: None }
        }
    }

    impl Frame for RegularFrame<'_> {
        fn roll(&mut self, pins: u16) -> Result<(), Error> {
            match self {
                Self { status: Pending, .. } => {
                    self.status = Open;
                    Ok(())
                },
                Self { .. } => {
                    Ok(())
                }
            }
        }

        fn score(&self) -> Option<u16> {
            match self {
                Self { status: Closed, .. } => Some(0),
                Self { .. } => None
            }
        }
    }
}

mod tenth {
    #[derive(Clone, Debug, Eq, PartialEq)]
    enum Status {
        Pending,
        RollTwo,
        RollTwoWithStrike,
        RollThree,
        RollThreeWithBonus,
        Closed,
    }

    use Status::*;
    use crate::{Error, Frame};

    pub struct TenthFrame<'a> {
        status: Status,
        previous_frame: Option<&'a dyn Frame>,
    }

    impl TenthFrame<'_> {
        pub fn new() -> Self {
            TenthFrame { status: Pending, previous_frame: None }
        }
    }

    impl Frame for TenthFrame<'_> {
        fn roll(&mut self, pins: u16) -> Result<(), Error> {
            match self {
                Self { status: Pending, .. } => {
                    self.status = RollTwo;
                    Ok(())
                },
                Self { .. } => {
                    Ok(())
                }
            }
        }

        fn score(&self) -> Option<u16> {
            match self {
                Self { status: Closed, .. } => Some(0),
                Self { .. } => None
            }
        }
    }
}

use regular::RegularFrame;

pub struct BowlingGame<'a> {
    frame: &'a mut dyn Frame
}

impl BowlingGame<'_> {
    pub fn new() -> Self {
        Self { frame: &RegularFrame::new() }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft)
        }

        self.frame.roll(pins)
    }

    pub fn score(&self) -> Option<u16> {
        self.frame.score()
    }
}
