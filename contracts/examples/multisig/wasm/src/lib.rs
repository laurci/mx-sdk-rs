// Code generated by the multiversx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           20
// Async Callback:                       1
// Total number of exported functions:  22

#![no_std]
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    multisig
    (
        deposit
        signed
        sign
        unsign
        discardAction
        getQuorum
        getNumBoardMembers
        getNumProposers
        getActionLastIndex
        proposeAddBoardMember
        proposeAddProposer
        proposeRemoveUser
        proposeChangeQuorum
        proposeTransferExecute
        proposeAsyncCall
        proposeSCDeployFromSource
        proposeSCUpgradeFromSource
        quorumReached
        performAction
        dnsRegister
        callBack
    )
}
