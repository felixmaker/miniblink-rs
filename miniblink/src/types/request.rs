#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
/// The asynchronous request state.
pub enum AsynRequestState {
    /// The request is ok.
    Ok = 0,
    /// The request is fail.
    Fail = 1,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
/// The request type.
pub enum RequestType {
    /// The request type is invalid.
    Invalidation = 0,
    /// The request type is get.
    Get = 1,
    /// The request type is post.
    Post = 2,
    /// The request type is put.
    Put = 3,
}
