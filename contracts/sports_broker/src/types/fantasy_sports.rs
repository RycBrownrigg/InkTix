use ink::primitives::AccountId;
use ink::prelude::string::String;
use ink::prelude::vec::Vec;

#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum FantasyLeagueType {
    SeasonLong,
    Weekly,
    Daily,
    Tournament,
    Custom,
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum FantasyLeagueStatus {
    Open,
    InProgress,
    Completed,
    Cancelled,
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum PlayerPosition {
    Quarterback,
    RunningBack,
    WideReceiver,
    TightEnd,
    Kicker,
    Defense,
    Flex,
    Unknown,
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum FantasyTeamStatus {
    Active,
    Eliminated,
    Champion,
    Inactive,
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct PlayerStats {
    pub player_id: u32,
    pub name: String,
    pub position: PlayerPosition,
    pub team_id: u32,
    pub points: u32,
    pub games_played: u32,
    pub touchdowns: u32,
    pub yards: u32,
    pub completion_percentage: Option<u32>,
    pub field_goal_percentage: Option<u32>,
    pub last_updated: u64,
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct FantasyLeague {
    pub league_id: u32,
    pub name: String,
    pub description: String,
    pub league_type: FantasyLeagueType,
    pub status: FantasyLeagueStatus,
    pub max_teams: u32,
    pub entry_fee: u128,
    pub prize_pool: u128,
    pub created_by: AccountId,
    pub created_at: u64,
    pub start_date: u64,
    pub end_date: u64,
    pub season_id: u32,
    pub sport_type: String,
    pub rules: String,
    pub scoring_system: String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct FantasyTeam {
    pub team_id: u32,
    pub league_id: u32,
    pub owner: AccountId,
    pub name: String,
    pub status: FantasyTeamStatus,
    pub total_points: u32,
    pub rank: u32,
    pub created_at: u64,
    pub last_updated: u64,
    pub players: Vec<u32>, // Player IDs
    pub captain_id: Option<u32>,
    pub vice_captain_id: Option<u32>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct FantasyLeagueParticipation {
    pub participation_id: u32,
    pub user_id: AccountId,
    pub league_id: u32,
    pub team_id: u32,
    pub ticket_id: u32,
    pub joined_at: u64,
    pub is_active: bool,
    pub bonus_points: u32,
    pub loyalty_multiplier: u32,
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct FantasyLeaderboard {
    pub league_id: u32,
    pub season_id: u32,
    pub entries: Vec<FantasyLeaderboardEntry>,
    pub last_updated: u64,
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct FantasyLeaderboardEntry {
    pub user_id: AccountId,
    pub team_name: String,
    pub total_points: u32,
    pub rank: u32,
    pub games_played: u32,
    pub win_streak: u32,
    pub bonus_points: u32,
    pub loyalty_multiplier: u32,
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct FantasyRewards {
    pub reward_id: u32,
    pub user_id: AccountId,
    pub league_id: u32,
    pub season_id: u32,
    pub reward_type: String,
    pub reward_amount: u128,
    pub reward_currency: String,
    pub loyalty_points: u32,
    pub claimed_at: Option<u64>,
    pub is_claimed: bool,
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct FantasyGameWeek {
    pub week_id: u32,
    pub league_id: u32,
    pub season_id: u32,
    pub start_date: u64,
    pub end_date: u64,
    pub games: Vec<u32>, // Game IDs
    pub is_active: bool,
    pub transfer_deadline: u64,
    pub captain_selection_deadline: u64,
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct FantasyTransfer {
    pub transfer_id: u32,
    pub team_id: u32,
    pub user_id: AccountId,
    pub player_out: u32,
    pub player_in: u32,
    pub transfer_cost: u32,
    pub transfer_time: u64,
    pub week_id: u32,
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct FantasySettings {
    pub max_transfers_per_week: u32,
    pub captain_multiplier: u32,
    pub vice_captain_multiplier: u32,
    pub bench_boost_enabled: bool,
    pub triple_captain_enabled: bool,
    pub wildcard_enabled: bool,
    pub free_hit_enabled: bool,
    pub max_players_per_team: u32,
    pub max_players_per_position: u32,
}

impl Default for FantasyLeagueType {
    fn default() -> Self {
        FantasyLeagueType::SeasonLong
    }
}

impl Default for FantasyLeagueStatus {
    fn default() -> Self {
        FantasyLeagueStatus::Open
    }
}

impl Default for PlayerPosition {
    fn default() -> Self {
        PlayerPosition::Unknown
    }
}

impl Default for FantasyTeamStatus {
    fn default() -> Self {
        FantasyTeamStatus::Active
    }
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            player_id: 0,
            name: String::new(),
            position: PlayerPosition::Unknown,
            team_id: 0,
            points: 0,
            games_played: 0,
            touchdowns: 0,
            yards: 0,
            completion_percentage: None,
            field_goal_percentage: None,
            last_updated: 0,
        }
    }
}

impl Default for FantasyLeague {
    fn default() -> Self {
        Self {
            league_id: 0,
            name: String::new(),
            description: String::new(),
            league_type: FantasyLeagueType::SeasonLong,
            status: FantasyLeagueStatus::Open,
            max_teams: 0,
            entry_fee: 0,
            prize_pool: 0,
            created_by: AccountId::from([0u8; 32]),
            created_at: 0,
            start_date: 0,
            end_date: 0,
            season_id: 0,
            sport_type: String::new(),
            rules: String::new(),
            scoring_system: String::new(),
        }
    }
}

impl Default for FantasyTeam {
    fn default() -> Self {
        Self {
            team_id: 0,
            league_id: 0,
            owner: AccountId::from([0u8; 32]),
            name: String::new(),
            status: FantasyTeamStatus::Active,
            total_points: 0,
            rank: 0,
            created_at: 0,
            last_updated: 0,
            players: Vec::new(),
            captain_id: None,
            vice_captain_id: None,
        }
    }
}

impl Default for FantasyLeagueParticipation {
    fn default() -> Self {
        Self {
            participation_id: 0,
            user_id: AccountId::from([0u8; 32]),
            league_id: 0,
            team_id: 0,
            ticket_id: 0,
            joined_at: 0,
            is_active: true,
            bonus_points: 0,
            loyalty_multiplier: 1,
        }
    }
}

impl Default for FantasyLeaderboard {
    fn default() -> Self {
        Self {
            league_id: 0,
            season_id: 0,
            entries: Vec::new(),
            last_updated: 0,
        }
    }
}

impl Default for FantasyLeaderboardEntry {
    fn default() -> Self {
        Self {
            user_id: AccountId::from([0u8; 32]),
            team_name: String::new(),
            total_points: 0,
            rank: 0,
            games_played: 0,
            win_streak: 0,
            bonus_points: 0,
            loyalty_multiplier: 1,
        }
    }
}

impl Default for FantasyRewards {
    fn default() -> Self {
        Self {
            reward_id: 0,
            user_id: AccountId::from([0u8; 32]),
            league_id: 0,
            season_id: 0,
            reward_type: String::new(),
            reward_amount: 0,
            reward_currency: String::new(),
            loyalty_points: 0,
            claimed_at: None,
            is_claimed: false,
        }
    }
}

impl Default for FantasyGameWeek {
    fn default() -> Self {
        Self {
            week_id: 0,
            league_id: 0,
            season_id: 0,
            start_date: 0,
            end_date: 0,
            games: Vec::new(),
            is_active: false,
            transfer_deadline: 0,
            captain_selection_deadline: 0,
        }
    }
}

impl Default for FantasyTransfer {
    fn default() -> Self {
        Self {
            transfer_id: 0,
            team_id: 0,
            user_id: AccountId::from([0u8; 32]),
            player_out: 0,
            player_in: 0,
            transfer_cost: 0,
            transfer_time: 0,
            week_id: 0,
        }
    }
}

impl Default for FantasySettings {
    fn default() -> Self {
        Self {
            max_transfers_per_week: 1,
            captain_multiplier: 2,
            vice_captain_multiplier: 1,
            bench_boost_enabled: true,
            triple_captain_enabled: true,
            wildcard_enabled: true,
            free_hit_enabled: true,
            max_players_per_team: 15,
            max_players_per_position: 3,
        }
    }
}
