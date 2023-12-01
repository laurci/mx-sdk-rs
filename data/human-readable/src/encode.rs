use std::{error::Error, fmt::Display};

use bech32::ToBase32;
use multiversx_sc_scenario::multiversx_sc::abi::{
    ContractAbi, StructFieldDescription, TypeContents, TypeDescription,
};
use serde_json::{Map, Value as JsonValue};

use crate::{format::HumanReadableValue, AnyValue, SingleValue};

pub fn encode_human_readable_value(
    input: &AnyValue,
    type_name: &str,
    contract_abi: &ContractAbi,
) -> Result<HumanReadableValue, Box<dyn Error>> {
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

    encode_any_value(input, &type_description, &contract_abi)
}

pub fn encode_any_value(
    input: &AnyValue,
    type_description: &TypeDescription,
    contract_abi: &ContractAbi,
) -> Result<HumanReadableValue, Box<dyn Error>> {
    match &type_description.contents {
        TypeContents::NotSpecified => encode_single_value(input, type_description.name.as_str()),
        TypeContents::Enum(_variants) => panic!("not supported"),
        TypeContents::Struct(fields) => encode_struct(input, &fields, &contract_abi),
        TypeContents::ExplicitEnum(_) => panic!("not supported"),
    }
}

fn encode_single_value(
    input: &AnyValue,
    type_name: &str,
) -> Result<HumanReadableValue, Box<dyn Error>> {
    match type_name {
        "BigUint" | "u64" | "u32" | "u16" | "usize" | "u8" => {
            let AnyValue::SingleValue(value) = input else {
                return Err(Box::new(EncodeError("expected single value")));
            };
            let SingleValue::UnsignedNumber(value) = value else {
                return Err(Box::new(EncodeError("expected unsigned number value")));
            };

            // could be biguint, so we convert to string first
            let json_value: JsonValue = serde_json::from_str(&value.to_string())
                .map_err(|_| Box::new(EncodeError("expected number value")))?;

            Ok(json_value.into())
        },
        "BigInt" | "i64" | "i32" | "i16" | "isize" | "i8" => {
            let AnyValue::SingleValue(value) = input else {
                return Err(Box::new(EncodeError("expected single value")));
            };
            let SingleValue::SignedNumber(value) = value else {
                return Err(Box::new(EncodeError("expected signed number value")));
            };

            // could be bigint, so we convert to string first
            let json_value: JsonValue = serde_json::from_str(&value.to_string())
                .map_err(|_| Box::new(EncodeError("expected number value")))?;

            Ok(json_value.into())
        },
        "ManagedBuffer" => {
            let AnyValue::SingleValue(value) = input else {
                return Err(Box::new(EncodeError("expected single value")));
            };
            let SingleValue::Bytes(value) = value else {
                return Err(Box::new(EncodeError("expected bytes value")));
            };

            Ok(JsonValue::Array(
                value
                    .iter()
                    .map(|b| JsonValue::Number(b.to_owned().into()))
                    .collect(),
            )
            .into())
        },
        "string" | "utf-8 string" => {
            let AnyValue::SingleValue(value) = input else {
                return Err(Box::new(EncodeError("expected single value")));
            };
            let SingleValue::Bytes(value) = value else {
                return Err(Box::new(EncodeError("expected bytes value")));
            };

            let str_value = String::from_utf8(value.to_vec())
                .map_err(|_| Box::new(EncodeError("expected utf-8 string value")))?;

            Ok(JsonValue::String(str_value).into())
        },
        "Address" => {
            let AnyValue::SingleValue(value) = input else {
                return Err(Box::new(EncodeError("expected single value")));
            };
            let SingleValue::Bytes(value) = value else {
                return Err(Box::new(EncodeError("expected bytes value")));
            };

            let address = bech32::encode("erd", value.to_base32(), bech32::Variant::Bech32)
                .map_err(|_| Box::new(EncodeError("failed to encode address")))?;

            Ok(JsonValue::String(address).into())
        },
        "bool" => {
            let AnyValue::SingleValue(value) = input else {
                return Err(Box::new(EncodeError("expected single value")));
            };
            let SingleValue::Bool(value) = value else {
                return Err(Box::new(EncodeError("expected bool value")));
            };

            Ok(JsonValue::Bool(value.to_owned()).into())
        },
        _ => Err(Box::new(EncodeError("unknown type"))),
    }
}

pub fn encode_struct(
    input: &AnyValue,
    fields: &Vec<StructFieldDescription>,
    contract_abi: &ContractAbi,
) -> Result<HumanReadableValue, Box<dyn Error>> {
    let AnyValue::Struct(struct_value) = input else {
        return Err(Box::new(EncodeError("expected struct value")));
    };
    let mut struct_fields = struct_value.0.iter();

    let mut field_values: Map<String, JsonValue> = Map::new();

    for field in fields.iter() {
        let value = struct_fields
            .find(|f| f.name == field.name)
            .ok_or_else(|| Box::new(EncodeError("missing field")))?;

        let value = encode_human_readable_value(&value.value, &field.field_type, contract_abi)?;
        field_values.insert(field.name.to_owned(), value.get_value().to_owned());
    }

    Ok(JsonValue::Object(field_values).into())
}

#[derive(Debug)]
pub struct EncodeError(&'static str);

impl Display for EncodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Error for EncodeError {}
