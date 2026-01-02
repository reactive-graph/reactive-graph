use crate::CodeGenerationConfig;
use crate::property::rust::property_instance::getters_and_setters::property_instance_getters_and_setters;
use crate::property::rust::property_instance::visibility::Visibility;
use crate::property::rust::property_instance_fields;
use crate::property::rust::property_instances_getter::generate_property_instances_getter;
use crate::rust::Rust;
use crate::type_definition::rust::AssocTypeDefinitionIdentType;
use crate::type_definition::rust::TypeIdent;
use proc_macro2::TokenStream;
use quote::quote;
use reactive_graph_generator_documentation::DocumentationConfig;
use reactive_graph_generator_documentation::DocumentationConfigPreset;
use reactive_graph_generator_documentation::FromDocumentationConfigPreset;
use reactive_graph_generator_documentation::GenerateDocumentation;
use reactive_graph_graph::PropertyType;
use reactive_graph_graph::PropertyTypeContainer;
use reactive_graph_graph::TypeDefinitionGetter;
use reactive_graph_graph::TypeDescriptionGetter;
use reactive_graph_graph::TypeResolver;

pub mod constructor;
pub mod into_relation_instance;
pub mod try_from_relation_instance;

pub fn generate_struct_definition<TY: TypeDefinitionGetter + PropertyTypeContainer + TypeDescriptionGetter + GenerateDocumentation<TY>>(
    type_: &TY,
    config: &CodeGenerationConfig,
    resolver: &TypeResolver,
    properties: &Vec<PropertyType>,
) -> TokenStream {
    let type_name_ident = TypeIdent::new(type_);
    let property_instance_fields = property_instance_fields(properties, config, resolver);
    let property_instance_getters = property_instance_getters_and_setters(properties, config, resolver, Visibility::Public);
    // let mut property_instance_fields = Vec::new();
    // let mut property_instance_getters = Vec::new();
    // for property in properties.into_iter() {
    //     property_instance_fields.push(generate_property_field(&property, config, resolver));
    //     property_instance_getters.push(generate_property_instance_getter_impl(&property, resolver, Visibility::Public));
    //     if property.mutability == Mutability::Mutable {
    //         property_instance_getters.push(generate_property_instance_setter_impl(&property, resolver, Visibility::Public));
    //     }
    // }
    let doc_comment = Rust::multiline_doc_comment(struct_doc_comment(type_, resolver));
    let typed_builders = if config.generate_builders {
        quote! {, typed_builder::TypedBuilder}
    } else {
        quote! {}
    };
    let constructor = constructor::generate_constructor(type_, &properties);
    let property_instances_getter = generate_property_instances_getter(type_, &properties);
    let into_relation_instance = into_relation_instance::generate_into_relation_instance(type_, &type_name_ident);
    let try_from_relation_instance = try_from_relation_instance::generate_try_from_relation_instance(type_, &type_name_ident);
    quote! {
        #[doc(newline)]
        #doc_comment
        #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize #typed_builders)]
        pub struct #type_name_ident {
            #[builder(setter(into))]
            outbound_id: uuid::Uuid,
            #[builder(setter(into))]
            id: reactive_graph_graph::RelationInstanceTypeId,
            #[builder(setter(into))]
            inbound_id: uuid::Uuid,
            #(#property_instance_fields)*
            extensions: reactive_graph_graph::Extensions,
        }

        #[doc(newline)]
        impl #type_name_ident {
            #constructor

            #[doc(newline)]
            pub fn outbound_id(&self) -> uuid::Uuid {
                self.outbound_id
            }

            #[doc(newline)]
            pub fn id(&self) -> reactive_graph_graph::RelationInstanceTypeId {
                self.id.clone()
            }

            #[doc(newline)]
            pub fn inbound_id(&self) -> uuid::Uuid {
                self.inbound_id
            }

            #(#property_instance_getters)*

            #[doc(newline)]
            #property_instances_getter

            #[doc(newline)]
            pub fn extensions(&self) -> reactive_graph_graph::Extensions {
                self.extensions.clone()
            }
        }

        #into_relation_instance
        #try_from_relation_instance
    }
}

#[inline]
pub fn struct_doc_comment<TY: TypeDescriptionGetter + GenerateDocumentation<TY>>(type_: &TY, resolver: &TypeResolver) -> String {
    type_
        .generate_documentation(&DocumentationConfig::new_from_preset(DocumentationConfigPreset::Short), resolver)
        .map(|documentation| format!(" {}", documentation.to_string()))
        .unwrap_or_else(|_| type_.description())
}
