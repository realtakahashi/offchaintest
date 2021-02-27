#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, traits::Get, debug};
use frame_system::ensure_signed;
use sp_runtime::{
	offchain::http,
	transaction_validity::{
	  TransactionValidity, TransactionLongevity, ValidTransaction, InvalidTransaction
	}
};
use sp_std::vec::Vec;
//use parity_scale_codec::{Decode, Encode};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;


//use alt_serde::{Deserialize, Deserializer};

const HTTP_HEADER_USER_AGENT: &str = "realtakahashi";

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: frame_system::Trait {
	/// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
	// A unique name is used to ensure that the pallet's storage items are isolated.
	// This name may be updated, but each pallet in the runtime must use a unique name.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Trait> as TemplateModule {
		// Learn more about declaring storage items:
		// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
		Something get(fn something): Option<u32>;
	}
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, AccountId),
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Trait> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

		fn offchain_worker(block: T::BlockNumber) {
			match Self::fetch_data() {
			  Ok(res) => debug::info!("Result: {}", core::str::from_utf8(&res).unwrap()),
			  Err(e) => debug::error!("Error fetch_data: {}", e),
			};
		  }
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn do_something(origin, something: u32) -> dispatch::DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let who = ensure_signed(origin)?;

			// Update storage.
			Something::put(something);

			// Emit an event.
			Self::deposit_event(RawEvent::SomethingStored(something, who));
			// Return a successful DispatchResult
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
		pub fn cause_error(origin) -> dispatch::DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match Something::get() {
				// Return an error if the value has not been set.
				None => Err(Error::<T>::NoneValue)?,
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					Something::put(new);
					Ok(())
				},
			}
		}
	}
}

// #[serde(crate = "alt_serde")]
// #[derive(Deserialize, Encode, Decode, Default)]
// struct GithubInfo {
// 	// Specify our own deserializing function to convert JSON string to vector of bytes
// 	#[serde(deserialize_with = "de_string_to_bytes")]
// 	login: Vec<u8>,
// 	#[serde(deserialize_with = "de_string_to_bytes")]
// 	body: Vec<u8>,
// 	update_at: DateTime<Utc>,
// 	created_at: DateTime<Utc>,
// }

// impl fmt::Debug for GithubInfo {
// 	// `fmt` converts the vector of bytes inside the struct back to string for
// 	//   more friendly display.
// 	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
// 		write!(
// 			f,
// 			"{{ login: {}, body: {}, created_at: {}, update_at: {} }}",
// 			str::from_utf8(&self.login).map_err(|_| fmt::Error)?,
// 			str::from_utf8(&self.body).map_err(|_| fmt::Error)?,
// 			&self.created_at,
// 			&self.update_at,
// 		)
// 	}
// }

impl<T: Trait> Module<T> {
	fn fetch_data() -> Result<Vec<u8>, &'static str> {
  
	  // Specifying the request
//	  let pending = http::Request::get("https://min-api.cryptocompare.com/data/price?fsym=BTC&tsyms=USD")
	let pending = http::Request::get("https://api.github.com/repos/realtakahashi/faucet_pallet/issues/2/comments")
		.add_header("User-Agent", HTTP_HEADER_USER_AGENT)
		.send()
		.map_err(|_| "Error in sending http GET request")?;
  
	  // Waiting for the response
	  let response = pending.wait()
		.map_err(|_| "Error in waiting http response back")?;
  
	  // Check if the HTTP response is okay
	  if response.code != 200 {
		debug::warn!("Unexpected status code: {}", response.code);
		return Err("Non-200 status code returned from http request");
	  }
 
	//   let resp_bytes = response.body().collect::<Vec<u8>>();

	//   let resp_str = str::from_utf8(&resp_bytes).map_err(|_| <Error<T>>::HttpFetchingError)?;
	//   // Print out our fetched JSON string
	//   debug::info!("{}", resp_str);

	//   // Deserializing JSON to struct, thanks to `serde` and `serde_derive`
	//   let gh_info: GithubInfo =
	// 	  serde_json::from_str(&resp_str).map_err(|_| <Error<T>>::HttpFetchingError)?;

	//   debug::info!("{}", gh_info);
	  // Collect the result in the form of bytes
	  Ok(response.body().collect::<Vec<u8>>())
	}
  }