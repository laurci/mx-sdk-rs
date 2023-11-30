use multiversx_sc_codec_human_readable::{decode_human_readable_value, format::HumanReadableValue};
use multiversx_sc_meta::abi_json::deserialize_abi_from_json;
use multiversx_sc_scenario::multiversx_sc::{abi::ContractAbi, codec::top_encode_to_vec_u8};

const EMPTY_ABI_JSON: &str = r#"{
    "name": "Test",
    "endpoints": [],
    "events": [],
    "esdtAttributes": [],
    "hasCallback": false,
    "types": {}
}"#;

#[test]
fn serialize_single_value_unsigned() {
    let abi: ContractAbi = deserialize_abi_from_json(EMPTY_ABI_JSON).unwrap().into();

    let value = "1234".parse::<HumanReadableValue>().unwrap();

    let result = decode_human_readable_value(&value, "u32", &abi).unwrap();
    let serialized = top_encode_to_vec_u8(&result).unwrap();
    assert_eq!(serialized, 1234u16.to_be_bytes().to_vec()); // should take only 2 bytes (top encoded)
}

#[test]
fn serialize_single_value_signed() {
    let abi: ContractAbi = deserialize_abi_from_json(EMPTY_ABI_JSON).unwrap().into();

    let value = "-1234".parse::<HumanReadableValue>().unwrap();

    let result = decode_human_readable_value(&value, "i32", &abi).unwrap();
    let serialized = top_encode_to_vec_u8(&result).unwrap();
    assert_eq!(serialized, (-1234 as i16).to_be_bytes().to_vec()); // should take only 2 bytes (top encoded)
}

#[test]
fn serialize_single_value_managed_buffer() {
    let abi: ContractAbi = deserialize_abi_from_json(EMPTY_ABI_JSON).unwrap().into();

    let value = "[12, 34]".parse::<HumanReadableValue>().unwrap();

    let result = decode_human_readable_value(&value, "ManagedBuffer", &abi).unwrap();
    let serialized = top_encode_to_vec_u8(&result).unwrap();
    assert_eq!(serialized, vec![12, 34]); // should take only 2 bytes (top encoded)
}

#[test]
fn serialize_single_value_bool() {
    let abi: ContractAbi = deserialize_abi_from_json(EMPTY_ABI_JSON).unwrap().into();

    let value = "true".parse::<HumanReadableValue>().unwrap();

    let result = decode_human_readable_value(&value, "bool", &abi).unwrap();
    let serialized = top_encode_to_vec_u8(&result).unwrap();
    assert_eq!(serialized, vec![1]); // should take only 2 bytes (top encoded)
}
