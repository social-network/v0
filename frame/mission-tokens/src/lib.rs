#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, dispatch, ensure, traits::Get, Parameter,
};
use frame_system::ensure_signed;
use sp_runtime::traits::{AtLeast32BitUnsigned, Member, StaticLookup, Zero};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub trait Trait: frame_system::Trait {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
    type Balance: Member + Parameter + AtLeast32BitUnsigned + Default + Copy;
    type MissionTokenId: Parameter + AtLeast32BitUnsigned + Default + Copy;
}

decl_storage! {
    trait Store for Module<T: Trait> as MissionTokens {
        Balances: map hasher(blake2_128_concat) (T::MissionTokenId, T::AccountId) => T::Balance;
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Trait>::AccountId,
        Balance = <T as Trait>::Balance,
        MissionTokenId = <T as Trait>::MissionTokenId,
    {
        /// Some assets were transferred. \[asset_id, from, to, amount\]
        Transferred(MissionTokenId, AccountId, AccountId, Balance),
    }
);

decl_error! {
    pub enum Error for Module<T: Trait> {
        /// Transfer amount should be non-zero
        AmountZero,
        /// Account balance must be greater than or equal to the transfer amount
        BalanceLow,
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        type Error = Error<T>;

        fn deposit_event() = default;

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn transfer(
            origin,
            #[compact] id: T::MissionTokenId,
            target: <T::Lookup as StaticLookup>::Source,
            #[compact] amount: T::Balance
        ) -> dispatch::DispatchResult {
            let origin = ensure_signed(origin)?;
            let origin_account = (id, origin.clone());
            let origin_balance = <Balances<T>>::get(&origin_account);
            let target = T::Lookup::lookup(target)?;

            ensure!(!amount.is_zero(), Error::<T>::AmountZero);
            ensure!(origin_balance >= amount, Error::<T>::BalanceLow);

            Self::deposit_event(RawEvent::Transferred(id, origin, target.clone(), amount));
            <Balances<T>>::insert(origin_account, origin_balance - amount);
            <Balances<T>>::mutate((id, target), |balance| *balance += amount);

            Ok(())
        }
    }
}

impl<T: Trait> Module<T> {
    pub fn balance(id: T::MissionTokenId, who: T::AccountId) -> T::Balance {
        <Balances<T>>::get((id, who))
    }

    pub fn mint(id: T::MissionTokenId, target: T::AccountId, amount: T::Balance) {
        <Balances<T>>::mutate((id, target), |balance| *balance += amount)
    }

    pub fn burn(id: T::MissionTokenId, target: T::AccountId, amount: T::Balance) {
        <Balances<T>>::mutate((id, target), |balance| *balance -= amount)
    }
}
