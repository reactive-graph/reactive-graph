use quote::ToTokens;
use std::cmp::Ordering;
use syn::File;
use syn::Item;

pub fn get_item_sort_key(item: &Item) -> Option<String> {
    match item {
        Item::Use(i) => Some(i.to_token_stream().to_string()),
        Item::Mod(i) => Some(i.ident.to_string()),
        Item::Struct(i) => Some(i.ident.to_string()),
        Item::Enum(i) => Some(i.ident.to_string()),
        Item::Type(i) => Some(i.ident.to_string()),
        Item::Trait(i) => Some(i.ident.to_string()),
        Item::Fn(i) => Some(i.sig.ident.to_string()),
        Item::Const(i) => Some(i.ident.to_string()),
        _ => None,
    }
}

pub fn get_item_type_order(item: &Item) -> u8 {
    match item {
        Item::Use(_) => 1,
        Item::Mod(_) => 2,
        Item::Struct(_) => 3,
        Item::Enum(_) => 4,
        Item::Type(_) => 5,
        Item::Trait(_) => 6,
        Item::Impl(_) => 7,
        Item::Fn(_) => 8,
        _ => 99,
    }
}

pub fn sort_file_items(file: &mut File) {
    file.items.sort_by(|a, b| {
        let order_cmp = get_item_type_order(a).cmp(&get_item_type_order(b));
        if order_cmp != Ordering::Equal {
            return order_cmp;
        }
        let key_a = get_item_sort_key(a);
        let key_b = get_item_sort_key(b);
        key_a.cmp(&key_b)
    });
}
