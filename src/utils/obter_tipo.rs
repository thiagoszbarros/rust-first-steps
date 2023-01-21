use std::any::type_name;

pub fn tipo<T>(_: T) -> &'static str {
    type_name::<T>()
}