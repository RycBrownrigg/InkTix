use crate::types::{CurrencyId, InkTixError, InkTixResult};

/// Currency conversion utility functions
pub struct ConversionUtils;

impl ConversionUtils {
    /// Convert amount to DOT equivalent
    pub fn convert_to_dot_equivalent(
        amount: u128, 
        currency: CurrencyId, 
        rates: &[u128]
    ) -> InkTixResult<u128> {
        match currency {
            CurrencyId::DOT => Ok(amount),
            _ => {
                let rate = rates[currency as usize];
                if rate == 0 {
                    return Err(InkTixError::InvalidData);
                }
                let dot_amount = amount.saturating_mul(rate) / 1_000_000_000_000;
                if dot_amount == 0 && amount > 0 {
                    return Err(InkTixError::InvalidData);
                }
                Ok(dot_amount)
            }
        }
    }

    /// Convert from DOT equivalent to target currency
    pub fn convert_from_dot_equivalent(
        dot_amount: u128, 
        target_currency: CurrencyId, 
        rates: &[u128]
    ) -> InkTixResult<u128> {
        match target_currency {
            CurrencyId::DOT => Ok(dot_amount),
            _ => {
                let rate = rates[target_currency as usize];
                if rate == 0 {
                    return Err(InkTixError::InvalidData);
                }
                let target_amount = dot_amount.saturating_mul(1_000_000_000_000) / rate;
                Ok(target_amount)
            }
        }
    }

    /// Convert VIP benefits to special access
    pub fn convert_vip_benefits_to_special_access(
        benefits: &[VIPBenefit]
    ) -> Vec<SpecialAccess> {
        let mut special_access = Vec::new();

        for benefit in benefits {
            match benefit {
                VIPBenefit::EarlyEntry => special_access.push(SpecialAccess::EarlyVenueEntry),
                VIPBenefit::MeetAndGreet => special_access.push(SpecialAccess::MeetAndGreet),
                VIPBenefit::BackstageAccess => special_access.push(SpecialAccess::BackstageAccess),
                VIPBenefit::SoundcheckAccess => special_access.push(SpecialAccess::SoundcheckAccess),
                VIPBenefit::ExclusiveMerchandise => special_access.push(SpecialAccess::ExclusiveMerchandise),
                VIPBenefit::ComplimentaryDrinks => special_access.push(SpecialAccess::ComplimentaryDrinks),
                VIPBenefit::PhotoOpportunity => special_access.push(SpecialAccess::PhotoOpportunity),
                VIPBenefit::PostShowAccess => special_access.push(SpecialAccess::PostShowAccess),
                VIPBenefit::PremiumSeating => (), // Handled by seat type
                VIPBenefit::DedicatedEntrance => (), // Venue logistics
                VIPBenefit::PreShowReception => special_access.push(SpecialAccess::VIPLounge),
                VIPBenefit::SignedMemorabilia => (), // Physical item, not access
                VIPBenefit::LimitedPoster => (), // Physical item, not access
                VIPBenefit::VIPLaminate => (), // Credential, not access
                VIPBenefit::ParkingIncluded => special_access.push(SpecialAccess::PriorityParking),
            }
        }

        special_access
    }
}