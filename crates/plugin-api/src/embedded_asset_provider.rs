// TODO: remove this macro once the migration has been done

#[macro_export]
macro_rules! embedded_asset_provider_impl {
    ($asset: ident, $tys: ident) => {{
        let mut entries = <$tys as inexor_rgf_graph::NamespacedTypeContainer>::new();
        for file in $asset::iter() {
            let filename = file.as_ref();
            if filename.starts_with(".") {
                // do nothing
                continue;
            }
            log::debug!("Loading resource {}", filename);
            match $asset::get(filename) {
                Some(asset) => match std::str::from_utf8(asset.data.as_ref()) {
                    Ok(asset_str) => {
                        if filename.ends_with(".json") {
                            match serde_json::from_str(asset_str) {
                                Ok(parsed_entry) => {
                                    let entry: <$tys as inexor_rgf_graph::NamespacedTypeContainer>::Type = parsed_entry;
                                    inexor_rgf_graph::NamespacedTypeContainer::push(&entries, entry);
                                }
                                Err(e) => log::error!("Error in parsing JSON file {}: {}", filename, e),
                            }
                        } else if filename.ends_with(".json5") {
                            match json5::from_str(asset_str) {
                                Ok(parsed_entry) => {
                                    let entry: <$tys as inexor_rgf_graph::NamespacedTypeContainer>::Type = parsed_entry;
                                    inexor_rgf_graph::NamespacedTypeContainer::push(&entries, entry);
                                }
                                Err(e) => log::error!("Error in parsing JSON5 file {}: {}", filename, e),
                            }
                        } else if filename.ends_with(".toml") {
                            match toml::from_str(asset_str) {
                                Ok(parsed_entry) => {
                                    let entry: <$tys as inexor_rgf_graph::NamespacedTypeContainer>::Type = parsed_entry;
                                    inexor_rgf_graph::NamespacedTypeContainer::push(&entries, entry);
                                }
                                Err(e) => log::error!("Error in parsing TOML file {}: {}", filename, e),
                            }
                        } else {
                            log::error!("Can't read type definition {}: Only JSON, JSON5 and TOML are supported.", filename);
                        }
                    }
                    Err(e) => log::error!("Error in decoding file to UTF-8 {}: {}", filename, e),
                },
                None => {}
            }
        }
        entries
    }};
}
