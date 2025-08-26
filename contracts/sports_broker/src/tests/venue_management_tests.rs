use crate::sports_broker::SportsBroker;
use crate::types::*;

/// Test venue management functionality
pub struct VenueManagementTests;

impl VenueManagementTests {
    /// Test comprehensive venue registration with all features
    pub fn test_comprehensive_venue_registration() {
        crate::tests::setup_with_test_env(|contract| {
            // Create venue amenities
            let amenities = vec![
                venue::VenueAmenity::WheelchairAccessible,
                venue::VenueAmenity::PremiumSeating,
                venue::VenueAmenity::FoodAndBeverage,
                venue::VenueAmenity::ParkingGarage,
            ];

            // Create parking information
            let parking_info = venue::ParkingInfo {
                total_spaces: 1000,
                reserved_spaces: 100,
                general_parking_price: 25_000_000_000_000_000_000, // 25 DOT
                premium_parking_price: 50_000_000_000_000_000_000, // 50 DOT
                valet_available: true,
                valet_price: 75_000_000_000_000_000_000, // 75 DOT
                parking_passes_available: true,
                parking_pass_price: 200_000_000_000_000_000_000, // 200 DOT
                parking_pass_duration: 86400 * 365,              // 1 year
                overflow_lots: vec![venue::OverflowLot {
                    name: "Lot B".to_string(),
                    capacity: 500,
                    price: 15_000_000_000_000_000_000, // 15 DOT
                    distance_from_venue: 500,          // 500 meters
                    shuttle_service: true,
                }],
            };

            // Create concession information
            let concession_info = venue::ConcessionInfo {
                food_available: true,
                beverage_available: true,
                alcohol_available: true,
                concession_credits_supported: true,
                credit_denomination: 1_000_000_000_000_000_000, // 1 DOT
                credit_packages: vec![venue::ConcessionCreditPackage {
                    id: 1,
                    name: "Food & Beverage Package".to_string(),
                    credit_amount: 50_000_000_000_000_000_000, // 50 DOT
                    price: 45_000_000_000_000_000_000,         // 45 DOT (10% discount)
                    discount_percentage: 10,
                    valid_duration: 86400 * 30, // 30 days
                    active: true,
                }],
                dietary_options: vec![
                    venue::DietaryOption::Vegetarian,
                    venue::DietaryOption::Vegan,
                    venue::DietaryOption::GlutenFree,
                ],
                average_meal_price: 15_000_000_000_000_000_000, // 15 DOT
            };

            // Create merchandise information
            let merchandise_info = venue::MerchandiseInfo {
                merchandise_available: true,
                online_store: true,
                exclusive_items: true,
                bundle_discounts: true,
                merchandise_bundles: vec![venue::MerchandiseBundle {
                    id: 1,
                    name: "Team Starter Pack".to_string(),
                    description: "Essential team merchandise bundle".to_string(),
                    items: vec![
                        venue::BundleItem {
                            item_name: "Team Jersey".to_string(),
                            item_type: venue::MerchandiseType::Clothing,
                            individual_price: 80_000_000_000_000_000_000, // 80 DOT
                            quantity: 1,
                        },
                        venue::BundleItem {
                            item_name: "Team Cap".to_string(),
                            item_type: venue::MerchandiseType::Accessories,
                            individual_price: 25_000_000_000_000_000_000, // 25 DOT
                            quantity: 1,
                        },
                    ],
                    bundle_price: 90_000_000_000_000_000_000, // 90 DOT
                    individual_price: 105_000_000_000_000_000_000, // 105 DOT
                    savings_amount: 15_000_000_000_000_000_000, // 15 DOT
                    savings_percentage: 14,
                    limited_quantity: Some(100),
                    active: true,
                }],
                loyalty_discounts: vec![venue::LoyaltyDiscount {
                    loyalty_tier: "Gold".to_string(),
                    discount_percentage: 15,
                    minimum_purchase: 50_000_000_000_000_000_000, // 50 DOT
                    applicable_categories: vec![
                        venue::MerchandiseType::Clothing,
                        venue::MerchandiseType::Accessories,
                    ],
                }],
                average_item_price: 35_000_000_000_000_000_000, // 35 DOT
            };

            // Create loyalty program
            let loyalty_program = venue::VenueLoyaltyProgram {
                active: true,
                points_per_dollar: 100, // 100 points per dollar spent
                tier_thresholds: vec![
                    venue::TierThreshold {
                        tier_name: "Bronze".to_string(),
                        points_required: 0,
                        benefits: vec!["Basic access".to_string()],
                    },
                    venue::TierThreshold {
                        tier_name: "Silver".to_string(),
                        points_required: 1000,
                        benefits: vec![
                            "Priority seating".to_string(),
                            "Concession discounts".to_string(),
                        ],
                    },
                    venue::TierThreshold {
                        tier_name: "Gold".to_string(),
                        points_required: 5000,
                        benefits: vec!["VIP access".to_string(), "Free parking".to_string()],
                    },
                ],
                venue_specific_benefits: vec![
                    venue::VenueBenefit::FreeParking,
                    venue::VenueBenefit::ConcessionDiscounts(10),
                    venue::VenueBenefit::MerchandiseDiscounts(15),
                ],
                partner_benefits: vec![venue::PartnerBenefit {
                    partner_name: "Local Restaurant".to_string(),
                    benefit_description: "20% off pre-game meals".to_string(),
                    discount_percentage: Some(20),
                    free_service: None,
                    minimum_tier: "Silver".to_string(),
                }],
            };

            // Create pricing tiers
            let pricing_tiers = vec![
                venue::VenuePricingTier {
                    id: 1,
                    name: "General Admission".to_string(),
                    description: "Standard seating with basic amenities".to_string(),
                    base_price_multiplier: 10000, // 1.0x
                    amenities_included: vec![venue::VenueAmenity::WheelchairAccessible],
                    parking_included: false,
                    concession_credits_included: 0,
                    merchandise_discount: 0,
                    active: true,
                },
                venue::VenuePricingTier {
                    id: 2,
                    name: "Premium Seating".to_string(),
                    description: "Premium seats with enhanced amenities".to_string(),
                    base_price_multiplier: 15000, // 1.5x
                    amenities_included: vec![
                        venue::VenueAmenity::PremiumSeating,
                        venue::VenueAmenity::FoodAndBeverage,
                    ],
                    parking_included: true,
                    concession_credits_included: 25_000_000_000_000_000_000, // 25 DOT
                    merchandise_discount: 10,
                    active: true,
                },
            ];

            let venue_id = contract.register_venue(
                "Metropolitan Stadium".to_string(),
                50000,
                "123 Sports Ave, Metropolis".to_string(),
                "Football".to_string(),
                venue::VenueType::Stadium,
                amenities,
                parking_info,
                concession_info,
                merchandise_info,
                loyalty_program,
                pricing_tiers,
            );

            assert_eq!(venue_id, 1);

            // Verify venue was created with all features
            let venue = contract.get_venue(venue_id).expect("Venue should exist");
            assert_eq!(venue.name, "Metropolitan Stadium");
            assert_eq!(venue.capacity, 50000);
            assert_eq!(venue.venue_type, venue::VenueType::Stadium);
            assert_eq!(venue.amenities.len(), 4);
            assert_eq!(venue.parking_info.total_spaces, 1000);
            assert_eq!(venue.concession_info.credit_packages.len(), 1);
            assert_eq!(venue.merchandise_info.merchandise_bundles.len(), 1);
            assert_eq!(venue.loyalty_program.tier_thresholds.len(), 3);
            assert_eq!(venue.pricing_tiers.len(), 2);
        });
    }

    /// Test parking pass creation and management
    pub fn test_parking_pass_management() {
        crate::tests::setup_with_test_env(|contract| {
            // First create a venue
            let venue_id = Self::create_test_venue(contract);

            // Create a parking pass
            let pass_id = contract
                .create_parking_pass(
                    venue_id,
                    venue::ParkingPassType::Season,
                    1234567890,               // valid_from
                    1234567890 + 86400 * 365, // valid_until (1 year)
                    "Main Lot".to_string(),
                    Some("A-123".to_string()),
                    200_000_000_000_000_000_000, // 200 DOT
                    "DOT".to_string(),
                )
                .expect("Should create parking pass");

            assert_eq!(pass_id, 1);

            // Verify parking pass was created
            let user_passes = contract.get_user_parking_passes();
            assert_eq!(user_passes.len(), 1);
            assert_eq!(user_passes[0].id, pass_id);
            assert_eq!(user_passes[0].venue_id, venue_id);
            assert_eq!(user_passes[0].pass_type, venue::ParkingPassType::Season);
            assert_eq!(user_passes[0].parking_lot, "Main Lot");
            assert_eq!(user_passes[0].space_number, Some("A-123".to_string()));

            // Verify venue parking passes
            let venue_passes = contract.get_venue_parking_passes(venue_id);
            assert_eq!(venue_passes.len(), 1);
            assert_eq!(venue_passes[0].id, pass_id);
        });
    }

    /// Test concession credits system
    pub fn test_concession_credits_system() {
        crate::tests::setup_with_test_env(|contract| {
            // First create a venue
            let venue_id = Self::create_test_venue(contract);

            // Purchase concession credits
            let credit_id = contract
                .purchase_concession_credits(
                    venue_id,
                    100_000_000_000_000_000_000, // 100 DOT
                    venue::ConcessionCreditType::General,
                    1234567890,                 // valid_from
                    1234567890 + 86400 * 30,    // valid_until (30 days)
                    90_000_000_000_000_000_000, // 90 DOT (10% discount)
                    "DOT".to_string(),
                )
                .expect("Should purchase concession credits");

            assert_eq!(credit_id, 1);

            // Verify concession credits were created
            let user_credits = contract.get_user_concession_credits();
            assert_eq!(user_credits.len(), 1);
            assert_eq!(user_credits[0].id, credit_id);
            assert_eq!(user_credits[0].credit_amount, 100_000_000_000_000_000_000);
            assert_eq!(
                user_credits[0].remaining_amount,
                100_000_000_000_000_000_000
            );
            assert_eq!(
                user_credits[0].credit_type,
                venue::ConcessionCreditType::General
            );

            // Use concession credits
            contract
                .use_concession_credits(
                    credit_id,
                    25_000_000_000_000_000_000, // 25 DOT
                    "Hot Dog Combo".to_string(),
                    "Section 101".to_string(),
                )
                .expect("Should use concession credits");

            // Verify credits were used
            let user_credits = contract.get_user_concession_credits();
            assert_eq!(user_credits[0].remaining_amount, 75_000_000_000_000_000_000);
            assert_eq!(user_credits[0].usage_history.len(), 1);
            assert_eq!(
                user_credits[0].usage_history[0].amount_used,
                25_000_000_000_000_000_000
            );
            assert_eq!(
                user_credits[0].usage_history[0].item_purchased,
                "Hot Dog Combo"
            );
        });
    }

    /// Test merchandise bundle system
    pub fn test_merchandise_bundle_system() {
        crate::tests::setup_with_test_env(|contract| {
            // First create a venue
            let venue_id = Self::create_test_venue(contract);

            // Create a merchandise bundle
            let bundle_id = contract
                .create_merchandise_bundle(
                    venue_id,
                    "Team Starter Pack".to_string(),
                    "Essential team merchandise bundle".to_string(),
                    vec![
                        venue::BundleItem {
                            item_name: "Team Jersey".to_string(),
                            item_type: venue::MerchandiseType::Clothing,
                            individual_price: 80_000_000_000_000_000_000, // 80 DOT
                            quantity: 1,
                        },
                        venue::BundleItem {
                            item_name: "Team Cap".to_string(),
                            item_type: venue::MerchandiseType::Accessories,
                            individual_price: 25_000_000_000_000_000_000, // 25 DOT
                            quantity: 1,
                        },
                    ],
                    90_000_000_000_000_000_000,  // 90 DOT bundle price
                    105_000_000_000_000_000_000, // 105 DOT individual price
                    Some(100),                   // Limited quantity
                )
                .expect("Should create merchandise bundle");

            assert_eq!(bundle_id, 1);

            // Verify bundle was created
            let venue_bundles = contract.get_venue_merchandise_bundles(venue_id);
            assert_eq!(venue_bundles.len(), 1);
            assert_eq!(venue_bundles[0].id, bundle_id);
            assert_eq!(venue_bundles[0].bundle_price, 90_000_000_000_000_000_000);
            assert_eq!(venue_bundles[0].savings_amount, 15_000_000_000_000_000_000);
            assert_eq!(venue_bundles[0].savings_percentage, 14);

            // Purchase the bundle
            let purchase_id = contract
                .purchase_merchandise_bundle(
                    venue_id,
                    bundle_id,
                    Some(10), // 10% loyalty discount
                    "Main Store".to_string(),
                    1234567890 + 86400 * 7, // Pickup deadline (7 days)
                )
                .expect("Should purchase merchandise bundle");

            assert_eq!(purchase_id, 1);

            // Verify purchase was created
            let user_bundles = contract.get_user_merchandise_bundles();
            assert_eq!(user_bundles.len(), 1);
            assert_eq!(user_bundles[0].id, purchase_id);
            assert_eq!(user_bundles[0].bundle_id, bundle_id);
            assert_eq!(user_bundles[0].loyalty_discount_applied, Some(10));
            assert_eq!(user_bundles[0].pickup_location, "Main Store");
            assert_eq!(user_bundles[0].items.len(), 2);
        });
    }

    /// Test venue capacity management
    pub fn test_venue_capacity_management() {
        crate::tests::setup_with_test_env(|contract| {
            // First create a venue
            let venue_id = Self::create_test_venue(contract);

            // Create a season for events
            let season_id = contract.create_season(
                "2024 Season".to_string(),
                "Football".to_string(),
                2024,
                1234567890,
                1234567890 + 86400 * 365,
            );

            // Create an event
            let event_id = contract.create_sports_event(
                "Home Opener".to_string(),
                1, // home_team_id
                2, // away_team_id
                venue_id,
                season_id,
                1234567890 + 86400 * 30,    // event_time (30 days from now)
                50000,                      // venue_capacity
                50_000_000_000_000_000_000, // base_ticket_price (50 DOT)
                GameType::RegularSeason,
            );

            // Reserve venue capacity
            let reservation_id = contract
                .reserve_venue_capacity(
                    venue_id,
                    event_id,
                    venue::ReservationType::Corporate,
                    1000,                       // capacity_reserved
                    10_000_000_000_000_000_000, // reservation_fee (10 DOT)
                    "DOT".to_string(),
                    1234567890 + 86400 * 25, // valid_until (25 days from now)
                )
                .expect("Should reserve venue capacity");

            assert_eq!(reservation_id, 1);

            // Verify capacity was reserved
            let venue = contract.get_venue(venue_id).expect("Venue should exist");
            assert_eq!(venue.capacity_management.reserved_capacity, 1000);
            assert_eq!(venue.capacity_management.available_capacity, 49000); // 50000 - 1000
        });
    }

    /// Test venue update functionality
    pub fn test_venue_update_functionality() {
        crate::tests::setup_with_test_env(|contract| {
            // First create a venue
            let venue_id = Self::create_test_venue(contract);

            // Update venue amenities
            let new_amenities = vec![
                venue::VenueAmenity::WheelchairAccessible,
                venue::VenueAmenity::PremiumSeating,
                venue::VenueAmenity::VIPLounges,
                venue::VenueAmenity::WiFi,
            ];

            contract
                .update_venue(
                    venue_id,
                    Some("Updated Stadium Name".to_string()),
                    Some(60000), // Increased capacity
                    Some(new_amenities),
                    None, // parking_info
                    None, // concession_info
                    None, // merchandise_info
                    None, // loyalty_program
                    None, // pricing_tiers
                )
                .expect("Should update venue");

            // Verify updates
            let venue = contract.get_venue(venue_id).expect("Venue should exist");
            assert_eq!(venue.name, "Updated Stadium Name");
            assert_eq!(venue.capacity, 60000);
            assert_eq!(venue.amenities.len(), 4);
            assert!(venue.amenities.contains(&venue::VenueAmenity::WiFi));
        });
    }

    /// Helper function to create a test venue with basic configuration
    fn create_test_venue(contract: &mut SportsBroker) -> u32 {
        let amenities = vec![
            venue::VenueAmenity::WheelchairAccessible,
            venue::VenueAmenity::FoodAndBeverage,
        ];

        let parking_info = venue::ParkingInfo {
            total_spaces: 1000,
            reserved_spaces: 100,
            general_parking_price: 25_000_000_000_000_000_000,
            premium_parking_price: 50_000_000_000_000_000_000,
            valet_available: false,
            valet_price: 0,
            parking_passes_available: true,
            parking_pass_price: 200_000_000_000_000_000_000,
            parking_pass_duration: 86400 * 365,
            overflow_lots: vec![],
        };

        let concession_info = venue::ConcessionInfo {
            food_available: true,
            beverage_available: true,
            alcohol_available: false,
            concession_credits_supported: true,
            credit_denomination: 1_000_000_000_000_000_000,
            credit_packages: vec![],
            dietary_options: vec![venue::DietaryOption::Vegetarian],
            average_meal_price: 15_000_000_000_000_000_000,
        };

        let merchandise_info = venue::MerchandiseInfo {
            merchandise_available: true,
            online_store: false,
            exclusive_items: false,
            bundle_discounts: false,
            merchandise_bundles: vec![],
            loyalty_discounts: vec![],
            average_item_price: 35_000_000_000_000_000_000,
        };

        let loyalty_program = venue::VenueLoyaltyProgram {
            active: true,
            points_per_dollar: 100,
            tier_thresholds: vec![],
            venue_specific_benefits: vec![],
            partner_benefits: vec![],
        };

        let pricing_tiers = vec![venue::VenuePricingTier {
            id: 1,
            name: "General Admission".to_string(),
            description: "Standard seating".to_string(),
            base_price_multiplier: 10000,
            amenities_included: vec![],
            parking_included: false,
            concession_credits_included: 0,
            merchandise_discount: 0,
            active: true,
        }];

        contract.register_venue(
            "Test Stadium".to_string(),
            50000,
            "123 Test St".to_string(),
            "Football".to_string(),
            venue::VenueType::Stadium,
            amenities,
            parking_info,
            concession_info,
            merchandise_info,
            loyalty_program,
            pricing_tiers,
        )
    }
}
