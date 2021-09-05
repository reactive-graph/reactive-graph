use waiter_di::*;

pub fn get<T>() -> Container<T> {
    Container::<T>::new()
}
