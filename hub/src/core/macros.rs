/// A macro to generate a topic string from a prefix and an ending.
#[macro_export]
macro_rules! topic {
    ($prexix:expr, $ending:expr) => {
        format!("{}/{}", $prexix, $ending)
    };
}
