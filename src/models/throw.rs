use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Throw {
    pub id: i32,
    pub value: String,
}
