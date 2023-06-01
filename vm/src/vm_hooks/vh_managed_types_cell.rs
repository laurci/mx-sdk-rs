use std::cell::{Ref, RefCell, RefMut};

use crate::tx_mock::TxManagedTypes;

use super::{
    ManagedTypesSource, VMHooksBigInt, VMHooksError, VMHooksManagedBuffer, VMHooksManagedTypes,
};

#[derive(Debug, Default)]
pub struct TxManagedTypesCell(RefCell<TxManagedTypes>);

impl ManagedTypesSource for TxManagedTypesCell {
    fn m_types_borrow(&self) -> Ref<TxManagedTypes> {
        self.0.borrow()
    }

    fn m_types_borrow_mut(&self) -> RefMut<TxManagedTypes> {
        self.0.borrow_mut()
    }
}

impl VMHooksError for TxManagedTypesCell {}
impl VMHooksBigInt for TxManagedTypesCell {}
impl VMHooksManagedBuffer for TxManagedTypesCell {}
impl VMHooksManagedTypes for TxManagedTypesCell {}
