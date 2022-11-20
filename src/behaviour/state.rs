/// The state of a behaviour.
#[derive(Debug, Clone, Copy)]
pub enum BehaviourState {
    Created,
    Valid,
    Ready,
    Connected,
}
