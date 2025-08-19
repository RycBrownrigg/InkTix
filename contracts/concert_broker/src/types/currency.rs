/// Multi-currency support for Acala integration
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum CurrencyId {
    DOT,
    ACA,
    AUSD,
    LDOT,
    KSM,
}