use std::{error::Error, fmt::Display};

use multiversx_sc_scenario::{
    multiversx_sc::abi::{ContractAbi, TypeContents, TypeDescription},
    num_bigint::{BigInt, BigUint},
};

use crate::{AnyValue, SingleValue};

pub fn default_value_for_abi_type(
    type_name: &str,
    contract_abi: &ContractAbi,
) -> Result<AnyValue, Box<dyn Error>> {
    let type_description =
        if let Some(type_description) = contract_abi.type_descriptions.0.get(type_name) {
            type_description.to_owned()
        } else {
            TypeDescription {
                docs: Vec::new(),
                name: type_name.to_string(),
                contents: TypeContents::NotSpecified,
            }
        };

    default_value_for_any_value(&type_description, contract_abi)
}

pub fn default_value_for_any_value(
    type_description: &TypeDescription,
    _contract_abi: &ContractAbi,
) -> Result<AnyValue, Box<dyn Error>> {
    match &type_description.contents {
        TypeContents::NotSpecified => {
            default_value_for_single_value(type_description.name.as_str())
        },
        TypeContents::Enum(_variants) => panic!("not supported"),
        TypeContents::Struct(_fields) => panic!("not supported"),
        TypeContents::ExplicitEnum(_) => panic!("not supported"),
    }
}

pub fn default_value_for_single_value(type_name: &str) -> Result<AnyValue, Box<dyn Error>> {
    match type_name {
        "BigUint" | "u64" | "u32" | "u16" | "usize" | "u8" => Ok(AnyValue::SingleValue(
            SingleValue::UnsignedNumber(BigUint::default()),
        )),
        "BigInt" | "i64" | "i32" | "i16" | "isize" | "i8" => Ok(AnyValue::SingleValue(
            SingleValue::SignedNumber(BigInt::default()),
        )),
        "ManagedBuffer" => Ok(AnyValue::SingleValue(SingleValue::Bytes(Vec::new().into()))),
        "string" | "utf-8 string" => Ok(AnyValue::SingleValue(SingleValue::String("".to_owned()))),
        "Address" => Ok(AnyValue::SingleValue(SingleValue::Bytes(
            vec![0u8; 32].into(),
        ))),
        "bool" => Ok(AnyValue::SingleValue(SingleValue::Bool(false))),

        _ => Err(Box::new(DefaultValueError("unknown type"))),
    }
}

#[derive(Debug)]
pub struct DefaultValueError(&'static str);

impl Display for DefaultValueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Error for DefaultValueError {}
