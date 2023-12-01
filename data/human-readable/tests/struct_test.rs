use multiversx_sc_codec_human_readable::{
    decode_human_readable_value, encode_human_readable_value, format::HumanReadableValue, AnyValue,
    SingleValue, StructField, StructValue,
};
use multiversx_sc_meta::abi_json::deserialize_abi_from_json;
use multiversx_sc_scenario::multiversx_sc::{abi::ContractAbi, codec::top_encode_to_vec_u8};

const ABI_JSON: &str = r#"{
    "name": "Test",
    "endpoints": [],
    "events": [],
    "esdtAttributes": [],
    "hasCallback": false,
    "types": {
        "TwoU8s": {
            "type": "struct",
            "fields": [
                {
                    "name": "first",
                    "type": "u8"
                },
                {
                    "name": "second",
                    "type": "u8"
                }
            ]
        },
        "NestedStruct": {
            "type": "struct",
            "fields": [
                {
                    "name": "first",
                    "type": "u8"
                },
                {
                    "name": "second",
                    "type": "TwoU8s"
                }
            ]
        }
    }
}"#;

#[test]
fn serialize_struct_two_u8s() {
    let abi: ContractAbi = deserialize_abi_from_json(ABI_JSON).unwrap().into();

    let value = r#"{ "first": 1, "second": 2 }"#.parse::<HumanReadableValue>().unwrap();

    let result = decode_human_readable_value(&value, "TwoU8s", &abi).unwrap();
    let serialized = top_encode_to_vec_u8(&result).unwrap();
    assert_eq!(
        serialized,
        vec![
            0, 0, 0, 1, 1, // first
            0, 0, 0, 1, 2 // second
        ]
    );
}

#[test]
fn deserialize_struct_two_u8s() {
    let abi: ContractAbi = deserialize_abi_from_json(ABI_JSON).unwrap().into();

    let value = AnyValue::Struct(StructValue(vec![
        StructField {
            name: "first".to_string(),
            value: AnyValue::SingleValue(SingleValue::UnsignedNumber(1u8.into())),
        },
        StructField {
            name: "second".to_string(),
            value: AnyValue::SingleValue(SingleValue::UnsignedNumber(2u8.into())),
        },
    ]));

    let result = encode_human_readable_value(&value, "TwoU8s", &abi).unwrap();
    assert_eq!(result.to_string(), r#"{"first":1,"second":2}"#.to_string());
}

#[test]
fn serialize_struct_nested() {
    let abi: ContractAbi = deserialize_abi_from_json(ABI_JSON).unwrap().into();

    let value = r#"{
        "first": 1,
        "second": {
            "first": 1,
            "second": 2
        }
    }"#
    .parse::<HumanReadableValue>()
    .unwrap();

    let result = decode_human_readable_value(&value, "NestedStruct", &abi).unwrap();
    let serialized = top_encode_to_vec_u8(&result).unwrap();
    assert_eq!(
        serialized,
        vec![
            0, 0, 0, 1, 1, // first
            0, 0, 0, 1, 1, // second.first
            0, 0, 0, 1, 2 // second.second
        ]
    );
}

#[test]
fn deserialize_struct_nested() {
    let abi: ContractAbi = deserialize_abi_from_json(ABI_JSON).unwrap().into();

    let value = AnyValue::Struct(StructValue(vec![
        StructField {
            name: "first".to_string(),
            value: AnyValue::SingleValue(SingleValue::UnsignedNumber(1u8.into())),
        },
        StructField {
            name: "second".to_string(),
            value: AnyValue::Struct(StructValue(vec![
                StructField {
                    name: "first".to_string(),
                    value: AnyValue::SingleValue(SingleValue::UnsignedNumber(1u8.into())),
                },
                StructField {
                    name: "second".to_string(),
                    value: AnyValue::SingleValue(SingleValue::UnsignedNumber(2u8.into())),
                },
            ])),
        },
    ]));

    let result = encode_human_readable_value(&value, "NestedStruct", &abi).unwrap();
    assert_eq!(
        result.to_string(),
        r#"{"first":1,"second":{"first":1,"second":2}}"#.to_string()
    );
}
