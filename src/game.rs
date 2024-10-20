use crate::objects::{Card, Points, Stack};
use rand::{seq::SliceRandom, thread_rng};

pub const NB_STK: usize = 4;
pub const HAND_SZ: usize = 10;
pub const NB_CARDS: u8 = 104;
pub const MAX_PLAYERS: usize = 10;

pub type Stacks = [Stack; NB_STK];

macro_rules! ifdebug {
    ($($tk:tt)*) => {};
    ($($tk:tt)*) => { $($tk)* };
}

// NOT COPY OR CLONE
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trust<T>(T);

impl<T> Trust<T> {
    pub fn read(&self) -> &T {
        &self.0
    }
}

pub mod utils {
    use super::*;
    pub fn most_compat_stack(stacks: &Stacks, card: Card) -> Option<usize> {
        stacks
            .iter()
            .enumerate()
            .filter(|(_, stk)| stk.top() < card)
            .max_by_key(|(_, stk)| stk.top())
            .map(|(idx, _)| idx)
    }

    pub fn nb_stacks_clamp(idx: usize) -> usize {
        idx.max(0).min(NB_STK)
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
        ifdebug! {
            println!("New round begins");
            println!("Current stacks are:");
            for stk in &self.stacks {
                println!("- {}", stk);
            }
        };
        let mut played = Vec::new();
        for (idx, player) in self.players.iter_mut().enumerate() {
            let Trust(card) = player.play_card(&self.stacks);
            played.push((card, idx));
            ifdebug! {
                println!("Player {} chooses {}", idx, card);
            }
        }
        played.sort_by_key(|&(card, _)| card);
        for &(card, played_by) in &played {
            let idx = utils::most_compat_stack(&self.stacks, card)
                .unwrap_or_else(|| self.players[played_by].resolve_underflow(&self.stacks, &played));
            let score = self.stacks[utils::nb_stacks_clamp(idx)].push(card);
            ifdebug! {
                println!(
                    "Card {} is placed, earning {} points to player {}",
                    card, score, played_by
                );
            }
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
