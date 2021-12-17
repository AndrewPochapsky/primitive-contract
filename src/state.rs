use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, StdError, Uint128};
use cw_storage_plus::{Item, Map};

pub const DEFAULT_KEY: &str = "default";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum Primitive {
    Uint128(Uint128),
    String(String),
    Bool(bool),
    Vec(Vec<Primitive>),
}

fn parse_error(type_name: String) -> StdError {
    StdError::ParseErr {
        target_type: type_name.clone(),
        msg: format!("Primitive is not a {}", type_name),
    }
}

// These are methods to help the calling user quickly retreive the data in the Primitive as they
// often already know what the type should be.
impl Primitive {
    pub fn is_invalid(&self) -> bool {
        match self {
            // Avoid infinite recursion problem by not allowing nested vectors.
            Primitive::Vec(vector) => vector
                .iter()
                .any(|p| matches!(p, Primitive::Vec(_)) || p.is_invalid()),
            _ => false,
        }
    }

    pub fn try_get_uint128(&self) -> Result<Uint128, StdError> {
        match self {
            Primitive::Uint128(value) => Ok(*value),
            _ => Err(parse_error(String::from("Uint128"))),
        }
    }

    pub fn try_get_string(&self) -> Result<String, StdError> {
        match self {
            Primitive::String(value) => Ok(value.to_string()),
            _ => Err(parse_error(String::from("String"))),
        }
    }

    pub fn try_get_bool(&self) -> Result<bool, StdError> {
        match self {
            Primitive::Bool(value) => Ok(*value),
            _ => Err(parse_error(String::from("bool"))),
        }
    }

    pub fn try_get_vec(&self) -> Result<Vec<Primitive>, StdError> {
        match self {
            Primitive::Vec(vector) => Ok(vector.to_vec()),
            _ => Err(parse_error(String::from("Vec"))),
        }
    }
}

pub const DATA: Map<&str, Primitive> = Map::new("data");
pub const CONFIG: Item<Config> = Item::new("config");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_error() {
        assert_eq!(
            StdError::ParseErr {
                target_type: "target_type".to_string(),
                msg: "Primitive is not a target_type".to_string()
            },
            parse_error("target_type".to_string())
        );
    }

    #[test]
    fn try_get_uint128() {
        let primitive = Primitive::Uint128(Uint128::from(5 as u128));
        assert_eq!(
            Uint128::from(5 as u128),
            primitive.try_get_uint128().unwrap()
        );

        let primitive = Primitive::Bool(true);
        assert_eq!(
            parse_error("Uint128".to_string()),
            primitive.try_get_uint128().unwrap_err()
        );
    }

    #[test]
    fn try_get_string() {
        let primitive = Primitive::String("String".to_string());
        assert_eq!("String".to_string(), primitive.try_get_string().unwrap());

        let primitive = Primitive::Bool(true);
        assert_eq!(
            parse_error("String".to_string()),
            primitive.try_get_string().unwrap_err()
        );
    }

    #[test]
    fn try_get_bool() {
        let primitive = Primitive::Bool(true);
        assert_eq!(true, primitive.try_get_bool().unwrap());

        let primitive = Primitive::String("String".to_string());
        assert_eq!(
            parse_error("bool".to_string()),
            primitive.try_get_bool().unwrap_err()
        );
    }

    #[test]
    fn try_get_vec() {
        let primitive = Primitive::Vec(vec![Primitive::Bool(true)]);
        assert_eq!(
            vec![Primitive::Bool(true)],
            primitive.try_get_vec().unwrap()
        );

        let primitive = Primitive::String("String".to_string());
        assert_eq!(
            parse_error("Vec".to_string()),
            primitive.try_get_vec().unwrap_err()
        );
    }

    #[test]
    fn is_valid() {
        let valid_primitive = Primitive::Bool(true);
        assert!(!valid_primitive.is_invalid());

        let valid_primitive = Primitive::Uint128(Uint128::new(1_u128));
        assert!(!valid_primitive.is_invalid());

        let valid_primitive = Primitive::String("String".to_string());
        assert!(!valid_primitive.is_invalid());

        let valid_primitive = Primitive::Vec(vec![
            Primitive::Bool(true),
            Primitive::Uint128(Uint128::new(1_u128)),
            Primitive::String("String".to_string()),
        ]);
        assert!(!valid_primitive.is_invalid());

        let invalid_primitive = Primitive::Vec(vec![Primitive::Bool(true), Primitive::Vec(vec![])]);
        assert!(invalid_primitive.is_invalid());
    }
}
