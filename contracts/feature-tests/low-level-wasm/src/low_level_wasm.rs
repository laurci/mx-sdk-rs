#![no_std]

elrond_wasm::imports!();

// (param $p0 i32) (param $p1 i32) (param $p2 i32) (param $p3 i32) (param $p4 i64) (param $p5 i32) (param $p6 i32) (param $p7 i32)
#[elrond_wasm_derive::contract]
pub trait LowLevelWasmTest {
    #[storage_set("arg")]
    fn set_arg_i32(&self, index: usize, value: i32);

    #[storage_set("arg")]
    fn set_arg_i64(&self, index: usize, value: i64);

    #[endpoint]
    fn many_args(&self) {
        let _ = BigUint::from(12345u32);
        // self.use_many_args(1, 2, 3, 4);
        self.direct_esdt_nft_execute(
            &Address::zero(),
            &TokenIdentifier::egld(),
            2,
            &BigUint::from(3u32),
            4,
            &[],
            &ArgBuffer::new(),
        );
    }

    #[inline(never)]
    fn direct_esdt_nft_execute(
        &self,
        to: &Address,
        token: &TokenIdentifier,
        nonce: u64,
        amount: &BigUint,
        gas_limit: u64,
        function: &[u8],
        arg_buffer: &ArgBuffer,
    ) {
        self.set_arg_i64(1, nonce as i64);
        let _ = amount.to_bytes_be();
    }

    #[inline(never)]
    fn use_many_args(&self, a1: i32, a2: i32, a3: i64, a4: i32) {
        self.set_arg_i32(1, a1);
        self.set_arg_i32(2, a2);
        self.set_arg_i64(3, a3);
        self.set_arg_i32(4, a4);
    }
}
