#[derive(PartialEq, Debug)]
pub struct Car {
    pub color: String,
    pub transmission: Transmission,
    pub convertible: Convertible,
    pub mileage: Age,
}

pub type Age = (Condition, u32);

#[derive(PartialEq, Debug)]
pub enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

#[derive(PartialEq, Debug)]
pub enum Condition {
    New,
    Used,
}

#[derive(PartialEq, Debug)]
pub enum Convertible {
    HardTop,
    SoftTop,
}