use crate::game::{Trust, Player, Stacks};
use crate::objects::Card;

pub struct Naive {
    increasing: bool,
    hand: Vec<Trust<Card>>,
}

impl Naive {
    pub fn new(increasing: bool) -> Self {
        Self {
            increasing,
            hand: Vec::new(),
        }
    }
}

impl Player for Naive {
    fn receive_card(&mut self, c: Trust<Card>) {
        self.hand.push(c);
    }

    fn preprocess(&mut self) {
        self.hand.sort();
        if !self.increasing {
            self.hand.reverse();
        }
    }

    fn play_card(&mut self, _: &Stacks) -> Trust<Card> {
        self.hand.pop().unwrap()
    }

    fn resolve_underflow(&mut self, stacks: &Stacks, _: &[(Card, usize)]) -> usize {
        stacks.iter().enumerate().min_by_key(|(_, stk)| stk.points()).unwrap().0
    }
}
