use six_qui_prend::{clients::naive::Naive, game, supervisor::Supervisor};

const NB_GAMES: usize = 100000;

const NB_INCR: usize = 3;
const NB_RAND: usize = 3;
const NB_DECR: usize = 3;

fn main() {
    assert!(NB_INCR + NB_RAND + NB_DECR < game::MAX_PLAYERS);
    let mut supervisor = Supervisor::repeat(NB_GAMES);
    for _ in 0..NB_INCR {
        supervisor.add_player("incr", || Box::new(Naive::new_increasing()));
    }
    for _ in 0..NB_RAND {
        supervisor.add_player("rand", || Box::new(Naive::new_random()));
    }
    for _ in 0..NB_DECR {
        supervisor.add_player("decr", || Box::new(Naive::new_decreasing()));
    }
    let scores = supervisor.run();
    println!("Average of {} games", NB_GAMES);
    for (idx, (strat, score)) in scores.iter().enumerate() {
        println!(
            "  Player {} (using strategy '{}'): {}",
            idx,
            strat,
            score.average()
        );
    }
}
