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
