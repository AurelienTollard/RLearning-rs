use q_agent::{QAgent, QParams};

pub mod agent;
pub mod q_agent;
pub mod random_agent;

#[derive(Clone, Eq, Hash, PartialEq)]
struct StateSpace {
    x: u32,
}

#[derive(Clone)]
struct Action {
    up: bool,
}

fn main() {
    let mut agent = QAgent::<StateSpace, Action>::new(
        vec![Action { up: true }, Action { up: false }],
        QParams {
            alpha: 0.1,
            gamma: 0.6
        },
    );

    let mut current_x: u32 = 0;
    let mut action: Action;
    let mut reward: f32 = 0.0;
    for _ in 0..100 {
        action = agent.qlearning_step(reward, StateSpace { x: current_x });
        current_x = (current_x + 1) % 5;
        reward = if action.up { 10.0 } else { -0.1 };
        println!("action up = {}", action.up);
    }
}
