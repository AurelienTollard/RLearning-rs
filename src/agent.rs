/*
 S stand for state struct,
 A stand for action strcut
*/
pub trait Agent<S, A> {
    fn get_state(&self) -> Option<&S>;
    fn set_new_state(&mut self, state: S);
    fn choose_action(&mut self) -> &A;
    fn update_reward(&mut self, reward: f64);
}
