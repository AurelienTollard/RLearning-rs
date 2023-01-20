use rand::Rng;

use super::agent::Agent;
use std::hash::Hash;

pub struct RandomAgent<S: Eq + Hash, A> {
    current_state: Option<S>,
    action_vec: Vec<A>,
    reward: f64,
}

impl<S: Eq + Hash, A> RandomAgent<S, A> {
    /// Creates a new random agent that performs actions from the given space
    pub fn new(actions: Vec<A>) -> RandomAgent<S, A> {
        RandomAgent {
            current_state: None,
            action_vec: actions,
            reward: 0.0,
        }
    }
}

impl<S: Eq + Hash, A> Agent<S, A> for RandomAgent<S, A> {
    fn get_state(&self) -> Option<&S> {
        self.current_state.as_ref()
    }

    fn choose_action(&mut self) -> &A {
        &self.action_vec[rand::thread_rng().gen_range(0..self.action_vec.len())]
    }

    fn update_reward(&mut self, reward: f64) {
        self.reward = reward;
    }

    fn set_new_state(&mut self, state: S) {
        self.current_state = Some(state);
    }
}
