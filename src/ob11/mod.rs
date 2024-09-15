use ob_types_macro::native_data;

pub mod action;
pub mod event;
pub mod message;

#[native_data(serde(rename_all = "lowercase"))]
pub enum Sex {
    Male,
    Female,
    Unknown,
}
