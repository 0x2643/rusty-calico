use crate::error::Error;
use crate::result::Result;
use calico_consensus_core::constants::SOMPI_PER_CALICO;
use std::fmt::Display;

pub fn try_parse_required_nonzero_calico_as_sompi_u64<S: ToString + Display>(calico_amount: Option<S>) -> Result<u64> {
    if let Some(calico_amount) = calico_amount {
        let sompi_amount = calico_amount
            .to_string()
            .parse::<f64>()
            .map_err(|_| Error::custom(format!("Supplied Calico amount is not valid: '{calico_amount}'")))?
            * SOMPI_PER_CALICO as f64;
        if sompi_amount < 0.0 {
            Err(Error::custom("Supplied Calico amount is not valid: '{calico_amount}'"))
        } else {
            let sompi_amount = sompi_amount as u64;
            if sompi_amount == 0 {
                Err(Error::custom("Supplied required calico amount must not be a zero: '{calico_amount}'"))
            } else {
                Ok(sompi_amount)
            }
        }
    } else {
        Err(Error::custom("Missing Calico amount"))
    }
}

pub fn try_parse_required_calico_as_sompi_u64<S: ToString + Display>(calico_amount: Option<S>) -> Result<u64> {
    if let Some(calico_amount) = calico_amount {
        let sompi_amount = calico_amount
            .to_string()
            .parse::<f64>()
            .map_err(|_| Error::custom(format!("Supplied Calico amount is not valid: '{calico_amount}'")))?
            * SOMPI_PER_CALICO as f64;
        if sompi_amount < 0.0 {
            Err(Error::custom("Supplied Calico amount is not valid: '{calico_amount}'"))
        } else {
            Ok(sompi_amount as u64)
        }
    } else {
        Err(Error::custom("Missing Calico amount"))
    }
}

pub fn try_parse_optional_calico_as_sompi_i64<S: ToString + Display>(calico_amount: Option<S>) -> Result<Option<i64>> {
    if let Some(calico_amount) = calico_amount {
        let sompi_amount = calico_amount
            .to_string()
            .parse::<f64>()
            .map_err(|_e| Error::custom(format!("Supplied Calico amount is not valid: '{calico_amount}'")))?
            * SOMPI_PER_CALICO as f64;
        if sompi_amount < 0.0 {
            Err(Error::custom("Supplied Calico amount is not valid: '{calico_amount}'"))
        } else {
            Ok(Some(sompi_amount as i64))
        }
    } else {
        Ok(None)
    }
}
