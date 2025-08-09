#[macro_export]
macro_rules! entity_model {
    (
        $ident: ident
        $(,
            $accessor_type: tt
            $(
            $accessor_name: ident
            $accessor_data_type: tt
            )?
        )*
        $(,)?
    ) => {
        pub struct $ident(reactive_graph_reactive_model_impl::ReactiveEntity);

        impl $ident {
            $(
                reactive_graph_graph::rx_accessor!(pub $accessor_type $($accessor_name $accessor_data_type)?);
            )*
        }

        impl $crate::ReactiveInstanceGetter<reactive_graph_reactive_model_impl::ReactiveEntity> for $ident {
            fn get_reactive_instance(&self) -> &reactive_graph_reactive_model_impl::ReactiveEntity {
                &self.0
            }
        }

        impl From<reactive_graph_reactive_model_impl::ReactiveEntity> for $ident {
            fn from(reactive_entity: reactive_graph_reactive_model_impl::ReactiveEntity) -> Self {
                $ident(reactive_entity)
            }
        }

        impl reactive_graph_graph::PropertyInstanceGetter for $ident {
            fn get<S: Into<String>>(&self, property_name: S) -> Option<serde_json::Value> {
                self.0.get(property_name)
            }

            fn as_bool<S: Into<String>>(&self, property_name: S) -> Option<bool> {
                self.0.as_bool(property_name)
            }

            fn as_u64<S: Into<String>>(&self, property_name: S) -> Option<u64> {
                self.0.as_u64(property_name)
            }

            fn as_i64<S: Into<String>>(&self, property_name: S) -> Option<i64> {
                self.0.as_i64(property_name)
            }

            fn as_f64<S: Into<String>>(&self, property_name: S) -> Option<f64> {
                self.0.as_f64(property_name)
            }

            fn as_string<S: Into<String>>(&self, property_name: S) -> Option<String> {
                self.0.as_string(property_name)
            }

            fn as_array<S: Into<String>>(&self, property_name: S) -> Option<Vec<serde_json::Value>> {
                self.0.as_array(property_name)
            }

            fn as_object<S: Into<String>>(&self, property_name: S) -> Option<serde_json::Map<String, serde_json::Value>> {
                self.0.as_object(property_name)
            }
        }

        impl reactive_graph_graph::PropertyInstanceSetter for $ident {
            fn set_checked<S: Into<String>>(&self, property_name: S, value: serde_json::Value) {
                self.0.set_checked(property_name, value);
            }

            fn set<S: Into<String>>(&self, property_name: S, value: serde_json::Value) {
                self.0.set(property_name, value);
            }

            fn set_no_propagate_checked<S: Into<String>>(&self, property_name: S, value: serde_json::Value) {
                self.0.set_no_propagate_checked(property_name, value);
            }

            fn set_no_propagate<S: Into<String>>(&self, property_name: S, value: serde_json::Value) {
                self.0.set_no_propagate(property_name, value);
            }

            fn mutability<S: Into<String>>(&self, property_name: S) -> Option<reactive_graph_graph::Mutability> {
                self.0.mutability(property_name)
            }

            fn set_mutability<S: Into<String>>(&self, property_name: S, mutability: reactive_graph_graph::Mutability) {
                self.0.set_mutability(property_name, mutability);
            }
        }

        impl reactive_graph_graph::NamespacedTypeGetter for $ident {
            fn namespace(&self) -> Namespace {
                self.0.ty.namespace()
            }

            fn type_name(&self) -> String {
                self.0.ty.type_name()
            }
        }

        impl reactive_graph_graph::TypeDefinitionGetter for $ident {
            fn type_definition(&self) -> reactive_graph_graph::TypeDefinition {
                self.0.ty.type_definition()
            }
        }

        impl std::fmt::Display for $ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", &self.0)
            }
        }
    };
}
