#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
/// The asynchronous request state.
pub enum AsynRequestState {
    /// The request is ok.
    Ok = 0,
    /// The request is fail.
    Fail = 1,
}

