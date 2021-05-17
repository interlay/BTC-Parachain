use codec::{Decode, Encode};
use frame_support::traits::Currency;
use sp_std::fmt::Debug;

pub(crate) type Backing<T> =
    <<T as currency::Config<currency::Backing>>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

pub(crate) type Issuing<T> =
    <<T as currency::Config<currency::Issuing>>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

/// Bonded participant which can suggest and vote on proposals.
#[derive(Encode, Decode, Default, Clone, PartialEq, Debug)]
pub struct StakedRelayer<Balance, BlockNumber> {
    // total stake for this participant
    pub stake: Balance,
    // the height at which the participant bonded
    pub height: BlockNumber,
}
