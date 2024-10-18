use crate::game::{Game, Player};
use crate::objects::Points;

pub struct Supervisor {
    games: Vec<Game>,
    scores: Vec<Points>,
}

impl Supervisor {
    pub fn repeat(sz: usize) -> Self {
        let mut games = Vec::new();
        for _ in 0..sz {
            games.push(Game::new());
        }
        Self {
            games,
            scores: Vec::new(),
        }
    }

    pub fn add_player<PGen>(&mut self, gen: PGen)
    where PGen: Fn() -> Box<dyn Player>
    {
        for game in &mut self.games {
            game.add_player(gen());
        }
        self.scores.push(Points::null());
    }

    pub fn run(mut self) -> Vec<Points> {
        while let Some(game) = self.games.pop() {
            let pts = game.run();
            for (i, pt) in pts.into_iter().enumerate() {
                self.scores[i] += pt;
            }
        }
        self.scores
    }
}
