// use crate::*;
// use frame_support::{assert_ok, impl_outer_event, impl_outer_origin, parameter_types};
// use parity_scale_codec::{alloc::sync::Arc, Decode};
// use parking_lot::RwLock;
// use sp_core::{
// 	offchain::{
// 		testing::{self, OffchainState, PoolState},
// 		OffchainExt, TransactionPoolExt,
// 	},
// 	sr25519::{self, Signature},
// 	testing::KeyStore,
// 	traits::KeystoreExt,
// 	H256,
// };
// use sp_io::TestExternalities;
// use sp_runtime::{
// 	testing::{Header, TestXt},
// 	traits::{BlakeTwo256, IdentityLookup, Verify},
// 	Perbill,
// };

// use pallet_balances::{self as balances, Reasons};

// parameter_types! {
//     pub const ExistentialDeposit: u64 = 1; // Should be greather than zero
// }
// // impl pallet_balances::Trait for Test {
// //     type Balance = u64;
// // //    type Event = MetaEvent;
// // 	type Event = ();
// // 	type DustRemoval = ();
// //     type ExistentialDeposit = ExistentialDeposit;
// //     type AccountStore = StoredMap<Self::AccountId, AccountData<Self::Balance>>;
// // //    type AccountStore = ();
// //     type WeightInfo = ();
// //     type MaxLocks = ();
// // }

// impl_outer_origin! {
// 	pub enum Origin for Test  where system = system {}
// }

// // Configure a mock runtime to test the pallet.

// #[derive(Clone, Eq, PartialEq)]
// pub struct Test;
// parameter_types! {
// 	pub const BlockHashCount: u64 = 250;
// 	pub const MaximumBlockWeight: Weight = 1024;
// 	pub const MaximumBlockLength: u32 = 2 * 1024;
// 	pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
// }

// impl system::Trait for Test {
// 	type BaseCallFilter = ();
// 	type Origin = Origin;
// 	type Call = ();
// 	type Index = u64;
// 	type BlockNumber = u64;
// 	type Hash = H256;
// 	type Hashing = BlakeTwo256;
// 	type AccountId = u64;
// 	type Lookup = IdentityLookup<Self::AccountId>;
// 	type Header = Header;
// 	type Event = ();
// 	type BlockHashCount = BlockHashCount;
// 	type MaximumBlockWeight = MaximumBlockWeight;
// 	type DbWeight = ();
// 	type BlockExecutionWeight = ();
// 	type ExtrinsicBaseWeight = ();
// 	type MaximumExtrinsicWeight = MaximumBlockWeight;
// 	type MaximumBlockLength = MaximumBlockLength;
// 	type AvailableBlockRatio = AvailableBlockRatio;
// 	type Version = ();
// 	type PalletInfo = ();
// 	type AccountData = ();
// 	type OnNewAccount = ();
// 	type OnKilledAccount = ();
// 	type SystemWeightInfo = ();
// }

// impl Trait for Test {
// 	type Event = ();
// 	type AuthorityId = crypto::TestAuthId;
// 	type Call = Call<Test>;
// 	type Currency = Balances;
// }


// use crate as pallet_offchain;

// impl_outer_event! {
// 	pub enum TestEvent for Test {
// 		system<T>,
// 		pallet_offchain<T>,
// 	}
// }

// // --- mocking offchain-demo trait

// type TestExtrinsic = TestXt<Call<Test>, ()>;

// parameter_types! {
// 	pub const UnsignedPriority: u64 = 100;
// }

// // impl Trait for TestRuntime {
// // 	type AuthorityId = crypto::TestAuthId;
// // 	type Call = Call<TestRuntime>;
// // 	type Event = TestEvent;
// // 	type Currency = ;
// // }

// impl<LocalCall> system::offchain::CreateSignedTransaction<LocalCall> for Test
// where
// 	Call<Test>: From<LocalCall>,
// {
// 	fn create_transaction<C: frame_system::offchain::AppCrypto<Self::Public, Self::Signature>>(
// 		call: Call<Test>,
// 		_public: <Signature as Verify>::Signer,
// 		_account: <Test as system::Trait>::AccountId,
// 		index: <Test as system::Trait>::Index,
// 	) -> Option<(
// 		Call<Test>,
// 		<TestExtrinsic as sp_runtime::traits::Extrinsic>::SignaturePayload,
// 	)> {
// 		Some((call, (index, ())))
// 	}
// }

// impl frame_system::offchain::SigningTypes for Test {
// 	type Public = <Signature as Verify>::Signer;
// 	type Signature = Signature;
// }

// impl<C> frame_system::offchain::SendTransactionTypes<C> for Test
// where
// 	Call<Test>: From<C>,
// {
// 	type OverarchingCall = Call<Test>;
// 	type Extrinsic = TestExtrinsic;
// }

// pub type TemplateModule = Module<Test>;
// pub type Balances = pallet_balances::Module<Test>;

// // Build genesis storage according to the mock runtime.
// pub fn new_test_ext() -> sp_io::TestExternalities {
// 	system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
// }
