#[macro_export]
macro_rules! properties {
    (
        /// The ident of the properties model.
        $properties: ident
        $(,
            (
                /// The ident of the property.
                $property_ident: ident,

                /// The name for serialization.
                $property_name: expr,

                /// The default value.
                $property_default_value: expr
            )
        )*
    ) => {
        #[allow(non_camel_case_types)]
        #[derive(strum_macros::AsRefStr, strum_macros::IntoStaticStr, strum_macros::Display)]
        pub enum $properties {
            $(
                #[strum(serialize = $property_name)]
                $property_ident,
            )*
        }

        impl $properties {
            pub fn default_value(&self) -> serde_json::Value {
                match self {
                    $(
                        $properties::$property_ident => serde_json::json!($property_default_value),
                    )*
                }
            }
            pub fn properties() -> Vec<indradb::NamedProperty> {
                vec![
                    $(
                        indradb::NamedProperty::from($properties::$property_ident),
                    )*
                ]
            }
        }

        impl From<$properties> for indradb::NamedProperty {
            fn from(p: $properties) -> Self {
                indradb::NamedProperty {
                    name: indradb::Identifier::new(p.to_string()).unwrap(),
                    value: p.default_value(),
                }
            }
        }

        impl From<$properties> for String {
            fn from(p: $properties) -> Self {
                p.to_string()
            }
        }
    };
}
