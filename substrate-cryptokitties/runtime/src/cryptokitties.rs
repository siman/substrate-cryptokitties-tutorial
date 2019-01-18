use srml_support::{StorageMap, dispatch::Result};
use system::ensure_signed;
// ACTION: Continue to add required libraries here


#[derive(Encode, Decode, Default, Clone, PartialEq, Debug)]
pub struct Kitty<Hash, Balance> {
    // ACTION: Define the properties of your kitty struct here
    //      - `id` as a `Hash`
    //      - `name` as a `Vec<u8>`
    //      - `dna` as a `Hash`
    //      - `price` as a `Balance`
    //      - `gen` as a `u64`
}

pub trait Trait: balances::Trait {}

decl_storage! {
    trait Store for Module<T: Trait> as KittyStorage {
        // ACTION: Update this storage item to store a `Kitty<T::Hash, T::Balance>`
        Value: map T::AccountId => u64;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

        fn set_value(origin, name: Vec<u8>) -> Result {
            let sender = ensure_signed(origin)?;

            // ACTION: Create a `Kitty` object named `new_kitty` here
            //      HINT: You can generate a hash with `<T as system::Trait>::Hashing::hash_of(&0)`
            //            And you can generate a `0` balance with `<T::Balance as As<u64>>::sa(0)`

            // ACTION: Update this function to store your `new_kitty`
            <Value<T>>::insert(sender, value);

            Ok(())
        }
    }
}