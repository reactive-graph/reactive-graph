use inexor_rgf_reactive_model_api::ReactiveInstance;
use inexor_rgf_reactive_model_api::ReactiveInstanceContainer;

use crate::BehaviourConnectFailed;
use crate::BehaviourDisconnectFailed;
use crate::BehaviourInitializationFailed;
use crate::BehaviourReconnectFailed;
use crate::BehaviourShutdownFailed;
use crate::BehaviourTypesContainer;

#[allow(drop_bounds)]
pub trait BehaviourTransitions<ID: Clone, T: ReactiveInstance<ID> + BehaviourTypesContainer>:
    ReactiveInstanceContainer<ID, T> + BehaviourInit<ID, T> + BehaviourShutdown<ID, T> + BehaviourConnect<ID, T> + BehaviourDisconnect<ID, T> + Drop
{
    /// Reconnects the reactive streams.
    fn reconnect(&self) -> Result<(), BehaviourReconnectFailed> {
        self.disconnect().map_err(BehaviourReconnectFailed::BehaviourDisconnectFailed)?;
        self.connect().map_err(BehaviourReconnectFailed::BehaviourConnectFailed)?;
        Ok(())
    }
}

pub trait BehaviourInit<ID: Clone, T: ReactiveInstance<ID>> {
    /// Initializes the behaviour. For example, calculates and propagates the initial value.
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        Ok(())
    }
}

pub trait BehaviourShutdown<ID: Clone, T: ReactiveInstance<ID>> {
    /// Destructs the behaviour.
    fn shutdown(&self) -> Result<(), BehaviourShutdownFailed> {
        Ok(())
    }
}

pub trait BehaviourConnect<ID: Clone, T: ReactiveInstance<ID>> {
    /// Connects the reactive streams.
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        Ok(())
    }
}

pub trait BehaviourDisconnect<ID: Clone, T: ReactiveInstance<ID>> {
    /// Disconnects the reactive streams.
    fn disconnect(&self) -> Result<(), BehaviourDisconnectFailed> {
        Ok(())
    }
}
