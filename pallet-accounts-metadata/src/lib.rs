#![cfg_attr(not(feature = "std"), no_std)]

//! A Pallet to demonstrate using currency imbalances
//!
//! WARNING: never use this code in production (for demonstration/teaching purposes only)
//! it only checks for signed extrinsics to enable arbitrary minting/slashing!!!

use frame_support::{
	decl_event, decl_module, decl_storage, decl_error,
	traits::{Currency, ReservableCurrency, Vec},
};
use frame_system::{self as system, ensure_signed};
use sp_runtime::{DispatchResult};
use codec::{Decode, Encode};
use fixed::{types::U16F16};

pub trait Trait: system::Trait {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;

	/// Currency type for this pallet.
	type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
}

decl_event!(
	pub enum Event<T>
	where
		AccountId = <T as system::Trait>::AccountId,
	{
		MetadataStored(AccountId, Metadata),
	}
);

decl_error! {
	/// Error for the vesting module.
	pub enum Error for Module<T: Trait> {
		MetadataExists,

	}
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Default, Debug)]
pub struct Metadata {
    vesting: LockupMetadata,
    lockup: LockupMetadata,
    minting: MintingMetadata
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Default, Debug)]
pub struct LockupMetadata {
    min_price: U16F16,
    // number of months
    period: u32
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Default, Debug)]
pub struct MintingMetadata {
    reward: U16F16,
    currency: Vec<u8>,
    start_period: u32,
    // number of months
    period: u32
}

decl_storage! {
	trait Store for Module<T: Trait> as TFTBridgeModule {
        pub AccountsMetadata get(fn accounts_metadata): map hasher(blake2_128_concat) T::AccountId => Metadata;
    }
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

        #[weight = 10_000]
		fn propose_metadata(origin, target: T::AccountId, meta: Metadata){
            ensure_signed(origin)?;
            return Self::update_or_insert_metadata(target, meta);
		}
	}
}

impl<T: Trait> Module<T> {
	pub fn update_or_insert_metadata(target: T::AccountId, meta: Metadata) -> DispatchResult {        
        AccountsMetadata::<T>::insert(&target, meta.clone());

        Self::deposit_event(RawEvent::MetadataStored(target, meta));

        Ok(())
    }
}
