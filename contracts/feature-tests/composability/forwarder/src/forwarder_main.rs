#![no_std]

mod call_async;
mod call_sync;
mod call_transf_exec;
mod contract_change_owner;
mod contract_deploy;
mod contract_update;
mod esdt;
mod nft;
mod roles;
mod sft;
mod storage;

elrond_wasm::imports!();

/// Test contract for investigating contract calls.
#[elrond_wasm_derive::contract]
pub trait Forwarder:
	call_sync::ForwarderSyncCallModule
	+ call_async::ForwarderAsyncCallModule
	+ call_transf_exec::ForwarderTransferExecuteModule
	+ contract_change_owner::ChangeOwnerModule
	+ contract_deploy::DeployContractModule
	+ contract_update::UpgradeContractModule
	+ esdt::ForwarderEsdtModule
	+ sft::ForwarderSftModule
	+ nft::ForwarderNftModule
	+ roles::ForwarderRolesModule
	+ storage::ForwarderStorageModule
{
	#[init]
	fn init(&self) {}

	#[endpoint]
	fn send_egld(
		&self,
		to: &Address,
		amount: &Self::BigUint,
		#[var_args] opt_data: OptionalArg<BoxedBytes>,
	) {
		let data = match &opt_data {
			OptionalArg::Some(data) => data.as_slice(),
			OptionalArg::None => &[],
		};
		self.send().direct_egld(to, amount, data);
	}

	#[payable("EGLD")]
	#[endpoint]
	fn accept_egld_transfer_nft(
		&self,
		nft_id: TokenIdentifier,
		nft_nonce: u64,
		nft_amount: Self::BigUint,
	) -> SCResult<()> {
		require!(nft_amount != 0, "Cannot transfer zero amount");
		let balance = self.blockchain().get_esdt_balance(
			&self.blockchain().get_sc_address(),
			&nft_id,
			nft_nonce,
		);
		require!(balance >= nft_amount, "Not enough NFT balance");

		SCResult::from_result(self.send().direct_esdt_nft_execute(
			&self.blockchain().get_caller(),
			&nft_id,
			nft_nonce,
			&nft_amount,
			0,
			&[],
			&ArgBuffer::new(),
		))
	}

	#[payable("*")]
	#[endpoint]
	fn accept_anything_transfer_nft(
		&self,
		nft_id: TokenIdentifier,
		nft_nonce: u64,
		nft_amount: Self::BigUint,
	) -> SCResult<()> {
		require!(nft_amount != 0, "Cannot transfer zero amount");
		let balance = self.blockchain().get_esdt_balance(
			&self.blockchain().get_sc_address(),
			&nft_id,
			nft_nonce,
		);
		require!(balance >= nft_amount, "Not enough NFT balance");

		SCResult::from_result(self.send().direct_esdt_nft_execute(
			&self.blockchain().get_caller(),
			&nft_id,
			nft_nonce,
			&nft_amount,
			0,
			&[],
			&ArgBuffer::new(),
		))
	}

	#[payable("*")]
	#[endpoint]
	fn accept_anything_with_payable_annotations_transfer_nft(
		&self,
		#[payment_token] _payment_token: TokenIdentifier,
		#[payment_nonce] _payment_nonce: u64,
		#[payment_amount] _payment_amount: Self::BigUint,
		nft_id: TokenIdentifier,
		nft_nonce: u64,
		nft_amount: Self::BigUint,
	) -> SCResult<()> {
		require!(nft_amount != 0, "Cannot transfer zero amount");
		let balance = self.blockchain().get_esdt_balance(
			&self.blockchain().get_sc_address(),
			&nft_id,
			nft_nonce,
		);
		require!(balance >= nft_amount, "Not enough NFT balance");

		SCResult::from_result(self.send().direct_esdt_nft_execute(
			&self.blockchain().get_caller(),
			&nft_id,
			nft_nonce,
			&nft_amount,
			0,
			&[],
			&ArgBuffer::new(),
		))
	}
}
