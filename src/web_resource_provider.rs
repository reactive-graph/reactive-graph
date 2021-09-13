use actix_web_static_files::deps::static_files::Resource;
use std::collections::HashMap;

pub trait WebResourceProvider: Send + Sync {
    fn get_web_resources(&self) -> HashMap<&'static str, Resource>;
}
