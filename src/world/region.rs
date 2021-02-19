use std::collections::HashMap;
use std::rc::Rc;

use uuid::Uuid;

use super::Value;

#[derive(Default)]
pub struct Region {
    pub uuid: Option<Rc<Uuid>>,
    pub parent_uuid: Option<Rc<Uuid>>,
    pub demographics: Demographics,
    pub data: HashMap<RegionField, Value>,
}

#[derive(Default)]
pub struct Demographics {}

pub enum RegionField {}
