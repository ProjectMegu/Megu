pub mod scc;
pub use scc::scc;

pub fn bind_result<T, E>(iter: impl Iterator<Item = Result<T, E>>) -> Result<Vec<T>, E> {
    iter.collect()
}
