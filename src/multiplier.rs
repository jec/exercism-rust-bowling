#[derive(Debug)]
pub enum Multiplier {
    One,
    Two,
    Three,
    TwoTwo,
    ThreeTwo,
}

impl Multiplier {
    pub fn multiplier(&self) -> u16 {
        match self {
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 3,
            Self::TwoTwo => 2,
            Self::ThreeTwo => 3,
        }
    }

    pub fn strike(&self) -> Self {
        match self {
            Self::One => Self::TwoTwo,
            Self::Two => Self::TwoTwo,
            Self::Three => Self::TwoTwo,
            Self::TwoTwo => Self::ThreeTwo,
            Self::ThreeTwo => Self::ThreeTwo,
        }
    }

    pub fn spare(&self) -> Self {
        match self {
            Self::One => Self::Two,
            Self::Two => Self::Two,
            Self::Three => Self::Two,
            Self::TwoTwo => Self::Three,
            Self::ThreeTwo => Self::Three,
        }
    }

    pub fn open(&self) -> Self {
        match self {
            Self::One => Self::One,
            Self::Two => Self::One,
            Self::Three => Self::One,
            Self::TwoTwo => Self::Two,
            Self::ThreeTwo => Self::Two,
        }
    }

    pub fn calculate_score(&self, roll: u16, pins: u16, score: u16) -> (u16, Multiplier) {
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
