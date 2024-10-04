use std::rc::Rc;

use xla_builder_sys::{xla_builder_create, xla_builder_destroy, XlaBuilder};

struct BuilderRaw {
    ptr: *mut XlaBuilder,
}

impl Drop for BuilderRaw {
    fn drop(&mut self) {
        unsafe { xla_builder_destroy(self.ptr) };
    }
}

#[derive(Clone)]
pub struct Builder {
    raw: Rc<BuilderRaw>,
}

impl Builder {
    pub fn new(name: &str) -> Self {
        let name_bytes = name.as_bytes();
        let ptr = unsafe { xla_builder_create(name_bytes.as_ptr() as *const i8, name_bytes.len()) };
        Self {
            raw: Rc::new(BuilderRaw { ptr }),
        }
    }
}
