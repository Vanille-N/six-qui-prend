use six_qui_prend::{
    supervisor::Supervisor,
    clients::naive::Naive,
};

fn main() {
    let mut supervisor = Supervisor::repeat(1_000_000);
    for _ in 0..5 {
        supervisor.add_player(|| Box::new(Naive::new(true)));
    }
    for _ in 0..5 {
        supervisor.add_player(|| Box::new(Naive::new(false)));
    }
    let scores = supervisor.run();
    println!("{:?}", scores);
}
