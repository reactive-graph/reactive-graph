pub mod rust;

pub trait CodeGenTarget {
    fn lang();
    fn version();
}
