use crate::state::Primitive;
use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// If name is not specified the sender's address will be used as the name of the primitive.
    /// This allows each user to have easy access to a single primitive if that is all they
    /// require.
    SetValue {
        name: Option<String>,
        value: Primitive,
    },
    /// If name is not specified the sender's address will be used as the name of the primitive.
    DeleteValue { name: Option<String> },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
    /// If name is not specified the given address will be used as the name of the primitive.
    GetValue {
        address: Addr,
        name: Option<String>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetValueResponse {
    pub name: String,
    pub value: Primitive,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    pub owner: Addr,
}
