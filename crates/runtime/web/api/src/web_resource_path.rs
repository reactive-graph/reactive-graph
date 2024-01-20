use serde::Deserialize;

#[derive(Deserialize)]
pub struct WebResourcePathInfo {
    pub web_resource_context_path: String,
    pub path: String,
}

#[derive(Deserialize)]
pub struct RootPathInfo {
    pub path: String,
}
