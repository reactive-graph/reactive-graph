use std::sync::Arc;
use std::sync::RwLock;

use crate::di::profiles;
use crate::di::Provider;

use crate::application::Application;
use crate::di_container_get;

pub fn init_application() -> Arc<dyn Application> {
    let mut container = di_container_get::<profiles::Default>();
    let container = &mut container;
    let application = Provider::<dyn Application>::create(container);
    Arc::new(application)
}

pub fn rw_application() -> Arc<RwLock<dyn Application>> {
    let mut container = di_container_get::<profiles::Default>();
    let container = &mut container;
    let application = Provider::<dyn Application>::create(container);
    Arc::new(RwLock::new(application))
}
