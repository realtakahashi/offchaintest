use crate::*;
use frame_support::{assert_ok, assert_noop, impl_outer_event, impl_outer_origin, parameter_types};
use parity_scale_codec::{alloc::sync::Arc};
use parking_lot::RwLock;
use sp_core::{
	offchain::{
		testing::{self, OffchainState, PoolState},
		OffchainExt, TransactionPoolExt,
	},
	sr25519::{self, Signature},
	testing::KeyStore,
	traits::KeystoreExt,
	H256,
};
use sp_io::TestExternalities;
use sp_runtime::{
	testing::{Header, TestXt},
	traits::{BlakeTwo256, IdentityLookup, Verify},
	Perbill,
};
use pallet_balances;

use crate as pallet_offchain;

impl_outer_origin! {
	pub enum Origin for TestRuntime where system = system {}
}

impl_outer_event! {
	pub enum TestEvent for TestRuntime {
		system<T>,
		pallet_offchain<T>,
		pallet_balances<T>,
	}
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TestRuntime;

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const MaximumBlockWeight: u32 = 1_000_000;
	pub const MaximumBlockLength: u32 = 10 * 1_000_000;
	pub const AvailableBlockRatio: Perbill = Perbill::one();
	pub const ExistentialDeposit: u64 = 1;
}

// The TestRuntime implements two pallet/frame traits: system, and simple_event
impl system::Trait for TestRuntime {
	type BaseCallFilter = ();
	type Origin = Origin;
	type Index = u64;
	type Call = ();
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = sr25519::Public;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = TestEvent;
	type BlockHashCount = BlockHashCount;
	type MaximumBlockWeight = MaximumBlockWeight;
	type DbWeight = ();
	type BlockExecutionWeight = ();
	type ExtrinsicBaseWeight = ();
	type MaximumExtrinsicWeight = MaximumBlockWeight;
	type MaximumBlockLength = MaximumBlockLength;
	type AvailableBlockRatio = AvailableBlockRatio;
	type Version = ();
	type PalletInfo = ();
	// type AccountData = ();
	type AccountData = pallet_balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
}

impl pallet_balances::Trait for TestRuntime {
	type Balance = u64;
	type MaxLocks = ();
	type Event = TestEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = system::Module<TestRuntime>;
	type WeightInfo = ();
}
// --- mocking offchain-demo trait

type TestExtrinsic = TestXt<Call<TestRuntime>, ()>;

parameter_types! {
	pub const UnsignedPriority: u64 = 100;
}

impl Trait for TestRuntime {
	type AuthorityId = crypto::TestAuthId;
	type Call = Call<TestRuntime>;
	type Event = TestEvent;
	type Currency = pallet_balances::Module<Self>;
}

impl<LocalCall> system::offchain::CreateSignedTransaction<LocalCall> for TestRuntime
where
	Call<TestRuntime>: From<LocalCall>,
{
	fn create_transaction<C: frame_system::offchain::AppCrypto<Self::Public, Self::Signature>>(
		call: Call<TestRuntime>,
		_public: <Signature as Verify>::Signer,
		_account: <TestRuntime as system::Trait>::AccountId,
		index: <TestRuntime as system::Trait>::Index,
	) -> Option<(
		Call<TestRuntime>,
		<TestExtrinsic as sp_runtime::traits::Extrinsic>::SignaturePayload,
	)> {
		Some((call, (index, ())))
	}
}

impl frame_system::offchain::SigningTypes for TestRuntime {
	type Public = <Signature as Verify>::Signer;
	type Signature = Signature;
}

impl<C> frame_system::offchain::SendTransactionTypes<C> for TestRuntime
where
	Call<TestRuntime>: From<C>,
{
	type OverarchingCall = Call<TestRuntime>;
	type Extrinsic = TestExtrinsic;
}

pub type System = system::Module<TestRuntime>;
pub type OffChainTest = Module<TestRuntime>;

struct ExternalityBuilder;

impl ExternalityBuilder {
	pub fn build() -> (
		TestExternalities,
		Arc<RwLock<PoolState>>,
		Arc<RwLock<OffchainState>>,
	) {
		const PHRASE: &str =
			"expire stage crawl shell boss any story swamp skull yellow bamboo copy";

		let (offchain, offchain_state) = testing::TestOffchainExt::new();
		let (pool, pool_state) = testing::TestTransactionPoolExt::new();
		let keystore = KeyStore::new();
		keystore
			.write()
			.sr25519_generate_new(KEY_TYPE, Some(&format!("{}/hunter1", PHRASE)))
			.unwrap();

		let storage = system::GenesisConfig::default()
			.build_storage::<TestRuntime>()
			.unwrap();

		let mut t = TestExternalities::from(storage);
		t.register_extension(OffchainExt::new(offchain));
		t.register_extension(TransactionPoolExt::new(pool));
		t.register_extension(KeystoreExt(keystore));
		t.execute_with(|| System::set_block_number(1));
		(t, pool_state, offchain_state)
	}
}

#[test]
fn send_some_testnet_token_works() {
	let (mut t, _, _) = ExternalityBuilder::build();
	t.execute_with(|| {
		let param = pallet_offchain::FaucetData {
			id: 11,
			login: b"test".to_vec(),
			created_at: b"1976-09-24T16:00:00Z".to_vec(),
			address: b"306721211d5404bd9da88e0204360a1a9ab8b87c66c1bc2fcdd37f3c2222cc20".to_vec(),
		};
		let mut params = Vec::<_>::new();
		params.push(param.clone());
		let acct: <TestRuntime as system::Trait>::AccountId = Default::default();
		assert_ok!(OffChainTest::send_some_testnet_token(
			Origin::signed(acct),
			params
		));
		assert_eq!(<LatestFaucetData>::get(), Some(param));
		let block_number = <Sendlist<TestRuntime>>::get(acct);
		assert_eq!(block_number!=Some(0), true);
	});
}
