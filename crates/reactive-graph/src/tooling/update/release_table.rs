use self_update::update::Release;
use std::slice;
use tabled::settings::object::Rows;
use tabled::settings::Width;
use tabled::Table;
use tabled::Tabled;

#[derive(Clone, Debug, Tabled)]
pub struct ReleaseTable {
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

impl ReleaseTable {
    pub fn render_one(release: &Release) {
        Self::render(slice::from_ref(release));
    }

    pub fn render(releases: &[Release]) {
        Self::render_table(releases.iter().map(ReleaseTable::from).collect::<Vec<ReleaseTable>>());
    }

    pub fn render_table(releases: Vec<Self>) {
        let mut table = Table::new(releases);
        table.modify(Rows::new(1..), Width::wrap(40));
        println!("{}", table);
    }
}

impl From<&Release> for ReleaseTable {
    fn from(release: &Release) -> Self {
        let (asset_name, download_url) = match release.asset_for(env!("VERGEN_CARGO_TARGET_TRIPLE"), None) {
            Some(release_asset) => (release_asset.name, release_asset.download_url),
            None => ("Not available".to_string(), String::new()),
        };
        ReleaseTable {
            name: release.name.clone(),
            version: release.version.clone(),
            date: release.date[0..10].to_string(),
            asset_name,
            download_url,
        }
    }
}
