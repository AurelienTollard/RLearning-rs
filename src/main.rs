use agent::Agent;
use random_agent::RandomAgent;

pub mod agent;
pub mod q_agent;
pub mod random_agent;

fn main() {
    let mut agent = RandomAgent::<u32, u32>::new(vec![1, 2, 3, 4]);

    for _ in 0..100 {
        println!("{}", agent.choose_action());
    }
}
