#![deny(warnings)]
#![cfg_attr(test, feature(proc_macro_hygiene))]
#![cfg_attr(not(feature = "std"), no_std)]

mod types;

use cumulus_primitives::ParaId;
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage,
    dispatch::{DispatchError, DispatchResult},
    traits::Get,
    transactional,
};
use frame_system::ensure_signed;
use sp_runtime::traits::Convert;
use sp_std::{convert::TryInto, prelude::*};
use types::{Backing, Issuing};
pub use types::{CurrencyAdapter, CurrencyId, NativeAsset};
use xcm::v0::{Error as XcmError, ExecuteXcm, Junction::*, MultiAsset, NetworkId, Order, Xcm};
use xcm_executor::traits::LocationConversion;

/// Configuration trait of this pallet.
pub trait Config:
    frame_system::Config + currency::Config<currency::Backing> + currency::Config<currency::Issuing>
{
    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;

    type AccountId32Convert: Convert<Self::AccountId, [u8; 32]>;

    type ParaId: Get<ParaId>;

    type AccountIdConverter: LocationConversion<Self::AccountId>;

    type XcmExecutor: ExecuteXcm;
}

decl_storage! {
    trait Store for Module<T: Config> as ParachainTokens {
    }
}

// The pallet's events.
decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Config>::AccountId,
        Backing = Backing<T>,
        Issuing = Issuing<T>,
    {
        /// Transferred collateral to parachain.
        /// [origin, para_id, recipient, network, amount]
        TransferBacking(AccountId, ParaId, AccountId, NetworkId, Backing),
        /// Transferred issued tokens to parachain.
        /// [origin, para_id, recipient, network, amount]
        TransferIssuing(AccountId, ParaId, AccountId, NetworkId, Issuing),
    }
);

// The pallet's dispatchable functions.
decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        type Error = Error<T>;

        fn deposit_event() = default;

        /// Transfer collateral to parachain.
        #[weight = 1000]
        #[transactional]
        pub fn transfer_backing_to_parachain(
            origin,
            para_id: ParaId,
            recipient: T::AccountId,
            network: NetworkId,
            #[compact] amount: Backing<T>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            if para_id == T::ParaId::get() {
                return Ok(());
            }

            let xcm_origin = T::AccountIdConverter::try_into_location(who.clone()).map_err(|_| Error::<T>::BadLocation)?;

            let raw_amount = Self::tokens_to_u128(amount)?;
            T::XcmExecutor::execute_xcm(
                xcm_origin,
                Self::transfer_to_parachain(
                    para_id,
                    recipient.clone(),
                    network.clone(),
                    CurrencyId::DOT,
                    raw_amount
                ),
            ).map_err(Error::<T>::from)?;

            Self::deposit_event(Event::<T>::TransferBacking(
                who,
                para_id,
                recipient,
                network,
                amount,
            ));

            Ok(())
        }

        /// Transfer issued tokens to parachain.
        #[weight = 1000]
        #[transactional]
        pub fn transfer_issuing_to_parachain(
            origin,
            para_id: ParaId,
            recipient: T::AccountId,
            network: NetworkId,
            #[compact] amount: Issuing<T>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            if para_id == T::ParaId::get() {
                return Ok(());
            }

            let xcm_origin = T::AccountIdConverter::try_into_location(who.clone()).map_err(|_| Error::<T>::BadLocation)?;

            let raw_amount = Self::tokens_to_u128(amount)?;
            T::XcmExecutor::execute_xcm(
                xcm_origin,
                Self::transfer_to_parachain(
                    para_id,
                    recipient.clone(),
                    network.clone(),
                    CurrencyId::PolkaBTC,
                    raw_amount
                ),
            ).map_err(Error::<T>::from)?;

            Self::deposit_event(Event::<T>::TransferIssuing(
                who,
                para_id,
                recipient,
                network,
                amount,
            ));

            Ok(())
        }
    }
}

// "Internal" functions, callable by code.
impl<T: Config> Module<T> {
    fn transfer_to_parachain(
        para_id: ParaId,
        recipient: T::AccountId,
        network: NetworkId,
        currency_id: CurrencyId,
        amount: u128,
    ) -> Xcm {
        Xcm::WithdrawAsset {
            assets: vec![MultiAsset::ConcreteFungible {
                id: GeneralKey(currency_id.into()).into(),
                amount,
            }],
            effects: vec![Order::DepositReserveAsset {
                assets: vec![MultiAsset::All],
                dest: (Parent, Parachain { id: para_id.into() }).into(),
                effects: vec![Order::DepositAsset {
                    assets: vec![MultiAsset::All],
                    dest: AccountId32 {
                        network,
                        id: T::AccountId32Convert::convert(recipient),
                    }
                    .into(),
                }],
            }],
        }
    }

    fn tokens_to_u128<R: TryInto<u128>>(x: R) -> Result<u128, DispatchError> {
        TryInto::<u128>::try_into(x).map_err(|_| Error::<T>::TryIntoIntError.into())
    }
}

decl_error! {
    pub enum Error for Module<T: Config> {
        BadLocation,
        XcmExecutionFailed,
        TryIntoIntError,
    }
}

impl<T: Config> From<XcmError> for Error<T> {
    fn from(_: XcmError) -> Self {
        Self::XcmExecutionFailed
    }
}
