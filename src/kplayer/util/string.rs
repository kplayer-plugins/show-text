extern "C" {
    fn NewString() -> i32;
    fn DeleteString(i: i32) -> i32;
    fn AppendChar(p: i32, i: i32) -> i32;
}

pub struct DynamicString {
    index: i32,
}

impl DynamicString {
    pub fn from(d: &[u8]) -> DynamicString {
        let mut str = DynamicString {
            index: unsafe { NewString() },
        };
        str.append(d);

        str
    }

    pub fn get_index(&self) -> i32 {
        self.index
    }

    fn append(&mut self, d: &[u8]) {
        unsafe {
            for i in d {
                self.index = AppendChar(self.index, (*i) as i32)
            }
        }
    }

    fn drop(&mut self) {
        unsafe {
            DeleteString(self.index);
        }
    }
}

impl Drop for DynamicString {
    fn drop(&mut self) {
        unsafe {
            DeleteString(self.index);
        }
    }
}
