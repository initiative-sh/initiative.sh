use std::rc::Rc;

use uuid::Uuid;

pub struct Entity {
    pub uuid: Rc<Uuid>,
}

impl Entity {
    const ROOT_UUID: Uuid = Uuid::from_bytes([0xFF; 16]);
}

impl Entity {
    pub fn new_root() -> Self {
        Self {
            uuid: Rc::new(Self::ROOT_UUID),
        }
    }
}
