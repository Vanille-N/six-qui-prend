use crate::game::{Player, Stacks, Trust};
use crate::objects::Card;

enum Strat {
    Increasing,
    Decreasing,
    Random,
}

pub struct Naive {
    strat: Strat,
    hand: Vec<Trust<Card>>,
}

impl Naive {
    fn new(strat: Strat) -> Self {
        Self {
            strat,
            hand: Vec::new(),
        }
    }

    pub fn new_increasing() -> Self {
        Self::new(Strat::Increasing)
    }

    pub fn new_decreasing() -> Self {
        Self::new(Strat::Decreasing)
    }

    pub fn new_random() -> Self {
        Self::new(Strat::Random)
    }
}

impl Player for Naive {
    fn receive_card(&mut self, c: Trust<Card>) {
        self.hand.push(c);
    }

    fn preprocess(&mut self) {
        match self.strat {
            Strat::Increasing => {
                self.hand.sort();
                self.hand.reverse();
            }
            Strat::Decreasing => {
                self.hand.sort();
            }
            Strat::Random => {}
        }
    }

    fn play_card(&mut self, _: &Stacks) -> Trust<Card> {
        self.hand.pop().unwrap()
    }

    fn resolve_underflow(&mut self, stacks: &Stacks, _: &[(Card, usize)]) -> usize {
        stacks
            .iter()
            .enumerate()
            .min_by_key(|(_, stk)| stk.points())
            .unwrap()
            .0
    }
}
