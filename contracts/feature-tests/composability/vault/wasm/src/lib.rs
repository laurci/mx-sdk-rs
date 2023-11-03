// Code generated by the multiversx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           16
// Async Callback (empty):               1
// Total number of exported functions:  18

#![no_std]

// Configuration that works with rustc < 1.73.0.
// TODO: Recommended rustc version: 1.73.0 or newer.
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    vault
    (
        init => init
        upgrade => upgrade
        echo_arguments => echo_arguments
        echo_arguments_without_storage => echo_arguments_without_storage
        echo_caller => echo_caller
        accept_funds => accept_funds
        accept_funds_echo_payment => accept_funds_echo_payment
        accept_funds_single_esdt_transfer => accept_funds_single_esdt_transfer
        reject_funds => reject_funds
        retrieve_funds_with_transfer_exec => retrieve_funds_with_transfer_exec
        retrieve_funds => retrieve_funds
        retrieve_multi_funds_async => retrieve_multi_funds_async
        burn_and_create_retrive_async => burn_and_create_retrive_async
        get_owner_address => get_owner_address
        call_counts => call_counts
        num_called_retrieve_funds_promises => num_called_retrieve_funds_promises
        num_async_calls_sent_from_child => num_async_calls_sent_from_child
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
