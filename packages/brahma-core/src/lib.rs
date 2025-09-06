


pub mod callback;
pub mod executor;
pub mod file_utils;
pub mod server;


// Re-exports for convenience (optional):
// `brahma_core::server::start_server` can be used by other Rust consumers.
pub use server::*;


#[cfg(test)]
mod tests {
// basic smoke tests to ensure the crate compiles and modules are reachable.
#[test]
fn it_compiles() {
// no-op test; actual napi-linked behaviour is covered in integration
// tests or by the Node.js side.
assert!(true);
}
}