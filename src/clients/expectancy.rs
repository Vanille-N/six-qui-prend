use crate::game::{Player, Stacks, Trust};
use crate::objects::Card;

struct Basic {
    hand: Vec<Trust<Card>>,
}

struct Strat {
    idx: usize,
    proba_lose: f64,
    avg_points: f64,
}

impl Strat {
    fn default() -> Self {
        Self {
            idx: 0,
            proba_lose: 1.0,
            avg_points: 1000.0,
        }
    }

    fn score(&self) -> f64 {
        self.proba_lose * self.avg_points
    }

    fn keep_best(&mut self, other: Self) {
        if self.score() > other.score() {
            self.idx = other.idx;
            self.proba_lose = other.proba_lose;
            self.avg_points = other.avg_points;
        }
    }
}

impl Player for Basic {
    fn receive_card(&mut self, c: Trust<Card>) {
        self.hand.push(c);
    }
    fn preprocess(&mut self) {}
    fn play_card(&mut self, stacks: &Stacks) -> Trust<Card> {
        let mut best = Strat::default();
        for stack in stacks.iter() {
            for (idx, card) in self.hand.iter().enumerate() {}
        }
    }
    fn resolve_underflow(&mut self, stacks: &Stacks, cards: &[(Card, usize)]) -> usize {
        stacks
            .iter()
            .enumerate()
            .min_by_key(|(_, stk)| stk.points())
            .unwrap()
            .0
    }
}
