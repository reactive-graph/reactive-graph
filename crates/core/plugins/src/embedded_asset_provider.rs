#[macro_export]
macro_rules! embedded_asset_provider_impl {
    // TODO: reduce boilerplate somehow
    ($asset: ident, $ty: ident) => {{
        let mut entries = Vec::new();
        for file in $asset::iter() {
            let filename = file.as_ref();
            if filename.starts_with(".") {
                // do nothing
                continue;
            } else if !filename.ends_with(".json") {
                log::error!("Can't read type definition {}: Only JSON is supported.", filename);
                continue;
            }
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
    ($asset: ident, $ty: ident, json5) => {{
        let mut entries = Vec::new();
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
                                    let entry: $ty = parsed_entry;
                                    entries.push(entry);
                                }
                                Err(e) => log::error!("Error in parsing JSON file {}: {}", filename, e),
                            }
                        } else if filename.ends_with(".json5") {
                            match json5::from_str(asset_str) {
                                Ok(parsed_entry) => {
                                    let entry: $ty = parsed_entry;
                                    entries.push(entry);
                                }
                                Err(e) => log::error!("Error in parsing JSON5 file {}: {}", filename, e),
                            }
                        } else {
                            log::error!("Can't read type definition {}: Only JSON and JSON5 are supported.", filename);
                        }
                    }
                    Err(e) => log::error!("Error in decoding file to UTF-8 {}: {}", filename, e),
                },
                None => {}
            }
        }
        entries
    }};
    ($asset: ident, $ty: ident, toml) => {{
        let mut entries = Vec::new();
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
                                    let entry: $ty = parsed_entry;
                                    entries.push(entry);
                                }
                                Err(e) => log::error!("Error in parsing JSON file {}: {}", filename, e),
                            }
                        } else if filename.ends_with(".toml") {
                            match toml::from_str(asset_str) {
                                Ok(parsed_entry) => {
                                    let entry: $ty = parsed_entry;
                                    entries.push(entry);
                                }
                                Err(e) => log::error!("Error in parsing TOML file {}: {}", filename, e),
                            }
                        } else {
                            log::error!("Can't read type definition {}: Only JSON and TOML are supported.", filename);
                        }
                    }
                    Err(e) => log::error!("Error in decoding file to UTF-8 {}: {}", filename, e),
                },
                None => {}
            }
        }
        entries
    }};
    ($asset: ident, $ty: ident, json5, toml) => {{
        let mut entries = Vec::new();
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
                                    let entry: $ty = parsed_entry;
                                    entries.push(entry);
                                }
                                Err(e) => log::error!("Error in parsing JSON file {}: {}", filename, e),
                            }
                        } else if filename.ends_with(".json5") {
                            match json5::from_str(asset_str) {
                                Ok(parsed_entry) => {
                                    let entry: $ty = parsed_entry;
                                    entries.push(entry);
                                }
                                Err(e) => log::error!("Error in parsing JSON5 file {}: {}", filename, e),
                            }
                        } else if filename.ends_with(".toml") {
                            match toml::from_str(asset_str) {
                                Ok(parsed_entry) => {
                                    let entry: $ty = parsed_entry;
                                    entries.push(entry);
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
