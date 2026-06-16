use uuid::{Uuid};

pub struct Object {
    id: Uuid,
    type_: u32,
    subtype: u32
}