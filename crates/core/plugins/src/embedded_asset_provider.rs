#[macro_export]
macro_rules! embedded_asset_provider_impl {
    ($asset: ident, $ty: ident) => {{
        let mut entries = Vec::new();
        for file in $asset::iter() {
            let filename = file.as_ref();
            log::debug!("Loading resource {}", filename);
            match $asset::get(filename) {
                Some(asset) => match std::str::from_utf8(asset.data.as_ref()) {
                    Ok(json_str) => match serde_json::from_str(json_str) {
                        Ok(parsed_entry) => {
                            let entry: $ty = parsed_entry;
                            entries.push(entry);
                        }
                        Err(e) => log::error!("Error in parsing JSON file {}: {}", filename, e),
                    },
                    Err(e) => log::error!("Error in decoding file to UTF-8 {}: {}", filename, e),
                },
                None => {}
            }
        }
        entries
    }};
}
