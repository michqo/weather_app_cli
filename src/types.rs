use serde::Deserialize;

#[allow(non_snake_case, dead_code)]
#[derive(Deserialize)]
pub struct Temp {
    pub y: i32,
    pub m: i32,
    pub d: i32,
    pub h: i32,
    pub averageTemp: String,
}

#[derive(Deserialize)]
pub struct Average {
    pub average: f32,
}
