use std::fmt::Debug;

use multiversx_sc::api::RawHandle;

use crate::tx_mock::TxManagedTypes;

use super::{VMHooksBigInt, VMHooksError, VMHooksManagedBuffer};

pub trait VMHooksManagedTypes: VMHooksBigInt + VMHooksManagedBuffer + VMHooksError + Debug {
    fn mb_to_big_int_unsigned(&self, buffer_handle: RawHandle, bi_handle: RawHandle) {
        let bytes = self.m_types_borrow().mb_to_boxed_bytes(buffer_handle);
        self.m_types_borrow_mut()
            .bi_set_unsigned_bytes(bi_handle, bytes.as_slice());
    }

    fn mb_to_big_int_signed(&self, buffer_handle: RawHandle, bi_handle: RawHandle) {
        let bytes = self.m_types_borrow().mb_to_boxed_bytes(buffer_handle);
        self.m_types_borrow_mut()
            .bi_set_signed_bytes(bi_handle, bytes.as_slice());
    }

    fn mb_from_big_int_unsigned(&self, buffer_handle: RawHandle, bi_handle: RawHandle) {
        let bi_bytes = self.m_types_borrow().bi_get_unsigned_bytes(bi_handle);
        self.m_types_borrow_mut()
            .mb_set(buffer_handle, bi_bytes.into_vec());
    }

    fn mb_from_big_int_signed(&self, buffer_handle: RawHandle, bi_handle: RawHandle) {
        let bi_bytes = self.m_types_borrow().bi_get_signed_bytes(bi_handle);
        self.m_types_borrow_mut()
            .mb_set(buffer_handle, bi_bytes.into_vec());
    }
}
