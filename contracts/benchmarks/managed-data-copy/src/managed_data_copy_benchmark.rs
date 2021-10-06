#![no_std]

elrond_wasm::imports!();

#[elrond_wasm::contract]
pub trait ManagedDataCopyBenchmark {
    #[init]
    fn init(&self) {}

    #[endpoint]
    fn concat_numbers(&self, n: usize) -> ManagedVec<usize> {
        let mut result = ManagedVec::new();
        for i in 1..=n {
            result.push(i);
        }
        result
    }

    #[endpoint]
    fn sum(&self, n: usize) -> usize {
        let long_vec = self.concat_numbers(n);
        let mut sum = 0;
        for i in long_vec.into_iter() {
            sum += i;
        }
        sum
    }
}
