#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, dispatch, ensure, traits::Get, Parameter,
};
use frame_system::ensure_signed;
use sp_runtime::traits::AtLeast32Bit;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub trait Trait: frame_system::Trait {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
    type MissionId: Parameter + AtLeast32Bit + Default + Copy;
}

decl_storage! {
    trait Store for Module<T: Trait> as ValidatorRegistry {
        MissionOf get(fn mission_of): map hasher(blake2_128_concat) T::AccountId => T::MissionId;
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Trait>::AccountId,
        MissionId = <T as Trait>::MissionId,
    {
        Registered(AccountId, MissionId),
        Unregistered(AccountId, MissionId),
    }
);

decl_error! {
    pub enum Error for Module<T: Trait> {
        AlreadyRegistered,
        NotFound,
        InvalidMissionId,
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        type Error = Error<T>;

        fn deposit_event() = default;

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn register(origin, mission_id: T::MissionId) -> dispatch::DispatchResult {
            let validator = ensure_signed(origin)?;

            Self::validate_mission_id(mission_id)?;
            ensure!(!<MissionOf<T>>::contains_key(&validator), Error::<T>::AlreadyRegistered);

            <MissionOf<T>>::insert(&validator, mission_id);

            Self::deposit_event(RawEvent::Registered(validator, mission_id));
            Ok(())
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn unregister(origin) -> dispatch::DispatchResult {
            let validator = ensure_signed(origin)?;

            ensure!(<MissionOf<T>>::contains_key(&validator), Error::<T>::NotFound);

            let mission_id = <MissionOf<T>>::get(&validator);
            <MissionOf<T>>::remove(&validator);

            Self::deposit_event(RawEvent::Unregistered(validator, mission_id));
            Ok(())
        }
    }
}

impl<T: Trait> Module<T> {
    fn validate_mission_id(mission_id: T::MissionId) -> dispatch::DispatchResult {
        ensure!(
            mission_id > 0.into() && mission_id < 13.into(),
            Error::<T>::InvalidMissionId
        );

        Ok(())
    }
}
