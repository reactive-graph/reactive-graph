use crate::model::ReactiveInstance;
use crate::BehaviourConnectFailed;
use crate::BehaviourDisconnectFailed;
use crate::BehaviourInitializationFailed;
use crate::BehaviourShutdownFailed;

#[allow(drop_bounds)]
pub trait BehaviourTransitions<T: ReactiveInstance>: Drop {
    /// Initializes the behaviour. For example, calculates and propagates the initial value.
    fn init(&self) -> Result<(), BehaviourInitializationFailed>;

    /// Connects the reactive streams.
    fn connect(&self) -> Result<(), BehaviourConnectFailed>;

    /// Disconnects the reactive streams.
    fn disconnect(&self) -> Result<(), BehaviourDisconnectFailed>;

    /// Destructs the behaviour.
    fn shutdown(&self) -> Result<(), BehaviourShutdownFailed>;
}
