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
    fn transfer_field(
        &mut self,
        source_ptr: &str,
        target: &mut Self,
        target_ptr: &str,
    ) -> Option<()>;
}

pub trait ParseData {
    fn is_str(&self) -> bool;
    fn is_i64(&self) -> bool;
    fn is_u64(&self) -> bool;
    fn is_f64(&self) -> bool;
    fn is_bool(&self) -> bool;
    fn is_arr(&self) -> bool;
    fn is_obj(&self) -> bool;
    fn is_none(&self) -> bool;

    fn as_str(&self) -> Option<&str>;
    fn as_i64(&self) -> Option<i64>;
    fn as_u64(&self) -> Option<u64>;
    fn as_f64(&self) -> Option<f64>;
    fn as_bool(&self) -> Option<bool>;
    fn as_arr(&self) -> Option<&Vec<Self>>
    where
        Self: Sized;
}

pub trait DataImpl: ParseData + JSONPointer {}
impl<T: ParseData + JSONPointer> DataImpl for T {}

pub use data::Data;
