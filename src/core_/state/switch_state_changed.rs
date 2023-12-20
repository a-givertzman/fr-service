#![allow(non_snake_case)]

use super::switch_state::SwitchState;

///
/// Returns true once if inner SwitchState is just changed the state
pub struct SwitchStateChanged<TState, TInput> {
    switchState: SwitchState<TState, TInput>,
    prev: TState,
}
///
/// 
impl<TState: std::fmt::Debug + Eq + core::hash::Hash + Clone, TInput: Clone> SwitchStateChanged<TState, TInput> {
    ///
    /// 
    pub fn new(switchState: SwitchState<TState, TInput>) -> Self {
        let prev = switchState.state();
        Self {
            switchState,
            prev,
        }
    }
    ///
    ///
    pub fn add(& mut self, value: TInput) {
        self.switchState.add(value)
    }
    ///
    pub fn state(&self) -> TState {
        self.switchState.state()
    }
    ///
    /// resets current state to initial
    pub fn reset(&mut self) {
        self.switchState.reset();
    }
    ///
    /// 
    pub fn changed(&mut self) -> bool {
        let changed = self.switchState.state() != self.prev;
        self.prev = self.switchState.state();
        changed
    }
}