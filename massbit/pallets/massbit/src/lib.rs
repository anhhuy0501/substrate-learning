#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Encode, Decode};
use frame_support::{
	decl_module, decl_storage, decl_event, decl_error, StorageValue, StorageDoubleMap, Parameter,
/*traits::Randomness,*/ RuntimeDebug, dispatch::{DispatchError, /*DispatchResult*/},
};
/*use sp_io::hashing::blake2_128;*/
use frame_system::ensure_signed;
use sp_runtime::traits::{AtLeast32BitUnsigned, Bounded,  One, CheckedAdd};

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
pub struct Worker{
	pub ip: u32,
}

pub trait Trait: pallet_balances::Trait{
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
	type WorkerIndex: Parameter + AtLeast32BitUnsigned + Bounded + Default + Copy;
}
decl_storage! {
	trait Store for Module<T: Trait> as Massbit {
		/// Stores all the worker
		pub Workers get(fn workers): double_map hasher(blake2_128_concat) T::AccountId, hasher(blake2_128_concat) T::WorkerIndex => Option<Worker>;
		/// Stores the next worker ID
		pub NextWorkerId get(fn next_worker_id): T::WorkerIndex;
	}
}

decl_event! {
	pub enum Event<T> where
		<T as frame_system::Trait>::AccountId,
		<T as Trait>::WorkerIndex,
		//<T as pallet_balances::Trait>::Balance,
	{
		/// A Worker is regitered. \[owner, worker_id, worker\]
		WorkerRegistered(AccountId, WorkerIndex, Worker),
		
	}
}

decl_error! {
	pub enum Error for Module<T: Trait> {
		WorkersIdOverflow,
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		type Error = Error<T>;

		fn deposit_event() = default;

		/// Create a new worker
		#[weight = 1000]
		pub fn create(origin, ip: u32) {
			let sender = ensure_signed(origin)?;

			let worker_id = Self::get_next_worker_id()?;

			// Create and store worker
			let worker = Worker{
				ip: ip,
			};
			Workers::<T>::insert(&sender, worker_id, worker.clone());
			

			// Emit event
			Self::deposit_event(RawEvent::WorkerRegistered(sender, worker_id, worker))
		}
	}
}
impl<T: Trait> Module<T> {
	fn get_next_worker_id() -> sp_std::result::Result<T::WorkerIndex, DispatchError> {
		NextWorkerId::<T>::try_mutate(|next_id| -> sp_std::result::Result<T::WorkerIndex, DispatchError> {
			let current_id = *next_id;
			*next_id = next_id.checked_add(&One::one()).ok_or(Error::<T>::WorkersIdOverflow)?;
			Ok(current_id)
		})
	}
}