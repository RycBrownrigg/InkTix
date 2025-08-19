use crate::types::{VIPPackage, VIPBenefit, InkTixError, InkTixResult};
use ink::prelude::vec::Vec;

/// VIP management business logic
pub struct VIPManager {
    pub vip_packages: Vec<VIPPackage>,
}

impl VIPManager {
    pub fn new() -> Self {
        Self {
            vip_packages: Vec::new(),
        }
    }

    /// Add VIP package to an event
    pub fn add_vip_package(
        &mut self,
        package_name: String,
        price_premium: u128,
        benefits: Vec<VIPBenefit>,
        limited_quantity: Option<u32>,
        description: String,
    ) -> InkTixResult<u32> {
        if package_name.is_empty() || description.is_empty() {
            return Err(InkTixError::InvalidData);
        }

        let vip_package = VIPPackage {
            package_name,
            price_premium,
            benefits,
            limited_quantity,
            available_quantity: limited_quantity,
            description,
        };

        let package_id = self.vip_packages.len() as u32;
        self.vip_packages.push(vip_package);

        Ok(package_id)
    }

    /// Get VIP package by ID
    pub fn get_vip_package(&self, package_id: u32) -> Option<&VIPPackage> {
        self.vip_packages.get(package_id as usize)
    }

    /// Get all VIP packages
    pub fn get_all_vip_packages(&self) -> &[VIPPackage] {
        &self.vip_packages
    }

    /// Update VIP package availability
    pub fn update_vip_package_availability(
        &mut self,
        package_id: u32,
        new_available_quantity: Option<u32>,
    ) -> InkTixResult<()> {
        if package_id >= self.vip_packages.len() as u32 {
            return Err(InkTixError::NotFound);
        }

        if let Some(package) = self.vip_packages.get_mut(package_id as usize) {
            package.available_quantity = new_available_quantity;
            Ok(())
        } else {
            Err(InkTixError::NotFound)
        }
    }
}