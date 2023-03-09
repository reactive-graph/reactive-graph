use crate::model::ReactiveInstance;
use crate::BehaviourConnectFailed;
use crate::BehaviourDisconnectFailed;
use crate::BehaviourInitializationFailed;
use crate::BehaviourReactiveInstanceContainer;
use crate::BehaviourReconnectFailed;
use crate::BehaviourShutdownFailed;

#[allow(drop_bounds)]
pub trait BehaviourTransitions<T: ReactiveInstance>:
    BehaviourReactiveInstanceContainer<T> + BehaviourInit<T> + BehaviourShutdown<T> + BehaviourConnect<T> + BehaviourDisconnect<T> + Drop
{
    /// Reconnects the reactive streams.
    fn reconnect(&self) -> Result<(), BehaviourReconnectFailed> {
        self.disconnect().map_err(BehaviourReconnectFailed::BehaviourDisconnectFailed)?;
        self.connect().map_err(BehaviourReconnectFailed::BehaviourConnectFailed)?;
        Ok(())
    }
}

pub trait BehaviourInit<T: ReactiveInstance> {
    /// Initializes the behaviour. For example, calculates and propagates the initial value.
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        Ok(())
    }
}

pub trait BehaviourShutdown<T: ReactiveInstance> {
    /// Destructs the behaviour.
    fn shutdown(&self) -> Result<(), BehaviourShutdownFailed> {
        Ok(())
    }
}

pub trait BehaviourConnect<T: ReactiveInstance> {
    /// Connects the reactive streams.
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        Ok(())
    }
}

pub trait BehaviourDisconnect<T: ReactiveInstance> {
    /// Disconnects the reactive streams.
    fn disconnect(&self) -> Result<(), BehaviourDisconnectFailed> {
        Ok(())
    }
}
