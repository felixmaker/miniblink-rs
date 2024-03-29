pub(crate) mod handler;
pub(crate) mod raw;
pub(crate) mod app;

pub trait FromFFI<T> {
    fn from(value: T) -> Self;
}

pub trait ToFFI<T> {
    fn to(&self) -> T;
}
