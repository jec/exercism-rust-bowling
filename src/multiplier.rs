/// Implements a state machine representing the bonus multiplier for the current
/// and subsequent rolls
///
/// When a player rolls a strike, the next two rolls get 2x multipliers. When a
/// player rolls a spare, the next roll gets a 2x. These can be thought of as an
/// array, with the first element in the array representing the multiplier
/// applied to the current roll and the second element (if present) applied to
/// the subsequent roll. These multipliers are additive, so that if the next
/// roll already has a 2x multiplier pending, and the current roll is a strike
/// or a spare, then the next roll's multiplier is incremented to 3x.
///
/// State table:
///
/// | ↓ Current state |          |        |      |
/// |         Input → | Strike   | Spare  | Open |
/// |    Next state ↘ |          |        |      |
/// |-----------------|----------|--------|------|
/// | One             | TwoTwo   | Two    | One  |
/// | Two             | TwoTwo   | Two    | One  |
/// | Three           | TwoTwo   | Two    | One  |
/// | TwoTwo          | ThreeTwo | Three* | Two  |
/// | ThreeTwo        | ThreeTwo | Three* | Two  |
///
/// * This appears impossible. Read the comment on the variant `Three` below.
#[derive(Debug, PartialEq)]
pub enum Multiplier {
    // [1] -- the current roll is counted just once; no bonuses are in effect
    One,

    // [2] -- the current roll is counted twice, e.g. the previous roll was a
    // spare
    Two,

    // [3] -- The state table theorizes that a spare with a current state of
    // TwoTwo or ThreeTwo would cause a transition to Three. However, this would
    // require a spare to immediately follow a strike, and that cannot happen.
    // (According to the table, a state of TwoTwo or ThreeTwo must have been
    // caused by a strike.)
    Three,

    // [2, 2] -- the current and subsequent rolls are both counted twice, e.g.
    // the previous roll was a strike
    TwoTwo,

    // [3, 2] -- the current roll is counted thrice and subsequent roll twice,
    // e.g. the previous two rolls were strikes
    ThreeTwo,
}

impl Multiplier {
    /// Returns the integer multiplier for the first (or only) half of the
    /// symbolic multiplier
    pub fn multiplier(&self) -> u16 {
        match self {
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 3,
            Self::TwoTwo => 2,
            Self::ThreeTwo => 3,
        }
    }

    /// Given the current state and a rolled strike, returns the next state
    pub fn strike(&self) -> Self {
        match self {
            Self::One => Self::TwoTwo,
            Self::Two => Self::TwoTwo,
            Self::Three => Self::TwoTwo,
            Self::TwoTwo => Self::ThreeTwo,
            Self::ThreeTwo => Self::ThreeTwo,
        }
    }

    /// Given the current state and a rolled spare, returns the next state
    pub fn spare(&self) -> Self {
        match self {
            Self::One => Self::Two,
            Self::Two => Self::Two,
            Self::Three => Self::Two,
            Self::TwoTwo => Self::Three,
            Self::ThreeTwo => Self::Three,
        }
    }

    /// Given the current state and a roll of neither strike nor spare, returns
    /// the next state
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
        let result = if roll == pins {
            if pins == 10 {
                (score + roll * self.multiplier(), self.strike())
            } else {
                (score + roll * self.multiplier(), self.spare())
            }
        } else {
            (score + roll * self.multiplier(), self.open())
        };

        // The state graph suggests that Three isn't possible. Let's test that.
        assert_ne!(result.1, Self::Three);

        result
    }
}
