use std::{error::Error, fmt::Display};

use crate::{
    format::HumanReadableValue,
    multiversx_sc::abi::{TypeContents, TypeDescription},
    SingleValue, StructField, StructValue,
};
use bech32::FromBase32;
use multiversx_sc_scenario::{
    multiversx_sc::abi::{ContractAbi, StructFieldDescription},
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

    decode_any_value(input, &type_description, &contract_abi)
}

pub fn decode_any_value(
    input: &HumanReadableValue,
    type_description: &TypeDescription,
    contract_abi: &ContractAbi,
) -> Result<AnyValue, Box<dyn Error>> {
    match &type_description.contents {
        TypeContents::NotSpecified => decode_single_value(input, type_description.name.as_str()),
        TypeContents::Enum(_) => todo!(),
        TypeContents::Struct(fields) => decode_struct(input, &fields, &contract_abi),
        TypeContents::ExplicitEnum(_) => panic!("not supported"),
    }
}

fn decode_single_value(
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
        "ManagedBuffer" => {
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
        "Address" => {
            let str_value = input
                .get_value()
                .as_str()
                .ok_or_else(|| Box::new(InterpretError("expected string value")))?;

            let (_, address_bytes_u5, _) = bech32::decode(str_value)
                .map_err(|_| Box::new(InterpretError("failed to parse address")))?;
            let address_bytes = Vec::<u8>::from_base32(&address_bytes_u5)
                .map_err(|_| Box::new(InterpretError("failed to parse address")))?;

            if address_bytes.len() != 32 {
                return Err(Box::new(InterpretError(
                    "Invalid address length after decoding",
                )));
            }

            Ok(AnyValue::SingleValue(SingleValue::Bytes(
                address_bytes.into(),
            )))
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

pub fn decode_struct(
    input: &HumanReadableValue,
    fields: &Vec<StructFieldDescription>,
    contract_abi: &ContractAbi,
) -> Result<AnyValue, Box<dyn Error>> {
    let mut field_values: Vec<StructField> = vec![];

    for field in fields.iter() {
        let value = input
            .child(&field.name)
            .ok_or_else(|| Box::new(InterpretError("missing field")))?;
        let value = decode_human_readable_value(&value, &field.field_type, &contract_abi)?;
        field_values.push(StructField {
            name: field.name.clone(),
            value,
        });
    }

    Ok(AnyValue::Struct(StructValue(field_values)))
}

#[derive(Debug)]
pub struct InterpretError(&'static str);

impl Display for InterpretError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Error for InterpretError {}
