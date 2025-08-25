pub mod core_tests;
pub mod event_management_tests;
pub mod analytics_tests;
pub mod anti_scalping_tests;
pub mod loyalty_tests;
pub mod season_pass_tests;
pub mod fantasy_sports_tests;
pub mod advanced_team_loyalty_tests;

pub use core_tests::*;
pub use event_management_tests::*;
pub use analytics_tests::*;
pub use anti_scalping_tests::*;
pub use loyalty_tests::*;
pub use season_pass_tests::*;
pub use fantasy_sports_tests::*;
pub use advanced_team_loyalty_tests::*;

use crate::sports_broker::SportsBroker;
use ink::env::test;
use ink::env::DefaultEnvironment;

/// Setup test environment with proper ink! test context
pub fn setup_with_test_env<F>(test_fn: F) 
where 
    F: FnOnce(&mut SportsBroker) -> ()
{
    ink::env::test::run_test::<DefaultEnvironment, _>(|_| {
        let accounts = test::default_accounts::<DefaultEnvironment>();
        test::set_caller::<DefaultEnvironment>(accounts.alice);
        test::set_value_transferred::<DefaultEnvironment>(0);
        test::set_block_timestamp::<DefaultEnvironment>(1704067200000);
        test::set_block_number::<DefaultEnvironment>(1);
        test::set_contract::<DefaultEnvironment>(accounts.alice);

        let mut contract = SportsBroker::new();
        test_fn(&mut contract);
        Ok(())
    }).unwrap();
}
