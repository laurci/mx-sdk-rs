// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           27
// Async Callback (empty):               1
// Total number of exported functions:  29

#![no_std]

// Configuration that works with rustc < 1.73.0.
// TODO: Recommended rustc version: 1.73.0 or newer.
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    abi_tester
    (
        init => init
        echo_abi_test_type => echo_abi_test_type
        echo_enum => echo_enum
        take_managed_type => take_managed_type
        multi_result_3 => multi_result_3
        multi_result_4 => multi_result_4
        var_args => var_args
        multi_result_vec => multi_result_vec
        optional_arg => optional_arg
        optional_result => optional_result
        address_vs_h256 => address_vs_h256
        managed_address_vs_byte_array => managed_address_vs_byte_array
        esdt_local_role => esdt_local_role
        esdt_token_payment => esdt_token_payment
        esdt_token_data => esdt_token_data
        sample_storage_mapper => sample_storage_mapper
        item_for_vec => item_for_vec
        item_for_array_vec => item_for_array_vec
        item_for_managed_vec => item_for_managed_vec
        item_for_array => item_for_array
        item_for_box => item_for_box
        item_for_boxed_slice => item_for_boxed_slice
        item_for_ref => item_for_ref
        item_for_slice => item_for_slice
        item_for_option => item_for_option
        payable_egld => payable_egld
        payable_some_token => payable_some_token
        payable_any_token => payable_any_token
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
