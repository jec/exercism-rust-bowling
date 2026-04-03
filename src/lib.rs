#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    frame: Box<dyn Frame>,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            frame: Box::new(FirstPendingFrame),
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        self.frame = self.frame.roll(pins)?;
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        self.frame.score()
    }
}

#[derive(Debug)]
enum Multiplier {
    One,
    Two,
    Three,
    TwoTwo,
    ThreeTwo,
}

impl Multiplier {
    fn multiplier(&self) -> u16 {
        match self {
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 3,
            Self::TwoTwo => 2,
            Self::ThreeTwo => 3,
        }
    }

    fn strike(&self) -> Self {
        match self {
            Self::One => Self::TwoTwo,
            Self::Two => Self::TwoTwo,
            Self::Three => Self::TwoTwo,
            Self::TwoTwo => Self::ThreeTwo,
            Self::ThreeTwo => Self::ThreeTwo,
        }
    }

    fn spare(&self) -> Self {
        match self {
            Self::One => Self::Two,
            Self::Two => Self::Two,
            Self::Three => Self::Two,
            Self::TwoTwo => Self::Three,
            Self::ThreeTwo => Self::Three,
        }
    }

    fn open(&self) -> Self {
        match self {
            Self::One => Self::One,
            Self::Two => Self::One,
            Self::Three => Self::One,
            Self::TwoTwo => Self::Two,
            Self::ThreeTwo => Self::Two,
        }
    }

    fn calculate_score(&self, roll: u16, pins: u16, score: u16) -> (u16, Multiplier) {
        if roll == pins {
            if pins == 10 {
                (score + roll * self.multiplier(), self.strike())
            } else {
                (score + roll * self.multiplier(), self.spare())
            }
        } else {
            (score + roll * self.multiplier(), self.open())
        }
    }
}

trait Frame {
    fn roll(&mut self, pins: u16) -> Result<Box<dyn Frame>, Error>;
    fn score(&self) -> Option<u16>;
}

/// First frame with no rolls in game
#[derive(Debug)]
struct FirstPendingFrame;

/// A frame with no rolls; first roll pending
#[derive(Debug)]
struct PendingFrame {
    frame_number: u16,
    score: u16,
    bonuses: Multiplier,
}

/// A frame with one roll that was not a strike; second roll pending
#[derive(Debug)]
struct OpenFrame {
    frame_number: u16,
    pins: u16,
    score: u16,
    bonuses: Multiplier,
}

/// A tenth frame with no rolls; first roll pending
#[derive(Debug)]
struct TenthPendingFrame {
    score: u16,
    bonuses: Multiplier,
}

/// A tenth frame with one roll that was not a strike; second roll pending
#[derive(Debug)]
struct TenthOpenFrame {
    pins: u16,
    score: u16,
    bonuses: Multiplier,
}

/// A tenth frame with one roll that was a strike; second and third rolls pending
#[derive(Debug)]
struct TenthStrikeFrame {
    score: u16,
    bonuses: Multiplier,
}

/// A tenth frame with two previous rolls; third and final roll pending
#[derive(Debug)]
struct TenthFinalFrame {
    pins: u16,
    score: u16,
}

/// A tenth frame that has been completed with either two or three rolls; no more rolls allowed
#[derive(Debug)]
struct TenthClosedFrame {
    score: u16,
}

impl Frame for FirstPendingFrame {
    fn roll(&mut self, pins: u16) -> Result<Box<dyn Frame>, Error> {
        println!("roll({:?}, {})", self, pins);

        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        if pins == 10 {
            Ok(Box::new(PendingFrame {
                frame_number: 2,
                score: 10,
                bonuses: Multiplier::TwoTwo,
            }))
        } else {
            Ok(Box::new(OpenFrame {
                frame_number: 1,
                pins: 10 - pins,
                score: pins,
                bonuses: Multiplier::One,
            }))
        }
    }

    fn score(&self) -> Option<u16> {
        None
    }
}

impl Frame for PendingFrame {
    fn roll(&mut self, pins: u16) -> Result<Box<dyn Frame>, Error> {
        println!("roll({:?}, {})", self, pins);

        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        let (score, bonuses) = self.bonuses.calculate_score(pins, 10, self.score);

        if pins == 10 {
            if self.frame_number == 9 {
                Ok(Box::new(TenthPendingFrame { score, bonuses }))
            } else {
                Ok(Box::new(PendingFrame {
                    frame_number: self.frame_number + 1,
                    score,
                    bonuses,
                }))
            }
        } else {
            Ok(Box::new(OpenFrame {
                frame_number: self.frame_number,
                pins: 10 - pins,
                score,
                bonuses,
            }))
        }
    }

    fn score(&self) -> Option<u16> {
        None
    }
}

impl Frame for OpenFrame {
    fn roll(&mut self, pins: u16) -> Result<Box<dyn Frame>, Error> {
        println!("roll({:?}, {})", self, pins);

        if pins > self.pins {
            return Err(Error::NotEnoughPinsLeft);
        }

        let (score, bonuses) = self.bonuses.calculate_score(pins, self.pins, self.score);

        if pins == self.pins {
            if self.frame_number == 9 {
                Ok(Box::new(TenthPendingFrame { score, bonuses }))
            } else {
                Ok(Box::new(PendingFrame {
                    frame_number: self.frame_number + 1,
                    score,
                    bonuses,
                }))
            }
        } else if self.frame_number == 9 {
            Ok(Box::new(TenthPendingFrame { score, bonuses }))
        } else {
            Ok(Box::new(PendingFrame {
                frame_number: self.frame_number + 1,
                score,
                bonuses,
            }))
        }
    }

    fn score(&self) -> Option<u16> {
        None
    }
}

impl Frame for TenthPendingFrame {
    fn roll(&mut self, pins: u16) -> Result<Box<dyn Frame>, Error> {
        println!("roll({:?}, {})", self, pins);

        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        let score = self.score + pins * self.bonuses.multiplier();

        // Give no further bonus scoring for this roll, regardless of whether it's a strike.
        let bonuses = self.bonuses.open();

        if pins == 10 {
            Ok(Box::new(TenthStrikeFrame { score, bonuses }))
        } else {
            Ok(Box::new(TenthOpenFrame {
                pins: 10 - pins,
                score,
                bonuses,
            }))
        }
    }

    fn score(&self) -> Option<u16> {
        None
    }
}

impl Frame for TenthStrikeFrame {
    fn roll(&mut self, pins: u16) -> Result<Box<dyn Frame>, Error> {
        println!("roll({:?}, {})", self, pins);

        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        // No bonuses carry over to next roll.
        let (score, _bonuses) = self.bonuses.calculate_score(pins, 10, self.score);

        if pins == 10 {
            Ok(Box::new(TenthFinalFrame { pins: 10, score }))
        } else {
            Ok(Box::new(TenthFinalFrame {
                pins: 10 - pins,
                score,
            }))
        }
    }

    fn score(&self) -> Option<u16> {
        None
    }
}

impl Frame for TenthOpenFrame {
    fn roll(&mut self, pins: u16) -> Result<Box<dyn Frame>, Error> {
        println!("roll({:?}, {})", self, pins);

        if pins > self.pins {
            return Err(Error::NotEnoughPinsLeft);
        }

        // No bonuses carry over to next roll.
        let (score, _bonuses) = self.bonuses.calculate_score(pins, 10, self.score);

        if pins == self.pins {
            Ok(Box::new(TenthFinalFrame { pins: 10, score }))
        } else {
            Ok(Box::new(TenthClosedFrame { score }))
        }
    }

    fn score(&self) -> Option<u16> {
        None
    }
}

impl Frame for TenthFinalFrame {
    fn roll(&mut self, pins: u16) -> Result<Box<dyn Frame>, Error> {
        println!("roll({:?}, {})", self, pins);

        if pins > self.pins {
            return Err(Error::NotEnoughPinsLeft);
        }

        Ok(Box::new(TenthClosedFrame {
            score: self.score + pins,
        }))
    }

    fn score(&self) -> Option<u16> {
        None
    }
}

impl Frame for TenthClosedFrame {
    fn roll(&mut self, _pins: u16) -> Result<Box<dyn Frame>, Error> {
        println!("roll({:?}, {})", self, _pins);

        Err(Error::GameComplete)
    }

    fn score(&self) -> Option<u16> {
        Some(self.score)
    }
}
