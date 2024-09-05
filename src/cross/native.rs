use super::JSONPointer;

pub type Data = serde_json::Value;

impl JSONPointer for Data {
    fn pointer(&self, pointer: &str) -> Option<&Self> {
        self.pointer(pointer)
    }

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
