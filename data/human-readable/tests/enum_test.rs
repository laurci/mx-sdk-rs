use multiversx_sc_codec_human_readable::{decode_human_readable_value, format::HumanReadableValue};
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
        "SimpleEnum": {
            "type": "enum",
            "variants": [
                {
                    "name": "First",
                    "discriminant": 0
                },
                {
                    "name": "Second",
                    "discriminant": 1
                }
            ]
        },
        "EnumWithStruct": {
            "type": "enum",
            "variants": [
                {
                    "name": "First",
                    "discriminant": 0
                },
                {
                    "name": "Second",
                    "discriminant": 1,
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
                }
            ]
        },
        "EnumWithTupleStruct": {
            "type": "enum",
            "variants": [
                {
                    "name": "First",
                    "discriminant": 0
                },
                {
                    "name": "Second",
                    "discriminant": 1,
                    "fields": [
                        {
                            "name": "0",
                            "type": "TwoU8s"
                        }
                    ]
                }
            ]
        },
        "EnumWithTupleValues": {
            "type": "enum",
            "variants": [
                {
                    "name": "First",
                    "discriminant": 0
                },
                {
                    "name": "Second",
                    "discriminant": 1,
                    "fields": [
                        {
                            "name": "0",
                            "type": "u8"
                        },
                        {
                            "name": "1",
                            "type": "u8"
                        }
                    ]
                }
            ]
        },
        "EnumWithTupleValuesAndStruct": {
            "type": "enum",
            "variants": [
                {
                    "name": "First",
                    "discriminant": 0
                },
                {
                    "name": "Second",
                    "discriminant": 1,
                    "fields": [
                        {
                            "name": "0",
                            "type": "u8"
                        },
                        {
                            "name": "1",
                            "type": "u8"
                        },
                        {
                            "name": "2",
                            "type": "TwoU8s"
                        }
                    ]
                }
            ]
        }
    }
}"#;

#[test]
fn serialize_enum_only_discriminant() {
    let abi: ContractAbi = deserialize_abi_from_json(ABI_JSON).unwrap().into();

    let value = r#""Second""#.parse::<HumanReadableValue>().unwrap();

    let result = decode_human_readable_value(&value, "SimpleEnum", &abi).unwrap();
    let serialized = top_encode_to_vec_u8(&result).unwrap();
    assert_eq!(serialized, vec![1]);
}

#[test]
fn serialize_enum_with_struct() {
    let abi: ContractAbi = deserialize_abi_from_json(ABI_JSON).unwrap().into();

    let value =
        r#"{ "Second": { "first": 1, "second": 2 } }"#.parse::<HumanReadableValue>().unwrap();

    let result = decode_human_readable_value(&value, "EnumWithStruct", &abi).unwrap();
    let serialized = top_encode_to_vec_u8(&result).unwrap();
    assert_eq!(
        serialized,
        vec![
            1, // discriminant
            0, 0, 0, 1, 1, // first
            0, 0, 0, 1, 2 // second
        ]
    );
}

#[test]
fn serialize_enum_tuple_with_struct() {
    let abi: ContractAbi = deserialize_abi_from_json(ABI_JSON).unwrap().into();

    let value =
        r#"{ "Second": { "first": 1, "second": 2 } }"#.parse::<HumanReadableValue>().unwrap();

    let result = decode_human_readable_value(&value, "EnumWithTupleStruct", &abi).unwrap();
    let serialized = top_encode_to_vec_u8(&result).unwrap();
    assert_eq!(
        serialized,
        vec![
            1, // discriminant
            0, 0, 0, 1, 1, // first
            0, 0, 0, 1, 2 // second
        ]
    );
}

#[test]
fn serialize_enum_tuple_with_values() {
    let abi: ContractAbi = deserialize_abi_from_json(ABI_JSON).unwrap().into();

    let value = r#"{ "Second": [1, 2] }"#.parse::<HumanReadableValue>().unwrap();

    let result = decode_human_readable_value(&value, "EnumWithTupleValues", &abi).unwrap();
    let serialized = top_encode_to_vec_u8(&result).unwrap();
    assert_eq!(
        serialized,
        vec![
            1, // discriminant
            0, 0, 0, 1, 1, // 0
            0, 0, 0, 1, 2 // 1
        ]
    );
}

#[test]
fn serialize_enum_tuple_with_values_and_struct() {
    let abi: ContractAbi = deserialize_abi_from_json(ABI_JSON).unwrap().into();

    let value = r#"{ "Second": [1, 2, { "first": 1, "second": 2 }] }"#
        .parse::<HumanReadableValue>()
        .unwrap();

    let result = decode_human_readable_value(&value, "EnumWithTupleValuesAndStruct", &abi).unwrap();
    let serialized = top_encode_to_vec_u8(&result).unwrap();
    assert_eq!(
        serialized,
        vec![
            1, // discriminant
            0, 0, 0, 1, 1, // 0
            0, 0, 0, 1, 2, // 1
            0, 0, 0, 1, 1, // 2.first
            0, 0, 0, 1, 2 // 2.second
        ]
    );
}
