use std::{error::Error, fmt::Display};

use crate::{
    format::HumanReadableValue,
    multiversx_sc::abi::{TypeContents, TypeDescription},
    SingleValue,
};
use multiversx_sc_scenario::{
    multiversx_sc::abi::ContractAbi,
    num_bigint::{BigInt, BigUint},
};

use crate::AnyValue;

pub fn decode_human_readable_value(
    input: &HumanReadableValue,
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

    decode_any_value(input, &type_description)
}

pub fn decode_any_value(
    input: &HumanReadableValue,
    type_description: &TypeDescription,
) -> Result<AnyValue, Box<dyn Error>> {
    match &type_description.contents {
        TypeContents::NotSpecified => interpret_single_value(input, type_description.name.as_str()),
        TypeContents::Enum(_) => todo!(),
        TypeContents::Struct(_) => todo!(),
        TypeContents::ExplicitEnum(_) => panic!("not supported"),
    }
}

fn interpret_single_value(
    input: &HumanReadableValue,
    type_name: &str,
) -> Result<AnyValue, Box<dyn Error>> {
    match type_name {
        "BigUint" | "u64" | "u32" | "u16" | "usize" | "u8" => {
            let number_value = input
                .get_value()
                .as_number()
                .ok_or_else(|| Box::new(InterpretError("expected unsigned number value")))?;

            let value = number_value.to_string().parse::<BigUint>()?;
            Ok(AnyValue::SingleValue(SingleValue::UnsignedNumber(value)))
        },
        "BigInt" | "i64" | "i32" | "i16" | "isize" | "i8" => {
            let number_value = input
                .get_value()
                .as_number()
                .ok_or_else(|| Box::new(InterpretError("expected number value")))?;

            let value = number_value.to_string().parse::<BigInt>()?;
            Ok(AnyValue::SingleValue(SingleValue::SignedNumber(value)))
        },
        "ManagedBuffer" | "Address" => {
            let array_value = input
                .get_value()
                .as_array()
                .ok_or_else(|| Box::new(InterpretError("expected bytes value")))?;

            let mut bytes = vec![0u8; array_value.len()];
            for (i, value) in array_value.iter().enumerate() {
                let number_value = value
                    .as_u64()
                    .ok_or_else(|| Box::new(InterpretError("expected byte value")))?;
                if number_value > 255 {
                    return Err(Box::new(InterpretError("expected byte value")));
                }
                bytes[i] = number_value as u8;
            }

            Ok(AnyValue::SingleValue(SingleValue::Bytes(bytes.into())))
        },
        "bool" => {
            let bool_value = input
                .get_value()
                .as_bool()
                .ok_or_else(|| Box::new(InterpretError("expected bool value")))?;

            Ok(AnyValue::SingleValue(SingleValue::Bool(bool_value.into())))
        },
        _ => Err(Box::new(InterpretError("unknown type"))),
    }
}

#[derive(Debug)]
pub struct InterpretError(&'static str);

impl Display for InterpretError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Error for InterpretError {}
