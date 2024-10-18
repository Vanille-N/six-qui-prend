use crate::objects::{Card, Stack, Points};
use rand::{thread_rng, seq::SliceRandom};

pub const NB_STK: usize = 4;
pub const HAND_SZ: usize = 10;
pub const NB_CARDS: u8 = 104;
pub const MAX_PLAYERS: usize = 10;

pub type Stacks = [Stack; NB_STK];

// NOT COPY OR CLONE
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trust<T>(T);

impl<T> Trust<T> {
    pub fn read(&self) -> &T {
        &self.0
    }
}

pub trait Player {
    fn receive_card(&mut self, c: Trust<Card>);
    fn preprocess(&mut self);
    fn play_card(&mut self, stacks: &Stacks) -> Trust<Card>;
    fn resolve_underflow(&mut self, stacks: &Stacks, cards: &[(Card, usize)]) -> usize;
}

pub struct Game {
    stacks: Stacks,
    deck: Vec<Card>,
    players: Vec<Box<dyn Player>>,
    scores: Vec<Points>,
}

impl Game {
    pub fn new() -> Self {
        let mut stacks = [Stack::new(), Stack::new(), Stack::new(), Stack::new()];
        let mut deck = (1..=NB_CARDS).map(|v| Card::new(v)).collect::<Vec<_>>();
        deck.shuffle(&mut thread_rng());
        for i in 0..NB_STK {
            stacks[i].push(deck.pop().unwrap());
        }
        Self {
            stacks,
            deck,
            players: Vec::new(),
            scores: Vec::new(),
        }
    }

    pub fn add_player(&mut self, mut p: Box<dyn Player>) {
        assert!(self.players.len() < MAX_PLAYERS);
        for _ in 0..HAND_SZ {
            p.receive_card(Trust(self.deck.pop().unwrap()));
        }
        p.preprocess();
        self.players.push(p);
        self.scores.push(Points::null());
    }

    fn round(&mut self) {
        let mut played = Vec::new();
        for (idx, player) in self.players.iter_mut().enumerate() {
            let Trust(card) = player.play_card(&self.stacks);
            played.push((card, idx));
        }
        played.sort_by_key(|&(card, _)| card);
        for &(card, played_by) in &played {
            let compat = self.stacks.iter_mut().filter(|stk| stk.top() < card)
                .max_by_key(|stk| stk.top());
            let chosen = match compat {
                None => {
                    let chosen = self.players[played_by].resolve_underflow(&self.stacks, &played);
                    &mut self.stacks[chosen.max(0).min(NB_STK)]
                },
                Some(stk) => stk,
            };
            let score = chosen.push(card);
            self.scores[played_by] += score;
        }
    }

    pub fn run(mut self) -> Vec<Points> {
        for _ in 0..HAND_SZ {
            self.round();
        }
        self.scores
    }
}
