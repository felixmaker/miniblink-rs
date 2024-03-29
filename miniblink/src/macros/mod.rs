pub(crate) mod handler;
pub(crate) mod props;

pub trait FromFFI<T> {
    fn from(value: T) -> Self;
}

pub trait ToFFI<T> {
    fn to(&self) -> T;
}
