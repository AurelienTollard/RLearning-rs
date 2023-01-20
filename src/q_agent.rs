use rand::Rng;

use super::agent::Agent;
use std::{collections::HashMap, hash::Hash};

pub struct QAgent<S: Eq + Hash, A> {
    q_table: HashMap<S, Vec<f32>>,
    current_state: Option<S>,
    previous_state: Option<S>,
    current_action: Option<A>,
    previous_action: Option<A>,
    previous_action_index: usize,
    action_vec: Vec<A>,
    reward: f64,
}

impl<S: Eq + Hash + Clone, A: Clone> Agent<S, A> for QAgent<S, A> {
    fn get_state(&self) -> Option<&S> {
        self.current_state.as_ref()
    }

    fn choose_action(&mut self) -> &A {
        let action_size = self.action_vec.len();

        // Pick best action based on current state
        let action_index: usize;
        if let Some(action_vec) = self.q_table.get(self.current_state.as_ref().unwrap()) {
            action_index = action_vec
                .iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.total_cmp(b))
                .map(|(index, _)| index)
                .unwrap();
        } else {
            // The state couldn't be found in the hashtable, we need to add it
            self.q_table.insert(self.current_state.as_ref().unwrap().clone(), vec![0.0; action_size]);
            action_index = rand::thread_rng().gen_range(0..action_size);
        }

        let action = &self.action_vec[action_index];

        if self.current_action.is_some() {
            self.previous_action = self.current_action.clone();
        }
        self.current_action = Some(action.clone());
        self.current_action.as_ref().unwrap()
    }

    fn update_reward(&mut self, reward: f64) {
        self.reward = reward;
    }

    fn set_new_state(&mut self, state: S) {
        if self.current_state.is_some() {
            self.previous_state = self.current_state.clone();
        }
        self.current_state = Some(state);
    }
}

impl<S: Eq + Hash + Clone, A: Clone> QAgent<S, A> {
    /// Creates a new random agent that performs actions from the given space
    pub fn new(actions: Vec<A>) -> QAgent<S, A> {
        QAgent {
            action_vec: actions,
            reward: 0.0,
            q_table: HashMap::new(),
            current_state: None,
            previous_state: None,
            current_action: None,
            previous_action: None,
            previous_action_index: 0,
        }
    }

    pub fn qlearning_step(&mut self, reward: f64, new_state: S) -> &A {
        self.set_new_state(new_state);
        self.update_reward(reward);
        self.update_qtable();
        self.choose_action()
    }

    pub fn update_qtable(&mut self) {
        self.q_table.get_mut(self.previous_state.as_ref().unwrap()).unwrap()[self.previous_action_index] = 0.0;
    }
}
