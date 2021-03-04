#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

use core::{convert::TryInto, fmt};
use frame_support::{
	debug, decl_error, decl_event, decl_module, decl_storage, dispatch::DispatchResult,dispatch,
};
use parity_scale_codec::{Decode, Encode};

use frame_system::{
	self as system, ensure_none, ensure_signed,
	offchain::{
		AppCrypto, CreateSignedTransaction, SendSignedTransaction, SendUnsignedTransaction,
		SignedPayload, SigningTypes, Signer, SubmitTransaction,
	},
};
use sp_core::crypto::KeyTypeId;
use sp_runtime::{
	RuntimeDebug,
	offchain as rt_offchain,
	offchain::{
		http,
		storage::StorageValueRef,
		storage_lock::{StorageLock, BlockAndTime},
	},
	transaction_validity::{
		InvalidTransaction, TransactionSource, TransactionValidity,
		ValidTransaction,
	},
};
use sp_std::{
	prelude::*, str,
	collections::vec_deque::VecDeque,
};
use alt_serde::{Deserialize, Deserializer};
use sp_runtime::offchain::Timestamp;
use chrono::{NaiveDateTime, Duration};

const FAUCET_CHECK_INTERVAL: u64 = 60000;

#[derive(Clone, Eq, PartialEq, Default, Encode, Decode, Hash, Debug)]
//#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct FaucetData {
    pub id: u64,
	pub login: Vec<u8>,
    pub created_at: Vec<u8>,
}

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
		LatestFaucetData get(fn latest_faucet_data): Option<FaucetData>;
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
		// Error returned when fetching github info
		HttpFetchingError,
		OffchainSignedTxError, // todo : need to change
		NoLocalAcctForSigning, // todo : need to change

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
			let value = match LatestFaucetData::get() {
				None => None,
				Some(old) => Some(old),
			};
			debug::info!("value is {:#?}",value);
			
			Self::check_faucet_datas(value);
		}

		#[weight = 10000]
		pub fn send_some_testnet_token(origin, faucet_data: FaucetData) -> DispatchResult {
			let who = ensure_signed(origin)?;
			// todo: check who
			debug::info!("submit_number_signed: ({}, {:?})", faucet_data.id, who);
			let latest_faucet_data = match LatestFaucetData::get(){
				Some(data) => data,
				None => {
					// todo: transfer token
					debug::info!("Need to transfer token.");
					LatestFaucetData::put(faucet_data);
					return Ok(());
				},
			};
			if faucet_data.id != latest_faucet_data.id {
				// todo: need to change error.
				let param_created_at_str = str::from_utf8(&faucet_data.created_at).map_err(|_| Error::<T>::NoneValue)?;
				let latest_created_at_str = str::from_utf8(&latest_faucet_data.created_at).map_err(|_| Error::<T>::NoneValue)?;
				let created_at_for_param_data = NaiveDateTime::parse_from_str(param_created_at_str, "%Y/%m/%d %H:%M:%S").unwrap();
				let created_at_for_latest_data = NaiveDateTime::parse_from_str(latest_created_at_str, "%Y/%m/%d %H:%M:%S").unwrap();
				let duration: Duration = created_at_for_param_data - created_at_for_latest_data;
				if duration.num_seconds() >= 0 {
					// todo: token transfer
					LatestFaucetData::put(faucet_data);
				}
			}
			Ok(())
		}

		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[weight = 10_000]
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
		#[weight = 10_000]
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

const HTTP_HEADER_USER_AGENT: &str = "realtakahashi";
const HTTP_REMOTE_REQUEST: &str = "https://api.github.com/repos/realtakahashi/faucet_pallet/issues/2/comments";

#[serde(crate = "alt_serde")]
#[derive(Deserialize, Encode, Decode, Default)]
struct User{
	#[serde(deserialize_with = "de_string_to_bytes")]
	login: Vec<u8>,
	id: u64,
	#[serde(deserialize_with = "de_string_to_bytes")]
	node_id: Vec<u8>,
	#[serde(deserialize_with = "de_string_to_bytes")]
	avatar_url: Vec<u8>,
	#[serde(deserialize_with = "de_string_to_bytes")]
	gravatar_id: Vec<u8>,
	#[serde(deserialize_with = "de_string_to_bytes")]
	url: Vec<u8>,
	#[serde(deserialize_with = "de_string_to_bytes")]
	html_url: Vec<u8>,
	#[serde(deserialize_with = "de_string_to_bytes")]
	followers_url: Vec<u8>,
	#[serde(deserialize_with = "de_string_to_bytes")]
	following_url: Vec<u8>,
	#[serde(deserialize_with = "de_string_to_bytes")]
	gists_url: Vec<u8>,
	#[serde(deserialize_with = "de_string_to_bytes")]
	starred_url: Vec<u8>,
	#[serde(deserialize_with = "de_string_to_bytes")]
	subscriptions_url: Vec<u8>,
	#[serde(deserialize_with = "de_string_to_bytes")]
	organizations_url: Vec<u8>,
	#[serde(deserialize_with = "de_string_to_bytes")]
	repos_url: Vec<u8>,
	#[serde(deserialize_with = "de_string_to_bytes")]
	events_url: Vec<u8>,
	#[serde(deserialize_with = "de_string_to_bytes")]
	received_events_url: Vec<u8>,
	#[serde(deserialize_with = "de_string_to_bytes")]
	r#type: Vec<u8>,
	site_admin: bool,
}

#[serde(crate = "alt_serde")]
#[derive(Deserialize, Encode, Decode, Default)]
struct GithubInfo {
	//Specify our own deserializing function to convert JSON string to vector of bytes
	#[serde(deserialize_with = "de_string_to_bytes")]
	url: Vec<u8>,
	id: u64,
	#[serde(deserialize_with = "de_string_to_bytes")]
	node_id: Vec<u8>,
	user: User,
	#[serde(deserialize_with = "de_string_to_bytes")]
	created_at: Vec<u8>,
	#[serde(deserialize_with = "de_string_to_bytes")]
	updated_at: Vec<u8>,
	#[serde(deserialize_with = "de_string_to_bytes")]
	body: Vec<u8>,
}

// struct GithubInfo {
// 	// Specify our own deserializing function to convert JSON string to vector of bytes
// 	#[serde(deserialize_with = "de_string_to_bytes")]
// 	login: Vec<u8>,
// 	#[serde(deserialize_with = "de_string_to_bytes")]
// 	blog: Vec<u8>,
// 	public_repos: u32,
// }

pub fn de_string_to_bytes<'de, D>(de: D) -> Result<Vec<u8>, D::Error>
where
	D: Deserializer<'de>,
{
	let s: &str = Deserialize::deserialize(de)?;
	Ok(s.as_bytes().to_vec())
}

impl fmt::Debug for GithubInfo {
	// `fmt` converts the vector of bytes inside the struct back to string for
	//   more friendly display.
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{{ url: {}, id: {}, node_id: {}, login: {}, created_at: {}, updated_at: {}, body: {} }}",
			str::from_utf8(&self.url).map_err(|_| fmt::Error)?,
			self.id,
			str::from_utf8(&self.node_id).map_err(|_| fmt::Error)?,
			str::from_utf8(&self.user.login).map_err(|_| fmt::Error)?,
			str::from_utf8(&self.created_at).map_err(|_| fmt::Error)?,
			str::from_utf8(&self.updated_at).map_err(|_| fmt::Error)?,
			str::from_utf8(&self.body).map_err(|_| fmt::Error)?,
		)
	}
}

impl<T: Trait> Module<T> {
	fn check_faucet_datas(faucet_data:Option<FaucetData>){
		let last_check_time = StorageValueRef::persistent(b"offchain-test::last_check_time");
		let now = sp_io::offchain::timestamp().unix_millis();
		if let Some(Some(last_check_time)) = last_check_time.get::<u64>() {
			debug::info!("last_check_time: {:?}", last_check_time);
			if now - last_check_time < FAUCET_CHECK_INTERVAL{
				return;
			}
		}		

		debug::info!("######## faucet executed");
		last_check_time.set(&now);

		let g_info = match Self::fetch_data(){
			Ok(res)=>res,
			Err(e)=>{
				debug::error!("Error fetch_data: {}", e);
				return
			}
		};

		let target_faucet_data = match Self::check_fetch_data(g_info){
			Ok(res)=>res,
			Err(e)=>{
				debug::error!("Error check_data: {}", e);
				return
			}
		};

		Self::offchain_signed_tx(target_faucet_data);
	}

	fn check_fetch_data(g_info Vec<GithubInfo>) -> Result<FaucetData, &'static str>{
		
	}

	fn fetch_data() -> Result<Vec<GithubInfo>, &'static str> {
	  // Specifying the request
	  let pending = http::Request::get(HTTP_REMOTE_REQUEST)
		.add_header("User-Agent", HTTP_HEADER_USER_AGENT)
		.add_header("Accept-Charset", "UTF-8")
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
 
	  let resp_bytes = response.body().collect::<Vec<u8>>();

	  let resp_str = str::from_utf8(&resp_bytes).map_err(|_| <Error<T>>::HttpFetchingError)?;
	  // Print out our fetched JSON string
	  debug::info!("$$$$ response string $$$$");
	  debug::info!("{}", resp_str);

	  let resp_str2 = resp_str.replace(r"\r\n","<br>");
	  debug::info!("$$$$ response string22222 $$$$");
	  debug::info!("{}", resp_str2);

	  // Deserializing JSON to struct, thanks to `serde` and `serde_derive`
	  let gh_info: Vec<GithubInfo> =
		  serde_json::from_str(&resp_str2).map_err(|_| <Error<T>>::HttpFetchingError)?;

	  debug::info!("$$$$ json string $$$$");
	  debug::info!("{:#?}", gh_info);
	  // Collect the result in the form of bytes
	  Ok(gh_info)
	}

	fn offchain_signed_tx(faucet_data: FaucetData) -> Result<(), Error<T>> {
		// We retrieve a signer and check if it is valid.
		//   Since this pallet only has one key in the keystore. We use `any_account()1 to
		//   retrieve it. If there are multiple keys and we want to pinpoint it, `with_filter()` can be chained,
		//   ref: https://substrate.dev/rustdocs/v2.0.0/frame_system/offchain/struct.Signer.html
		let signer = Signer::<T, T::AuthorityId>::any_account();

		// `result` is in the type of `Option<(Account<T>, Result<(), ()>)>`. It is:
		//   - `None`: no account is available for sending transaction
		//   - `Some((account, Ok(())))`: transaction is successfully sent
		//   - `Some((account, Err(())))`: error occured when sending the transaction
		let result = signer.send_signed_transaction(|_acct|
			// This is the on-chain function
			Call::send_some_testnet_token(faucet_data)
		);

		// Display error if the signed tx fails.
		if let Some((acc, res)) = result {
			if res.is_err() {
				debug::error!("failure: offchain_signed_tx: tx sent: {:?}", acc.id);
				return Err(<Error<T>>::OffchainSignedTxError);
			}
			// Transaction is sent successfully
			return Ok(());
		}

		// The case of `None`: no account is available for sending
		debug::error!("No local account available");
		Err(<Error<T>>::NoLocalAcctForSigning)
	}
}