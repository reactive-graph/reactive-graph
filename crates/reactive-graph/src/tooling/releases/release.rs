use serde::Serialize;
use tabled::Tabled;

pub const TARGET_TRIPLE: &str = env!("VERGEN_CARGO_TARGET_TRIPLE");

#[derive(Clone, Debug, Serialize, Tabled)]
pub struct Release {
    #[tabled(rename = "Release Name")]
    pub name: String,
    #[tabled(rename = "Version")]
    pub version: String,
    #[tabled(rename = "Date")]
    pub date: String,
    #[tabled(rename = "Asset Name")]
    pub asset_name: String,
    #[tabled(rename = "Download URL")]
    pub download_url: String,
}

impl From<&self_update::update::Release> for Release {
    fn from(release: &self_update::update::Release) -> Self {
        let (asset_name, download_url) = match release.asset_for(TARGET_TRIPLE, None) {
            Some(release_asset) => (release_asset.name, release_asset.download_url),
            None => ("Not available".to_string(), String::new()),
        };
        Release {
            name: release.name.clone(),
            version: release.version.clone(),
            date: release.date[0..10].to_string(),
            asset_name,
            download_url,
        }
    }
}
