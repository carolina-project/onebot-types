use super::{DataImpl, JSONPointer, ParseData};

pub type Data = serde_json::Value;

impl JSONPointer for Data {
    #[inline]
    fn pointer(&self, pointer: &str) -> Option<&Self> {
        self.pointer(pointer)
    }

    #[inline]
    fn pointer_mut(&mut self, pointer: &str) -> Option<&mut Self> {
        self.pointer_mut(pointer)
    }

    fn transfer_field(
        &mut self,
        source_ptr: &str,
        target: &mut Self,
        target_ptr: &str,
    ) -> Option<()> {
        let source = self.pointer_mut(source_ptr)?.take();
        target.set_by_pointer(target_ptr, source);
        Some(())
    }

    fn set_by_pointer(&mut self, ptr: &str, data: Self) {
        let parts: Vec<&str> = ptr.trim_start_matches('/').split('/').collect();
        let mut current = self;

        for i in 0..parts.len() {
            let key = parts[i];

            if i == parts.len() - 1 {
                current[key] = data;
                break;
            } else {
                if !current.get(key).is_some() {
                    current[key] = Data::Object(serde_json::Map::new());
                }
                current = current.get_mut(key).unwrap();
            }
        }
    }
}
impl ParseData for Data {
    #[inline]
    fn is_str(&self) -> bool {
        self.is_string()
    }

    #[inline]
    fn is_i64(&self) -> bool {
        self.is_f64()
    }

    #[inline]
    fn is_u64(&self) -> bool {
        self.is_u64()
    }

    #[inline]
    fn is_f64(&self) -> bool {
        self.is_f64()
    }

    #[inline]
    fn is_bool(&self) -> bool {
        self.is_boolean()
    }

    #[inline]
    fn is_arr(&self) -> bool {
        self.is_array()
    }

    #[inline]
    fn is_obj(&self) -> bool {
        self.is_object()
    }

    #[inline]
    fn is_none(&self) -> bool {
        self.is_null()
    }

    #[inline]
    fn as_str(&self) -> Option<&str> {
        self.as_str()
    }

    #[inline]
    fn as_i64(&self) -> Option<i64> {
        self.as_i64()
    }

    #[inline]
    fn as_u64(&self) -> Option<u64> {
        self.as_u64()
    }

    #[inline]
    fn as_f64(&self) -> Option<f64> {
        self.as_f64()
    }

    #[inline]
    fn as_bool(&self) -> Option<bool> {
        self.as_bool()
    }

    #[inline]
    fn as_arr(&self) -> Option<&Vec<Self>>
    where
        Self: Sized,
    {
        self.as_array()
    }
}

impl DataImpl for Data {
    fn new() -> Self {
        Data::default()
    }
}
