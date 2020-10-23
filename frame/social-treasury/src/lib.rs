#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_error, decl_event, decl_module, decl_storage, dispatch, traits::Get};
use frame_system::ensure_signed;
use pallet_staking::EraIndex;
use sp_runtime::traits::UniqueSaturatedInto;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub trait Trait:
    frame_system::Trait
    + pallet_treasury::Trait
    + pallet_staking::Trait
    + pallet_mission_tokens::Trait
    + pallet_validator_registry::Trait
{
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

decl_storage! {
    trait Store for Module<T: Trait> as SocialTreasury {
        Something get(fn something): Option<u32>;
        NextEraForProcessing get(fn next_era_for_processing): Option<EraIndex>;
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Trait>::AccountId,
    {
        SomethingStored(u32, AccountId),
    }
);

decl_error! {
    pub enum Error for Module<T: Trait> {
        /// Error names should be descriptive.
        NoneValue,
        /// Errors should have helpful documentation associated with them.
        StorageOverflow,
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        type Error = Error<T>;

        fn deposit_event() = default;

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn do_something(origin, something: u32) -> dispatch::DispatchResult {
            let who = ensure_signed(origin)?;

            Something::put(something);

            Self::deposit_event(RawEvent::SomethingStored(something, who));
            Ok(())
        }

        #[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
        pub fn cause_error(origin) -> dispatch::DispatchResult {
            let _who = ensure_signed(origin)?;

            match Something::get() {
                None => Err(Error::<T>::NoneValue.into()),
                Some(old) => {
                    let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
                    Something::put(new);
                    Ok(())
                },
            }
        }

        fn on_finalize() {
            match <pallet_staking::Module<T>>::current_era() {
                None => (),
                Some(current_era) => {
                    let era = NextEraForProcessing::get().unwrap_or(0);
                    if era < current_era {
                        let reward_points = <pallet_staking::Module<T>>::eras_reward_points(era);
                        let treasury_account_id = <pallet_treasury::Module<T>>::account_id();

                        for (account_id, points) in reward_points.individual {
                            let mission_id = <pallet_validator_registry::Module<T>>::mission_of(account_id);
                            if mission_id > 0.into() {
                                let mission_token_id = (mission_id.unique_saturated_into() as u32).into();
                                <pallet_mission_tokens::Module<T>>::mint(
                                    mission_token_id,
                                    treasury_account_id.clone(),
                                    points.into()
                                );
                            }
                        }

                        let next_era = era + 1;
                        NextEraForProcessing::put(next_era);
                    }
                }
            }
        }
    }
}
