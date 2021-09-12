use actix_web_static_files::ResourceFiles;

pub trait WebResourceProvider: Send + Sync {
    fn get_web_resources(&self) -> ResourceFiles;
}
