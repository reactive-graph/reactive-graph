use dashmap::mapref::multiple::RefMulti;
use std::cmp::Ordering;
use std::hash::Hash;

pub(crate) fn sort_by_key<ID, TY>(a: &RefMulti<ID, TY>, b: &RefMulti<ID, TY>) -> Ordering
where
    ID: Hash + Ord,
    ID: Eq,
{
    Ord::cmp(&a.key(), &b.key())
}
