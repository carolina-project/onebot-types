#[cfg_attr(target_arch = "wasm32", path = "./wasm.rs")]
#[cfg_attr(not(target_arch = "wasm32"), path = "./native.rs")]
mod data;

/// A trait for data that can be accessed by JSON Pointer
pub trait JSONPointer {
    /// get data by JSON Pointer
    fn pointer(&self, pointer: &str) -> Option<&Self>;
    /// get mutable data by JSON Pointer
    fn pointer_mut(&mut self, pointer: &str) -> Option<&mut Self>;
    fn set_by_pointer(&mut self, ptr: &str, data: Self);
    /// transfer data field from source data to target data by JSON Pointer
    fn transfer_field(&mut self, source_ptr: &str, target: &mut Self, target_ptr: &str) -> Option<()>;
}

pub use data::Data;
