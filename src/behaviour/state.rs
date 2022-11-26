/// The state of a behaviour.
#[derive(Debug, Clone, Copy)]
pub enum BehaviourState {
    /// The behaviour has been created.
    Created,

    /// The behaviour has been validated.
    Valid,

    /// The behaviour is initialized but not connected.
    Ready,

    /// The behaviour is connected.
    Connected,
}
