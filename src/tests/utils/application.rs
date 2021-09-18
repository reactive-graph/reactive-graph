use std::sync::{Arc, RwLock};

use waiter_di::{profiles, Provider};

use crate::application::Application;
use crate::di::di_container;

pub fn init_application() -> Arc<dyn Application> {
    let mut container = di_container::get::<profiles::Default>();
    let container = &mut container;
    let application = Provider::<dyn Application>::create(container);
    Arc::new(application)
}

pub fn rw_application() -> Arc<RwLock<dyn Application>> {
    let mut container = di_container::get::<profiles::Default>();
    let container = &mut container;
    let application = Provider::<dyn Application>::create(container);
    Arc::new(RwLock::new(application))
}
