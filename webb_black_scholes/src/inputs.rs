// File specifically for the inputs of the black scholes merton model

use std::fmt::{Display, Formatter, Result as fmtResult};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum OptionType {
    Call,
    Put,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Inputs {
    // inputs here
}

// methods for inputs
impl Inputs {}
