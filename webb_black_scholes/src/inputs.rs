// File specifically for the inputs of the black scholes merton model

use std::fmt::{Display, Formatter, Result as fmtResult};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum OptionType {
    Call,
    Put,
}

impl Display for OptionType {
    fn fmt(&self, f: &mut Formatter) -> fmtResult {
        match self {
            OptionType::Call => write!(f, "Call"),
            OptionType::Put => write!(f, "Put"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Inputs {
    /// Type of Option (Call or Put)
    pub option_type: OptionType,
    /// Stock Price
    pub s: f32,
    /// Strike Price of Option
    pub k: f32,
    /// Option Price
    pub p: Option<f32>,
    /// Risk-free Rate
    pub r: f32,
    /// Dividend Yield
    pub q: f32,
    /// Time to Maturity (years)
    pub t: f32,
    /// Volatility
    pub sigma: Option<f32>,
}

/// Methods for calculating the price, greeks, and implied volatility of an option.
impl Inputs {
    pub fn new(
        option_type: OptionType,
        s: f32,
        k: f32,
        p: Option<f32>,
        r: f32,
        q: f32,
        t: f32,
        sigma: Option<f32>,
    ) -> Self {
        Self {
            option_type,
            s,
            k,
            p,
            r,
            q,
            t,
            sigma,
        }
    }
}

impl Display for Inputs {
    fn fmt(&self, f: &mut Formatter) -> fmtResult {
        writeln!(f, "Option Type: {}", self.option_type)?;
        writeln!(f, "Stock Price: {:.2}", self.s)?;
        writeln!(f, "Strike Price: {:.2}", self.k)?;
        match self.p {
            Some(p) => writeln!(f, "Option Price: {:.2}", p)?,
            None => writeln!(f, "Option Price: None")?,
        }
        writeln!(f, "Risk-free Rate: {:.4}", self.r)?;
        writeln!(f, "Dividend Yield: {:.4}", self.q)?;
        writeln!(f, "Time to Maturity: {:.4}", self.t)?;
        match self.sigma {
            Some(sigma) => writeln!(f, "Volatility: {:.4}", sigma)?,
            None => writeln!(f, "Volatility: None")?,
        }
        Ok(())
    }
}
