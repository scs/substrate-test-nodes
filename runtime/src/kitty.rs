/// A runtime module template with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references


/// For more guidance on Substrate modules, see the example module
/// https://github.com/paritytech/substrate/blob/master/srml/example/src/lib.rs

use support::{decl_event, decl_module, decl_storage, dispatch::Result, ensure, StorageValue, traits::Randomness};
use system::ensure_signed;
use codec::{Decode, Encode};

#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Kitty<Hash, Balance> {
    id: Hash,
    price: Balance,
}

/// The module's configuration trait.
pub trait Trait: balances::Trait {
    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// This module's storage items.
decl_storage! {
	trait Store for Module<T: Trait> as Kitty
	{
        pub Kitties get(kitty): map u64 => Kitty<T::Hash, T::Balance>;
    	pub KittyCount get(kitty_count): u64;
    	pub KittyIndex: map T::AccountId => u64;
    }
}

// The module's dispatchable functions.
decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

        fn create_kitty(origin, price: T::Balance) -> Result {
            let sender = ensure_signed(origin)?;

			if !<KittyIndex<T>>::exists(&sender) {
				Self::_create_kitty(&sender, price)?;

				// let the sender know where his kitty is stored
				let key = <KittyIndex<T>>::get(&sender);
				Self::deposit_event(RawEvent::StoredKitty(sender, key));
			} else {
				Self::_update_kitty(&sender, price)?;

				// let the sender know where his kitty is stored
				let key = <KittyIndex<T>>::get(&sender);
				Self::deposit_event(RawEvent::UpdatedKitty(sender, key));
			}
            Ok(())
        }

        fn update_kitty(origin, price: T::Balance) {
        	let sender = ensure_signed(origin)?;
        	ensure!(<KittyIndex<T>>::exists(&sender), "Cannot update nonexistent Kitty");
        	Self::_update_kitty(&sender, price)?;

			// let the sender know where his kitty is stored
			let key = <KittyIndex<T>>::get(&sender);
			Self::deposit_event(RawEvent::UpdatedKitty(sender, key))
        }
    }
}

decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		StoredKitty(AccountId, u64),
		UpdatedKitty(AccountId, u64),
	}
);

impl<T: Trait> Module<T> {
    fn _create_kitty(sender: &T::AccountId, price: T::Balance) -> Result {
        if <KittyIndex<T>>::exists(sender) {
            return Self::_update_kitty(sender, price);
        }

        let kitty_count = Self::kitty_count();
        let new_kitty_count = kitty_count.checked_add(1).
            ok_or("[KittyModule]: Overflow adding new kitty")?;

		let random_seed = <randomness_collective_flip::Module<T>>::random_seed();
        let new_kitty = Kitty {
            id: random_seed,
            price,
        };

        <Kitties<T>>::insert(kitty_count, &new_kitty);
        <KittyCount>::put(new_kitty_count);
        <KittyIndex<T>>::insert(sender, kitty_count);

        Ok(())
    }

    fn _update_kitty(sender: &T::AccountId, price: T::Balance) -> Result {
        let key = <KittyIndex<T>>::get(sender);
        let mut kitty = <Kitties<T>>::get(key);
        kitty.price = price;
        <Kitties<T>>::insert(key, kitty);

        Ok(())
    }
}

/// tests for this module
#[cfg(test)]
mod tests {
	use primitives::H256;
	use sr_primitives::{
		Perbill, testing::Header, traits::{BlakeTwo256, IdentityLookup}, weights::Weight,
	};
	use support::{assert_ok, impl_outer_origin, parameter_types};

	use super::*;

	impl_outer_origin! {
		pub enum Origin for Test {}
	}

	// For testing the module, we construct most of a mock runtime. This means
	// first constructing a configuration type (`Test`) which `impl`s each of the
	// configuration traits of modules we want to use.
	#[derive(Clone, Eq, PartialEq)]
	pub struct Test;
	parameter_types! {
		pub const BlockHashCount: u64 = 250;
		pub const MaximumBlockWeight: Weight = 1024;
		pub const MaximumBlockLength: u32 = 2 * 1024;
		pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
	}
	impl system::Trait for Test {
		type Origin = Origin;
		type Call = ();
		type Index = u64;
		type BlockNumber = u64;
		type Hash = H256;
		type Hashing = BlakeTwo256;
		type AccountId = u64;
		type Lookup = IdentityLookup<Self::AccountId>;
		type Header = Header;
		type Event = ();
		type BlockHashCount = BlockHashCount;
		type MaximumBlockWeight = MaximumBlockWeight;
		type MaximumBlockLength = MaximumBlockLength;
		type AvailableBlockRatio = AvailableBlockRatio;
		type Version = ();
	}
	impl Trait for Test {
		type Event = ();
	}
	type TemplateModule = Module<Test>;

	// This function basically just builds a genesis storage key/value store according to
	// our desired mockup.
	fn new_test_ext() -> runtime_io::TestExternalities {
		system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
	}

	#[test]
	fn it_works_for_default_value() {
		new_test_ext().execute_with(|| {
			// Just a dummy test for the dummy funtion `do_something`
			// calling the `do_something` function with a value 42
			assert_ok!(TemplateModule::do_something(Origin::signed(1), 42));
			// asserting that the stored value is equal to what we stored
			assert_eq!(TemplateModule::something(), Some(42));
		});
	}
}
