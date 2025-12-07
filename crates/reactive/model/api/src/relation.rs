#[macro_export]
macro_rules! relation_model {
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
        pub struct $ident(reactive_graph_reactive_model_impl::ReactiveRelation);

        impl $ident {
            $(
                reactive_graph_graph::rx_accessor!(pub $accessor_type $($accessor_name $accessor_data_type)?);
            )*
        }

        impl $crate::ReactiveInstanceGetter<reactive_graph_reactive_model_impl::ReactiveRelation> for $ident {
            fn get_reactive_instance(&self) -> &reactive_graph_reactive_model_impl::ReactiveRelation {
                &self.0
            }
        }

        impl From<reactive_graph_reactive_model_impl::ReactiveRelation> for $ident {
            fn from(reactive_relation: reactive_graph_reactive_model_impl::ReactiveRelation) -> Self {
                $ident(reactive_relation)
            }
        }

        impl reactive_graph_graph::PropertyInstanceGetter for $ident {
            fn get(&self, property_name: &str) -> Option<serde_json::Value> {
                self.0.get(property_name)
            }

            fn as_bool(&self, property_name: &str) -> Option<bool> {
                self.0.as_bool(property_name)
            }

            fn as_u64(&self, property_name: &str) -> Option<u64> {
                self.0.as_u64(property_name)
            }

            fn as_i64(&self, property_name: &str) -> Option<i64> {
                self.0.as_i64(property_name)
            }

            fn as_f64(&self, property_name: &str) -> Option<f64> {
                self.0.as_f64(property_name)
            }

            fn as_string(&self, property_name: &str) -> Option<String> {
                self.0.as_string(property_name)
            }

            fn as_array(&self, property_name: &str) -> Option<Vec<serde_json::Value>> {
                self.0.as_array(property_name)
            }

            fn as_object(&self, property_name: &str) -> Option<serde_json::Map<String, serde_json::Value>> {
                self.0.as_object(property_name)
            }
        }

        impl reactive_graph_graph::PropertyInstanceSetter for $ident {
            fn set_checked(&self, property_name: &str, value: serde_json::Value) {
                self.0.set_checked(property_name, value);
            }

            fn set(&self, property_name: &str, value: serde_json::Value) {
                self.0.set(property_name, value);
            }

            fn set_no_propagate_checked(&self, property_name: &str, value: serde_json::Value) {
                self.0.set_no_propagate_checked(property_name, value);
            }

            fn set_no_propagate(&self, property_name: &str, value: serde_json::Value) {
                self.0.set_no_propagate(property_name, value);
            }

            fn mutability(&self, property_name: &str) -> Option<reactive_graph_graph::Mutability> {
                self.0.mutability(property_name)
            }

            fn set_mutability(&self, property_name: &str, mutability: reactive_graph_graph::Mutability) {
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
