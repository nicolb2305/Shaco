#![allow(dead_code, non_snake_case, non_camel_case_types)]
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ActiveBoostsLcdsStoreFulfillmentNotification {
    pub inventory_type: String,
    pub data: Value,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ActiveBoostsLcdsSummonerActiveBoostsDTO {
    pub summoner_id: u64,
    pub ip_boost_end_date: i64,
    pub ip_boost_per_win_count: u32,
    pub ip_loyalty_boost: u32,
    pub xp_boost_end_date: i64,
    pub xp_boost_per_win_count: u32,
    pub xp_loyalty_boost: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct AlertDTO {
    pub alert_time: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct BannedChampion {
    pub champion_id: i32,
    pub team_id: i32,
    pub pick_turn: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct BasePlayerDTO {
    pub code: i32,
    pub message: String,
    pub data: MatchedPlayerDTO,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct BindingAsyncCancelEvent {
    pub async_token: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct BindingAsyncFailureEvent {
    pub async_token: u32,
    pub error: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct BindingCallbackEvent {
    pub id: u32,
    pub parameters: Vec<Value>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct BindingFullApiHelp {
    pub functions: Vec<BindingFullFunctionHelp>,
    pub types: Vec<BindingFullTypeHelp>,
    pub events: Vec<BindingFullEventHelp>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct BindingFullArgumentHelp {
    pub name: String,
    pub description: String,
    #[serde(rename = "type")]
    pub type_: BindingFullTypeIdentifier,
    pub optional: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct BindingFullEnumValueHelp {
    pub name: String,
    pub description: String,
    pub value: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct BindingFullEventHelp {
    pub name: String,
    pub name_space: String,
    #[serde(rename = "type")]
    pub type_: BindingFullTypeIdentifier,
    pub description: String,
    pub tags: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct BindingFullFieldHelp {
    pub name: String,
    pub description: String,
    pub offset: u32,
    #[serde(rename = "type")]
    pub type_: BindingFullTypeIdentifier,
    pub optional: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct BindingFullFunctionHelp {
    pub name: String,
    pub name_space: String,
    pub arguments: Vec<BindingFullArgumentHelp>,
    pub returns: BindingFullTypeIdentifier,
    pub description: String,
    pub help: String,
    pub thread_safe: bool,
    #[serde(rename = "async")]
    pub async_: String,
    pub tags: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct BindingFullTypeHelp {
    pub name: String,
    pub name_space: String,
    pub size: u32,
    pub description: String,
    pub tags: Vec<String>,
    pub fields: Vec<BindingFullFieldHelp>,
    pub values: Vec<BindingFullEnumValueHelp>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct BindingFullTypeIdentifier {
    #[serde(rename = "type")]
    pub type_: String,
    pub element_type: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct BindingGenericAsyncEvent {
    pub async_token: u32,
    pub data: Value,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct BindingGenericEvent {
    pub data: Value,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct BoostTeamSkinRentalDTO {
    pub summoner_name: String,
    pub skin_unlock_mode: String,
    pub price: i64,
    pub ip_reward: i64,
    pub ip_reward_for_purchaser: i64,
    pub available_skins: Vec<i64>,
    pub unlocked: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Bracket {
    pub version: i32,
    pub tournament_id: i64,
    pub phase_id: i64,
    pub id: i64,
    pub size: i32,
    pub matches: Vec<BracketMatch>,
    pub rosters: Vec<BracketRoster>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct BracketMatch {
    pub id: i64,
    pub round: i32,
    pub order: i32,
    pub roster_id1: i64,
    pub roster_id2: i64,
    pub result_history: String,
    pub lowest_possible_position: i32,
    pub highest_possible_position: i32,
    pub round_start_time: i64,
    pub game_start_time: i64,
    pub status: ClientBracketMatchStatus,
    pub winner_id: i64,
    pub game_id: i64,
    pub loser_bracket: bool,
    pub forfeit_roster_id: i64,
    pub fail_roster_status: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct BracketRoster {
    pub roster_id: i64,
    pub name: String,
    pub short_name: String,
    pub logo: i32,
    pub logo_color: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct BuildInfo {
    pub branch: String,
    pub patchline: String,
    pub version: String,
    pub patchline_visible_name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChampSelectLcdsGameDTO {
    pub id: u64,
    pub game_state: String,
    pub queue_type_name: String,
    pub room_name: String,
    pub room_password: String,
    pub multi_user_chat_j_w_t: String,
    pub muc_jwt_dto: MucJwtDto,
    pub name: String,
    pub player_champion_selections: Vec<ChampSelectLcdsPlayerChampionSelectionDTO>,
    pub banned_champions: Vec<BannedChampion>,
    pub team_one: Vec<Value>,
    pub team_two: Vec<Value>,
    pub status_of_participants: String,
    pub optimistic_lock: i64,
    pub pick_turn: i32,
    pub spectator_delay: i64,
    pub game_mutators: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChampSelectLcdsGameTimerDTO {
    pub current_game_state: String,
    pub remaining_time_in_millis: f64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChampSelectLcdsObfuscatedParticipant {
    pub game_unique_id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChampSelectLcdsPlayerChampionSelectionDTO {
    pub summoner_internal_name: String,
    pub champion_id: i32,
    pub selected_skin_index: i32,
    pub spell1_id: i32,
    pub spell2_id: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChampSelectLcdsPlayerParticipant {
    pub summoner_name: String,
    pub summoner_internal_name: String,
    pub summoner_id: u64,
    pub pick_mode: i32,
    pub pick_turn: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChampSelectLcdsPointSummary {
    pub points_to_next_roll: i32,
    pub current_points: i32,
    pub number_of_rolls: i32,
    pub max_rolls: i32,
    pub points_cost_to_roll: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChampSelectLcdsPotentialTradersDTO {
    pub potential_traders: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChampSelectLcdsRollResult {
    pub champion_id: i32,
    pub point_summary: ChampSelectLcdsPointSummary,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChampSelectLcdsTradeContractDTO {
    pub requester_internal_summoner_name: String,
    pub responder_internal_summoner_name: String,
    pub requester_champion_id: i32,
    pub responder_champion_id: i32,
    pub state: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChampionMasteryPublicDTO {
    pub champion_id: i32,
    pub champion_level: i32,
    pub champion_points: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChampionScoutingDTO {
    pub champion_id: i32,
    pub win_count: i32,
    pub game_count: i32,
    pub kda: f32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClashEventData {
    pub earned_date: String,
    pub reward_type: String,
    pub tournament_id: i64,
    pub tournament_name: String,
    pub tier: String,
    pub bracket: i64,
    pub season_id: i32,
    pub theme: String,
    pub roster_id: i64,
    pub team_name: String,
    pub team_short_name: String,
    pub team_logo_name: String,
    pub team_logo_chroma_id: String,
    pub player_u_u_i_ds: Vec<String>,
    pub reward_spec: ClashRewardSpec,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClashOfflineNotification {
    pub tournament_id: i64,
    pub reason: String,
    pub meta_data: HashMap<String, String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClashRewardConfigClient {
    pub name: String,
    pub key_def: Vec<ClashRewardKeyType>,
    pub entries: Vec<ClashRewardConfigEntry>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClashRewardConfigEntry {
    pub key: String,
    pub vals: Vec<ClashRewardOutput>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClashRewardDefinition {
    pub reward_type: ClashRewardType,
    pub reward_spec: ClashRewardSpec,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClashRewardOutput {
    pub primary: ClashRewardDefinition,
    pub alternative: ClashRewardDefinition,
    pub grant: ClashRewardTime,
    pub show: ClashRewardTime,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClashRewardSpec {
    pub pedestal: String,
    pub cup: String,
    pub gem: String,
    pub tier: String,
    pub bracket: String,
    pub theme: String,
    pub level: String,
    pub season_id: String,
    pub name: String,
    pub quantity: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClashSeasonRewardResult {
    pub player_id: u64,
    pub season_id: i32,
    pub season_vp: i32,
    pub banned: bool,
    pub honor_level: i32,
    pub eligible: bool,
    pub rewards: Vec<ClashRewardDefinition>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClientConfigDepInjectorEntitlements {
    pub access_token: String,
    pub token: String,
    pub subject: String,
    pub issuer: String,
    pub entitlements: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClientConfigDepInjectorEntitlementsUpdate {
    pub entitlements_update_type: ClientConfigDepInjectorEntitlementsUpdateType,
    pub entitlements_token_resource: ClientConfigDepInjectorEntitlements,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClientDynamicConfig {
    pub configs: String,
    pub delta: bool,
    pub compressed: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct CollectionsLcdsChampionDTO {
    pub end_date: u64,
    pub purchase_date: u64,
    pub win_count_remaining: i32,
    pub sources: Vec<String>,
    pub active: bool,
    pub bot_enabled: bool,
    pub champion_id: i32,
    pub champion_skins: Vec<CollectionsLcdsChampionSkinDTO>,
    pub free_to_play: bool,
    pub free_to_play_reward: bool,
    pub f2p_reward_sources: Vec<String>,
    pub owned: bool,
    pub purchased: u64,
    pub ranked_play_enabled: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct CollectionsLcdsChampionSkinDTO {
    pub end_date: u64,
    pub purchase_date: u64,
    pub win_count_remaining: i32,
    pub sources: Vec<String>,
    pub champion_id: i32,
    pub free_to_play_reward: bool,
    pub f2p_reward_sources: Vec<String>,
    pub last_selected: bool,
    pub owned: bool,
    pub skin_id: i32,
    pub still_obtainable: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct CollectionsLcdsClientDynamicConfigurationNotification {
    pub configs: String,
    pub delta: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct CollectionsLcdsRentalUpdateNotification {
    pub inventory_type: String,
    pub data: Value,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct CollectionsLcdsStoreFulfillmentNotification {
    pub inventory_type: String,
    pub data: Value,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ContentItemIdentifier {
    pub item_id: i32,
    pub inventory_type: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct CraftLootRefTransactionDTO {
    pub client_id: String,
    pub puuid: String,
    pub player_id: u64,
    pub account_id: i64,
    pub recipe_name: String,
    pub loot_name_ref_ids: Vec<LootNameRefId>,
    pub repeat: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct CrashReportingEnvironment {
    pub environment: String,
    pub user_name: String,
    pub user_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct DownloadUrlRequestV2 {
    pub platform_id: String,
    pub game_id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct DownloadUrlResponseV2 {
    pub url: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ElevationRequest {
    pub action: ElevationAction,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct EndOfGameLcdsClientReportV1 {
    pub offender_summoner_id: u64,
    pub game_id: u64,
    pub offenses: String,
    pub comments: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct EndOfGameLcdsEndOfGameStats {
    pub difficulty: String,
    pub game_id: u64,
    pub game_length: u32,
    pub game_mode: String,
    pub game_mutators: Vec<String>,
    pub game_type: String,
    pub invalid: bool,
    pub queue_type: String,
    pub ranked: bool,
    pub report_game_id: u64,
    pub room_name: String,
    pub room_password: String,
    pub multi_user_chat_j_w_t: String,
    pub muc_jwt_dto: MucJwtDto,
    pub my_team_info: EndOfGameLcdsTeamInfo,
    pub other_team_info: EndOfGameLcdsTeamInfo,
    pub team_player_participant_stats: Vec<EndOfGameLcdsPlayerParticipantStatsSummary>,
    pub other_team_player_participant_stats: Vec<EndOfGameLcdsPlayerParticipantStatsSummary>,
    pub my_team_status: String,
    pub leveled_up: bool,
    pub new_spells: Vec<EndOfGameLcdsSpell>,
    pub previous_level: u64,
    pub rp_earned: i32,
    pub skin_index: i32,
    pub skin_id: i32,
    pub summoner_name: String,
    pub user_id: u64,
    pub base_points: i32,
    pub battle_boost_ip_earned: i32,
    pub boost_ip_earned: i32,
    pub first_win_bonus: i32,
    pub ip_earned: i32,
    pub ip_total: i32,
    pub boost_xp_earned: i32,
    pub experience_earned: i32,
    pub experience_total: i32,
    pub loyalty_boost_xp_earned: i32,
    pub previous_xp_total: u64,
    pub time_until_next_first_win_bonus: i32,
    pub caused_early_surrender: bool,
    pub early_surrender_accomplice: bool,
    pub team_early_surrendered: bool,
    pub game_ended_in_early_surrender: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct EndOfGameLcdsHarassmentReport {
    pub game_id: u64,
    pub report_source: String,
    pub reporting_summoner_id: u64,
    pub reported_summoner_id: u64,
    pub offense: String,
    pub comment: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct EndOfGameLcdsPlayerParticipantStatsSummary {
    pub puuid: String,
    pub level: i32,
    pub user_id: u64,
    pub team_id: i32,
    pub game_id: u64,
    pub leaver: bool,
    pub summoner_name: String,
    pub skin_name: String,
    pub profile_icon_id: i32,
    pub wins: i32,
    pub leaves: i32,
    pub losses: i32,
    pub bot_player: bool,
    pub spell1_id: i32,
    pub spell2_id: i32,
    pub champion_id: i32,
    pub skin_index: i32,
    pub selected_position: String,
    pub detected_team_position: String,
    pub statistics: Vec<EndOfGameLcdsRawStatDTO>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct EndOfGameLcdsPointsPenalty {
    #[serde(rename = "type")]
    pub type_: String,
    pub penalty: f64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct EndOfGameLcdsRawStatDTO {
    pub value: i64,
    pub stat_type_name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct EndOfGameLcdsSpell {
    pub spell_id: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct EndOfGameLcdsTeamId {
    pub full_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct EndOfGameLcdsTeamInfo {
    pub team_id: EndOfGameLcdsTeamId,
    pub name: String,
    pub tag: String,
    pub seconds_until_eligible_for_deletion: i64,
    pub member_status_string: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct EntitlementsToken {
    pub access_token: String,
    pub token: String,
    pub subject: String,
    pub issuer: String,
    pub entitlements: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct EogLcdsGameDTO {
    pub id: u64,
    pub game_state: String,
    pub team_one: Vec<PlayerParticipant>,
    pub team_two: Vec<PlayerParticipant>,
    pub banned_champions: Vec<BannedChampion>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExternalPluginsResource {
    pub state: ExternalPluginsAvailability,
    pub error_string: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct FailedInvite {
    pub player_id: u64,
    pub exception: ClientRequestError,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct GameQueuesLcdsGameQueueConfig {
    pub disallow_free_champions: bool,
    pub game_mode: String,
    pub game_mutators: Vec<String>,
    pub game_type_config_id: i32,
    pub id: i32,
    pub map_id: i32,
    pub maximum_participant_list_size: u32,
    pub min_level: u32,
    pub minimum_participant_list_size: u32,
    pub num_players_per_team: u32,
    pub queue_state: String,
    pub ranked: bool,
    pub supported_map_ids: Vec<i32>,
    #[serde(rename = "type")]
    pub type_: String,
    pub last_toggled_off_time: u64,
    pub last_toggled_on_time: u64,
    pub removal_from_game_allowed: bool,
    pub removal_from_game_delay_minutes: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct GameflowLcdsGameDTO {
    pub id: u64,
    pub map_id: i32,
    pub game_state: String,
    pub queue_type_name: String,
    pub game_mode: String,
    pub game_type_config_id: i32,
    pub max_num_players: i32,
    pub game_type: String,
    pub spectator_delay: i32,
    pub game_queue_config_id: i32,
    pub team_one: Vec<Value>,
    pub team_two: Vec<Value>,
    pub player_champion_selections: Vec<Value>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct GameflowLcdsPlayerCredentialsDto {
    pub game_id: u64,
    pub server_ip: String,
    pub server_port: u16,
    pub encryption_key: String,
    pub observer: bool,
    pub observer_server_ip: String,
    pub observer_server_port: u16,
    pub observer_encryption_key: String,
    pub summoner_id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct GameflowLcdsReconnectInfoDto {
    pub player_credentials: GameflowLcdsPlayerCredentialsDto,
    pub game: GameflowLcdsGameDTO,
    pub reconnect_delay: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct IdsDTO {
    pub mission_ids: Vec<String>,
    pub series_ids: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LCDSBroadcastNotification {
    pub broadcast_messages: Vec<LolServiceStatusBroadcastMessage>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LCDSChampionReward {
    pub champion_id: i32,
    pub skins: Vec<i32>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LCDSGlobalRewards {
    pub all_champions: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LCDSLoyaltyRewards {
    pub champions: Vec<LCDSChampionReward>,
    pub global: LCDSGlobalRewards,
    pub ip_boost: i32,
    pub xp_boost: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LCDSLoyaltyStateChangeNotification {
    pub account_id: u64,
    pub notification_category: LCDSLoyaltyStateChangeNotificationCategory,
    pub rewards: LCDSLoyaltyRewards,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LCDSPlayerMessagingSimpleMessage {
    pub account_id: u64,
    pub msg_id: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub title_code: String,
    pub body_code: String,
    pub params: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LCDSPlayerMessagingSimpleMessageResponse {
    pub account_id: u64,
    pub msg_id: String,
    pub command: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LcdsBotParticipant {
    pub summoner_id: u64,
    pub summoner_name: String,
    pub summoner_internal_name: String,
    pub bot_skill_level: i32,
    pub team_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LcdsFailedJoinPlayer {
    pub summoner: LcdsSummoner,
    pub reason_failed: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LcdsGameDTO {
    pub id: i64,
    pub name: String,
    pub map_id: i32,
    pub game_type_config_id: i32,
    pub game_type: String,
    pub game_mode: String,
    pub max_num_players: i32,
    pub spectators_allowed: String,
    pub owner_summary: LcdsPlayerParticipant,
    pub team_one: Vec<LcdsPlayerParticipant>,
    pub team_two: Vec<LcdsPlayerParticipant>,
    pub observers: Vec<LcdsPlayerParticipant>,
    pub practice_game_rewards_disabled_reasons: Vec<String>,
    pub game_mutators: Vec<String>,
    pub room_name: String,
    pub room_password: String,
    pub multi_user_chat_j_w_t: String,
    pub muc_jwt_dto: MucJwtDto,
    pub passback_url: String,
    pub passback_data_packet: String,
    pub optimistic_lock: i64,
    pub game_state: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LcdsGameInviteBaseRuntimeException {
    pub root_cause_classname: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LcdsGameMap {
    pub map_id: i32,
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub min_custom_players: i32,
    pub total_players: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LcdsGameNotification {
    #[serde(rename = "type")]
    pub type_: String,
    pub message_code: String,
    pub message_argument: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LcdsInvitationRequest {
    pub invitation_id: String,
    pub owner: LcdsPlayer,
    pub inviter: LcdsInviter,
    pub invitation_state: LcdsInvitationState,
    pub game_meta_data: String,
    pub invite_type: String,
    pub invite_payload: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LcdsInviteFailed {
    pub summoner_id: u64,
    pub summoner_name: String,
    pub exception: LcdsGameInviteBaseRuntimeException,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LcdsInvitePrivileges {
    pub can_invite: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LcdsInvitee {
    pub summoner_id: u64,
    pub summoner_name: String,
    pub invitee_state: LcdsInviteeState,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LcdsInviter {
    pub summoner_id: u64,
    pub summoner_name: String,
    pub previous_season_highest_tier: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LcdsLobbyStatus {
    pub invitation_id: String,
    pub owner: LcdsPlayer,
    pub chat_key: String,
    pub members: Vec<LcdsMember>,
    pub invitees: Vec<LcdsInvitee>,
    pub game_meta_data: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LcdsMember {
    pub summoner_id: u64,
    pub summoner_name: String,
    pub has_delegated_invite_power: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LcdsPayloadDto {
    pub method: String,
    pub headers: HashMap<String, String>,
    pub path: String,
    pub body: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LcdsPlayer {
    pub summoner_id: u64,
    pub summoner_name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LcdsPlayerParticipant {
    pub summoner_id: u64,
    pub summoner_name: String,
    pub summoner_internal_name: String,
    pub bot_skill_level: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LcdsPracticeGameConfig {
    pub game_name: String,
    pub game_mode: String,
    pub game_map: LcdsGameMap,
    pub max_num_players: i32,
    pub game_type_config: i32,
    pub game_password: String,
    pub allow_spectators: String,
    pub passback_url: Option<String>,
    pub passback_data_packet: Option<String>,
    pub game_mutators: Vec<String>,
    pub region: String,
    pub game_version: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LcdsPracticeGameSearchResult {
    pub id: u64,
    pub name: String,
    pub owner: LcdsPlayerParticipant,
    pub max_num_players: i32,
    pub private_game: bool,
    pub team1_count: i32,
    pub team2_count: i32,
    pub spectator_count: i32,
    pub game_map: LcdsGameMap,
    pub game_map_id: i32,
    pub game_mode: String,
    pub pick_type: String,
    pub allow_spectators: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LcdsRemovedFromLobbyNotification {
    pub removal_reason: LcdsRemovalReason,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LcdsSimpleMessage {
    pub account_id: u64,
    pub msg_id: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub params: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LcdsSimpleMessageResponse {
    pub account_id: u64,
    pub msg_id: String,
    pub command: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LcdsStartChampSelectDto {
    pub invalid_players: Vec<LcdsFailedJoinPlayer>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LcdsSummoner {
    pub sum_id: u64,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LobbyClientDynamicConfigurationNotification {
    pub configs: String,
    pub delta: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LogEvent {
    pub severity: LogSeverityLevels,
    pub message: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolAccountVerificationAVSConfig {
    pub enabled: bool,
    pub disable_get_active_phone_number_call: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolAccountVerificationConfirmActivationPinRequest {
    pub one_time_pin: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolAccountVerificationConfirmActivationPinResponse {
    pub data: LolAccountVerificationPinResponseData,
    pub client_message_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolAccountVerificationConfirmDeactivationPinRequest {
    pub one_time_pin: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolAccountVerificationConfirmDeactivationPinResponse {
    pub data: LolAccountVerificationPinResponseData,
    pub client_message_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolAccountVerificationIsVerifiedResponse {
    pub success: bool,
    pub message: String,
    pub status: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolAccountVerificationLoginSession {
    pub state: LolAccountVerificationLoginSessionState,
    pub summoner_id: u64,
    pub account_id: u64,
    pub username: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolAccountVerificationPhoneNumberObfuscated {
    pub country_code: String,
    pub ends_with: String,
    pub length: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolAccountVerificationPhoneNumberResponse {
    pub data: LolAccountVerificationPhoneNumberResponseData,
    pub error: LolAccountVerificationResponseError,
    pub client_message_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolAccountVerificationPhoneNumberResponseData {
    pub phone_number_obfuscated: LolAccountVerificationPhoneNumberObfuscated,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolAccountVerificationPinResponseData {
    pub status: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolAccountVerificationResponseError {
    pub error_code: String,
    pub message: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolAccountVerificationSendActivationPinRequest {
    pub phone_number: String,
    pub locale: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolAccountVerificationSendActivationPinResponse {
    pub data: LolAccountVerificationSendActivationPinResponseData,
    pub error: LolAccountVerificationResponseError,
    pub client_message_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolAccountVerificationSendActivationPinResponseData {
    pub pin_expires_at_epoch_millis: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolAccountVerificationSendDeactivationPinRequest {
    pub locale: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolAccountVerificationSendDeactivationPinResponse {
    pub data: LolAccountVerificationSendActivationPinResponseData,
    pub error: LolAccountVerificationResponseError,
    pub client_message_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolActiveBoostsActiveBoosts {
    pub summoner_id: u64,
    pub ip_boost_end_date: String,
    pub ip_boost_per_win_count: u32,
    pub ip_loyalty_boost: u32,
    pub xp_boost_end_date: String,
    pub xp_boost_per_win_count: u32,
    pub xp_loyalty_boost: u32,
    pub first_win_of_the_day_start_time: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolActiveBoostsEndOfGameStats {
    pub time_until_next_first_win_bonus: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolActiveBoostsLoginDataPacket {
    pub time_until_first_win_of_day: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolAntiAddictionAntiAddictionState {
    pub policy_type: LolAntiAddictionPolicyType,
    pub localization_key: String,
    pub anti_addiction_token: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolAntiAddictionAntiAddictionToken {
    pub anti_addiction_token: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolBannersBannerFlag {
    pub item_id: i32,
    pub theme: String,
    pub level: i64,
    pub season_id: i64,
    pub earned_date_iso8601: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolBannersBannerFrame {
    pub level: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolBannersCapClashFlagEntitlementPayload {
    pub reward_spec: LolBannersClashV2FlagRewardSpec,
    pub reward_type: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolBannersCapClashFrameEntitlementPayload {
    pub reward_spec: LolBannersClashV2FrameRewardSpec,
    pub reward_type: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolBannersClashV2FlagRewardSpec {
    pub theme: String,
    pub level: String,
    pub season_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolBannersClashV2FrameRewardSpec {
    pub level: String,
    pub season_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolBannersInventoryItemWithPayload {
    pub uuid: String,
    pub item_id: i32,
    pub inventory_type: String,
    pub purchase_date: String,
    pub payload: Value,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolBannersInventoryItemsByType {
    pub t_o_u_r_n_a_m_e_n_t__f_l_a_g: Vec<LolBannersTournamentFlagInventoryItem>,
    pub t_o_u_r_n_a_m_e_n_t__f_r_a_m_e: Vec<LolBannersTournamentFrameInventoryItem>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolBannersInventoryResponse {
    pub items: LolBannersInventoryItemsByType,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolBannersLoadout {
    pub id: String,
    pub name: String,
    pub scope: String,
    pub loadout: HashMap<String, LolBannersLoadoutsSlot>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolBannersLoadoutsSlot {
    pub item_id: i32,
    pub inventory_type: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolBannersSummonerProfileUpdate {
    pub key: String,
    pub value: Value,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolBannersTournamentFlagInventoryItem {
    pub item_id: i32,
    pub payload: LolBannersCapClashFlagEntitlementPayload,
    pub purchase_date: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolBannersTournamentFrameInventoryItem {
    pub payload: LolBannersCapClashFrameEntitlementPayload,
    pub purchase_date: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCareerStatsChampionQueueStatsResponse {
    pub champion_id: i32,
    pub queue_type: LolCareerStatsCareerStatsQueueType,
    pub position: LolCareerStatsSummonersRiftPosition,
    pub rank_tier: LolCareerStatsRankedTier,
    pub stats: Value,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCareerStatsChampionStatistics {
    pub champion_id: i32,
    pub experts: Vec<LolCareerStatsExpertPlayer>,
    pub queue_stats: Vec<LolCareerStatsStatisticsByQueue>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCareerStatsEntitlementsToken {
    pub entitlements: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCareerStatsExpertPlayer {
    pub champion_id: i32,
    pub position: LolCareerStatsSummonersRiftPosition,
    pub summoner_id: u64,
    pub summoner_name: String,
    pub num_of_games: i32,
    pub win_rate: f32,
    pub expert_rank: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCareerStatsPositionStatistics {
    pub position: LolCareerStatsSummonersRiftPosition,
    pub experts: Vec<LolCareerStatsExpertPlayer>,
    pub queue_stats: Vec<LolCareerStatsStatisticsByQueue>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCareerStatsPositionStatsQueryRequest {
    pub queue_type: LolCareerStatsCareerStatsQueueType,
    pub position: LolCareerStatsSummonersRiftPosition,
    pub rank_tier: LolCareerStatsRankedTier,
    pub season: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCareerStatsStatisticsByQueue {
    pub queue_type: LolCareerStatsCareerStatsQueueType,
    pub stats: Value,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCareerStatsStatisticsPercentilesResponse {
    pub champion_id: i32,
    pub queue_type: LolCareerStatsCareerStatsQueueType,
    pub position: LolCareerStatsSummonersRiftPosition,
    pub rank_tier: LolCareerStatsRankedTier,
    pub season: u32,
    pub stats: Value,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCareerStatsStatsQueryRequest {
    pub champion_id: i32,
    pub queue_type: LolCareerStatsCareerStatsQueueType,
    pub position: LolCareerStatsSummonersRiftPosition,
    pub rank_tier: LolCareerStatsRankedTier,
    pub season: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogBundled {
    pub flexible: bool,
    pub items: Vec<LolCatalogBundledItem>,
    pub minimum_prices: Vec<LolCatalogBundledItemCost>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogBundledItem {
    pub inventory_type: String,
    pub item_id: i32,
    pub quantity: u32,
    pub discount_prices: Vec<LolCatalogBundledItemCost>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogBundledItemCost {
    pub currency: String,
    pub cost: i64,
    pub discount: Option<f32>,
    pub cost_type: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogCatalogItem {
    pub item_id: i32,
    pub item_instance_id: String,
    pub active: bool,
    pub inventory_type: String,
    pub inactive_date: String,
    pub prices: Vec<LolCatalogItemCost>,
    pub release_date: String,
    pub sale: Option<LolCatalogSale>,
    pub sub_inventory_type: Option<String>,
    pub tags: Option<Vec<String>>,
    pub icon_url: String,
    pub localizations: HashMap<String, LolCatalogItemLocalization>,
    pub bundled: Option<LolCatalogBundled>,
    pub item_requirements: Option<Vec<LolCatalogItemKey>>,
    pub metadata: Option<Vec<LolCatalogItemMetadataEntry>>,
    pub offer_id: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogCatalogPluginItem {
    pub item_id: i32,
    pub item_instance_id: String,
    pub owned: bool,
    pub ownership_type: Option<LolCatalogInventoryOwnership>,
    pub inventory_type: String,
    pub sub_inventory_type: String,
    pub name: String,
    pub sub_title: String,
    pub description: String,
    pub image_path: String,
    pub purchase_date: u64,
    pub release_date: u64,
    pub inactive_date: u64,
    pub prices: Vec<LolCatalogCatalogPluginPrice>,
    pub tags: Option<Vec<String>>,
    pub metadata: Option<Vec<LolCatalogItemMetadataEntry>>,
    pub active: bool,
    pub sale: Option<LolCatalogSale>,
    pub quest_skin_info: Option<LolCatalogSkinLineInfo>,
    pub offer_id: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogCatalogPluginItemAssets {
    pub splash_path: String,
    pub icon_path: String,
    pub tile_path: String,
    pub emblems: Vec<LolCatalogChampionSkinEmblem>,
    pub colors: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogCatalogPluginItemWithDetails {
    pub item: LolCatalogCatalogPluginItem,
    pub quantity: u32,
    pub required_items: Option<Vec<LolCatalogCatalogPluginItemWithDetails>>,
    pub bundled_items: Option<Vec<LolCatalogCatalogPluginItemWithDetails>>,
    pub minimum_bundle_prices: Option<Vec<LolCatalogCatalogPluginPrice>>,
    pub bundled_discount_prices: Option<Vec<LolCatalogCatalogPluginPrice>>,
    pub assets: LolCatalogCatalogPluginItemAssets,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogCatalogPluginPrice {
    pub currency: String,
    pub cost: i64,
    pub cost_type: Option<String>,
    pub sale: Option<LolCatalogCatalogPluginRetailDiscount>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogCatalogPluginRetailDiscount {
    pub start_date: String,
    pub end_date: String,
    pub discount: Option<f32>,
    pub cost: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogChampionSkinEmblem {
    pub name: String,
    pub emblem_path: LolCatalogChampionSkinEmblemPath,
    pub emblem_position: LolCatalogChampionSkinEmblemPosition,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogChampionSkinEmblemPath {
    pub large: String,
    pub small: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogChampionSkinEmblemPosition {
    pub vertical: String,
    pub horizontal: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogGameDataChampion {
    pub skins: Vec<LolCatalogGameDataChampionSkin>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogGameDataChampionChroma {
    pub id: i64,
    pub name: String,
    pub chroma_path: String,
    pub colors: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogGameDataChampionSkin {
    pub id: i64,
    pub name: String,
    pub splash_path: String,
    pub chromas: Vec<LolCatalogGameDataChampionChroma>,
    pub emblems: Vec<LolCatalogChampionSkinEmblem>,
    pub uncentered_splash_path: String,
    pub tile_path: String,
    pub chroma_path: String,
    pub quest_skin_info: Option<LolCatalogSkinLineInfo>,
    pub colors: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogGameDataChampionSummary {
    pub id: i64,
    pub name: String,
    pub square_portrait_path: String,
    pub skins: Vec<LolCatalogGameDataChampionSkin>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogGameDataItemReference {
    pub item_id: i32,
    pub inventory_type: String,
    pub content_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogGameDataStatstone {
    pub name: String,
    pub content_id: String,
    pub item_id: i32,
    pub is_retired: bool,
    pub is_duration: bool,
    pub is_epic: bool,
    pub category: String,
    pub description: String,
    pub icon_full: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogGameDataStatstonePack {
    pub name: String,
    pub description: String,
    pub item_id: i32,
    pub content_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogGameDataStatstoneSet {
    pub name: String,
    pub statstones: Vec<LolCatalogGameDataStatstone>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogGameDataStatstonesInfo {
    pub statstone_data: Vec<LolCatalogGameDataStatstoneSet>,
    pub pack_data: Vec<LolCatalogGameDataStatstonePack>,
    pub pack_id_to_stat_stones_ids: Value,
    pub series_id_to_stat_stone_ids: Value,
    pub pack_id_to_sub_pack_ids: Value,
    pub collection_id_to_stat_stone_ids: Value,
    pub pack_id_to_champ_ids: Value,
    pub champ_id_to_pack_ids: Value,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogGameDataSummonerEmote {
    pub id: i64,
    pub name: String,
    pub inventory_icon: String,
    pub description: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogGameDataSummonerIcon {
    pub id: i64,
    pub title: String,
    pub image_path: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogGameDataWardSkin {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub ward_image_path: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogInventoryContent {
    pub item_id: i64,
    pub ownership_type: LolCatalogInventoryOwnership,
    pub purchase_date: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogItemChoiceDetails {
    pub item: LolCatalogCatalogPluginItem,
    pub background_image: String,
    pub contents: Vec<LolCatalogItemDetails>,
    pub discount: String,
    pub full_price: i64,
    pub display_type: String,
    pub metadata: Value,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogItemCost {
    pub currency: String,
    pub cost: i64,
    pub discount: Option<f32>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogItemDetails {
    pub title: String,
    pub sub_title: String,
    pub description: String,
    pub icon_url: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogItemKey {
    pub inventory_type: String,
    pub item_id: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogItemLocalization {
    pub language: String,
    pub name: String,
    pub description: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogItemMetadataEntry {
    #[serde(rename = "type")]
    pub type_: String,
    pub value: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogSale {
    pub start_date: String,
    pub end_date: String,
    pub prices: Vec<LolCatalogItemCost>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogSkinLineDescriptionInfo {
    pub title: String,
    pub description: String,
    pub icon_path: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogSkinLineInfo {
    pub name: String,
    pub description_info: Vec<LolCatalogSkinLineDescriptionInfo>,
    pub splash_path: String,
    pub tile_path: String,
    pub collection_card_path: String,
    pub uncentered_splash_path: String,
    pub collection_description: String,
    pub tiers: Vec<LolCatalogSkinLineTier>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogSkinLineTier {
    pub id: i64,
    pub name: String,
    pub stage: i64,
    pub description: Option<String>,
    pub splash_path: String,
    pub uncentered_splash_path: String,
    pub tile_path: String,
    pub load_screen_path: String,
    pub short_name: String,
    pub splash_video_path: Option<String>,
    pub collection_splash_video_path: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChallengesChallengeData {
    pub id: i64,
    pub category: String,
    pub legacy: bool,
    pub percentile: f64,
    pub position: i32,
    pub players_in_level: i32,
    pub init_value: f64,
    pub previous_level: String,
    pub previous_value: f64,
    pub previous_threshold: f64,
    pub new_levels: Vec<String>,
    pub current_level: String,
    pub current_value: f64,
    pub current_threshold: f64,
    pub current_level_achieved_time: i64,
    pub next_level: String,
    pub next_threshold: f64,
    pub friends_at_levels: Vec<LolChallengesFriendLevelsData>,
    pub id_list_type: LolChallengesChallengeRequirementMappingName,
    pub available_ids: Vec<i32>,
    pub completed_ids: Vec<i32>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChallengesChallengeLevelData {
    pub level: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChallengesChallengePlayerData {
    pub id: String,
    pub puuid: String,
    pub source: LolChallengesSource,
    pub tags: HashMap<String, String>,
    pub total_points: LolChallengesChallengePoints,
    pub category_points: HashMap<String, LolChallengesChallengePoints>,
    pub player_challenges: Vec<LolChallengesChallengeData>,
    pub level_points: HashMap<String, i64>,
    pub preferences: LolChallengesChallengesPlayerPreferences,
    pub apex_lader_update_time: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChallengesChallengePoints {
    pub current: i32,
    pub max: i32,
    pub level: String,
    pub percentile: f64,
    pub position: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChallengesChallengeSeason {
    pub season_id: i32,
    pub season_start: i64,
    pub season_end: i64,
    pub is_active: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChallengesChallengeSignedUpdatePayload {
    pub tokens_by_type: HashMap<String, Value>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChallengesChallengeThreshold {
    pub value: f64,
    pub rewards: Vec<LolChallengesChallengeThresholdReward>,
    pub reward_group_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChallengesChallengeThresholdReward {
    pub category: String,
    pub id: String,
    pub quantity: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChallengesChallengesPlayerPreferences {
    pub banner_accent: String,
    pub title: String,
    pub challenge_ids: Vec<i64>,
    pub crest_border: String,
    pub prestige_crest_border_level: i32,
    pub signed_j_w_t_payload: LolChallengesChallengeSignedUpdatePayload,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChallengesChallengesRMSNotification {
    pub ack_required: bool,
    pub payload: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChallengesChallengesRMSPayload {
    pub puuid: String,
    pub id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChallengesEndOfGameStats {
    pub game_id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChallengesFriendLevelsData {
    pub level: String,
    pub friends: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChallengesFriendResource {
    pub puuid: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChallengesGameDataChallengeConfig {
    pub name: String,
    pub description: String,
    pub description_short: String,
    pub icon_path: String,
    pub tags: HashMap<String, String>,
    pub source: String,
    pub queue_ids: Vec<i32>,
    pub level_to_icon_path: HashMap<String, String>,
    pub thresholds: HashMap<String, LolChallengesChallengeThreshold>,
    pub leaderboard: bool,
    pub reverse_direction: bool,
    pub end_timestamp: i64,
    pub seasons: Vec<i32>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChallengesGameDataChallengeTitle {
    pub name: String,
    pub item_id: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChallengesGameDataChallengesData {
    pub challenges: HashMap<String, LolChallengesGameDataChallengeConfig>,
    pub titles: HashMap<String, LolChallengesGameDataChallengeTitle>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChallengesGameflowGameData {
    pub queue: LolChallengesQueue,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChallengesGameflowSession {
    pub phase: LolChallengesGameflowPhase,
    pub game_data: LolChallengesGameflowGameData,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChallengesInventoryItem {
    pub item_id: i32,
    pub uuid: String,
    pub purchase_date: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChallengesLcuChallengeNotification {
    pub id: u64,
    pub msg_id: String,
    pub level: String,
    pub update_reason: String,
    pub challenge_id: i64,
    pub category_challenges: Vec<LolChallengesUIChallenge>,
    pub display_type: LolChallengesNotificationDisplayType,
    pub challenge_name: String,
    pub icon_path: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChallengesLoginPlayerData {
    pub client_player_data_list: Vec<LolChallengesChallengePlayerData>,
    pub seasons: Vec<LolChallengesChallengeSeason>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChallengesQueue {
    pub id: i32,
    pub game_mode: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChallengesSequenceEvent {
    pub name: String,
    pub priority: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChallengesUICategoryProgress {
    pub level: String,
    pub category: String,
    pub position_percentile: f64,
    pub current: i32,
    pub max: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChallengesUIChallenge {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub description_short: String,
    pub icon_path: String,
    pub category: String,
    pub next_level_icon_path: String,
    pub current_level: String,
    pub next_level: String,
    pub previous_level: String,
    pub previous_value: f64,
    pub current_value: f64,
    pub current_threshold: f64,
    pub next_threshold: f64,
    pub points_awarded: i64,
    pub percentile: f64,
    pub current_level_achieved_time: i64,
    pub position: i32,
    pub players_in_level: i32,
    pub is_apex: bool,
    pub is_capstone: bool,
    pub game_modes: Vec<String>,
    pub friends_at_levels: Vec<LolChallengesFriendLevelsData>,
    pub capstone_id: i64,
    pub capstone_group_id: i64,
    pub capstone_group_name: String,
    pub parent_capstone_name: String,
    pub source: String,
    pub thresholds: HashMap<String, LolChallengesUIChallengeThreshold>,
    pub level_to_icon_path: HashMap<String, String>,
    pub value_mapping: String,
    pub has_leaderboard: bool,
    pub is_reverse_direction: bool,
    pub priority: f64,
    pub id_list_type: LolChallengesChallengeRequirementMappingName,
    pub available_ids: Vec<i32>,
    pub completed_ids: Vec<i32>,
    pub retire_timestamp: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChallengesUIChallengePenalty {
    pub reason: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChallengesUIChallengeReward {
    pub category: String,
    pub quantity: u64,
    pub name: String,
    pub asset: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChallengesUIChallengeThreshold {
    pub value: f64,
    pub rewards: Vec<LolChallengesUIChallengeReward>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChallengesUIPlayerSummary {
    pub total_challenge_score: i64,
    pub points_until_next_rank: i64,
    pub overall_challenge_level: String,
    pub position_percentile: f64,
    pub is_apex: bool,
    pub apex_leaderboard_position: i32,
    pub title: LolChallengesUITitle,
    pub banner_id: String,
    pub crest_id: String,
    pub prestige_crest_border_level: i32,
    pub category_progress: Vec<LolChallengesUICategoryProgress>,
    pub top_challenges: Vec<LolChallengesUIChallenge>,
    pub apex_ladder_update_time: i64,
    pub selected_challenges_string: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChallengesUITitle {
    pub item_id: i32,
    pub content_id: String,
    pub name: String,
    pub challenge_id: i64,
    pub challenge_name: String,
    pub challenge_description: String,
    pub level: String,
    pub level_to_icon_path: HashMap<String, String>,
    pub purchase_date: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChallengesUserResource {
    pub lol: HashMap<String, String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectBenchChampion {
    pub champion_id: i32,
    pub is_priority: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectChampGridChampion {
    pub id: i32,
    pub name: String,
    pub square_portrait_path: String,
    pub free_to_play: bool,
    pub loyalty_reward: bool,
    pub xbox_g_p_reward: bool,
    pub free_to_play_for_queue: bool,
    pub owned: bool,
    pub rented: bool,
    pub disabled: bool,
    pub roles: Vec<String>,
    pub mastery_points: i32,
    pub mastery_level: i32,
    pub mastery_chest_granted: bool,
    pub selection_status: LolChampSelectChampionSelection,
    pub positions_favorited: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectChampSelectAction {
    pub id: i64,
    pub actor_cell_id: i64,
    pub champion_id: i32,
    #[serde(rename = "type")]
    pub type_: String,
    pub completed: bool,
    pub is_ally_action: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectChampSelectBannedChampions {
    pub my_team_bans: Vec<i32>,
    pub their_team_bans: Vec<i32>,
    pub num_bans: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectChampSelectChatRoomDetails {
    pub multi_user_chat_id: String,
    pub multi_user_chat_password: String,
    pub multi_user_chat_j_w_t: String,
    pub muc_jwt_dto: LolChampSelectMucJwtDto,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectChampSelectMySelection {
    pub selected_skin_id: Option<i32>,
    pub spell1_id: Option<u64>,
    pub spell2_id: Option<u64>,
    pub ward_skin_id: Option<i64>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectChampSelectPinDropNotification {
    pub pin_drop_summoners: Vec<LolChampSelectChampSelectPinDropSummoner>,
    pub map_side: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectChampSelectPinDropSummoner {
    pub slot_id: u32,
    pub position: String,
    pub lane: String,
    pub lane_position: u32,
    pub is_local_summoner: bool,
    pub is_placeholder: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectChampSelectPlayerSelection {
    pub cell_id: i64,
    pub champion_id: i32,
    pub selected_skin_id: i32,
    pub ward_skin_id: i64,
    pub spell1_id: u64,
    pub spell2_id: u64,
    pub team: i32,
    pub assigned_position: String,
    pub champion_pick_intent: i32,
    pub summoner_id: u64,
    pub puuid: String,
    pub entitled_feature_type: String,
    pub name_visibility_type: String,
    pub obfuscated_summoner_id: u64,
    pub obfuscated_puuid: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectChampSelectSession {
    pub game_id: u64,
    pub timer: LolChampSelectChampSelectTimer,
    pub chat_details: LolChampSelectChampSelectChatRoomDetails,
    pub my_team: Vec<LolChampSelectChampSelectPlayerSelection>,
    pub their_team: Vec<LolChampSelectChampSelectPlayerSelection>,
    pub trades: Vec<LolChampSelectChampSelectTradeContract>,
    pub pick_order_swaps: Vec<LolChampSelectChampSelectSwapContract>,
    pub actions: Vec<Value>,
    pub bans: LolChampSelectChampSelectBannedChampions,
    pub local_player_cell_id: i64,
    pub is_spectating: bool,
    pub allow_skin_selection: bool,
    pub allow_duplicate_picks: bool,
    pub allow_battle_boost: bool,
    pub boostable_skin_count: i32,
    pub allow_rerolling: bool,
    pub rerolls_remaining: u32,
    pub allow_locked_events: bool,
    pub locked_event_index: i32,
    pub bench_enabled: bool,
    pub bench_champions: Vec<LolChampSelectBenchChampion>,
    pub entitled_feature_state: LolChampSelectEntitledFeatureState,
    pub counter: i64,
    pub recovery_counter: i64,
    pub skip_champion_select: bool,
    pub has_simultaneous_bans: bool,
    pub has_simultaneous_picks: bool,
    pub is_custom_game: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectChampSelectSummoner {
    pub cell_id: i64,
    pub slot_id: u32,
    pub spell1_icon_path: String,
    pub spell2_icon_path: String,
    pub assigned_position: String,
    pub summoner_id: u64,
    pub puuid: String,
    pub entitled_feature_type: String,
    pub name_visibility_type: String,
    pub obfuscated_summoner_id: u64,
    pub obfuscated_puuid: String,
    pub active_action_type: String,
    pub champion_icon_style: String,
    pub skin_splash_path: String,
    pub acting_background_animation_state: String,
    pub status_message_key: String,
    pub champion_id: i32,
    pub champion_name: String,
    pub pick_sniped_class: String,
    pub current_champion_vote_percent_integer: i32,
    pub skin_id: i32,
    pub ban_intent_square_portrat_path: String,
    pub is_on_players_team: bool,
    pub should_show_selected_skin: bool,
    pub should_show_expanded: bool,
    pub is_acting_now: bool,
    pub should_show_acting_bar: bool,
    pub is_self: bool,
    pub should_show_ban_intent_icon: bool,
    pub is_pick_intenting: bool,
    pub is_done_picking: bool,
    pub is_placeholder: bool,
    pub should_show_spells: bool,
    pub should_show_ring_animations: bool,
    pub are_summoner_actions_complete: bool,
    pub trade_id: i64,
    pub swap_id: i64,
    pub show_trades: bool,
    pub show_swaps: bool,
    pub show_muted: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectChampSelectSwapContract {
    pub id: i64,
    pub cell_id: i64,
    pub state: LolChampSelectChampSelectSwapState,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectChampSelectSwapNotification {
    pub id: i64,
    pub requestor_index: i64,
    pub responder_index: i64,
    pub state: LolChampSelectChampSelectSwapState,
    pub other_summoner_index: i64,
    pub initiated_by_local_player: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectChampSelectTimer {
    pub adjusted_time_left_in_phase: i64,
    pub total_time_in_phase: i64,
    pub phase: String,
    pub is_infinite: bool,
    pub internal_now_in_epoch_ms: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectChampSelectTradeContract {
    pub id: i64,
    pub cell_id: i64,
    pub state: LolChampSelectChampSelectTradeState,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectChampSelectTradeNotification {
    pub id: i64,
    pub responder_index: i64,
    pub state: LolChampSelectChampSelectTradeState,
    pub other_summoner_index: i64,
    pub responder_champion_name: String,
    pub requester_champion_name: String,
    pub requester_champion_splash_path: String,
    pub initiated_by_local_player: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectChampionQuestSkinInfo {
    pub splash_path: String,
    pub tile_path: String,
    pub tiers: Vec<LolChampSelectCollectionsChampionQuestSkin>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectChampionSelection {
    pub selected_by_me: bool,
    pub ban_intented_by_me: bool,
    pub ban_intented: bool,
    pub is_banned: bool,
    pub pick_intented: bool,
    pub pick_intented_by_me: bool,
    pub pick_intented_position: String,
    pub picked_by_other_or_banned: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectCollectionsChampionChroma {
    pub champion_id: i32,
    pub chroma_path: Option<String>,
    pub id: i32,
    pub name: String,
    pub ownership: LolChampSelectCollectionsOwnership,
    pub disabled: bool,
    pub still_obtainable: bool,
    pub colors: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectCollectionsChampionMastery {
    pub champion_id: i32,
    pub champion_level: i32,
    pub champion_points: i32,
    pub chest_granted: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectCollectionsChampionMinimal {
    pub ban_vo_path: String,
    pub choose_vo_path: String,
    pub disabled_queues: Vec<String>,
    pub id: i32,
    pub name: String,
    pub ownership: LolChampSelectCollectionsOwnership,
    pub roles: Vec<String>,
    pub square_portrait_path: String,
    pub stinger_sfx_path: String,
    pub base_splash_path: String,
    pub free_to_play: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectCollectionsChampionQuestSkin {
    pub champion_id: i32,
    pub chroma_path: Option<String>,
    pub id: i32,
    pub name: String,
    pub ownership: LolChampSelectCollectionsOwnership,
    pub is_base: bool,
    pub disabled: bool,
    pub still_obtainable: bool,
    pub splash_path: String,
    pub tile_path: String,
    pub stage: u32,
    pub short_name: String,
    pub splash_video_path: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectCollectionsChampionSkin {
    pub champion_id: i32,
    pub chroma_path: Option<String>,
    pub id: i32,
    pub name: String,
    pub ownership: LolChampSelectCollectionsOwnership,
    pub is_base: bool,
    pub disabled: bool,
    pub still_obtainable: bool,
    pub splash_path: String,
    pub tile_path: String,
    pub chromas: Vec<LolChampSelectCollectionsChampionChroma>,
    pub quest_skin_info: LolChampSelectChampionQuestSkinInfo,
    pub emblems: Vec<LolChampSelectCollectionsChampionSkinEmblem>,
    pub rarity_gem_path: String,
    pub splash_video_path: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectCollectionsChampionSkinEmblem {
    pub name: String,
    pub emblem_path: LolChampSelectCollectionsChampionSkinEmblemPath,
    pub positions: LolChampSelectCollectionsChampionSkinEmblemPosition,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectCollectionsChampionSkinEmblemPath {
    pub large: String,
    pub small: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectCollectionsChampionSkinEmblemPosition {
    pub vertical: String,
    pub horizontal: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectCollectionsChampionSkinMinimal {
    pub champion_id: i32,
    pub chroma_path: Option<String>,
    pub id: i32,
    pub name: String,
    pub ownership: LolChampSelectCollectionsOwnership,
    pub is_base: bool,
    pub disabled: bool,
    pub still_obtainable: bool,
    pub splash_path: String,
    pub tile_path: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectCollectionsOwnership {
    pub loyalty_reward: bool,
    pub xbox_g_p_reward: bool,
    pub owned: bool,
    pub rental: LolChampSelectCollectionsRental,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectCollectionsRental {
    pub rented: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectEntitledFeatureState {
    pub additional_rerolls: u32,
    pub unlocked_skin_ids: Vec<i32>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectGameDataSummonerSpell {
    pub id: u64,
    pub icon_path: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectLegacyChampSelectAction {
    pub id: i64,
    pub actor_cell_id: i64,
    pub champion_id: i32,
    #[serde(rename = "type")]
    pub type_: String,
    pub completed: bool,
    pub is_ally_action: bool,
    pub is_in_progress: bool,
    pub pick_turn: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectLegacyChampSelectBannedChampions {
    pub my_team_bans: Vec<i32>,
    pub their_team_bans: Vec<i32>,
    pub num_bans: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectLegacyChampSelectChatRoomDetails {
    pub multi_user_chat_id: String,
    pub multi_user_chat_password: String,
    pub multi_user_chat_j_w_t: String,
    pub muc_jwt_dto: LolChampSelectLegacyMucJwtDto,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectLegacyChampSelectMySelection {
    pub selected_skin_id: Option<i32>,
    pub spell1_id: Option<u64>,
    pub spell2_id: Option<u64>,
    pub ward_skin_id: Option<i64>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectLegacyChampSelectPlayerSelection {
    pub cell_id: i64,
    pub champion_id: i32,
    pub selected_skin_id: i32,
    pub ward_skin_id: i64,
    pub spell1_id: u64,
    pub spell2_id: u64,
    pub team: i32,
    pub assigned_position: String,
    pub champion_pick_intent: i32,
    pub player_type: String,
    pub summoner_id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectLegacyChampSelectSession {
    pub timer: LolChampSelectLegacyChampSelectTimer,
    pub chat_details: LolChampSelectLegacyChampSelectChatRoomDetails,
    pub my_team: Vec<LolChampSelectLegacyChampSelectPlayerSelection>,
    pub their_team: Vec<LolChampSelectLegacyChampSelectPlayerSelection>,
    pub trades: Vec<LolChampSelectLegacyChampSelectTradeContract>,
    pub actions: Vec<Value>,
    pub bans: LolChampSelectLegacyChampSelectBannedChampions,
    pub local_player_cell_id: i64,
    pub is_spectating: bool,
    pub allow_skin_selection: bool,
    pub allow_battle_boost: bool,
    pub allow_rerolling: bool,
    pub rerolls_remaining: u32,
    pub has_simultaneous_bans: bool,
    pub has_simultaneous_picks: bool,
    pub is_custom_game: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectLegacyChampSelectTimer {
    pub adjusted_time_left_in_phase: i64,
    pub total_time_in_phase: i64,
    pub phase: String,
    pub is_infinite: bool,
    pub internal_now_in_epoch_ms: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectLegacyChampSelectTradeContract {
    pub id: i64,
    pub cell_id: i64,
    pub state: LolChampSelectLegacyChampSelectTradeState,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectLegacyChampionSelectPreferences {
    pub skins: HashMap<String, i32>,
    pub spells: HashMap<String, Value>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectLegacyCollectionsChampion {
    pub disabled_queues: Vec<String>,
    pub free_to_play: bool,
    pub id: i32,
    pub ownership: LolChampSelectLegacyCollectionsOwnership,
    pub active: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectLegacyCollectionsOwnership {
    pub loyalty_reward: bool,
    pub xbox_g_p_reward: bool,
    pub owned: bool,
    pub rental: LolChampSelectLegacyCollectionsRental,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectLegacyCollectionsRental {
    pub rented: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectLegacyGameflowGameClient {
    pub running: bool,
    pub visible: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectLegacyGameflowGameData {
    pub queue: LolChampSelectLegacyQueue,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectLegacyGameflowGameDodge {
    pub state: LolChampSelectLegacyGameflowGameDodgeState,
    pub dodge_ids: Vec<u64>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectLegacyGameflowSession {
    pub phase: LolChampSelectLegacyGameflowPhase,
    pub game_client: LolChampSelectLegacyGameflowGameClient,
    pub game_data: LolChampSelectLegacyGameflowGameData,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectLegacyInventoryItemWithPayload {
    pub item_id: i64,
    pub payload: Value,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectLegacyLobbyStatus {
    pub queue_id: i32,
    pub is_custom: bool,
    pub is_leader: bool,
    pub is_spectator: bool,
    pub allowed_play_again: bool,
    pub member_summoner_ids: Vec<u64>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectLegacyLoginSession {
    pub state: LolChampSelectLegacyLoginSessionStates,
    pub summoner_id: u64,
    pub connected: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectLegacyMucJwtDto {
    pub jwt: String,
    pub channel_claim: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectLegacyPlayerStatus {
    pub current_lobby_status: Option<LolChampSelectLegacyLobbyStatus>,
    pub last_queued_lobby_status: Option<LolChampSelectLegacyLobbyStatus>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectLegacyQueue {
    pub game_type_config: LolChampSelectLegacyQueueGameTypeConfig,
    pub are_free_champions_allowed: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectLegacyQueueGameTypeConfig {
    pub max_allowable_bans: i32,
    pub allow_trades: bool,
    pub battle_boost: bool,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectLegacySettingCategoryResource {
    pub schema_version: i32,
    pub data: LolChampSelectLegacyChampionSelectPreferences,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectLegacySummoner {
    pub summoner_id: u64,
    pub summoner_level: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectLegacyTeamBoost {
    pub summoner_name: String,
    pub skin_unlock_mode: String,
    pub price: i64,
    pub ip_reward: i64,
    pub ip_reward_for_purchaser: i64,
    pub available_skins: Vec<i64>,
    pub unlocked: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectLoginSession {
    pub summoner_id: Option<u64>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectMucJwtDto {
    pub jwt: String,
    pub channel_claim: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectMutedPlayerInfo {
    pub puuid: String,
    pub summoner_id: u64,
    pub obfuscated_puuid: String,
    pub obfuscated_summoner_id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectSettingsResource {
    pub data: Value,
    pub schema_version: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectSfxNotification {
    pub delay_millis: i64,
    pub path: String,
    pub event_type: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectSkinSelectorChildSkin {
    pub champion_id: i32,
    pub chroma_preview_path: Option<String>,
    pub id: i32,
    pub name: String,
    pub ownership: LolChampSelectCollectionsOwnership,
    pub is_base: bool,
    pub disabled: bool,
    pub still_obtainable: bool,
    pub is_champion_unlocked: bool,
    pub splash_path: String,
    pub splash_video_path: Option<String>,
    pub tile_path: String,
    pub unlocked: bool,
    pub is_unlocked_from_entitled_feature: bool,
    pub parent_skin_id: i32,
    pub colors: Vec<String>,
    pub stage: u32,
    pub short_name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectSkinSelectorInfo {
    pub selected_skin_id: i32,
    pub is_skin_granted_from_boost: bool,
    pub selected_champion_id: i32,
    pub champion_name: String,
    pub skin_selection_disabled: bool,
    pub show_skin_selector: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectSkinSelectorSkin {
    pub champion_id: i32,
    pub chroma_preview_path: Option<String>,
    pub id: i32,
    pub name: String,
    pub ownership: LolChampSelectCollectionsOwnership,
    pub is_base: bool,
    pub disabled: bool,
    pub still_obtainable: bool,
    pub is_champion_unlocked: bool,
    pub splash_path: String,
    pub splash_video_path: Option<String>,
    pub tile_path: String,
    pub unlocked: bool,
    pub is_unlocked_from_entitled_feature: bool,
    pub child_skins: Vec<LolChampSelectSkinSelectorChildSkin>,
    pub emblems: Vec<LolChampSelectCollectionsChampionSkinEmblem>,
    pub rarity_gem_path: String,
    pub group_splash: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectTeamBoost {
    pub summoner_id: i64,
    pub skin_unlock_mode: String,
    pub price: i64,
    pub ip_reward: i64,
    pub ip_reward_for_purchaser: i64,
    pub available_skins: Vec<i64>,
    pub unlocked: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsChampionQuestSkinInfo {
    pub name: String,
    pub description_info: Vec<LolChampionsQuestSkinDescriptionInfo>,
    pub splash_path: String,
    pub uncentered_splash_path: String,
    pub tile_path: String,
    pub collection_card_path: String,
    pub collection_description: String,
    pub tiers: Vec<LolChampionsCollectionsChampionQuestSkin>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsCollectionsChampion {
    pub alias: String,
    pub title: String,
    pub ban_vo_path: String,
    pub choose_vo_path: String,
    pub disabled_queues: Vec<String>,
    pub id: i32,
    pub name: String,
    pub ownership: LolChampionsCollectionsOwnership,
    pub purchased: u64,
    pub roles: Vec<String>,
    pub square_portrait_path: String,
    pub stinger_sfx_path: String,
    pub base_load_screen_path: String,
    pub base_splash_path: String,
    pub active: bool,
    pub bot_enabled: bool,
    pub free_to_play: bool,
    pub ranked_play_enabled: bool,
    pub passive: LolChampionsCollectionsChampionSpell,
    pub skins: Vec<LolChampionsCollectionsChampionSkin>,
    pub spells: Vec<LolChampionsCollectionsChampionSpell>,
    pub tactical_info: LolChampionsCollectionsChampionTacticalInfo,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsCollectionsChampionChroma {
    pub champion_id: i32,
    pub chroma_path: Option<String>,
    pub id: i32,
    pub name: String,
    pub ownership: LolChampionsCollectionsOwnership,
    pub disabled: bool,
    pub still_obtainable: bool,
    pub last_selected: bool,
    pub colors: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsCollectionsChampionMinimal {
    pub alias: String,
    pub title: String,
    pub ban_vo_path: String,
    pub choose_vo_path: String,
    pub disabled_queues: Vec<String>,
    pub id: i32,
    pub name: String,
    pub ownership: LolChampionsCollectionsOwnership,
    pub purchased: u64,
    pub roles: Vec<String>,
    pub square_portrait_path: String,
    pub stinger_sfx_path: String,
    pub base_load_screen_path: String,
    pub base_splash_path: String,
    pub active: bool,
    pub bot_enabled: bool,
    pub free_to_play: bool,
    pub ranked_play_enabled: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsCollectionsChampionPlayableCounts {
    pub champions_owned: u32,
    pub champions_rented: u32,
    pub champions_free_to_play: u32,
    pub champions_loyalty_reward: u32,
    pub champions_xbox_g_p_reward: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsCollectionsChampionQuestSkin {
    pub champion_id: i32,
    pub chroma_path: Option<String>,
    pub id: i32,
    pub name: String,
    pub ownership: LolChampionsCollectionsOwnership,
    pub is_base: bool,
    pub disabled: bool,
    pub still_obtainable: bool,
    pub last_selected: bool,
    pub splash_path: String,
    pub tile_path: String,
    pub stage: u32,
    pub description: String,
    pub uncentered_splash_path: String,
    pub load_screen_path: String,
    pub short_name: String,
    pub splash_video_path: String,
    pub collection_splash_video_path: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsCollectionsChampionSkin {
    pub champion_id: i32,
    pub chroma_path: Option<String>,
    pub id: i32,
    pub name: String,
    pub ownership: LolChampionsCollectionsOwnership,
    pub is_base: bool,
    pub disabled: bool,
    pub still_obtainable: bool,
    pub last_selected: bool,
    pub splash_path: String,
    pub tile_path: String,
    pub chromas: Vec<LolChampionsCollectionsChampionChroma>,
    pub quest_skin_info: LolChampionsChampionQuestSkinInfo,
    pub emblems: Vec<LolChampionsCollectionsChampionSkinEmblem>,
    pub uncentered_splash_path: String,
    pub load_screen_path: String,
    pub rarity_gem_path: String,
    pub splash_video_path: Option<String>,
    pub collection_splash_video_path: Option<String>,
    pub skin_type: Option<String>,
    pub features_text: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsCollectionsChampionSkinEmblem {
    pub name: String,
    pub emblem_path: LolChampionsCollectionsChampionSkinEmblemPath,
    pub positions: LolChampionsCollectionsChampionSkinEmblemPosition,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsCollectionsChampionSkinEmblemPath {
    pub large: String,
    pub small: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsCollectionsChampionSkinEmblemPosition {
    pub vertical: String,
    pub horizontal: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsCollectionsChampionSkinMinimal {
    pub champion_id: i32,
    pub chroma_path: Option<String>,
    pub id: i32,
    pub name: String,
    pub ownership: LolChampionsCollectionsOwnership,
    pub is_base: bool,
    pub disabled: bool,
    pub still_obtainable: bool,
    pub last_selected: bool,
    pub splash_path: String,
    pub tile_path: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsCollectionsChampionSpell {
    pub name: String,
    pub description: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsCollectionsChampionTacticalInfo {
    pub style: u32,
    pub difficulty: u32,
    pub damage_type: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsCollectionsOwnership {
    pub loyalty_reward: bool,
    pub xbox_g_p_reward: bool,
    pub owned: bool,
    pub rental: LolChampionsCollectionsRental,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsCollectionsRental {
    pub end_date: u64,
    pub purchase_date: u64,
    pub win_count_remaining: i32,
    pub rented: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsGameDataChampion {
    pub id: i32,
    pub alias: String,
    pub title: String,
    pub ban_vo_path: String,
    pub choose_vo_path: String,
    pub name: String,
    pub square_portrait_path: String,
    pub stinger_sfx_path: String,
    pub passive: LolChampionsGameDataChampionSpell,
    pub roles: Vec<String>,
    pub skins: Vec<LolChampionsGameDataChampionSkin>,
    pub spells: Vec<LolChampionsGameDataChampionSpell>,
    pub tactical_info: LolChampionsGameDataChampionTacticalInfo,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsGameDataChampionChroma {
    pub id: i32,
    pub colors: Vec<String>,
    pub chroma_path: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsGameDataChampionQuestSkin {
    pub id: i32,
    pub name: String,
    pub stage: u32,
    pub description: String,
    pub splash_path: String,
    pub splash_video_path: String,
    pub collection_splash_video_path: String,
    pub uncentered_splash_path: String,
    pub tile_path: String,
    pub load_screen_path: String,
    pub short_name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsGameDataChampionSkin {
    pub id: i32,
    pub name: String,
    pub chromas: Vec<LolChampionsGameDataChampionChroma>,
    pub quest_skin_info: LolChampionsGameDataQuestSkinInfo,
    pub emblems: Vec<LolChampionsCollectionsChampionSkinEmblem>,
    pub splash_path: String,
    pub uncentered_splash_path: String,
    pub tile_path: String,
    pub load_screen_path: String,
    pub rarity_gem_path: String,
    pub splash_video_path: Option<String>,
    pub collection_splash_video_path: Option<String>,
    pub chroma_path: Option<String>,
    pub skin_type: Option<String>,
    pub features_text: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsGameDataChampionSpell {
    pub name: String,
    pub description: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsGameDataChampionSummary {
    pub id: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsGameDataChampionTacticalInfo {
    pub style: u32,
    pub difficulty: u32,
    pub damage_type: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsGameDataQuestSkinDescriptionInfo {
    pub title: String,
    pub description: String,
    pub icon_path: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsGameDataQuestSkinInfo {
    pub name: String,
    pub description_info: Vec<LolChampionsGameDataQuestSkinDescriptionInfo>,
    pub splash_path: String,
    pub uncentered_splash_path: String,
    pub tile_path: String,
    pub collection_card_path: String,
    pub collection_description: String,
    pub tiers: Vec<LolChampionsGameDataChampionQuestSkin>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsInventoryItemWithPayload {
    pub uuid: String,
    pub item_id: i32,
    pub inventory_type: String,
    pub purchase_date: String,
    pub quantity: u64,
    pub expiration_date: String,
    pub f2p: bool,
    pub rental: bool,
    pub loyalty: bool,
    pub loyalty_sources: Vec<String>,
    pub owned: bool,
    pub payload: Value,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsLcdsDynamicClientConfig {
    pub disabled_champions: Value,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsLoginSession {
    pub state: LolChampionsLoginSessionStates,
    pub summoner_id: u64,
    pub connected: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsLoyaltyStatusNotification {
    pub status: LolChampionsLoyaltyStatus,
    pub reload_inventory: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsPlayerNotification {
    pub background_url: String,
    pub critical: bool,
    pub data: HashMap<String, String>,
    pub detail_key: String,
    pub icon_url: String,
    pub id: u64,
    pub source: String,
    pub state: String,
    pub title_key: String,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsQuestSkinDescriptionInfo {
    pub title: String,
    pub description: String,
    pub icon_path: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsSummoner {
    pub summoner_id: u64,
    pub summoner_level: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatActiveConversationResource {
    pub id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatAuthResourceRsoAccessToken {
    pub token: String,
    pub scopes: Vec<String>,
    pub expiry: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatBlocked {
    pub pid: String,
    pub name: String,
    pub game_name: String,
    pub game_tag: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatBlockedList {
    pub blocked: Vec<LolChatBlocked>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatBlockedPlayerResource {
    pub summoner_id: u64,
    pub icon: i32,
    pub id: String,
    pub name: String,
    pub pid: String,
    pub puuid: String,
    pub game_name: String,
    pub game_tag: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatChampSelection {
    pub summoner_internal_name: String,
    pub champion_id: i32,
    pub selected_skin_index: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatChatDomainConfig {
    pub p2_p_domain_name: Option<String>,
    pub custom_game_domain_name: Option<String>,
    pub champ_select_domain_name: Option<String>,
    pub post_game_domain_name: Option<String>,
    pub club_domain_name: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatChatFriendUpdate {
    pub pid: String,
    pub group: String,
    pub note: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatChatMessage {
    #[serde(rename = "type")]
    pub type_: LolChatMessageType,
    pub id: String,
    pub cid: String,
    pub pid: String,
    pub mid: String,
    pub name: String,
    pub game_name: String,
    pub game_tag: String,
    pub time: String,
    pub body: String,
    pub read: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatChatMessageList {
    pub messages: Vec<LolChatChatMessage>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatChatPlatformLoginSession {
    pub state: LolChatChatPlatformLoginSessionState,
    pub account_id: u64,
    pub username: String,
    pub user_auth_token: String,
    pub gas_token: Value,
    pub summoner_id: Option<u64>,
    pub id_token: String,
    pub puuid: String,
    pub is_new_player: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatChatServiceDynamicClientConfig {
    pub lcu_social: Option<LolChatLcuSocialConfig>,
    pub chat_domain: Option<LolChatChatDomainConfig>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatChatSettings {
    pub chat_group_mobile: bool,
    pub chat_group_offline: bool,
    pub chat_g_b_g: bool,
    pub recently_played_open: bool,
    pub recently_played_first_open: bool,
    pub chat_filter_disabled: bool,
    pub friend_request_toasts_disabled: bool,
    pub link_click_warning_enabled: bool,
    pub more_unreads_enabled: bool,
    pub show_when_typing_enabled: bool,
    pub bounce_dock_icon_enabled: bool,
    pub message_notifications_enabled: bool,
    pub use_player_preferences: bool,
    pub chat_window_docked_height: u32,
    pub sort_by: String,
    #[serde(rename = "chat-status-message")]
    pub chat_status_message: String,
    #[serde(rename = "closed-conversations")]
    pub closed_conversations: HashMap<String, u64>,
    #[serde(rename = "hidden-conversations")]
    pub hidden_conversations: HashMap<String, u64>,
    #[serde(rename = "muted-conversations")]
    pub muted_conversations: HashMap<String, i8>,
    #[serde(rename = "roster-group-collapsed")]
    pub roster_group_collapsed: HashMap<String, bool>,
    pub chat_window: LolChatChatWindowSettings,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatChatSummoner {
    pub summoner_id: u64,
    pub profile_icon_id: i32,
    pub display_name: String,
    pub summoner_level: u32,
    pub puuid: String,
    pub unnamed: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatChatWindowSettings {
    pub detached: bool,
    pub height: u32,
    pub left: i32,
    pub top: i32,
    pub width: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatCidBody {
    pub cid: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatConfigStatus {
    pub readiness: LolChatConfigReadinessEnum,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatContentCookies {
    pub content_id: String,
    pub content_path: String,
    pub cookies: Vec<LolChatcookie>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatConversation {
    pub cid: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub mid: String,
    pub unread_count: u64,
    pub muted: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatConversationJoinFederated {
    pub id: String,
    pub password: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub domain: String,
    pub target_region: String,
    pub hidden: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatConversationList {
    pub conversations: Vec<LolChatConversation>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatConversationMessageResource {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub from_summoner_id: u64,
    pub from_id: String,
    pub from_pid: String,
    pub from_obfuscated_summoner_id: u64,
    pub body: String,
    pub timestamp: String,
    pub is_historical: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatConversationResource {
    pub id: String,
    pub name: String,
    pub pid: String,
    pub game_name: String,
    pub game_tag: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub inviter_id: String,
    pub password: String,
    pub target_region: String,
    pub is_muted: bool,
    pub unread_message_count: u64,
    pub last_message: Option<LolChatConversationMessageResource>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatConversationUpdate {
    pub cid: String,
    pub muted: bool,
    pub hidden: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatDebugResource {
    pub is_x_m_p_p_logging_enabled: Option<bool>,
    pub keep_alive_interval: Option<u32>,
    pub async_wait_interval: Option<u32>,
    pub min_reconnect_interval: Option<u32>,
    pub max_reconnect_interval: Option<u32>,
    pub trigger_chat_disconnect: Option<bool>,
    pub fail_next_keep_alive: Option<bool>,
    pub fail_next_chat_login: Option<bool>,
    pub fail_next_chat_logout: Option<bool>,
    pub fail_all_chat_login: Option<bool>,
    pub enable_chat_filtering: Option<bool>,
    pub silence_chat_while_in_game: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatEndOfGamePlayer {
    pub puuid: String,
    pub is_local_player: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatEndOfGameStats {
    pub teams: Vec<LolChatEndOfGameTeam>,
    pub local_player: LolChatEndOfGamePlayer,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatEndOfGameTeam {
    pub players: Vec<LolChatEndOfGamePlayer>,
    pub is_player_team: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatError {
    pub cid: String,
    pub class: String,
    pub code: String,
    pub id: String,
    pub pid: String,
    pub subtype: String,
    pub text: String,
    pub ts: String,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatErrorList {
    pub errors: Vec<LolChatError>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatErrorResource {
    pub id: u64,
    pub from: String,
    pub code: u64,
    pub message: String,
    pub text: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatFriend {
    pub pid: String,
    pub group: String,
    pub display_group: String,
    pub name: String,
    pub game_name: String,
    pub game_tag: String,
    pub note: String,
    pub puuid: String,
    pub region: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatFriendCountsResource {
    pub num_friends: u32,
    pub num_friends_online: u32,
    pub num_friends_available: u32,
    pub num_friends_away: u32,
    pub num_friends_in_queue: u32,
    pub num_friends_in_champ_select: u32,
    pub num_friends_in_game: u32,
    pub num_friends_mobile: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatFriendGroup {
    pub name: String,
    pub collapsed: bool,
    pub is_meta_group: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatFriendGroupCreate {
    pub name: String,
    pub collapsed: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatFriendGroupList {
    pub groups: Vec<LolChatFriendGroup>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatFriendGroupOrder {
    pub groups: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatFriendGroupUpdate {
    pub name: String,
    pub new_name: String,
    pub collapsed: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatFriendList {
    pub friends: Vec<LolChatFriend>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatFriendRequest {
    pub pid: String,
    pub name: String,
    pub game_name: String,
    pub game_tag: String,
    pub subscription: LolChatFriendSubscriptionType,
    pub note: String,
    pub puuid: String,
    pub region: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatFriendRequestAdd {
    pub pid: String,
    pub name: String,
    pub game_name: String,
    pub game_tag: String,
    pub note: String,
    pub puuid: String,
    pub region: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatFriendRequestList {
    pub requests: Vec<LolChatFriendRequest>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatFriendRequestResource {
    pub summoner_id: u64,
    pub icon: i32,
    pub id: String,
    pub name: String,
    pub pid: String,
    pub puuid: String,
    pub game_name: String,
    pub game_tag: String,
    pub note: String,
    pub direction: LolChatFriendRequestDirection,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatFriendResource {
    pub summoner_id: u64,
    pub id: String,
    pub name: String,
    pub pid: String,
    pub puuid: String,
    pub game_name: String,
    pub game_tag: String,
    pub icon: i32,
    pub availability: String,
    pub platform_id: String,
    pub patchline: String,
    pub product: String,
    pub product_name: String,
    pub summary: String,
    pub time: u64,
    pub status_message: String,
    pub note: String,
    pub last_seen_online_timestamp: Option<String>,
    pub is_p2_p_conversation_muted: bool,
    pub group_id: u32,
    pub display_group_id: u32,
    pub group_name: String,
    pub display_group_name: String,
    pub lol: HashMap<String, String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatGameDataChampionSummary {
    pub id: i32,
    pub alias: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatGameflowGameData {
    pub game_id: u64,
    pub queue: LolChatQueue,
    pub team_one: Vec<LolChatTeamPlayerEntry>,
    pub team_two: Vec<LolChatTeamPlayerEntry>,
    pub player_champion_selections: Vec<LolChatChampSelection>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatGameflowGameMap {
    pub id: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatGameflowSession {
    pub phase: LolChatGameflowPhase,
    pub game_data: LolChatGameflowGameData,
    pub map: LolChatGameflowGameMap,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatGroupResource {
    pub id: u32,
    pub name: String,
    pub is_meta_group: bool,
    pub is_localized: bool,
    pub priority: i32,
    pub collapsed: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatIdBody {
    pub id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatLcuSocialConfig {
    pub force_chat_filter: bool,
    pub queue_job_grace_seconds: u64,
    pub silence_chat_while_in_game: bool,
    pub aggressive_scanning: bool,
    pub replace_rich_messages: bool,
    pub game_name_tagline_enabled: bool,
    pub allow_group_by_game: bool,
    pub platform_to_region_map: HashMap<String, String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatLobbyMember {
    pub id: u64,
    pub is_owner: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatLobbyPlayerStatus {
    pub current_lobby_status: Option<LolChatLobbyStatus>,
    pub last_queued_lobby_status: Option<LolChatLobbyStatus>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatLobbyStatus {
    pub queue_id: i32,
    pub is_custom: bool,
    pub is_practice_tool: bool,
    pub is_leader: bool,
    pub custom_spectator_policy: LolChatQueueCustomGameSpectatorPolicy,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatMessage {
    #[serde(rename = "type")]
    pub type_: String,
    pub id: String,
    pub cid: String,
    pub pid: String,
    pub mid: String,
    pub name: String,
    pub game_name: String,
    pub game_tag: String,
    pub time: String,
    pub body: String,
    pub read: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatMessageList {
    pub messages: Vec<LolChatMessage>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatMessagePost {
    pub cid: String,
    pub message: String,
    #[serde(rename = "type")]
    pub type_: LolChatMessageType,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatMessageSend {
    pub message: String,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatMultiGamePresence {
    pub state: LolChatAccountState,
    pub msg: Option<String>,
    pub pid: String,
    pub puuid: String,
    pub region: String,
    pub name: String,
    pub game_name: String,
    pub game_tag: String,
    pub resource: String,
    pub product: String,
    pub summary: String,
    pub patchline: Option<String>,
    pub platform: Option<String>,
    pub location: Option<String>,
    pub details: Option<String>,
    pub actor: Option<String>,
    pub time: Option<u64>,
    pub private_jwt: Option<String>,
    pub private: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatMultiGamePresenceList {
    pub presences: Vec<LolChatMultiGamePresence>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatMultiGamePresenceSharedPayload {
    pub product: Option<String>,
    pub patchline: Option<String>,
    pub platform: Option<String>,
    pub location: Option<String>,
    pub details: Option<String>,
    pub actor: Option<String>,
    pub time: Option<u64>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatMultiGamePresenceUpdate {
    pub state: LolChatAccountState,
    pub msg: Option<String>,
    pub private_jwt: Option<String>,
    pub private: Option<String>,
    pub shared_jwt: Option<String>,
    pub shared: Option<LolChatMultiGamePresenceSharedPayload>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatMutedPlayerInfo {
    pub puuid: String,
    pub summoner_id: u64,
    pub obfuscated_puuid: String,
    pub obfuscated_summoner_id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatNameBody {
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatParticipant {
    pub cid: String,
    pub pid: String,
    pub name: String,
    pub game_name: String,
    pub game_tag: String,
    pub muted: bool,
    pub puuid: String,
    pub region: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatParticipantList {
    pub participants: Vec<LolChatParticipant>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatPatchlineMetadata {
    pub product_id: String,
    pub id: String,
    pub content_paths: HashMap<String, String>,
    pub content_cookies: Vec<LolChatContentCookies>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatPidBody {
    pub pid: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatPlayerMuteStatus {
    pub puuid: String,
    pub obfuscated_puuid: String,
    pub is_player_muted: bool,
    pub is_settings_muted: bool,
    pub is_system_muted: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatPlayerMuteUpdate {
    pub puuids: Vec<String>,
    pub is_muted: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatPlayerPreferences {
    pub data: String,
    pub hash: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub modified: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatPluginRegionLocaleChangedEvent {
    pub region: String,
    pub locale: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatPresenceProduct {
    pub product: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatProductMetadata {
    pub id: String,
    pub name: String,
    pub patchlines: HashMap<String, LolChatPatchlineMetadata>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatProductMetadataMap {
    pub products: HashMap<String, LolChatProductMetadata>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatQueue {
    pub id: i32,
    #[serde(rename = "type")]
    pub type_: String,
    pub game_mode: String,
    pub game_type_config: LolChatQueueGameTypeConfig,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatQueueGameTypeConfig {
    pub id: i64,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatRankedQueueStats {
    pub queue_type: LolChatLeagueQueueType,
    pub is_provisional: bool,
    pub tier: LolChatLeagueTier,
    pub division: LolChatLeagueDivision,
    pub wins: i32,
    pub games: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatRankedStats {
    pub highest_ranked_entry: Option<LolChatRankedQueueStats>,
    pub ranked_regalia_level: i32,
    pub highest_previous_season_achieved_tier: LolChatLeagueTier,
    pub highest_previous_season_achieved_division: LolChatLeagueDivision,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatRsoAuthorization {
    pub current_platform_id: String,
    pub current_account_id: u64,
    pub subject: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatSanitizeRequest {
    pub texts: Vec<String>,
    pub level: Option<u32>,
    pub aggressive_scan: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatSanitizeResponse {
    pub texts: Option<Vec<String>>,
    pub modified: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatSanitizerStatus {
    pub ready: bool,
    pub platform_i_d: String,
    pub locale: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatSession {
    pub state: LolChatChatSessionState,
    pub pid: String,
    pub name: String,
    pub game_name: String,
    pub game_tag: String,
    pub resource: String,
    pub loaded: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatSessionResource {
    pub session_state: LolChatSessionState,
    pub session_expire: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatSettingsResource {
    pub data: Value,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatSpectateGameInfoResource {
    pub drop_in_spectate_game_id: String,
    pub game_queue_type: String,
    pub allow_observe_mode: String,
    pub puuid: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatSummonerStatus {
    pub ready: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatTeamPlayerEntry {
    pub summoner_id: u64,
    pub summoner_internal_name: String,
    pub summoner_name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatTranslateRequest {
    pub keys: Vec<String>,
    pub product_id: String,
    pub locale: String,
    pub patchline: String,
    pub blocking: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatTranslateResponse {
    pub results: Vec<LolChatTranslateResult>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatTranslateResult {
    pub found: bool,
    pub key: String,
    pub product_id: String,
    pub value: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatUserResource {
    pub summoner_id: u64,
    pub id: String,
    pub name: String,
    pub pid: String,
    pub puuid: String,
    pub obfuscated_summoner_id: u64,
    pub game_name: String,
    pub game_tag: String,
    pub icon: i32,
    pub availability: String,
    pub platform_id: String,
    pub patchline: String,
    pub product: String,
    pub product_name: String,
    pub summary: String,
    pub time: u64,
    pub status_message: Option<String>,
    pub last_seen_online_timestamp: Option<String>,
    pub lol: HashMap<String, String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatcookie {
    pub url: String,
    pub name: String,
    pub value: String,
    pub domain: String,
    pub path: String,
    pub secure: bool,
    pub httponly: bool,
    pub expires: Option<i64>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashBlockedPlayerResource {
    pub summoner_id: u64,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashBracket {
    pub tournament_id: i64,
    pub id: i64,
    pub size: i32,
    pub matches: Vec<BracketMatch>,
    pub rosters: Vec<BracketRoster>,
    pub version: i32,
    pub period: i32,
    pub is_complete: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashBracketReadyNotification {
    pub tournament_id: i64,
    pub bracket_id: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashBracketUpdateNotification {
    pub tournament_id: i64,
    pub bracket_id: i64,
    pub current_match_id: i64,
    pub notify_reason: LolClashRosterNotifyReason,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashChangeIconRequest {
    pub icon_id: i32,
    pub icon_color_id: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashChangeNameRequest {
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashClashConfig {
    pub visibility: LolClashClashVisibility,
    pub enabled_state: LolClashClashState,
    pub disabled_reason: String,
    pub estimated_enable_time_millis: u64,
    pub icon_config: String,
    pub honor_level_required: i32,
    pub is_playmode_restriction_enabled: bool,
    pub voice_retry_delay_seconds: i32,
    pub voice_retry_count_limit: i32,
    pub voice_no_delay_auto_start_seconds: i32,
    pub voice_random_start_min_seconds: i32,
    pub voice_random_start_max_seconds: i32,
    pub voice_eob_quit_delay_seconds: i32,
    pub reward_grant_retry_interval_seconds: i32,
    pub max_time_before_lockin_notify_seconds: i32,
    pub honor_refresh_retry_seconds: i32,
    pub event_sending_enabled: bool,
    pub disabled_events: Vec<String>,
    pub min_clash_summoner_level: u32,
    pub min_clash_notifications_summoner_level: u32,
    pub check_parties_registration: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashClashDisabledConfig {
    pub disabled_reason: String,
    pub estimated_enable_time_millis: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashClashSettingCategory {
    pub simple_state_flag_ids: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashClientFailedInvite {
    pub player_id: u64,
    pub exception: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashClubsSummoner {
    pub summoner_id: u64,
    pub display_name: String,
    pub profile_icon_id: i32,
    pub summoner_level: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashEogPlayerUpdateDTO {
    pub tournament_id: i64,
    pub game_id: i64,
    pub winner: bool,
    pub theme_vp: i32,
    pub season_vp: i32,
    pub lowest_position: i32,
    pub bracket_size: i32,
    pub bid: i32,
    pub tier: i32,
    pub earned_rewards: Vec<ClashRewardDefinition>,
    pub reward_progress: Vec<ClashRewardDefinition>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashFindPlayers {
    pub invitation_id: String,
    pub member_id: i64,
    pub page: i32,
    pub count: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashFindTeams {
    pub tournament_id: i64,
    pub page: i32,
    pub count: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashGameflowAvailability {
    pub is_available: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashGameflowGameDodge {
    pub state: LolClashMatchmakingDodgeState,
    pub dodge_ids: Vec<u64>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashGameflowPartiesRegistrationStatus {
    pub complete: bool,
    pub error_codes: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashGameflowSession {
    pub phase: LolClashGameflowPhase,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashKickRequest {
    pub summoner_id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashLftState {
    pub lft: bool,
    pub primary_pos: String,
    pub secondary_pos: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashLoginSession {
    pub state: LolClashLoginSessionState,
    pub summoner_id: Option<u64>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashMatchmakingDodgeData {
    pub state: LolClashMatchmakingDodgeState,
    pub dodger_id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashMatchmakingReadyCheckResource {
    pub state: LolClashMatchmakingReadyCheckState,
    pub player_response: LolClashMatchmakingReadyCheckResponse,
    pub dodge_warning: LolClashMatchmakingDodgeWarning,
    pub timer: f32,
    pub decliner_ids: Vec<u64>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashMatchmakingSearchResource {
    pub queue_id: i32,
    pub ready_check: LolClashMatchmakingReadyCheckResource,
    pub dodge_data: LolClashMatchmakingDodgeData,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashMemberBanUnbanNotification {
    pub player_id: i64,
    pub notify_player_id: i64,
    pub notify_puuid: String,
    pub reason: String,
    pub notify_reason: LolClashNotifyReason,
    pub tournaments: Vec<MemberBanUnbanTournament>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashNoShowPingDTO {
    pub tournament_id: i64,
    pub match_id: i64,
    pub dodge_time: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashNoShowPingResponse {
    pub tournament_id: i64,
    pub match_id: i64,
    pub dodge_time: i64,
    pub data: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashNoShowPingResponseData {
    pub login_time: i64,
    pub dodge_time: i64,
    pub gameflow_state: LolClashGameflowPhase,
    pub is_playmode_restricted: bool,
    pub ready_check_info: LolClashReadyCheckInfo,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashOfferTicketRequest {
    pub ticket_amount: i32,
    pub ticket_type: TicketType,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashPendingRosterNotification {
    pub pending_roster: PendingRosterDTO,
    pub notify_reason: LolClashNotifyReason,
    pub source_player_id: u64,
    pub target_player_id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashPlayerChatRoster {
    pub tournament_id: i64,
    pub start_time_ms: i64,
    pub end_time_ms: i64,
    pub tournament_state: LolClashTournamentState,
    pub player_state: LolClashPlayerState,
    pub is_registered: bool,
    pub name: String,
    pub short_name: String,
    pub icon_id: i32,
    pub icon_color_id: i32,
    pub logo_url: String,
    pub invitation_id: String,
    pub multi_user_chat_id: String,
    pub multi_user_chat_password: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashPlayerData {
    pub tickets: HashMap<String, i32>,
    pub is_clash_banned: bool,
    pub tier: i32,
    pub lft: bool,
    pub primary_pos: String,
    pub secondary_pos: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashPlayerNotification {
    pub source: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub id: u64,
    pub background_url: String,
    pub data: HashMap<String, String>,
    pub state: String,
    pub icon_url: String,
    pub title_key: String,
    pub detail_key: String,
    pub created: String,
    pub expires: String,
    pub critical: bool,
    pub dismissible: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashPlayerNotificationData {
    pub notify_reason: LolClashNotifyReason,
    pub roster_notify_reason: LolClashRosterNotifyReason,
    pub tournament_notify_reason: LolClashTournamentNotifyReason,
    pub source_summoner_id: u64,
    pub target_summoner_id: u64,
    pub notification: LolClashPlayerNotification,
    pub key_suffix: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashPlayerRewards {
    pub season_vp: i32,
    pub theme_vp: Vec<LolClashThemeVp>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashPlayerTournamentData {
    pub state: LolClashPlayerState,
    pub roster_id: String,
    pub bracket_id: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashPlayerUpdateNotification {
    pub player: PlayerDTO,
    pub notify_reason: LolClashNotifyReason,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashPlaymodeRestrictedInfo {
    pub is_restricted: bool,
    pub tournament_id: i64,
    pub presence_state: LolClashPresenceState,
    pub roster_id: String,
    pub phase_id: i64,
    pub ready_for_voice: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashProfileInfo {
    pub honor_level: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashQueue {
    pub id: i32,
    pub map_id: i32,
    pub name: String,
    pub short_name: String,
    pub description: String,
    pub detailed_description: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub game_mode: String,
    pub category: LolClashQueueGameCategory,
    pub game_type_config: LolClashQueueGameTypeConfig,
    pub num_players_per_team: i32,
    pub minimum_participant_list_size: i32,
    pub maximum_participant_list_size: i32,
    pub min_level: u32,
    pub is_ranked: bool,
    pub are_free_champions_allowed: bool,
    pub is_team_builder_managed: bool,
    pub queue_availability: LolClashQueueAvailability,
    pub queue_rewards: LolClashQueueReward,
    pub spectator_enabled: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashQueueGameTypeConfig {
    pub id: i64,
    pub name: String,
    pub max_allowable_bans: i32,
    pub allow_trades: bool,
    pub exclusive_pick: bool,
    pub duplicate_pick: bool,
    pub team_champion_pool: bool,
    pub cross_team_champion_pool: bool,
    pub advanced_learning_quests: bool,
    pub battle_boost: bool,
    pub death_match: bool,
    pub do_not_remove: bool,
    pub learning_quests: bool,
    pub onboard_coop_beginner: bool,
    pub reroll: bool,
    pub main_pick_timer_duration: i32,
    pub post_pick_timer_duration: i32,
    pub ban_timer_duration: i32,
    pub pick_mode: String,
    pub ban_mode: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashQueueReward {
    pub is_ip_enabled: bool,
    pub is_xp_enabled: bool,
    pub is_champion_points_enabled: bool,
    pub party_size_ip_rewards: Vec<i32>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashRankedScoutingMember {
    pub player_id: u64,
    pub champion_scouting_data: Vec<LolClashRankedScoutingTopChampion>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashRankedScoutingTopChampion {
    pub champion_id: i32,
    pub rank: i32,
    pub win_count: i32,
    pub game_count: i32,
    pub win_rate: i32,
    pub kda: String,
    pub kda_classification: LolClashKdaClassification,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashReadyCheckInfo {
    pub timestamp_received: i64,
    pub timestamp_response_complete: i64,
    pub timestamp_last_clash_gameflow_dodge: i64,
    pub queue_id: i32,
    pub is_accept_successful: bool,
    pub accept_error: String,
    pub ready_check_resource: LolClashMatchmakingReadyCheckResource,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashRegisteredRosterNotification {
    pub roster: RosterDTO,
    pub notify_reason: LolClashRosterNotifyReason,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashRoster {
    pub tournament_id: i64,
    pub invitation_id: String,
    pub id: String,
    pub name: String,
    pub short_name: String,
    pub icon_id: i32,
    pub icon_color_id: i32,
    pub captain_summoner_id: u64,
    pub tier: i32,
    pub points: i32,
    pub wins: i32,
    pub losses: i32,
    pub current_bracket_wins: i32,
    pub num_completed_periods: i32,
    pub is_eliminated: bool,
    pub is_registered: bool,
    pub is_active_in_current_phase: bool,
    pub is_current_bracket_complete: bool,
    pub high_tier_variance: bool,
    pub members: Vec<LolClashRosterMember>,
    pub available_logos: Vec<RewardLogo>,
    pub suggested_invites: Vec<LolClashSuggestedInvite>,
    pub phase_infos: Vec<LolClashRosterPhaseInfo>,
    pub withdraw: Option<RosterWithdraw>,
    pub is_clash_banned: bool,
    pub lft: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashRosterDetails {
    pub name: String,
    pub short_name: String,
    pub icon_id: i32,
    pub icon_color_id: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashRosterDynamicStateNotification {
    pub roster_dynamic_state: RosterDynamicStateDTO,
    pub notify_reason: LolClashRosterNotifyReason,
    pub source_player_id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashRosterMatchAggregatedStats {
    pub round: i32,
    pub duration_ms: i64,
    pub opponent_short_name: String,
    pub opponent_icon_id: i32,
    pub opponent_icon_color_id: i32,
    pub win: bool,
    pub loser_bracket: bool,
    pub game_id: i64,
    pub kills: i32,
    pub opponent_kills: i32,
    pub player_champion_ids: HashMap<String, i32>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashRosterMember {
    pub summoner_id: u64,
    pub state: LolClashRosterMemberState,
    pub current_buyin: i32,
    pub buyin_type: TicketType,
    pub previous_buyin: i32,
    pub incoming_offers: Vec<LolClashTicketOffer>,
    pub position: Position,
    pub replaced_summoner_id: u64,
    pub tier: i32,
    pub invite_type: InviteType,
    pub inviter_id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashRosterPeriodAggregatedStats {
    pub period: i32,
    pub bracket_size: i32,
    pub time: i64,
    pub match_stats: Vec<LolClashRosterMatchAggregatedStats>,
    pub player_bids: HashMap<String, i32>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashRosterPhaseInfo {
    pub phase_id: i64,
    pub period: i32,
    pub checkin_time: i64,
    pub is_bracket_complete: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashRosterPlayerAggregatedStats {
    pub raw_stats_sum: HashMap<String, i32>,
    pub raw_stats_max: HashMap<String, i32>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashRosterPlayerNotification {
    pub roster: RosterDTO,
    pub notify_reason: LolClashRosterNotifyReason,
    pub player_notification_d_t_o: PlayerDTO,
    pub source_player_id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashRosterStats {
    pub roster_id: i64,
    pub tournament_theme_id: i32,
    pub tournament_name_loc_key: String,
    pub tournament_name_loc_key_secondary: String,
    pub start_time_ms: i64,
    pub end_time_ms: i64,
    pub tournament_periods: i32,
    pub tier: i32,
    pub roster_name: String,
    pub roster_short_name: String,
    pub roster_icon_id: i32,
    pub roster_icon_color_id: i32,
    pub period_stats: Vec<LolClashRosterPeriodAggregatedStats>,
    pub player_stats: HashMap<String, LolClashRosterPlayerAggregatedStats>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashRosterWithdrawNotification {
    pub version: i32,
    pub tournament_id: i64,
    pub roster_id: i64,
    pub withdraw: RosterWithdraw,
    pub notify_reason: LolClashRosterNotifyReason,
    pub source_player_id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashScoutingChampionMastery {
    pub champion_id: i32,
    pub champion_level: i32,
    pub champion_points: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashScoutingChampions {
    pub player_id: u64,
    pub total_mastery_score: u64,
    pub top_masteries: Vec<LolClashScoutingChampionMastery>,
    pub top_season_champions: Vec<LolClashScoutingSeasonChampion>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashScoutingSeasonChampion {
    pub champion_id: i32,
    pub win_count: i32,
    pub game_count: i32,
    pub win_rate: i32,
    pub kda: String,
    pub kda_classification: LolClashKdaClassification,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashSetPositionRequest {
    pub position: Position,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashSetTicketRequest {
    pub ticket_amount: i32,
    pub ticket_type: TicketType,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashSettingCategory {
    pub schema_version: i32,
    pub data: Value,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashSimpleStateFlag {
    pub id: String,
    pub status: LolClashSimpleStateStatus,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashSuggestedInvite {
    pub summoner_id: u64,
    pub suggester_summoner_id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashSuggestionInvite {
    pub inviter_id: u64,
    pub invitee_players: Vec<u64>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashSuggestionInvitee {
    pub captain_id: u64,
    pub invitee_id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashTeamOpenState {
    pub invitation_id: String,
    pub captain_id: i64,
    pub open_team: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashThemeVp {
    pub theme_id: i32,
    pub vp: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashThirdPartyApiPlayer {
    pub summoner_id: u64,
    pub role: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashThirdPartyApiRoster {
    pub captain: LolClashThirdPartyApiPlayer,
    pub members: Vec<LolClashThirdPartyApiPlayer>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashTicketOffer {
    pub summoner_id: u64,
    pub amount: i32,
    pub ticket_type: TicketType,
    pub is_accepted: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashTournament {
    pub id: i64,
    pub theme_id: i32,
    pub name_loc_key: String,
    pub name_loc_key_secondary: String,
    pub buy_in_options: Vec<i32>,
    pub buy_in_options_premium: Vec<i32>,
    pub entry_fee: i32,
    pub roster_size: i32,
    pub allow_roster_creation: bool,
    pub roster_create_deadline: i64,
    pub scouting_duration_ms: i64,
    pub start_time_ms: i64,
    pub end_time_ms: i64,
    pub last_theme_of_season: bool,
    pub bracket_size: String,
    pub queue_id: i32,
    pub is_sms_restriction_enabled: bool,
    pub is_honor_restriction_enabled: bool,
    pub is_ranked_restriction_enabled: bool,
    pub phases: Vec<LolClashTournamentPhase>,
    pub reward_config: Vec<ClashRewardConfigClient>,
    pub tier_configs: Vec<TierConfig>,
    pub bracket_formation_init_delay_ms: i64,
    pub bracket_formation_interval_ms: i64,
    pub status: TournamentStatusEnum,
    pub resume_time: i64,
    pub lft: bool,
    pub max_invites: i32,
    pub max_suggestions_per_player: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashTournamentGameEnd {
    pub tournament_id: i64,
    pub tournament_name_loc_key: String,
    pub tournament_name_loc_key_secondary: String,
    pub bracket_id: i64,
    pub old_bracket: Option<LolClashBracket>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashTournamentHistoryAndWinners {
    pub tournament_history: Vec<LolClashTournament>,
    pub tournament_winners: LolClashTournamentWinnerHistory,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashTournamentPhase {
    pub id: i64,
    pub tournament_id: i64,
    pub period: i32,
    pub lockin_start_time: i64,
    pub scouting_start_time: i64,
    pub cancelled: bool,
    pub limit_tiers: Vec<i32>,
    pub capacity_status: CapacityEnum,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashTournamentPhaseProgressNotificationDTO {
    pub tournament_id: i64,
    pub phase_id: i64,
    pub capacity_status: CapacityEnum,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashTournamentStateInfo {
    pub tournament_id: i64,
    pub state: LolClashTournamentState,
    pub current_phase_id: i64,
    pub next_phase_id: i64,
    pub next_state_change_time: i64,
    pub num_remaining_periods: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashTournamentSummary {
    pub state: LolClashTournamentState,
    pub tournament_id: i64,
    pub roster_id: String,
    pub bracket_id: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashTournamentUpdatedNotification {
    pub missing_player_ids: Vec<i64>,
    pub current_retry: i32,
    pub max_retry: i32,
    pub retry_seconds: i64,
    pub notify_reason: LolClashRosterNotifyReason,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashTournamentUpdatedNotificationDTO {
    pub reason: LolClashTournamentNotifyReason,
    pub tournament_id: i64,
    pub tournament: Option<TournamentDTO>,
    pub relevant_phase_id: Option<i64>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashTournamentWinnerHistory {
    pub tournament_id: i64,
    pub winners: Vec<LolClashTournamentWinnerInfo>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashTournamentWinnerInfo {
    pub roster_id: i64,
    pub tier: i32,
    pub short_name: String,
    pub name: String,
    pub logo: i32,
    pub logo_color: i32,
    pub create_time: i64,
    pub average_win_duration: i64,
    pub player_ids: Vec<u64>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolClashUserResource {
    pub summoner_id: u64,
    pub lol: HashMap<String, String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCollectionsAccountIdAndSummonerId {
    pub summoner_id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCollectionsCollectionsChampionMastery {
    pub player_id: u64,
    pub champion_id: i32,
    pub champion_level: i32,
    pub champion_points: i32,
    pub formatted_champion_points: String,
    pub last_play_time: u64,
    pub champion_points_since_last_level: i32,
    pub champion_points_until_next_level: i32,
    pub tokens_earned: i32,
    pub chest_granted: bool,
    pub highest_grade: String,
    pub formatted_mastery_goal: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCollectionsCollectionsChestEligibility {
    pub earnable_chests: u32,
    pub maximum_chests: u32,
    pub next_chest_recharge_time: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCollectionsCollectionsOwnership {
    pub loyalty_reward: bool,
    pub xbox_g_p_reward: bool,
    pub owned: bool,
    pub rental: LolCollectionsCollectionsRental,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCollectionsCollectionsRental {
    pub end_date: u64,
    pub purchase_date: u64,
    pub win_count_remaining: i32,
    pub rented: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCollectionsCollectionsSummonerBackdrop {
    pub summoner_id: u64,
    pub account_id: u64,
    pub profile_icon_id: i32,
    pub champion_id: i32,
    pub backdrop_type: LolCollectionsCollectionsSummonerBackdropType,
    pub backdrop_image: String,
    pub backdrop_video: String,
    pub backdrop_mask_color: String,
    pub puuid: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCollectionsCollectionsSummonerSpells {
    pub summoner_id: u64,
    pub spells: Vec<u64>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCollectionsCollectionsTopChampionMasteries {
    pub summoner_id: u64,
    pub score: u64,
    pub masteries: Vec<LolCollectionsCollectionsChampionMastery>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCollectionsCollectionsWardSkin {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub ownership: LolCollectionsCollectionsOwnership,
    pub ward_image_path: String,
    pub ward_shadow_image_path: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCollectionsCollectionsWardSkinList {
    pub ward_skin_list: Vec<i64>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCollectionsGameDataChampionMasteries {
    pub tree: LolCollectionsGameDataChampionMasteryTree,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCollectionsGameDataChampionMasteryGroup {
    pub id: u32,
    pub rows: Vec<LolCollectionsGameDataChampionMasteryRow>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCollectionsGameDataChampionMasteryRow {
    pub masteries: Vec<u32>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCollectionsGameDataChampionMasteryTree {
    pub groups: Vec<LolCollectionsGameDataChampionMasteryGroup>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCollectionsGameDataChampionQuestSkin {
    pub id: i32,
    pub name: String,
    pub splash_path: String,
    pub splash_video_path: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCollectionsGameDataChampionSkin {
    pub id: i32,
    pub is_base: bool,
    pub name: String,
    pub quest_skin_info: LolCollectionsGameDataQuestSkinInfo,
    pub splash_path: String,
    pub splash_video_path: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCollectionsGameDataChampionSummary {
    pub id: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCollectionsGameDataQuestSkinInfo {
    pub tiers: Vec<LolCollectionsGameDataChampionQuestSkin>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCollectionsGameDataSplashMetadata {
    pub calculated_color: String,
    pub override_color: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCollectionsInventoryItem {
    pub uuid: String,
    pub item_id: i32,
    pub inventory_type: String,
    pub purchase_date: String,
    pub quantity: u64,
    pub ownership_type: LolCollectionsItemOwnershipType,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCollectionsInventoryItemDTO {
    pub item_id: i32,
    pub inventory_type: String,
    pub expiration_date: String,
    pub purchase_date: String,
    pub quantity: u64,
    pub owned_quantity: u64,
    pub used_in_game_date: String,
    pub entitlement_id: String,
    pub entitlement_type_id: String,
    pub instance_id: String,
    pub instance_type_id: String,
    pub payload: Value,
    pub f2p: bool,
    pub rental: bool,
    pub loyalty: bool,
    pub lsb: bool,
    pub wins: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCollectionsInventoryItemWithPayload {
    pub uuid: String,
    pub item_id: i32,
    pub inventory_type: String,
    pub purchase_date: String,
    pub quantity: u64,
    pub ownership_type: LolCollectionsItemOwnershipType,
    pub payload: Value,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCollectionsLcdsDynamicClientConfig {
    pub disabled_champions: Value,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCollectionsLoginSession {
    pub state: LolCollectionsLoginSessionStates,
    pub summoner_id: u64,
    pub connected: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCollectionsNumberFormattingBehavior {
    pub digits_for_thousands_seperator: u32,
    pub trim_trailing_zeros_after_decimal: bool,
    pub western_number_grouping: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCollectionsNumberFormattingData {
    pub decimal_seperator: String,
    pub thousand_seperator: String,
    pub thousand_abbreviation: String,
    pub ten_thousand_abbreviation: String,
    pub million_abbreviation: String,
    pub one_hundred_million_abbreviation: String,
    pub billion_abbreviation: String,
    pub trillion_abbreviation: String,
    pub second_abbreviation: String,
    pub minute_abbreviation: String,
    pub hour_abbreviation: String,
    pub meters_abbreviation: String,
    pub kilometers_abbreviation: String,
    pub percentage_format: String,
    pub number_formatting_behavior: LolCollectionsNumberFormattingBehavior,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCollectionsPlayerNotification {
    pub background_url: String,
    pub critical: bool,
    pub data: HashMap<String, String>,
    pub detail_key: String,
    pub icon_url: String,
    pub id: u64,
    pub source: String,
    pub state: String,
    pub title_key: String,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCollectionsSummoner {
    pub summoner_id: u64,
    pub account_id: u64,
    pub summoner_level: u32,
    pub profile_icon_id: i32,
    pub puuid: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCollectionsSummonerProfile {
    pub background_skin_id: Option<i32>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCollectionsSummonerProfileUpdate {
    pub key: String,
    pub value: Value,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolContentTargetingAccountIdAndSummonerId {
    pub summoner_id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolContentTargetingCollectionsChampionMastery {
    pub player_id: u64,
    pub champion_id: i32,
    pub champion_level: i32,
    pub champion_points: i32,
    pub last_play_time: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolContentTargetingContentTargetingFilterResponse {
    pub filters: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolContentTargetingContentTargetingLocaleResponse {
    pub locale: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolContentTargetingDataModelResponse {
    pub response_code: i64,
    pub model_data: Value,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolContentTargetingGameflowGameData {
    pub game_id: u64,
    pub queue: LolContentTargetingQueue,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolContentTargetingGameflowSession {
    pub phase: LolContentTargetingGameflowPhase,
    pub game_data: LolContentTargetingGameflowGameData,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolContentTargetingGeoInfo {
    pub country: String,
    pub city: String,
    pub region: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolContentTargetingGeoInfoResponse {
    pub success: bool,
    pub geo_info: LolContentTargetingGeoInfo,
    pub error_message: String,
    pub is_latest: bool,
    pub is_initialized: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolContentTargetingLoginSession {
    pub state: LolContentTargetingLoginSessionState,
    pub summoner_id: u64,
    pub puuid: String,
    pub id_token: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolContentTargetingMission {
    pub id: String,
    pub status: String,
    pub completed_date: i64,
    pub internal_name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolContentTargetingPlatformConfig {
    pub enabled: bool,
    pub mapping: String,
    pub level_filter_enabled: bool,
    pub ranked_filter_enabled: bool,
    pub location_filters_enabled: bool,
    pub rank_filter_enabled: bool,
    pub a_b_test_filter_enabled: bool,
    pub a_b_test_filter_groups: u64,
    pub a_b_test_filter_salt: u64,
    pub entitlements_filter_enabled: bool,
    pub mastery_filter_enabled: bool,
    pub mastery_filter_level_threshold: u32,
    pub mastery_filter_days_since_last_played: u32,
    pub mastery_filter_champion_limit: u32,
    pub main_filter_enabled: bool,
    pub entitlements_prefix: String,
    pub targeting_attribute_storage_enabled: bool,
    pub targeting_attribute_storage_base_uri: String,
    pub missions_filter_enabled: bool,
    pub summoner_icon_filter_enabled: bool,
    pub asynchronous_event_handler_setup_delay_in_seconds: u32,
    pub tas_ingestion_delay_in_seconds: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolContentTargetingQueue {
    pub id: i32,
    pub map_id: i32,
    pub game_mode: String,
    pub category: LolContentTargetingQueueGameCategory,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolContentTargetingRankedQueueStats {
    pub queue_type: LolContentTargetingRankedQueue,
    pub tier: LolContentTargetingRankedTier,
    pub division: LolContentTargetingRankedDivision,
    pub is_provisional: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolContentTargetingRankedStats {
    pub queues: Vec<LolContentTargetingRankedQueueStats>,
    pub highest_ranked_entry: Option<LolContentTargetingRankedQueueStats>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolContentTargetingRegionLocale {
    pub locale: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolContentTargetingSettingsResource {
    pub data: Value,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolContentTargetingSummoner {
    pub summoner_level: u32,
    pub profile_icon_id: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolContentTargetingTargetingAttributes {
    pub result: HashMap<String, Value>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolContentTargetingToken {
    pub entitlements: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsAccountSettingsCategoryDataResource {
    pub type_to_last_opened_date: HashMap<String, i64>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsCapOffer {
    pub id: String,
    pub type_id: String,
    pub label: String,
    pub product_id: String,
    pub merchant_id: String,
    pub payload: Vec<Value>,
    pub active: bool,
    pub start_date: String,
    pub created_date: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsCompanionsGroupViewModel {
    pub group_name: String,
    pub group_id: u32,
    pub num_owned: u32,
    pub num_available: u32,
    pub items: Vec<LolCosmeticsCosmeticsCompanionViewModel>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsCompanionsGroupedViewModel {
    pub selected_loadout_item: LolCosmeticsCosmeticsCompanionViewModel,
    pub default_item_id: i32,
    pub groups: Vec<LolCosmeticsCompanionsGroupViewModel>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsCosmeticSettingsResource {
    pub data: Option<LolCosmeticsAccountSettingsCategoryDataResource>,
    pub schema_version: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsCosmeticsCompanion {
    pub content_id: String,
    pub item_id: i32,
    pub name: String,
    pub description: String,
    pub loadouts_icon: String,
    pub owned: bool,
    pub selected: bool,
    pub loyalty: bool,
    pub f2p: bool,
    pub rarity_value: u32,
    pub purchase_date: String,
    pub species: String,
    pub group_id: u32,
    pub color: String,
    pub level: u32,
    pub upgrades: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsCosmeticsCompanionViewModel {
    pub content_id: String,
    pub item_id: i32,
    pub name: String,
    pub description: String,
    pub loadouts_icon: String,
    pub owned: bool,
    pub selected: bool,
    pub loyalty: bool,
    pub f2p: bool,
    pub rarity_value: u32,
    pub purchase_date: String,
    pub is_recent_item: bool,
    pub species: String,
    pub group_id: u32,
    pub color: String,
    pub level: u32,
    pub upgrades: Vec<LolCosmeticsCosmeticsCompanionViewModel>,
    pub offer_data: Option<LolCosmeticsCapOffer>,
    pub star_shards_price: LolCosmeticsCosmeticsOfferPrice,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsCosmeticsOfferPrice {
    pub offer_id: String,
    pub price: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsCosmeticsTFTDamageSkin {
    pub content_id: String,
    pub item_id: i32,
    pub name: String,
    pub description: String,
    pub loadouts_icon: String,
    pub owned: bool,
    pub selected: bool,
    pub loyalty: bool,
    pub f2p: bool,
    pub rarity_value: u32,
    pub purchase_date: String,
    pub level: u32,
    pub group_id: u32,
    pub group_name: String,
    pub upgrades: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsCosmeticsTFTDamageSkinViewModel {
    pub content_id: String,
    pub item_id: i32,
    pub name: String,
    pub description: String,
    pub loadouts_icon: String,
    pub owned: bool,
    pub selected: bool,
    pub loyalty: bool,
    pub f2p: bool,
    pub rarity_value: u32,
    pub purchase_date: String,
    pub is_recent_item: bool,
    pub level: u32,
    pub group_id: u32,
    pub group_name: String,
    pub upgrades: Vec<LolCosmeticsCosmeticsTFTDamageSkinViewModel>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsCosmeticsTFTMapSkin {
    pub content_id: String,
    pub item_id: i32,
    pub name: String,
    pub description: String,
    pub loadouts_icon: String,
    pub owned: bool,
    pub selected: bool,
    pub loyalty: bool,
    pub f2p: bool,
    pub rarity_value: u32,
    pub purchase_date: String,
    pub group_id: u32,
    pub group_name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsCosmeticsTFTMapSkinViewModel {
    pub content_id: String,
    pub item_id: i32,
    pub name: String,
    pub description: String,
    pub loadouts_icon: String,
    pub owned: bool,
    pub selected: bool,
    pub loyalty: bool,
    pub f2p: bool,
    pub rarity_value: u32,
    pub purchase_date: String,
    pub is_recent_item: bool,
    pub group_id: u32,
    pub group_name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsCosmeticsTFTPlaybook {
    pub content_id: String,
    pub item_id: i32,
    pub name: String,
    pub description: String,
    pub loadouts_icon: String,
    pub owned: bool,
    pub selected: bool,
    pub loyalty: bool,
    pub f2p: bool,
    pub rarity_value: u32,
    pub purchase_date: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsCosmeticsTFTPlaybookAugment {
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsCosmeticsTFTPlaybookViewModel {
    pub content_id: String,
    pub item_id: i32,
    pub name: String,
    pub description: String,
    pub loadouts_icon: String,
    pub owned: bool,
    pub selected: bool,
    pub loyalty: bool,
    pub f2p: bool,
    pub rarity_value: u32,
    pub purchase_date: String,
    pub is_recent_item: bool,
    pub augments: Vec<LolCosmeticsCosmeticsTFTPlaybookAugment>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsGameDataCompanion {
    pub content_id: String,
    pub item_id: i32,
    pub name: String,
    pub loadouts_icon: String,
    pub description: String,
    pub species_name: String,
    pub species_id: u32,
    pub color_name: String,
    pub level: u32,
    pub rarity_value: u32,
    pub upgrades: Vec<String>,
    pub t_f_t_only: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsGameDataTFTCosmeticsDefaults {
    pub playbook: Value,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsGameDataTFTDamageSkin {
    pub content_id: String,
    pub item_id: i32,
    pub name: String,
    pub description: String,
    pub loadouts_icon: String,
    pub group_id: u32,
    pub group_name: String,
    pub rarity_value: u32,
    pub level: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsGameDataTFTMapSkin {
    pub content_id: String,
    pub item_id: i32,
    pub name: String,
    pub description: String,
    pub loadouts_icon: String,
    pub group_id: u32,
    pub group_name: String,
    pub rarity_value: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsGameDataTFTPlaybook {
    pub content_id: String,
    pub item_id: i32,
    pub name: String,
    pub description: String,
    pub translated_name: String,
    pub loadouts_icon: String,
    pub augments: Vec<LolCosmeticsCosmeticsTFTPlaybookAugment>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsLoadout {
    pub id: String,
    pub name: String,
    pub scope: String,
    pub item_id: i32,
    pub loadout: HashMap<String, LolCosmeticsLoadoutItem>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsLoadoutItem {
    pub inventory_type: String,
    pub content_id: String,
    pub item_id: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsLoadoutUpdateDto {
    pub loadout: HashMap<String, LolCosmeticsLoadoutItem>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsTFTDamageSkinGroupViewModel {
    pub group_name: String,
    pub group_id: u32,
    pub num_owned: u32,
    pub num_available: u32,
    pub items: Vec<LolCosmeticsCosmeticsTFTDamageSkinViewModel>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsTFTDamageSkinGroupedViewModel {
    pub selected_loadout_item: LolCosmeticsCosmeticsTFTDamageSkinViewModel,
    pub default_item_id: i32,
    pub groups: Vec<LolCosmeticsTFTDamageSkinGroupViewModel>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsTFTMapSkinGroupViewModel {
    pub group_name: String,
    pub group_id: u32,
    pub num_owned: u32,
    pub num_available: u32,
    pub items: Vec<LolCosmeticsCosmeticsTFTMapSkinViewModel>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsTFTMapSkinGroupedViewModel {
    pub selected_loadout_item: LolCosmeticsCosmeticsTFTMapSkinViewModel,
    pub default_item_id: i32,
    pub groups: Vec<LolCosmeticsTFTMapSkinGroupViewModel>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsTFTPlaybookGroupViewModel {
    pub group_name: String,
    pub group_id: u32,
    pub num_owned: u32,
    pub num_available: u32,
    pub items: Vec<LolCosmeticsCosmeticsTFTPlaybookViewModel>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsTFTPlaybookGroupedViewModel {
    pub selected_loadout_item: LolCosmeticsCosmeticsTFTPlaybookViewModel,
    pub default_item_id: i32,
    pub groups: Vec<LolCosmeticsTFTPlaybookGroupViewModel>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsTFTSettingsDataResource {
    pub icon_override: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsTFTSettingsResource {
    pub data: Option<LolCosmeticsTFTSettingsDataResource>,
    pub schema_version: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsUserResource {
    pub summoner_id: u64,
    pub lol: HashMap<String, String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolDiscordRpGameDataChampionSummary {
    pub id: i32,
    pub name: String,
    pub alias: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolDiscordRpPartyPresenceData {
    pub party_id: String,
    pub queue_id: i32,
    pub summoners: Vec<u64>,
    pub max_players: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEmailVerificationAccessToken {
    pub token: String,
    pub expiry: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEmailVerificationEmailUpdate {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEmailVerificationEmailVerificationSession {
    pub email: String,
    pub email_verified: bool,
    pub fatal_error: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEmailVerificationRegionLocale {
    pub region: String,
    pub locale: String,
    pub web_region: String,
    pub web_language: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEmailVerificationRemoteEmailVerificationSession {
    pub email: String,
    pub email_verified: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEmailVerificationValidationStatus {
    pub email_status: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameChampionMasteryGrade {
    pub player_id: u64,
    pub champion_id: i32,
    pub grade: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameChampionMasteryMini {
    pub player_id: u64,
    pub champion_id: i32,
    pub champion_level: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameChampionMasteryUpdate {
    pub id: String,
    pub game_id: u64,
    pub player_id: u64,
    pub champion_id: i32,
    pub has_leveled_up: bool,
    pub level: i64,
    pub points_before_game: i64,
    pub points_gained: i64,
    pub points_gained_individual_contribution: i64,
    pub bonus_points_gained: i64,
    pub points_since_last_level_before_game: i64,
    pub points_until_next_level_before_game: i64,
    pub points_until_next_level_after_game: i64,
    pub tokens_earned: i64,
    pub token_earned_after_game: bool,
    pub grade: String,
    pub score: i64,
    pub level_up_list: Vec<LolEndOfGameChampionMasteryMini>,
    pub member_grades: Vec<LolEndOfGameChampionMasteryGrade>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameEndOfGamePlayer {
    pub stats: Value,
    pub items: Vec<i32>,
    pub puuid: String,
    pub bot_player: bool,
    pub champion_id: i32,
    pub game_id: u64,
    pub leaver: bool,
    pub leaves: i32,
    pub level: i32,
    pub losses: i32,
    pub profile_icon_id: i32,
    pub spell1_id: i32,
    pub spell2_id: i32,
    pub summoner_name: String,
    pub team_id: i32,
    pub wins: i32,
    pub summoner_id: u64,
    pub selected_position: String,
    pub detected_team_position: String,
    pub skin_splash_path: String,
    pub skin_tile_path: String,
    pub skin_emblem_paths: Vec<String>,
    pub champion_name: String,
    pub champion_square_portrait_path: String,
    pub is_local_player: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameEndOfGamePlayerComplaintV2 {
    pub game_id: u64,
    pub reported_summoner_id: u64,
    pub offenses: String,
    pub comment: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameEndOfGamePlayerReport {
    pub reported_puuid: String,
    pub game_id: u64,
    pub offense: String,
    pub comment: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameEndOfGamePoints {
    pub point_change_from_champions_owned: i32,
    pub point_change_from_gameplay: i32,
    pub points_used: i32,
    pub previous_points: i32,
    pub points_until_next_reroll: i32,
    pub reroll_count: i32,
    pub total_points: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameEndOfGameStats {
    pub difficulty: String,
    pub game_id: u64,
    pub game_length: i32,
    pub game_mode: String,
    pub game_mutators: Vec<String>,
    pub game_type: String,
    pub invalid: bool,
    pub queue_type: String,
    pub ranked: bool,
    pub report_game_id: u64,
    pub multi_user_chat_id: String,
    pub multi_user_chat_password: String,
    pub multi_user_chat_j_w_t: String,
    pub teams: Vec<LolEndOfGameEndOfGameTeam>,
    pub local_player: LolEndOfGameEndOfGamePlayer,
    pub my_team_status: String,
    pub leveled_up: bool,
    pub new_spells: Vec<i32>,
    pub previous_level: u64,
    pub rp_earned: i32,
    pub base_points: i32,
    pub battle_boost_ip_earned: i32,
    pub boost_ip_earned: i32,
    pub first_win_bonus: i32,
    pub ip_earned: i32,
    pub ip_total: i32,
    pub boost_xp_earned: i32,
    pub experience_earned: i32,
    pub experience_total: i32,
    pub global_boost_xp_earned: i32,
    pub loyalty_boost_xp_earned: i32,
    pub xbgp_boost_xp_earned: i32,
    pub missions_xp_earned: i32,
    pub previous_xp_total: u64,
    pub next_level_xp: u64,
    pub current_level: u64,
    pub pre_level_up_experience_total: u64,
    pub pre_level_up_next_level_xp: u64,
    pub time_until_next_first_win_bonus: i32,
    pub caused_early_surrender: bool,
    pub early_surrender_accomplice: bool,
    pub team_early_surrendered: bool,
    pub game_ended_in_early_surrender: bool,
    pub reroll_data: LolEndOfGameEndOfGamePoints,
    pub team_boost: Option<LolEndOfGameEndOfGameTeamBoost>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameEndOfGameTeam {
    pub stats: Value,
    pub players: Vec<LolEndOfGameEndOfGamePlayer>,
    pub member_status_string: String,
    pub name: String,
    pub tag: String,
    pub full_id: String,
    pub team_id: i32,
    pub is_bottom_team: bool,
    pub is_player_team: bool,
    pub is_winning_team: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameEndOfGameTeamBoost {
    pub summoner_name: String,
    pub skin_unlock_mode: String,
    pub price: i64,
    pub ip_reward: i64,
    pub ip_reward_for_purchaser: i64,
    pub available_skins: Vec<i64>,
    pub unlocked: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameGameClientEndOfGame {
    pub game_client_e_o_g: LolEndOfGameGameClientEndOfGameStats,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameGameClientEndOfGameStats {
    pub game_id: u64,
    pub game_mode: String,
    pub stats_block: Value,
    pub queue_id: i32,
    pub queue_type: String,
    pub is_ranked: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameGameDataChampion {
    pub id: i32,
    pub skins: Vec<LolEndOfGameGameDataChampionSkin>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameGameDataChampionQuestSkin {
    pub id: i32,
    pub splash_path: String,
    pub tile_path: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameGameDataChampionSkin {
    pub id: i32,
    pub splash_path: String,
    pub tile_path: String,
    pub chromas: Vec<LolEndOfGameGameDataSkinChroma>,
    pub quest_skin_info: LolEndOfGameGameDataQuestSkinInfo,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameGameDataChampionSummary {
    pub id: i32,
    pub alias: String,
    pub name: String,
    pub square_portrait_path: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameGameDataCompanion {
    pub content_id: String,
    pub loadouts_icon: String,
    pub species_name: String,
    pub color_name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameGameDataQuestSkinInfo {
    pub tiers: Vec<LolEndOfGameGameDataChampionQuestSkin>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameGameDataSkinChroma {
    pub id: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameGameDataTftChampion {
    pub path: String,
    pub character_id: String,
    pub rarity: i32,
    pub display_name: String,
    pub square_icon_path: String,
    pub traits: Vec<LolEndOfGameGameDataTftTrait>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameGameDataTftItem {
    pub name: String,
    pub id: i32,
    pub loadouts_icon: String,
    pub name_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameGameDataTftTrait {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameGameflowAvailability {
    pub state: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameGameflowClient {
    pub observer_server_ip: String,
    pub observer_server_port: u16,
    pub running: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameGameflowGameData {
    pub game_id: u64,
    pub queue: LolEndOfGameQueue,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameGameflowSession {
    pub phase: LolEndOfGameGameflowPhase,
    pub game_client: LolEndOfGameGameflowClient,
    pub game_data: LolEndOfGameGameflowGameData,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameLobbyInvitation {
    pub state: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameLoginDataPacket {
    pub platform_id: String,
    pub simple_messages: Vec<LolEndOfGameSimpleMessage>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameLoginSession {
    pub state: LolEndOfGameLoginSessionStates,
    pub summoner_id: u64,
    pub account_id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameQueue {
    pub id: i32,
    #[serde(rename = "type")]
    pub type_: String,
    pub is_ranked: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameRerollDataBagForClientV1 {
    pub points_gained_last_game: i32,
    pub points_until_next_reroll: i32,
    pub reroll_count: i32,
    pub total_points: i32,
    pub maximum_rerolls: i32,
    pub point_cost_of_reroll: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameSimpleMessage {
    pub account_id: u64,
    pub msg_id: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub params: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameSummoner {
    pub summoner_id: u64,
    pub summoner_level: u32,
    pub xp_since_last_level: u64,
    pub xp_until_next_level: u64,
    pub puuid: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameTFTEndOfGameCompanionViewModel {
    pub species_name: String,
    pub color_name: String,
    pub icon: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameTFTEndOfGameItemViewModel {
    pub name: String,
    pub icon: String,
    pub id: i32,
    pub name_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameTFTEndOfGamePieceViewModel {
    pub name: String,
    pub icon: String,
    pub level: u32,
    pub price: u32,
    pub items: Vec<LolEndOfGameTFTEndOfGameItemViewModel>,
    pub traits: Vec<LolEndOfGameTFTEndOfGameTraitViewModel>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameTFTEndOfGamePlayerViewModel {
    pub summoner_id: u64,
    pub summoner_name: String,
    pub icon_id: i32,
    pub puuid: String,
    pub ffa_standing: u8,
    pub health: u8,
    pub rank: u8,
    pub is_local_player: bool,
    pub partner_group_id: u8,
    pub board_pieces: Vec<LolEndOfGameTFTEndOfGamePieceViewModel>,
    pub augments: Vec<LolEndOfGameTFTEndOfGameItemViewModel>,
    pub companion: LolEndOfGameTFTEndOfGameCompanionViewModel,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameTFTEndOfGameTraitViewModel {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameTFTEndOfGameViewModel {
    pub players: Vec<LolEndOfGameTFTEndOfGamePlayerViewModel>,
    pub local_player: Option<LolEndOfGameTFTEndOfGamePlayerViewModel>,
    pub game_length: u32,
    pub game_id: u64,
    pub queue_id: i32,
    pub queue_type: String,
    pub is_ranked: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEsportStreamNotificationsESportStreamNotificationsConfig {
    pub notifications_enabled: bool,
    pub notifications_service_endpoint: String,
    pub notifications_stream_u_r_l: String,
    pub notifications_stream_group_slug: String,
    pub notifications_asset_magick_u_r_l: String,
    pub use_service_endpoint_v2: bool,
    pub notifications_service_endpoint_v2: String,
    pub notifications_short_poll_minutes: i64,
    pub notifications_long_poll_minutes: i64,
    pub beapp_failure_long_poll_minutes: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEsportStreamNotificationsESportsAPIStreamgroups {
    pub id: i64,
    pub slug: String,
    pub title: String,
    pub live: bool,
    pub content: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEsportStreamNotificationsESportsAPIStreamgroupsRoot {
    pub streamgroups: Vec<LolEsportStreamNotificationsESportsAPIStreamgroups>,
    pub highlander_tournaments: Vec<LolEsportStreamNotificationsEsportsAPIHighlanderTournaments>,
    pub teams: Vec<LolEsportStreamNotificationsEsportsAPITeams>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEsportStreamNotificationsESportsLiveStreams {
    pub live_streams: Vec<LolEsportStreamNotificationsESportsStreams>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEsportStreamNotificationsESportsStreams {
    pub title: String,
    pub tournament_description: String,
    pub team_a_guid: String,
    pub team_a_id: i64,
    pub team_b_guid: String,
    pub team_b_id: i64,
    pub team_a_name: String,
    pub team_b_name: String,
    pub team_a_acronym: String,
    pub team_b_acronym: String,
    pub team_a_logo_url: String,
    pub team_b_logo_url: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEsportStreamNotificationsEsportsAPIHighlanderTournaments {
    pub id: String,
    pub description: String,
    pub title: String,
    pub brackets:
        HashMap<String, LolEsportStreamNotificationsEsportsAPIHighlanderTournamentsBrackets>,
    pub rosters:
        HashMap<String, LolEsportStreamNotificationsEsportsAPIHighlanderTournamentsRosters>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEsportStreamNotificationsEsportsAPIHighlanderTournamentsBrackets {
    pub id: String,
    pub matches:
        HashMap<String, LolEsportStreamNotificationsEsportsAPIHighlanderTournamentsMatches>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEsportStreamNotificationsEsportsAPIHighlanderTournamentsMatches {
    pub id: String,
    pub input: Vec<LolEsportStreamNotificationsEsportsAPIHighlanderTournamentsRoster>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEsportStreamNotificationsEsportsAPIHighlanderTournamentsRoster {
    pub roster: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEsportStreamNotificationsEsportsAPIHighlanderTournamentsRosters {
    pub id: String,
    pub name: String,
    pub team: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEsportStreamNotificationsEsportsAPITeams {
    pub id: i64,
    pub guid: String,
    pub slug: String,
    pub name: String,
    pub logo_url: String,
    pub acronym: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEsportStreamNotificationsGameflowSession {
    pub phase: LolEsportStreamNotificationsGameflowPhase,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEsportStreamNotificationsLiveMatch {
    pub id: String,
    pub stream_group: String,
    pub title: String,
    pub tournament_description: String,
    pub teams: Vec<LolEsportStreamNotificationsLiveMatchTeam>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEsportStreamNotificationsLiveMatchTeam {
    pub guid: String,
    pub name: String,
    pub acronym: String,
    pub logo_url: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEsportStreamNotificationsPlayerNotificationResource {
    pub background_url: String,
    pub critical: bool,
    pub data: HashMap<String, String>,
    pub detail_key: String,
    pub expires: String,
    pub icon_url: String,
    pub id: u64,
    pub source: String,
    pub state: String,
    pub title_key: String,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopAccessTokenResource {
    pub token: String,
    pub scopes: Vec<String>,
    pub expiry: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopBalance {
    pub currency_type: String,
    pub amount: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopBaseSkinLineDto {
    pub items: Vec<LolEventShopSkinLineItemDto>,
    pub localized_name: String,
    pub skin_line_descriptions: Vec<LolEventShopSkinLineDescriptionDto>,
    pub pricing_options: Vec<LolEventShopPriceOptionDto>,
    pub splash_path: String,
    pub uncentered_splash_path: String,
    pub collection_card_path: String,
    pub collection_description: String,
    pub tile_path: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopBundledItemPricingInfo {
    pub discount_prices: Vec<LolEventShopDiscountPricingInfo>,
    pub inventory_type: String,
    pub item_id: i32,
    pub quantity: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopCapOffer {
    pub id: String,
    pub type_id: String,
    pub label: String,
    pub product_id: String,
    pub merchant_id: String,
    pub payload: Vec<LolEventShopCapOfferPayloadEntry>,
    pub active: bool,
    pub start_date: String,
    pub created_date: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopCapOfferPayloadEntry {
    pub item_price_map: HashMap<String, i32>,
    pub item_instance_id: String,
    pub fulfillment_type_id: String,
    pub inventory_type_u_u_i_d: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopCapOrdersDataDto {
    pub id: String,
    pub sub_orders: Vec<LolEventShopCapOrdersSubOrderDto>,
    pub purchaser: LolEventShopCapOrdersTypedIdentifierDto,
    pub location: String,
    pub source: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopCapOrdersMetaDto {
    pub xid: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopCapOrdersOfferContextDto {
    pub quantity: u32,
    pub payment_option: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopCapOrdersOfferDto {
    pub id: String,
    pub product_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopCapOrdersOrderDto {
    pub data: LolEventShopCapOrdersDataDto,
    pub meta: LolEventShopCapOrdersMetaDto,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopCapOrdersSubOrderDto {
    pub recipient_id: String,
    pub offer_context: LolEventShopCapOrdersOfferContextDto,
    pub offer: LolEventShopCapOrdersOfferDto,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopCapOrdersTypedIdentifierDto {
    pub id: String,
    pub type_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopCatalogEntry {
    pub content_id: String,
    pub item_id: i32,
    pub offer_id: String,
    pub type_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopCatalogItem {
    pub item_id: i32,
    pub inventory_type: String,
    pub item_instance_id: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopCatalogPluginItem {
    pub item_id: i32,
    pub item_instance_id: String,
    pub owned: bool,
    pub inventory_type: String,
    pub sub_inventory_type: String,
    pub name: String,
    pub sub_title: String,
    pub description: String,
    pub image_path: String,
    pub purchase_date: u64,
    pub release_date: u64,
    pub inactive_date: u64,
    pub prices: Vec<LolEventShopCatalogPluginPrice>,
    pub tags: Option<Vec<String>>,
    pub metadata: Option<Vec<LolEventShopItemMetadataEntry>>,
    pub quest_skin_info: Option<LolEventShopSkinLineInfo>,
    pub active: bool,
    pub ownership_type: Option<LolEventShopInventoryOwnership>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopCatalogPluginItemAssets {
    pub splash_path: String,
    pub icon_path: String,
    pub tile_path: String,
    pub emblems: Vec<LolEventShopChampionSkinEmblem>,
    pub colors: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopCatalogPluginItemWithDetails {
    pub item: LolEventShopCatalogPluginItem,
    pub quantity: u32,
    pub required_items: Option<Vec<LolEventShopCatalogPluginItemWithDetails>>,
    pub bundled_items: Option<Vec<LolEventShopCatalogPluginItemWithDetails>>,
    pub minimum_bundle_prices: Option<Vec<LolEventShopCatalogPluginPrice>>,
    pub bundled_discount_prices: Option<Vec<LolEventShopCatalogPluginPrice>>,
    pub assets: LolEventShopCatalogPluginItemAssets,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopCatalogPluginPrice {
    pub currency: String,
    pub cost: i64,
    pub cost_type: Option<String>,
    pub sale: Option<LolEventShopCatalogPluginSale>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopCatalogPluginSale {
    pub start_date: String,
    pub end_date: String,
    pub discount: Option<f32>,
    pub cost: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopCategoryOffersUIData {
    pub category: LolEventShopOfferCategory,
    pub category_icon_path: String,
    pub offers: Vec<LolEventShopOfferUIData>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopCelebrationUIData {
    pub reward_track_items: Vec<LolEventShopRewardTrackItem>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopChampionSkinEmblem {
    pub name: String,
    pub emblem_path: LolEventShopChampionSkinEmblemPath,
    pub emblem_position: LolEventShopChampionSkinEmblemPosition,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopChampionSkinEmblemPath {
    pub large: String,
    pub small: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopChampionSkinEmblemPosition {
    pub vertical: String,
    pub horizontal: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopClaimSelectUIData {
    pub selected_reward_track_item: LolEventShopRewardTrackItem,
    pub unclaimed_reward_count: u16,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopClientCacheClearMessageDTO {
    pub regions: Vec<String>,
    pub clear_all: bool,
    pub inventory_types: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopContentDrop {
    pub patch: String,
    pub activation_date: String,
    pub offers: Vec<LolEventShopOffer>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopCounter {
    pub id: String,
    pub name: String,
    pub group_id: String,
    pub direction: String,
    pub start_value: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopCounterInstance {
    pub owner_id: String,
    pub product_id: String,
    pub group_id: String,
    pub counter_id: String,
    pub counter_value: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopCurrencyDTO {
    pub amount: i32,
    pub sub_currencies: HashMap<String, i32>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopDiscountPricingInfo {
    pub cost: i32,
    pub cost_type: String,
    pub currency: String,
    pub discount: f32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopEntityInstance {
    pub group_id: String,
    pub counters: Vec<LolEventShopCounterInstance>,
    pub milestones: Vec<LolEventShopMilestoneInstance>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopEventBackgroundUIData {
    pub background_image_path: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopEventHeaderUIData {
    pub event_icon_path: String,
    pub event_name: String,
    pub progress_end_date: String,
    pub shop_end_date: String,
    pub help_modal_image_path: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopEventShop {
    pub event_id: String,
    pub localized_name: String,
    pub background_image: String,
    pub navbar_icon_image: String,
    pub header_icon_image: String,
    pub start_date: String,
    pub progress_end_date: String,
    pub shop_end_date: String,
    pub localized_upsell_title: String,
    pub localized_upsell_tooltip_title: String,
    pub localized_upsell_tooltip_description: String,
    pub localized_upsell_button_text: String,
    pub upsell_background_image_url: String,
    pub upsell_tooltip_background_image_url: String,
    pub help_modal_image: String,
    pub event_pass_bundles_catalog_entry: Vec<LolEventShopCatalogEntry>,
    pub token_shop: LolEventShopTokenShop,
    pub reward_track: LolEventShopRewardTrack,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopEventShopClientConfig {
    pub enabled: bool,
    pub active_event_id: String,
    pub start_date: String,
    pub progress_end_date: String,
    pub shop_end_date: String,
    pub disabled_offer_ids: Vec<String>,
    pub content_drops: Vec<LolEventShopContentDrop>,
    pub activation_spread_seconds: f64,
    pub first_activation_threshold_seconds: f64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopEventShopError {
    pub error_message: String,
    pub error_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopEventShopInfoUIData {
    pub event_id: String,
    pub event_name: String,
    pub event_icon: String,
    pub event_token_image: String,
    pub current_token_balance: Option<i64>,
    pub locked_token_count: Option<i64>,
    pub unclaimed_reward_count: Option<i32>,
    pub is_pass_purchased: bool,
    pub is_event_active: bool,
    pub show_pip: bool,
    pub event_pass_bundles: Vec<LolEventShopCatalogEntry>,
    pub token_bundles: Vec<LolEventShopCatalogEntry>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopGrantorDescription {
    pub app_name: String,
    pub entity_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopGroup {
    pub id: String,
    pub product_id: String,
    pub name: String,
    pub repeat: LolEventShopRepeat,
    pub counters: Vec<LolEventShopCounter>,
    pub milestones: Vec<LolEventShopMilestone>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopInventoryCacheEntry {
    pub signed_inventory_jwt: String,
    pub expiration_m_s: u64,
    pub issued_at_m_s: u64,
    pub received_at_m_s: u64,
    pub valid: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopInventoryDTO {
    pub puuid: String,
    pub account_id: u64,
    pub summoner_id: u64,
    pub items: HashMap<String, Value>,
    pub expires: String,
    pub items_jwt: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopInventoryItem {
    pub uuid: String,
    pub item_id: i32,
    pub inventory_type: String,
    pub purchase_date: String,
    pub quantity: u64,
    pub ownership_type: LolEventShopItemOwnershipType,
    pub expiration_date: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopInventoryItemDTO {
    pub item_id: i32,
    pub inventory_type: String,
    pub expiration_date: String,
    pub purchase_date: String,
    pub quantity: u64,
    pub owned_quantity: u64,
    pub used_in_game_date: String,
    pub entitlement_id: String,
    pub entitlement_type_id: String,
    pub instance_id: String,
    pub instance_type_id: String,
    pub payload: Value,
    pub f2p: bool,
    pub rental: bool,
    pub loyalty: bool,
    pub loyalty_sources: Vec<String>,
    pub lsb: bool,
    pub wins: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopInventoryItemWithPayload {
    pub uuid: String,
    pub item_id: i32,
    pub inventory_type: String,
    pub purchase_date: String,
    pub quantity: u64,
    pub ownership_type: LolEventShopItemOwnershipType,
    pub expiration_date: String,
    pub f2p: bool,
    pub rental: bool,
    pub loyalty: bool,
    pub loyalty_sources: Vec<String>,
    pub owned: bool,
    pub payload: Value,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopInventoryNotification {
    pub id: i64,
    pub item_id: i32,
    pub inventory_type: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub acknowledged: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopInventoryResponseDTO {
    pub data: LolEventShopInventoryDTO,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopItem {
    pub item_id: String,
    pub inventory_type: String,
    pub price: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopItemChoiceDetails {
    pub item: LolEventShopCatalogPluginItem,
    pub background_image: String,
    pub contents: Vec<LolEventShopItemDetails>,
    pub discount: String,
    pub full_price: u32,
    pub display_type: String,
    pub purchase_options: Vec<LolEventShopPurchaseOption>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopItemChoices {
    pub choices: Vec<LolEventShopItemChoiceDetails>,
    pub validation_errors: Vec<LolEventShopValidationErrorEntry>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopItemCost {
    pub currency: String,
    pub cost: i64,
    pub discount: Option<f32>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopItemDefinition {
    pub item_id: i32,
    pub inventory_type: String,
    pub sub_inventory_type: String,
    pub name: String,
    pub description: String,
    pub sub_title: String,
    pub owned: bool,
    pub assets: LolEventShopCatalogPluginItemAssets,
    pub tags: Vec<String>,
    pub metadata: Vec<LolEventShopItemMetadataEntry>,
    pub bundled_item_price: Option<LolEventShopBundledItemPricingInfo>,
    pub loyalty_unlocked: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopItemDetails {
    pub title: String,
    pub sub_title: String,
    pub description: String,
    pub icon_url: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopItemKey {
    pub inventory_type: String,
    pub item_id: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopItemMetadataEntry {
    #[serde(rename = "type")]
    pub type_: String,
    pub value: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopItemOwnership {
    pub item_key: LolEventShopItemKey,
    pub quantity: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopItemPrice {
    pub currency_type: String,
    pub price: i64,
    pub purchasable: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopItemSale {
    pub start_date: String,
    pub end_date: String,
    pub discount: Option<f32>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopItemUIData {
    pub item_id: String,
    pub inventory_type: String,
    pub price: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopLargeSelectionDisplayUIData {
    pub selected_reward_track_item: LolEventShopRewardTrackItem,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopLevelBoostPurchaseUIData {
    pub offer_id: String,
    pub reward_track_items: Vec<LolEventShopRewardTrackItem>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopLoginSession {
    pub puuid: Option<String>,
    pub state: LolEventShopLoginSessionStates,
    pub summoner_id: u64,
    pub account_id: u64,
    pub id_token: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopLoyaltyRewards {
    pub free_rewarded_champions_count: i32,
    pub champion_ids: Vec<i32>,
    pub free_rewarded_skins_count: i32,
    pub skin_ids: Vec<i32>,
    pub ip_boost: i32,
    pub xp_boost: HashMap<String, i32>,
    pub loyalty_t_f_t_map_skin_count: i32,
    pub loyalty_t_f_t_companion_count: i32,
    pub loyalty_t_f_t_damage_skin_count: i32,
    pub loyalty_sources: HashMap<String, bool>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopLoyaltyRewardsSimplified {
    pub free_rewarded_champions_count: i32,
    pub champion_ids: Vec<i32>,
    pub free_rewarded_skins_count: i32,
    pub skin_ids: Vec<i32>,
    pub ip_boost: i32,
    pub xp_boost: i32,
    pub loyalty_t_f_t_map_skin_count: i32,
    pub loyalty_t_f_t_companion_count: i32,
    pub loyalty_t_f_t_damage_skin_count: i32,
    pub loyalty_sources: HashMap<String, bool>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopLoyaltyStatusNotification {
    pub status: LolEventShopLoyaltyStatus,
    pub rewards: LolEventShopLoyaltyRewardsSimplified,
    pub reload_inventory: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopMilestone {
    pub id: String,
    pub name: String,
    pub group_id: String,
    pub counter_id: String,
    pub trigger_value: i64,
    pub properties: HashMap<String, String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopMilestoneInstance {
    pub milestone_id: String,
    pub owner_id: String,
    pub product_id: String,
    pub group_id: String,
    pub counter_id: String,
    pub trigger_value: i64,
    pub repeat_sequence: u32,
    pub triggered: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopNavigationButtonUIData {
    pub active_event_id: String,
    pub show_pip: bool,
    pub icon_path: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopOffer {
    pub id: String,
    pub localized_title: String,
    pub localized_description: String,
    pub image: String,
    pub promotion_type: LolEventShopOfferPromotionType,
    pub offer_category: LolEventShopOfferCategory,
    pub items: Vec<LolEventShopItem>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopOfferUIData {
    pub id: String,
    pub localized_title: String,
    pub localized_description: String,
    pub image: String,
    pub highlighted: bool,
    pub offer_state: LolEventShopOfferStates,
    pub price: u32,
    pub items: Vec<LolEventShopItemUIData>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopOrderNotificationResource {
    pub event_type_id: String,
    pub event_type: String,
    pub status: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopPlayerNotification {
    pub critical: bool,
    pub detail_key: String,
    pub source: String,
    pub state: String,
    pub title_key: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub icon_url: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopPlayerSettingsData {
    pub last_time_seen: String,
    pub last_seen_token_balance: i64,
    pub last_seen_token_shop_offers_version: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopPriceDetail {
    pub item_key: LolEventShopItemKey,
    pub price: LolEventShopItemPrice,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopPriceOptionDto {
    pub price: i64,
    pub currency_type: String,
    pub currency_payment_option: Option<String>,
    pub currency_name: Option<String>,
    pub currency_image_path: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopProgressInfoUIData {
    pub token_image: String,
    pub pass_purchased: bool,
    pub event_pass_bundles_catalog_entry: Vec<LolEventShopCatalogEntry>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopPurchasableItem {
    pub item: LolEventShopItemDefinition,
    pub dependencies: Vec<LolEventShopItemDefinition>,
    pub bundled_items: Vec<LolEventShopItemDefinition>,
    pub sale: Option<LolEventShopItemSale>,
    pub purchase_options: Vec<LolEventShopPurchaseOption>,
    pub validation_errors: Vec<LolEventShopValidationErrorEntry>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopPurchaseItem {
    pub item_key: LolEventShopItemKey,
    pub quantity: i32,
    pub source: String,
    pub purchase_currency_info: LolEventShopItemPrice,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopPurchaseOfferOrderStatus {
    pub order_state: LolEventShopPurchaseOfferOrderStates,
    pub message: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopPurchaseOfferOrderStatuses {
    pub statuses: HashMap<String, LolEventShopPurchaseOfferOrderStatus>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopPurchaseOfferRequest {
    pub offer_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopPurchaseOfferRequestV3 {
    pub offer_id: String,
    pub currency_type: String,
    pub quantity: u32,
    pub price: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopPurchaseOfferResponseV3 {
    pub legacy: bool,
    pub order_dto: Option<LolEventShopCapOrdersOrderDto>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopPurchaseOption {
    pub price_details: Vec<LolEventShopPriceDetail>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopPurchaseRequest {
    pub items: Vec<LolEventShopPurchaseItem>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopPurchaseResponse {
    pub items: Vec<LolEventShopPurchaseItem>,
    pub transactions: Vec<LolEventShopTransaction>,
    pub use_r_m_s_confirmation: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopPurchaseWidgetConfig {
    pub enabled: bool,
    pub non_refundable_disclaimer_enabled: bool,
    pub always_show_purchase_disclaimer: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopRMSPayload {
    pub product_id: String,
    pub affinities: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopRegionLocale {
    pub region: String,
    pub locale: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopRepeat {
    pub count: i32,
    pub scope: u32,
    pub multiplier: f32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopRequestDTOSelectionRequestDTO {
    pub data: LolEventShopSelectionRequestDTO,
    pub metadata: LolEventShopRequestMetadataDTO,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopRequestDTOVectorSelectionRequestDTO {
    pub data: Vec<LolEventShopSelectionRequestDTO>,
    pub metadata: LolEventShopRequestMetadataDTO,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopRequestMetadataDTO {
    pub transaction_id: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopResponseDTOSvcRewardGrant {
    pub data: LolEventShopSvcRewardGrant,
    pub metadata: LolEventShopResponseMetadataDTO,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopResponseDTOMapRewardGroupIdSelectGrantStatus {
    pub data: HashMap<String, LolEventShopSelectGrantStatusResponse>,
    pub metadata: LolEventShopResponseMetadataDTO,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopResponseDTOVectorSvcRewardGrant {
    pub data: Vec<LolEventShopSvcRewardGrant>,
    pub metadata: LolEventShopResponseMetadataDTO,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopResponseDTOVectorSvcRewardGroup {
    pub data: Vec<LolEventShopSvcRewardGroup>,
    pub metadata: LolEventShopResponseMetadataDTO,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopResponseMetadataDTO {}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopReward {
    pub id: String,
    pub item_id: String,
    pub item_type: String,
    pub quantity: i32,
    pub fulfillment_source: String,
    pub media: HashMap<String, String>,
    pub localizations: HashMap<String, String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopRewardChoiceUIData {
    pub reward_track_items: Vec<LolEventShopRewardTrackItem>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopRewardGrant {
    pub info: LolEventShopSvcRewardGrant,
    pub reward_group: LolEventShopSvcRewardGroup,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopRewardTrack {
    pub track_config: LolEventShopRewardTrackConfiguration,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopRewardTrackConfiguration {
    pub id: String,
    pub premium_entitlement_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopRewardTrackItem {
    pub state: LolEventShopRewardTrackItemStates,
    pub reward_options: Vec<LolEventShopRewardTrackItemOption>,
    pub reward_tags: Vec<LolEventShopRewardTrackItemTag>,
    pub progress_required: i64,
    pub threshold: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopRewardTrackItemOption {
    pub state: LolEventShopRewardTrackItemStates,
    pub thumb_icon_path: String,
    pub large_display_image_path: String,
    pub selected: bool,
    pub override_footer: String,
    pub header_type: LolEventShopRewardTrackItemHeaderType,
    pub reward_name: String,
    pub reward_description: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopRewardTrackProgress {
    pub level: i16,
    pub level_progress: u16,
    pub future_level_progress: u16,
    pub pass_progress: i64,
    pub current_level_x_p: i64,
    pub total_level_x_p: i64,
    pub iteration: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopRewardTrackXP {
    pub current_level: i64,
    pub current_level_x_p: i64,
    pub total_level_x_p: i64,
    pub is_bonus_phase: bool,
    pub iteration: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopRewardsConfig {
    pub grant_filtering: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopRiotMessagingServiceMessage {
    pub resource: String,
    pub service: String,
    pub version: String,
    pub timestamp: i64,
    pub payload: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopRmsEntitlementPayload {
    pub item_id: String,
    pub item_type_id: String,
    pub entitlement_type_id: String,
    pub resource_operation: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopRmsStoreEntitlementItem {
    pub inventory_type: String,
    pub item_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopRmsStoreEntitlementPayload {
    pub transaction_id: String,
    pub items: Vec<LolEventShopRmsStoreEntitlementItem>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopRmsWalletPayload {}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopRmsXboxSubscriptionChange {
    pub puuid: String,
    pub subscription_id: String,
    pub active: String,
    pub identity_provider: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopSale {
    pub start_date: String,
    pub end_date: String,
    pub prices: Vec<LolEventShopItemCost>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopSelectionRequestDTO {
    pub grant_id: String,
    pub reward_group_id: String,
    pub selections: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopSelectionStrategyConfig {
    pub min_selections_allowed: u32,
    pub max_selections_allowed: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopSettingsResource {
    pub data: LolEventShopPlayerSettingsData,
    pub schema_version: i16,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopSimpleInventoryDTO {
    pub items: HashMap<String, Value>,
    pub items_jwt: String,
    pub expires: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopSimpleInventoryResponseDTO {
    pub data: LolEventShopSimpleInventoryDTO,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopSkinLineDescriptionDto {
    pub title: String,
    pub description: String,
    pub icon_image_path: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopSkinLineDescriptionInfo {
    pub title: String,
    pub description: String,
    pub icon_path: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopSkinLineInfo {
    pub name: String,
    pub description_info: Vec<LolEventShopSkinLineDescriptionInfo>,
    pub splash_path: String,
    pub tile_path: String,
    pub collection_card_path: String,
    pub uncentered_splash_path: String,
    pub collection_description: String,
    pub tiers: Vec<LolEventShopSkinLineTier>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopSkinLineItemDto {
    pub thumbnail_image_path: String,
    pub large_image_path: Option<String>,
    pub localized_long_name: String,
    pub localized_short_name: String,
    pub large_video_path: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopSkinLineTier {
    pub id: i64,
    pub name: String,
    pub stage: i64,
    pub description: Option<String>,
    pub splash_path: String,
    pub uncentered_splash_path: String,
    pub tile_path: String,
    pub load_screen_path: String,
    pub short_name: String,
    pub splash_video_path: Option<String>,
    pub collection_splash_video_path: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopSummonerIcon {
    pub item_id: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopSvcRewardGrant {
    pub id: String,
    pub grantee_id: String,
    pub reward_group_id: String,
    pub date_created: String,
    pub status: LolEventShopGrantStatus,
    pub grant_elements: Vec<LolEventShopSvcRewardGrantElement>,
    pub selected_ids: Vec<String>,
    pub viewed: bool,
    pub grantor_description: LolEventShopGrantorDescription,
    pub message_parameters: HashMap<String, Value>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopSvcRewardGrantElement {
    pub element_id: String,
    pub item_id: String,
    pub item_type: String,
    pub fulfillment_source: String,
    pub status: LolEventShopRewardStatus,
    pub quantity: i32,
    pub media: HashMap<String, String>,
    pub localizations: HashMap<String, String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopSvcRewardGroup {
    pub id: String,
    pub product_id: String,
    pub types: Vec<String>,
    pub rewards: Vec<LolEventShopReward>,
    pub child_reward_group_ids: Vec<String>,
    pub reward_strategy: LolEventShopRewardStrategy,
    pub selection_strategy_config: Option<LolEventShopSelectionStrategyConfig>,
    pub active: bool,
    pub media: HashMap<String, String>,
    pub localizations: HashMap<String, String>,
    pub celebration_type: LolEventShopCelebrationType,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopTokenShop {
    pub token_image: String,
    pub token_name: String,
    pub token_uuid: String,
    pub offers: Vec<LolEventShopOffer>,
    pub content_drops: Vec<LolEventShopContentDrop>,
    pub token_bundles_catalog_entry: Vec<LolEventShopCatalogEntry>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopTokenShopUIData {
    pub token_name: String,
    pub token_image: String,
    pub token_uuid: String,
    pub offers_version: u32,
    pub token_bundles_catalog_entry: Vec<LolEventShopCatalogEntry>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopTokenUpsell {
    pub id: String,
    pub internal_name: String,
    pub title: String,
    pub button_text: String,
    pub tooltip_title: String,
    pub tooltip_description: String,
    pub purchase_url: String,
    pub tooltip_background_url: String,
    pub background_url: String,
    pub currency_url: String,
    pub premium_currency_name: String,
    pub dependent_inventory_type: String,
    pub dependent_inventory_id: i32,
    pub currently_locked: LolEventShopTokenUpsellLockedType,
    pub locked_count: i32,
    pub start_date: String,
    pub end_date: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopTransaction {
    pub transaction_id: String,
    pub item_key: LolEventShopItemKey,
    pub item_name: String,
    pub icon_url: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopUnclaimedRewardsUIData {
    pub rewards_count: i32,
    pub locked_tokens_count: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopValidateOfferError {
    pub error_key: String,
    pub meta: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopValidateOfferRequestV3 {
    pub offer_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopValidateOfferResponseV3 {
    pub validation_errors: Vec<LolEventShopValidateOfferError>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopValidationError {
    pub error_code: String,
    pub message: String,
    pub error_details: HashMap<String, String>,
    pub response_items: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopValidationErrorEntry {
    pub id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopValidationRequest {
    pub items: Vec<LolEventShopValidationRequestItem>,
    pub owned_items: Vec<LolEventShopItemOwnership>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopValidationRequestItem {
    pub item_key: LolEventShopItemKey,
    pub quantity: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopValidationResponse {
    pub items: Vec<LolEventShopValidationResponseItem>,
    pub valid: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopValidationResponseItem {
    pub item_key: LolEventShopItemKey,
    pub quantity: i32,
    pub validation_currency_info: Vec<LolEventShopItemPrice>,
    pub sale: Option<LolEventShopSale>,
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopWallet {
    pub account_id: u64,
    pub balances: Vec<LolEventShopBalance>,
    pub version: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopWalletCacheEntry {
    pub signed_balances_jwt: String,
    pub expiration_m_s: u64,
    pub issued_at_m_s: u64,
    pub received_at_m_s: u64,
    pub valid: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopWalletDTO {
    pub puuid: String,
    pub account_id: i64,
    pub expires: String,
    pub balances: HashMap<String, i32>,
    pub balances_jwt: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopWalletResponseDTO {
    pub data: LolEventShopWalletDTO,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolEventShopXboxSubscriptionStatus {
    pub active: String,
    pub subscription_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolFeaturedModesAccountIdAndSummonerId {
    pub summoner_id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolFeaturedModesCollectionsOwnership {
    pub loyalty_reward: bool,
    pub xbox_g_p_reward: bool,
    pub owned: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolFeaturedModesCollectionsSummonerIcon {
    pub icon_id: i32,
    pub ownership: LolFeaturedModesCollectionsOwnership,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolFeaturedModesEligibility {
    pub queue_id: i32,
    pub eligible: bool,
    pub restrictions: Vec<LolFeaturedModesEligibilityRestriction>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolFeaturedModesEligibilityRestriction {
    pub restriction_code: LolFeaturedModesEligibilityRestrictionCode,
    pub restriction_args: HashMap<String, String>,
    pub expired_timestamp: u64,
    pub summoner_ids: Vec<u64>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolFeaturedModesFeaturedModesConfig {
    pub notifications_enabled: bool,
    pub queue_toggle_notification_minutes_threshold: u32,
    pub max_notification_save_delay_minutes: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolFeaturedModesGameflowAvailability {
    pub is_available: bool,
    pub state: LolFeaturedModesGameflowAvailabilityState,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolFeaturedModesGameflowSession {
    pub phase: LolFeaturedModesGameflowPhase,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolFeaturedModesLoginSession {
    pub state: LolFeaturedModesLoginSessionStates,
    pub summoner_id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolFeaturedModesMaps {
    pub game_mode_name: String,
    pub is_r_g_m: bool,
    pub assets: HashMap<String, String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolFeaturedModesPlayerNotificationResource {
    pub background_url: String,
    pub critical: bool,
    pub data: HashMap<String, String>,
    pub detail_key: String,
    pub expires: String,
    pub icon_url: String,
    pub id: u64,
    pub source: String,
    pub state: String,
    pub title_key: String,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolFeaturedModesQueue {
    pub id: i32,
    pub map_id: i32,
    pub name: String,
    pub game_mode: String,
    pub category: LolFeaturedModesQueueGameCategory,
    pub last_toggled_off_time: u64,
    pub last_toggled_on_time: u64,
    pub queue_availability: LolFeaturedModesQueueAvailability,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolGameClientChatGameClientChatMessageResource {
    pub from_summoner_name: String,
    pub body: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolGameClientChatMutedPlayerInfo {
    pub puuid: String,
    pub summoner_id: u64,
    pub obfuscated_puuid: String,
    pub obfuscated_summoner_id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolGameQueuesLoginDataPacket {
    pub game_type_configs: Vec<LolGameQueuesQueueGameTypeConfig>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolGameQueuesLoginSession {
    pub state: LolGameQueuesLoginSessionStates,
    pub summoner_id: u64,
    pub account_id: u64,
    pub connected: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolGameQueuesPlatformConfigEnabledMap {
    pub game_map_id: i32,
    pub min_players: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolGameQueuesQueue {
    pub id: i32,
    pub map_id: i32,
    pub name: String,
    pub short_name: String,
    pub description: String,
    pub detailed_description: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub game_mode: String,
    pub asset_mutator: String,
    pub max_tier_for_premade_size2: String,
    pub max_division_for_premade_size2: String,
    pub category: LolGameQueuesQueueGameCategory,
    pub game_type_config: LolGameQueuesQueueGameTypeConfig,
    pub num_players_per_team: i32,
    pub minimum_participant_list_size: i32,
    pub maximum_participant_list_size: i32,
    pub min_level: u32,
    pub is_ranked: bool,
    pub are_free_champions_allowed: bool,
    pub is_team_builder_managed: bool,
    pub queue_availability: LolGameQueuesQueueAvailability,
    pub queue_rewards: LolGameQueuesQueueReward,
    pub spectator_enabled: bool,
    pub champions_required_to_play: u32,
    pub allowable_premade_sizes: Vec<i32>,
    pub show_position_selector: bool,
    pub last_toggled_off_time: u64,
    pub last_toggled_on_time: u64,
    pub removal_from_game_allowed: bool,
    pub removal_from_game_delay_minutes: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolGameQueuesQueueCustomGame {
    pub subcategories: Vec<LolGameQueuesQueueCustomGameSubcategory>,
    pub queue_availability: LolGameQueuesQueueAvailability,
    pub spectator_policies: Vec<LolGameQueuesQueueCustomGameSpectatorPolicy>,
    pub spectator_slot_limit: u32,
    pub game_server_regions: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolGameQueuesQueueCustomGameSubcategory {
    pub map_id: i32,
    pub game_mode: String,
    pub mutators: Vec<LolGameQueuesQueueGameTypeConfig>,
    pub num_players_per_team: i32,
    pub minimum_participant_list_size: i32,
    pub maximum_participant_list_size: i32,
    pub max_player_count: i32,
    pub min_level: u32,
    pub queue_availability: LolGameQueuesQueueAvailability,
    pub custom_spectator_policies: Vec<LolGameQueuesQueueCustomGameSpectatorPolicy>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolGameQueuesQueueGameTypeConfig {
    pub id: i64,
    pub name: String,
    pub max_allowable_bans: i32,
    pub allow_trades: bool,
    pub exclusive_pick: bool,
    pub duplicate_pick: bool,
    pub team_champion_pool: bool,
    pub cross_team_champion_pool: bool,
    pub advanced_learning_quests: bool,
    pub battle_boost: bool,
    pub death_match: bool,
    pub do_not_remove: bool,
    pub learning_quests: bool,
    pub onboard_coop_beginner: bool,
    pub reroll: bool,
    pub main_pick_timer_duration: i32,
    pub post_pick_timer_duration: i32,
    pub ban_timer_duration: i32,
    pub pick_mode: String,
    pub ban_mode: String,
    pub game_mode_override: Option<String>,
    pub num_players_per_team_override: Option<i32>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolGameQueuesQueueReward {
    pub is_ip_enabled: bool,
    pub is_xp_enabled: bool,
    pub is_champion_points_enabled: bool,
    pub party_size_ip_rewards: Vec<i32>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolGameQueuesQueueTranslation {
    pub name: String,
    pub short_name: String,
    pub description: String,
    pub detailed_description: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolGameSettingsLoginSession {
    pub state: LolGameSettingsLoginSessionStates,
    pub summoner_id: u64,
    pub account_id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolGameSettingsgamesettingsgameclient {
    pub running: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolGameSettingsgamesettingsgameflowsession {
    pub game_client: LolGameSettingsgamesettingsgameclient,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowCrashReportingSettings {
    #[serde(rename = "type")]
    pub type_: String,
    pub enabled: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowGameModeSpellList {
    pub spells: Vec<u64>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowGameStateUpdate {
    pub id: u64,
    pub game_state: String,
    pub game_type: String,
    pub error_message: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowGameflowAvailability {
    pub is_available: bool,
    pub state: LolGameflowGameflowAvailabilityState,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowGameflowGameClient {
    pub server_ip: String,
    pub server_port: u16,
    pub observer_server_ip: String,
    pub observer_server_port: u16,
    pub running: bool,
    pub visible: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowGameflowGameData {
    pub game_id: u64,
    pub queue: LolGameflowQueue,
    pub is_custom_game: bool,
    pub game_name: String,
    pub password: String,
    pub team_one: Vec<Value>,
    pub team_two: Vec<Value>,
    pub player_champion_selections: Vec<Value>,
    pub spectators_allowed: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowGameflowGameDodge {
    pub state: LolGameflowGameflowGameDodgeState,
    pub dodge_ids: Vec<u64>,
    pub phase: LolGameflowGameflowPhase,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowGameflowGameMap {
    pub id: i64,
    pub name: String,
    pub map_string_id: String,
    pub game_mode: String,
    pub game_mode_name: String,
    pub game_mode_short_name: String,
    pub game_mutator: String,
    pub is_r_g_m: bool,
    pub description: String,
    pub platform_id: String,
    pub platform_name: String,
    pub assets: Value,
    pub categorized_content_bundles: Value,
    pub properties: Value,
    pub per_position_required_summoner_spells: HashMap<String, LolGameflowGameModeSpellList>,
    pub per_position_disallowed_summoner_spells: HashMap<String, LolGameflowGameModeSpellList>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowGameflowProcessInfo {
    pub pid: u32,
    pub raw_args: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowGameflowSession {
    pub phase: LolGameflowGameflowPhase,
    pub game_data: LolGameflowGameflowGameData,
    pub game_client: LolGameflowGameflowGameClient,
    pub map: LolGameflowGameflowGameMap,
    pub game_dodge: LolGameflowGameflowGameDodge,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowInstallPaths {
    pub game_install_root: String,
    pub game_executable_path: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowLobbyStatus {
    pub queue_id: i32,
    pub is_custom: bool,
    pub is_practice_tool: bool,
    pub is_leader: bool,
    pub is_spectator: bool,
    pub allowed_play_again: bool,
    pub member_summoner_ids: Vec<u64>,
    pub invited_summoner_ids: Vec<u64>,
    pub lobby_id: Option<String>,
    pub custom_spectator_policy: LolGameflowQueueCustomGameSpectatorPolicy,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowLoginSession {
    pub state: LolGameflowLoginSessionStates,
    pub summoner_id: u64,
    pub account_id: u64,
    pub connected: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowPatcherProductState {
    pub is_corrupted: bool,
    pub is_stopped: bool,
    pub is_up_to_date: bool,
    pub is_update_available: bool,
    pub action: LolGameflowPatcherProductStateAction,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowPlayerStatus {
    pub current_lobby_status: Option<LolGameflowLobbyStatus>,
    pub last_queued_lobby_status: Option<LolGameflowLobbyStatus>,
    pub can_invite_others_at_eog: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowQueue {
    pub id: i32,
    pub map_id: i32,
    pub name: String,
    pub short_name: String,
    pub description: String,
    pub detailed_description: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub game_mode: String,
    pub asset_mutator: String,
    pub category: LolGameflowQueueGameCategory,
    pub game_type_config: LolGameflowQueueGameTypeConfig,
    pub num_players_per_team: i32,
    pub minimum_participant_list_size: i32,
    pub maximum_participant_list_size: i32,
    pub min_level: u32,
    pub is_ranked: bool,
    pub are_free_champions_allowed: bool,
    pub is_team_builder_managed: bool,
    pub queue_availability: LolGameflowQueueAvailability,
    pub queue_rewards: LolGameflowQueueReward,
    pub spectator_enabled: bool,
    pub champions_required_to_play: u32,
    pub allowable_premade_sizes: Vec<i32>,
    pub show_position_selector: bool,
    pub last_toggled_off_time: u64,
    pub last_toggled_on_time: u64,
    pub removal_from_game_allowed: bool,
    pub removal_from_game_delay_minutes: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowQueueGameTypeConfig {
    pub id: i64,
    pub name: String,
    pub max_allowable_bans: i32,
    pub allow_trades: bool,
    pub exclusive_pick: bool,
    pub duplicate_pick: bool,
    pub team_champion_pool: bool,
    pub cross_team_champion_pool: bool,
    pub advanced_learning_quests: bool,
    pub battle_boost: bool,
    pub death_match: bool,
    pub do_not_remove: bool,
    pub learning_quests: bool,
    pub onboard_coop_beginner: bool,
    pub reroll: bool,
    pub main_pick_timer_duration: i32,
    pub post_pick_timer_duration: i32,
    pub ban_timer_duration: i32,
    pub pick_mode: String,
    pub ban_mode: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowQueueReward {
    pub is_ip_enabled: bool,
    pub is_xp_enabled: bool,
    pub is_champion_points_enabled: bool,
    pub party_size_ip_rewards: Vec<i32>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowRegionLocale {
    pub region: String,
    pub locale: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowRegistrationStatus {
    pub complete: bool,
    pub error_codes: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowReplaysSettingsData {
    #[serde(rename = "highlights-folder-path")]
    pub highlights_folder_path: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowReplaysSettingsResource {
    pub data: LolGameflowReplaysSettingsData,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowSpectateGameInfoResource {
    pub drop_in_spectate_game_id: String,
    pub game_queue_type: String,
    pub allow_observe_mode: String,
    pub puuid: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolGeoinfoGeoInfo {
    pub country: String,
    pub city: String,
    pub region: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolGeoinfoGeoInfoConfig {
    pub enabled: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolGeoinfoGeoInfoResponse {
    pub success: bool,
    pub geo_info: LolGeoinfoGeoInfo,
    pub error_message: String,
    pub is_latest: bool,
    pub is_initialized: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolGeoinfoLoginSession {
    pub state: LolGeoinfoLoginSessionState,
    pub summoner_id: u64,
    pub account_id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolGeoinfoWhereAmIRequest {
    pub ip_address: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolGeoinfoWhereAmIResponse {
    pub country: String,
    pub city: String,
    pub region: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHeartbeatLoginSession {
    pub state: LolHeartbeatLoginSessionStates,
    pub summoner_id: u64,
    pub account_id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHighlightsHighlight {
    pub id: u64,
    pub name: String,
    pub filepath: String,
    pub url: String,
    pub mtime_ms_utc: u64,
    pub mtime_iso8601: String,
    pub file_size_bytes: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHighlightsHighlightsConfig {
    pub is_highlights_enabled: bool,
    pub invalid_highlight_name_characters: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHighlightsHighlightsDynamicConfig {
    pub enabled: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHighlightsHighlightsSettingsData {
    #[serde(rename = "highlights-folder-path")]
    pub highlights_folder_path: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHighlightsHighlightsSettingsResource {
    pub data: LolHighlightsHighlightsSettingsData,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHoneyfruitAccountClaimStatus {
    pub linking_status: Option<LolHoneyfruitHoneyfruitLinkingServiceResponse>,
    pub migration_status: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHoneyfruitAccountDetails {
    pub puuid: String,
    pub platform_id: String,
    pub summoner_name: String,
    pub summoner_icon_id: i32,
    pub summoner_level: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHoneyfruitGAMHSMatchHistoryData {}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHoneyfruitGAMHSMatchHistoryList {
    pub games: Vec<LolHoneyfruitGAMHSMatchHistoryData>,
    pub active_puuid: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHoneyfruitGarenaRegionLeagueAccount {
    pub garena_puuid: String,
    pub platform_id: String,
    pub summoner_name: String,
    pub summoner_level: u32,
    pub summoner_icon_id: i32,
    pub garena_id: u64,
    pub is_reserved_summoner_name: bool,
    pub has_played_a_game: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHoneyfruitHoneyfruitAuthRedirectMock {
    pub redirect_latency: u32,
    pub redirect_status_code: u32,
    pub redirect_u_r_l: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHoneyfruitHoneyfruitLinkingAction {
    pub action: LolHoneyfruitHoneyfruitActionType,
    pub target: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHoneyfruitHoneyfruitLinkingNotification {
    pub linked_account: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHoneyfruitHoneyfruitLinkingServiceResponse {
    pub eligible: bool,
    pub reason_code: LolHoneyfruitHoneyfruitLinkingFailureReason,
    pub email: String,
    pub account_details: Option<LolHoneyfruitGarenaRegionLeagueAccount>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHoneyfruitHoneyfruitLinkingStatus {
    pub state: LolHoneyfruitHoneyfruitLinkingState,
    pub linked_account: String,
    pub error: LolHoneyfruitHoneyfruitLinkingStatusError,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHoneyfruitHoneyfruitRegionLocale {
    pub region: String,
    pub locale: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHoneyfruitHoneyfruitSettingCategoryResource {
    pub schema_version: u32,
    pub data: LolHoneyfruitHoneyfruitSettings,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHoneyfruitHoneyfruitSettingCategoryResourceAccountClaim {
    pub schema_version: u32,
    pub data: LolHoneyfruitHoneyfruitSettingsAccountClaim,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHoneyfruitHoneyfruitSettings {
    pub snooze_until_m_s: u64,
    pub is_snoozed_permanently: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHoneyfruitHoneyfruitSettingsAccountClaim {
    pub is_account_claim_auto_dismiss: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHoneyfruitHoneyfruitVNGPublisherSettings {
    pub visible: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHoneyfruitMatchHistoryGame {}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHoneyfruitMatchHistoryGameList {
    pub game_index_begin: u64,
    pub game_index_end: u64,
    pub game_begin_date: String,
    pub game_end_date: String,
    pub game_count: u64,
    pub games: Vec<LolHoneyfruitMatchHistoryGame>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHoneyfruitMatchHistoryList {
    pub platform_id: String,
    pub account_id: u64,
    pub games: LolHoneyfruitMatchHistoryGameList,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHoneyfruitSummoner {
    pub summoner_id: u64,
    pub account_id: u64,
    pub display_name: String,
    pub internal_name: String,
    pub profile_icon_id: i32,
    pub summoner_level: u32,
    pub xp_since_last_level: u64,
    pub xp_until_next_level: u64,
    pub percent_complete_for_next_level: u32,
    pub puuid: String,
    pub name_change_flag: bool,
    pub unnamed: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHoneyfruitV1AuthenticationRedirectInput {
    pub redirect_uri: String,
    pub language: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHoneyfruitV1AuthenticationResponse {
    #[serde(rename = "type")]
    pub type_: LolHoneyfruitV1ResponseType,
    pub success: LolHoneyfruitV1SuccessResponseDetails,
    pub country: String,
    pub error: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHoneyfruitV1SuccessResponseDetails {
    pub login_token: String,
    pub redirect_url: String,
    pub linked: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHoneyfruitVNGStatusResponse {
    pub action_required: bool,
    pub action_url: String,
    pub action_url_raw: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2AccountIdAndSummonerId {
    pub account_id: u64,
    pub summoner_id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2ApiHonorPlayerServerRequest {
    pub summoner_id: u64,
    pub honor_category: String,
    pub game_id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2Ballot {
    pub eligible_players: Vec<LolHonorV2EligiblePlayer>,
    pub game_id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2DynamicHonorMessage {
    pub message_id: String,
    pub value: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2EligiblePlayer {
    pub summoner_id: u64,
    pub summoner_name: String,
    pub champion_name: String,
    pub skin_splash_path: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2EndOfGamePlayer {
    pub bot_player: bool,
    pub game_id: u64,
    pub leaver: bool,
    pub summoner_name: String,
    pub summoner_id: u64,
    pub skin_splash_path: String,
    pub champion_name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2EndOfGameStats {
    pub game_id: u64,
    pub game_type: String,
    pub invalid: bool,
    pub queue_type: String,
    pub ranked: bool,
    pub report_game_id: u64,
    pub teams: Vec<LolHonorV2EndOfGameTeam>,
    pub local_player: LolHonorV2EndOfGamePlayer,
    pub game_ended_in_early_surrender: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2EndOfGameTeam {
    pub players: Vec<LolHonorV2EndOfGamePlayer>,
    pub is_player_team: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2GameflowGameData {
    pub game_id: u64,
    pub queue: LolHonorV2Queue,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2GameflowSession {
    pub phase: LolHonorV2GameflowPhase,
    pub game_data: LolHonorV2GameflowGameData,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2Honor {
    pub honor_category: String,
    pub voter_relationship: String,
    pub sender_puuid: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2HonorConfig {
    pub enabled: bool,
    pub seconds_to_vote: i32,
    pub recall_reward_enabled: bool,
    pub honor_visibility_enabled: bool,
    pub honor_suggestions_enabled: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2HonorInteraction {
    pub summoner_id: u64,
    pub display_name: String,
    pub game_id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2HonorRecipient {
    pub summoner_id: u64,
    pub game_id: u64,
    pub honors: Vec<LolHonorV2Honor>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2HonorSummoner {
    pub summoner_id: u64,
    pub display_name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2LoginSession {
    pub state: LolHonorV2LoginSessionStates,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2MutualHonor {
    pub game_id: u64,
    pub summoners: Vec<LolHonorV2MutualHonorPlayer>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2MutualHonorPlayer {
    pub summoner_id: u64,
    pub champion_id: i32,
    pub skin_id: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2ProfileInfo {
    pub honor_level: i32,
    pub checkpoint: i32,
    pub rewards_locked: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2Queue {
    pub id: i32,
    pub map_id: i32,
    #[serde(rename = "type")]
    pub type_: String,
    pub game_mode: String,
    pub removal_from_game_allowed: bool,
    pub removal_from_game_delay_minutes: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2Reward {
    pub reward_type: String,
    pub quantity: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2SequenceEvent {
    pub name: String,
    pub priority: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2ServiceProxyHonorPlayerServerRequest {
    pub summoner_id: u64,
    pub honor_type: String,
    pub game_id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2ServiceProxyRetrieveProfileResponse {
    pub honor_level: i32,
    pub checkpoint: i32,
    pub rewards_locked: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2VendedHonorChange {
    pub action_type: String,
    pub previous_state: LolHonorV2VendedHonorState,
    pub current_state: LolHonorV2VendedHonorState,
    pub reward: LolHonorV2Reward,
    pub dynamic_honor_message: LolHonorV2DynamicHonorMessage,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2VendedHonorState {
    pub level: i32,
    pub checkpoint: i32,
    pub rewards_locked: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2VendedReward {
    pub reward_type: String,
    pub quantity: i32,
    pub dynamic_honor_message: LolHonorV2DynamicHonorMessage,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2VoteCompletion {
    pub game_id: u64,
    pub full_team_vote: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHovercardChatSession {
    pub session_state: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHovercardContentCookies {
    pub content_id: String,
    pub content_path: String,
    pub cookies: Vec<LolHovercardcookie>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHovercardFriendResult {
    pub id: String,
    pub puuid: String,
    pub summoner_id: u64,
    pub name: String,
    pub account_id: u64,
    pub icon: i32,
    pub game_name: String,
    pub game_tag: String,
    pub availability: String,
    pub note: String,
    pub score: u64,
    pub patchline: String,
    pub platform_id: String,
    pub product: String,
    pub product_name: String,
    pub status_message: String,
    pub lol: HashMap<String, String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHovercardHovercardUserInfo {
    pub id: String,
    pub puuid: String,
    pub summoner_id: u64,
    pub name: String,
    pub account_id: u64,
    pub icon: i32,
    pub game_name: String,
    pub game_tag: String,
    pub availability: String,
    pub note: String,
    pub mastery_score: u64,
    pub patchline: String,
    pub platform_id: String,
    pub product: String,
    pub product_name: String,
    pub status_message: String,
    pub summoner_icon: i32,
    pub summoner_level: u32,
    pub remote_product: bool,
    pub remote_platform: bool,
    pub remote_product_icon_url: String,
    pub remote_product_backdrop_url: String,
    pub party_summoners: Vec<String>,
    pub lol: HashMap<String, String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHovercardPartyInfo {
    pub summoners: Option<Vec<u64>>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHovercardPatchlineMetadata {
    pub product_id: String,
    pub id: String,
    pub content_paths: HashMap<String, String>,
    pub content_cookies: Vec<LolHovercardContentCookies>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHovercardProductMetadata {
    pub id: String,
    pub name: String,
    pub patchlines: HashMap<String, LolHovercardPatchlineMetadata>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHovercardProductMetadataMap {
    pub products: HashMap<String, LolHovercardProductMetadata>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHovercardSummoner {
    pub summoner_id: u64,
    pub display_name: String,
    pub account_id: u64,
    pub summoner_level: u32,
    pub puuid: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHovercardSummonerIdAndIcon {
    pub profile_icon_id: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHovercardSummonerIdAndName {
    pub summoner_id: u64,
    pub display_name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHovercardTopChampionMastery {
    pub score: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolHovercardcookie {
    pub name: String,
    pub value: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolInventoryAccessTokenResource {
    pub token: String,
    pub scopes: Vec<String>,
    pub expiry: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolInventoryCatalogItem {
    pub item_id: i32,
    pub inventory_type: String,
    pub item_instance_id: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolInventoryClientCacheClearMessageDTO {
    pub regions: Vec<String>,
    pub clear_all: bool,
    pub inventory_types: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolInventoryCurrencyDTO {
    pub amount: i32,
    pub sub_currencies: HashMap<String, i32>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolInventoryInventoryCacheEntry {
    pub signed_inventory_jwt: String,
    pub expiration_m_s: u64,
    pub issued_at_m_s: u64,
    pub received_at_m_s: u64,
    pub valid: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolInventoryInventoryDTO {
    pub puuid: String,
    pub account_id: u64,
    pub summoner_id: u64,
    pub items: HashMap<String, Value>,
    pub expires: String,
    pub items_jwt: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolInventoryInventoryItem {
    pub uuid: String,
    pub item_id: i32,
    pub inventory_type: String,
    pub purchase_date: String,
    pub quantity: u64,
    pub ownership_type: LolInventoryItemOwnershipType,
    pub expiration_date: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolInventoryInventoryItemDTO {
    pub item_id: i32,
    pub inventory_type: String,
    pub expiration_date: String,
    pub purchase_date: String,
    pub quantity: u64,
    pub owned_quantity: u64,
    pub used_in_game_date: String,
    pub entitlement_id: String,
    pub entitlement_type_id: String,
    pub instance_id: String,
    pub instance_type_id: String,
    pub payload: Value,
    pub f2p: bool,
    pub rental: bool,
    pub loyalty: bool,
    pub loyalty_sources: Vec<String>,
    pub lsb: bool,
    pub wins: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolInventoryInventoryItemWithPayload {
    pub uuid: String,
    pub item_id: i32,
    pub inventory_type: String,
    pub purchase_date: String,
    pub quantity: u64,
    pub ownership_type: LolInventoryItemOwnershipType,
    pub expiration_date: String,
    pub f2p: bool,
    pub rental: bool,
    pub loyalty: bool,
    pub loyalty_sources: Vec<String>,
    pub owned: bool,
    pub payload: Value,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolInventoryInventoryNotification {
    pub id: i64,
    pub item_id: i32,
    pub inventory_type: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub acknowledged: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolInventoryInventoryResponseDTO {
    pub data: LolInventoryInventoryDTO,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolInventoryItemKey {
    pub inventory_type: String,
    pub item_id: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolInventoryLoginSession {
    pub state: LolInventoryLoginSessionStates,
    pub summoner_id: u64,
    pub account_id: u64,
    pub id_token: String,
    pub puuid: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolInventoryLoyaltyRewards {
    pub free_rewarded_champions_count: i32,
    pub champion_ids: Vec<i32>,
    pub free_rewarded_skins_count: i32,
    pub skin_ids: Vec<i32>,
    pub ip_boost: i32,
    pub xp_boost: HashMap<String, i32>,
    pub loyalty_t_f_t_map_skin_count: i32,
    pub loyalty_t_f_t_companion_count: i32,
    pub loyalty_t_f_t_damage_skin_count: i32,
    pub loyalty_sources: HashMap<String, bool>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolInventoryLoyaltyRewardsSimplified {
    pub free_rewarded_champions_count: i32,
    pub champion_ids: Vec<i32>,
    pub free_rewarded_skins_count: i32,
    pub skin_ids: Vec<i32>,
    pub ip_boost: i32,
    pub xp_boost: i32,
    pub loyalty_t_f_t_map_skin_count: i32,
    pub loyalty_t_f_t_companion_count: i32,
    pub loyalty_t_f_t_damage_skin_count: i32,
    pub loyalty_sources: HashMap<String, bool>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolInventoryLoyaltyStatusNotification {
    pub status: LolInventoryLoyaltyStatus,
    pub rewards: LolInventoryLoyaltyRewardsSimplified,
    pub reload_inventory: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolInventoryPlayerNotification {
    pub critical: bool,
    pub detail_key: String,
    pub source: String,
    pub state: String,
    pub title_key: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub icon_url: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolInventoryRiotMessagingServiceMessage {
    pub resource: String,
    pub service: String,
    pub version: String,
    pub timestamp: i64,
    pub payload: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolInventoryRmsEntitlementPayload {
    pub item_id: String,
    pub item_type_id: String,
    pub entitlement_type_id: String,
    pub resource_operation: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolInventoryRmsStoreEntitlementItem {
    pub inventory_type: String,
    pub item_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolInventoryRmsStoreEntitlementPayload {
    pub transaction_id: String,
    pub items: Vec<LolInventoryRmsStoreEntitlementItem>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolInventoryRmsWalletPayload {}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolInventoryRmsXboxSubscriptionChange {
    pub puuid: String,
    pub subscription_id: String,
    pub active: String,
    pub identity_provider: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolInventorySimpleInventoryDTO {
    pub items: HashMap<String, Value>,
    pub items_jwt: String,
    pub expires: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolInventorySimpleInventoryResponseDTO {
    pub data: LolInventorySimpleInventoryDTO,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolInventorySummonerIcon {
    pub item_id: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolInventoryWallet {
    pub ip: i64,
    pub rp: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolInventoryWalletCacheEntry {
    pub signed_balances_jwt: String,
    pub expiration_m_s: u64,
    pub issued_at_m_s: u64,
    pub received_at_m_s: u64,
    pub valid: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolInventoryWalletDTO {
    pub puuid: String,
    pub account_id: i64,
    pub expires: String,
    pub balances: HashMap<String, i32>,
    pub balances_jwt: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolInventoryWalletResponseDTO {
    pub data: LolInventoryWalletDTO,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolInventoryXboxSubscriptionStatus {
    pub active: String,
    pub subscription_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolItemSetsGameDataChampion {
    pub alias: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolItemSetsItemSet {
    pub uid: String,
    pub title: String,
    pub mode: String,
    pub map: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub sortrank: i32,
    pub started_from: String,
    pub associated_champions: Vec<i32>,
    pub associated_maps: Vec<i32>,
    pub blocks: Vec<LolItemSetsItemSetBlock>,
    pub preferred_item_slots: Vec<LolItemSetsPreferredItemSlot>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolItemSetsItemSetBlock {
    #[serde(rename = "type")]
    pub type_: String,
    pub hide_if_summoner_spell: String,
    pub show_if_summoner_spell: String,
    pub items: Vec<LolItemSetsItemSetItem>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolItemSetsItemSetItem {
    pub id: String,
    pub count: u16,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolItemSetsItemSets {
    pub timestamp: u64,
    pub account_id: u64,
    pub item_sets: Vec<LolItemSetsItemSet>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolItemSetsLoginSession {
    pub state: LolItemSetsLoginSessionStates,
    pub summoner_id: u64,
    pub account_id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolItemSetsNamecheckAuthorization {
    pub subject: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolItemSetsNamecheckLoginDataPacket {
    pub platform_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolItemSetsNamecheckPayload {
    pub name: String,
    pub name_validation_context: String,
    pub puuid: String,
    pub shard: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolItemSetsNamecheckResponse {
    pub errors: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolItemSetsPreferredItemSlot {
    pub id: String,
    pub preferred_item_slot: i16,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolItemSetsValidateItemSetNameInput {
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolItemSetsValidateItemSetNameResponse {
    pub success: bool,
    pub name_check_response: LolItemSetsNamecheckResponse,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolKickoutKickoutMessage {
    pub message: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolKrPlaytimeReminderPlaytimeReminder {
    pub hours: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolKrShutdownLawAllQueueShutdownStatus {
    pub is_all_queues_disabled: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolKrShutdownLawAntiAddictionState {
    pub policy_type: LolKrShutdownLawPolicyType,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolKrShutdownLawQueueShutdownStatus {
    pub is_disabled: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolKrShutdownLawRatingScreenInfo {
    pub shown: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolKrShutdownLawShutdownLawNotification {
    #[serde(rename = "type")]
    pub type_: LolKrShutdownLawShutdownLawStatus,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLeagueSessionAntiAddictionTokenEnvelope {
    pub anti_addiction_token: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLeagueSessionLeagueSessionRMSNotification {
    pub product: String,
    pub puuid: String,
    pub session_id: String,
    pub state: String,
    pub session_initiated_at: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLeagueSessionLeagueSessionTokenEnvelope {
    pub token: Option<String>,
    pub logout_on_failure: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLeaverBusterAllSummonerData {
    pub summoner: LolLeaverBusterSummoner,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLeaverBusterLeaverBusterNotificationResource {
    pub id: u32,
    pub msg_id: String,
    pub account_id: u64,
    #[serde(rename = "type")]
    pub type_: LolLeaverBusterLeaverBusterNotificationType,
    pub punished_games_remaining: i32,
    pub queue_lockout_timer_expiry_utc_millis_diff: u64,
    pub from_rms: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLeaverBusterLoginDataPacket {
    pub all_summoner_data: LolLeaverBusterAllSummonerData,
    pub simple_messages: Vec<LolLeaverBusterSimpleMessage>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLeaverBusterMatchmakingSearchErrorResource {
    pub id: i32,
    pub error_type: String,
    pub penalized_summoner_id: u64,
    pub penalty_time_remaining: f64,
    pub message: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLeaverBusterMatchmakingSearchResource {
    pub queue_id: i32,
    pub errors: Vec<LolLeaverBusterMatchmakingSearchErrorResource>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLeaverBusterPlayerNotificationResource {
    pub background_url: String,
    pub created: String,
    pub critical: bool,
    pub data: HashMap<String, String>,
    pub detail_key: String,
    pub expires: String,
    pub icon_url: String,
    pub id: u64,
    pub source: String,
    pub state: String,
    pub title_key: String,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLeaverBusterSimpleMessage {
    pub account_id: u64,
    pub msg_id: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub params: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLeaverBusterSummoner {
    pub acct_id: u64,
    pub sum_id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLicenseAgreementLicenseAgreement {
    pub id: String,
    pub license_type: LolLicenseAgreementLicenseAgreementType,
    pub text: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLicenseAgreementPluginRegionLocaleChangedEvent {
    pub region: String,
    pub locale: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLoadoutsAccessTokenResource {
    pub token: String,
    pub scopes: Vec<String>,
    pub expiry: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLoadoutsCreateLoadoutDTO {
    pub scope: String,
    pub item_id: Option<u32>,
    pub name: String,
    pub loadout: HashMap<String, LolLoadoutsItemKey>,
    pub refresh_time: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLoadoutsCreateLoadoutRequestDTO {
    pub service_to_jwts_map: HashMap<String, Value>,
    pub loadout: LolLoadoutsCreateLoadoutDTO,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLoadoutsCreateOrUpdateItemsRequest {
    pub id: u32,
    pub items: HashMap<String, Value>,
    pub inventory_j_w_ts: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLoadoutsFrontendInventoryResponse {
    pub entitlements: Vec<LolLoadoutsItemKey>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLoadoutsGameflowGameData {
    pub queue: LolLoadoutsQueue,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLoadoutsGameflowSession {
    pub phase: LolLoadoutsGameflowPhase,
    pub game_data: LolLoadoutsGameflowGameData,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLoadoutsGetItemsRequest {
    pub id: u32,
    pub inventory_types: Vec<String>,
    pub inventory_j_w_ts: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLoadoutsInventoryDTO {
    pub puuid: String,
    pub account_id: u64,
    pub summoner_id: u64,
    pub items: HashMap<String, Value>,
    pub expires: String,
    pub items_jwt: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLoadoutsInventoryItemDTO {
    pub item_id: i32,
    pub inventory_type: String,
    pub expiration_date: String,
    pub purchase_date: String,
    pub quantity: u64,
    pub used_in_game_date: String,
    pub entitlement_id: String,
    pub entitlement_type_id: String,
    pub instance_id: String,
    pub instance_type_id: String,
    pub f2p: bool,
    pub rental: bool,
    pub lsb: bool,
    pub wins: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLoadoutsInventoryResponseDTO {
    pub data: LolLoadoutsInventoryDTO,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLoadoutsItemKey {
    pub inventory_type: String,
    pub content_id: String,
    pub item_id: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLoadoutsLoadout {
    pub id: u32,
    pub name: String,
    pub items: HashMap<String, Value>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLoadoutsLoadoutRequestDTOBase {
    pub service_to_jwts_map: HashMap<String, Value>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLoadoutsLoginSession {
    pub state: LolLoadoutsLoginSessionStates,
    pub summoner_id: u64,
    pub account_id: u64,
    pub id_token: String,
    pub puuid: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLoadoutsQueue {
    pub is_team_builder_managed: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLoadoutsScopedLoadout {
    pub scope: String,
    pub item_id: Option<u32>,
    pub name: String,
    pub loadout: HashMap<String, LolLoadoutsItemKey>,
    pub refresh_time: String,
    pub id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLoadoutsSignGCORequestDTO {
    pub service_to_jwts_map: HashMap<String, Value>,
    pub loadout: HashMap<String, LolLoadoutsItemKey>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLoadoutsUpdateLoadoutDTO {
    pub id: String,
    pub name: String,
    pub loadout: HashMap<String, LolLoadoutsItemKey>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLoadoutsUpdateLoadoutRequestDTO {
    pub service_to_jwts_map: HashMap<String, Value>,
    pub loadout: LolLoadoutsUpdateLoadoutDTO,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyAccountIdAndSummonerId {
    pub account_id: u64,
    pub summoner_id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyAmbassadorMessage {
    pub http_status: i32,
    pub error_code: String,
    pub message: String,
    pub implementation_details: String,
    pub payload: Value,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyAutoFillQueueDto {
    pub queue_id: i32,
    pub auto_fill_eligible: bool,
    pub auto_fill_protected_for_streaking: bool,
    pub auto_fill_protected_for_promos: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyBotParticipantDto {
    pub champion_id: i32,
    pub bot_skill_level: i32,
    pub team: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyChampionSkinSelection {
    pub champion_id: i32,
    pub skin_id: Option<i32>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyChatBlockedPlayerResource {
    pub summoner_id: u64,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyChatFriend {
    pub summoner_id: u64,
    pub lol: HashMap<String, String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyChatFriendCounts {
    pub num_friends: u32,
    pub num_friends_online: u32,
    pub num_friends_available: u32,
    pub num_friends_away: u32,
    pub num_friends_in_queue: u32,
    pub num_friends_in_champ_select: u32,
    pub num_friends_in_game: u32,
    pub num_friends_mobile: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyCollectionsChampion {
    pub active: bool,
    pub bot_enabled: bool,
    pub id: i32,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyCollectionsChampionMinimal {
    pub disabled_queues: Vec<String>,
    pub id: i32,
    pub ownership: LolLobbyCollectionsOwnership,
    pub active: bool,
    pub free_to_play: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyCollectionsOwnership {
    pub loyalty_reward: bool,
    pub xbox_g_p_reward: bool,
    pub owned: bool,
    pub rental: LolLobbyCollectionsRental,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyCollectionsRental {}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyCustomEligibilityDto {
    pub eligible: bool,
    pub restrictions: Vec<LolLobbyGatekeeperRestrictionDto>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyCustomGameDto {
    pub party_id: String,
    pub game_type: String,
    pub map_id: i32,
    pub owner_puuid: String,
    pub allow_spectators: String,
    pub spectator_count: i32,
    pub team1_count: i32,
    pub team2_count: i32,
    pub max_num_players: i32,
    pub lobby_name: String,
    pub private_game: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyCustomGameSettingsDto {
    pub lobby_name: String,
    pub lobby_password: String,
    pub game_id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyCustomJoinOptionsDto {
    pub lobby_password: String,
    pub team: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyEligibility {
    pub queue_id: i32,
    pub eligible: bool,
    pub restrictions: Vec<LolLobbyEligibilityRestriction>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyEligibilityRestriction {
    pub restriction_code: LolLobbyEligibilityRestrictionCode,
    pub restriction_args: HashMap<String, String>,
    pub expired_timestamp: u64,
    pub summoner_ids: Vec<u64>,
    pub summoner_ids_string: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyFriendAvailabilityAnalytics {
    pub puuid: String,
    pub summoner_id: u64,
    pub platform_id: String,
    pub event_type: String,
    pub event_data: Value,
    pub event_timestamp: u64,
    pub num_friends: u32,
    pub num_friends_online: u32,
    pub num_friends_available: u32,
    pub num_friends_away: u32,
    pub num_friends_in_game: u32,
    pub num_friends_mobile: u32,
    pub num_friends_in_champ_select: u32,
    pub num_friends_in_queue: u32,
    pub party_type: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyGameDataChampionSummary {
    pub id: i32,
    pub alias: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyGameModeDto {
    pub game_type: String,
    pub max_team_size: i32,
    pub max_party_size: i32,
    pub bot_difficulty: Option<String>,
    pub queue_id: Option<i32>,
    pub game_customization: Option<HashMap<String, String>>,
    pub customs_settings: Option<LolLobbyCustomGameSettingsDto>,
    pub game_type_config_id: Option<i64>,
    pub map_id: Option<i32>,
    pub allow_spectators: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyGameflowGameClient {
    pub running: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyGameflowGameData {
    pub queue: LolLobbyQueue,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyGameflowGameDodge {
    pub state: LolLobbyMatchmakingDodgeState,
    pub dodge_ids: Vec<u64>,
    pub phase: LolLobbyGameflowPhase,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyGameflowSampleDto {
    pub state: String,
    pub probability: f64,
    pub interval_secs: f64,
    pub tags: Vec<LolLobbyGameflowSampleTag>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyGameflowSession {
    pub phase: LolLobbyGameflowPhase,
    pub game_data: LolLobbyGameflowGameData,
    pub game_client: LolLobbyGameflowGameClient,
    pub game_dodge: LolLobbyGameflowGameDodge,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyGatekeeperRestrictionDto {
    pub account_id: u64,
    pub reason: String,
    pub remaining_millis: i64,
    pub payload: String,
    pub queue_id: i32,
    pub puuid: String,
    pub details: HashMap<String, String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyInventoryCacheEntry {
    pub signed_inventory_jwt: String,
    pub expiration_m_s: u64,
    pub valid: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyInventoryItem {
    pub item_id: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyJoinPartyAnalytics {
    pub party_id: String,
    pub platform_id: String,
    pub event_type: String,
    pub event_data: Value,
    pub event_timestamp: u64,
    pub num_open_parties: i32,
    pub num_open_friends: i32,
    pub num_party_invites: i32,
    pub num_other_invites: i32,
    pub num_total_invites: i32,
    pub num_friends_online: i32,
    pub party_size: i32,
    pub game_mode: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLcdsDynamicClientConfig {
    pub party_rewards: LolLobbyLcdsPartyRewardsConfig,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLcdsGameMetaData {
    pub game_id: u64,
    pub map_id: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLcdsPartyRewardsConfig {
    pub enabled: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobby {
    pub queue_id: i32,
    pub map_id: i32,
    pub game_mode: String,
    pub invitation_id: String,
    pub multi_user_chat_id: String,
    pub multi_user_chat_password: String,
    pub multi_user_chat_j_w_t: String,
    pub muc_jwt_dto: LolLobbyMucJwtDto,
    pub members: Vec<LolLobbyLobbyMember>,
    pub local_member: LolLobbyLobbyMember,
    pub invitations: Vec<LolLobbyLobbyInvitation>,
    pub was_kicked: bool,
    pub removal_reason: LolLobbyLobbyRemovedFromGameReason,
    pub can_start_matchmaking: bool,
    pub show_position_selector: bool,
    pub show_position_excluder: bool,
    pub specifiable_position_preferences: Vec<String>,
    pub auto_fill_eligible: bool,
    pub custom_game_lobby: Option<LolLobbyLobbyCustomGameLobby>,
    pub is_custom: bool,
    pub auto_fill_protected_for_streaking: bool,
    pub auto_fill_protected_for_promos: bool,
    pub is_team_builder_managed: bool,
    pub maximum_participant_list_size: u32,
    pub premade_size_allowed: bool,
    pub required_position_coverage_met: bool,
    pub allowable_premade_sizes: Vec<i32>,
    pub queue_availability: LolLobbyQueueAvailability,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyBotChampion {
    pub active: bool,
    pub id: i32,
    pub name: String,
    pub bot_difficulties: Vec<LolLobbyLobbyBotDifficulty>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyBotParams {
    pub champion_id: i32,
    pub bot_difficulty: LolLobbyLobbyBotDifficulty,
    pub team_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyChangeGameDto {
    pub queue_id: i32,
    pub is_custom: bool,
    pub custom_game_lobby: Option<LolLobbyLobbyCustomGameLobby>,
    pub game_customization: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyChangeQueue {
    pub queue_id: i32,
    pub is_custom: bool,
    pub custom_game_lobby: Option<LolLobbyLobbyCustomGameLobby>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyCustomChampSelectStartResponse {
    pub success: bool,
    pub failed_players: Vec<LolLobbyLobbyCustomFailedPlayer>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyCustomFailedPlayer {
    pub summoner_id: u64,
    pub summoner_name: String,
    pub reason: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyCustomGame {
    pub id: u64,
    pub game_type: String,
    pub map_id: i32,
    pub owner_summoner_name: String,
    pub spectator_policy: String,
    pub filled_spectator_slots: i32,
    pub max_spectator_slots: u64,
    pub filled_player_slots: i32,
    pub max_player_slots: i32,
    pub lobby_name: String,
    pub has_password: bool,
    pub passback_url: String,
    pub party_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyCustomGameConfiguration {
    pub map_id: i32,
    pub game_mode: String,
    pub mutators: LolLobbyQueueGameTypeConfig,
    pub game_type_config: LolLobbyQueueGameTypeConfig,
    pub spectator_policy: LolLobbyQueueCustomGameSpectatorPolicy,
    pub team_size: i32,
    pub max_player_count: u32,
    pub tournament_game_mode: String,
    pub tournament_passback_url: String,
    pub tournament_passback_data_packet: String,
    pub game_server_region: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyCustomGameLobby {
    pub lobby_name: String,
    pub lobby_password: String,
    pub configuration: LolLobbyLobbyCustomGameConfiguration,
    pub team_one: Vec<LolLobbyLobbyMember>,
    pub team_two: Vec<LolLobbyLobbyMember>,
    pub spectators: Vec<LolLobbyLobbyMember>,
    pub practice_game_rewards_disabled_reasons: Vec<String>,
    pub game_id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyCustomJoinParameters {
    pub password: Option<String>,
    pub as_spectator: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyDto {
    pub party_id: String,
    pub party_type: String,
    pub members: Vec<LolLobbyLobbyParticipantDto>,
    pub local_member: LolLobbyLobbyParticipantDto,
    pub invitations: Vec<LolLobbyLobbyInvitationDto>,
    pub can_start_activity: bool,
    pub restrictions: Option<Vec<LolLobbyEligibilityRestriction>>,
    pub warnings: Option<Vec<LolLobbyEligibilityRestriction>>,
    pub game_config: LolLobbyLobbyGameConfigDto,
    pub multi_user_chat_id: String,
    pub multi_user_chat_password: String,
    // pub multi_user_chat_j_w_t: String,
    pub muc_jwt_dto: LolLobbyMucJwtDto,
    pub scarce_positions: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyGameConfigDto {
    pub game_mode: String,
    pub map_id: i32,
    pub queue_id: i32,
    pub pick_type: String,
    pub max_team_size: i32,
    pub max_lobby_size: i32,
    pub max_human_players: i32,
    pub allowable_premade_sizes: Vec<i32>,
    pub premade_size_allowed: bool,
    pub is_team_builder_managed: bool,
    pub is_custom: bool,
    pub show_position_selector: bool,
    pub is_lobby_full: bool,
    pub should_force_scarce_position_selection: bool,
    pub custom_lobby_name: String,
    pub custom_mutator_name: String,
    pub custom_team100: Vec<LolLobbyLobbyParticipantDto>,
    pub custom_team200: Vec<LolLobbyLobbyParticipantDto>,
    pub custom_spectators: Vec<LolLobbyLobbyParticipantDto>,
    pub custom_spectator_policy: LolLobbyQueueCustomGameSpectatorPolicy,
    pub custom_rewards_disabled_reasons: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyInvitation {
    pub id: String,
    pub from_summoner_id: u64,
    pub to_summoner_id: u64,
    pub state: LolLobbyLobbyInvitationState,
    pub error_type: String,
    pub timestamp: String,
    pub invitation_meta_data: Value,
    pub eligibility: LolLobbyEligibility,
    pub from_summoner_name: String,
    pub to_summoner_name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyInvitationDto {
    pub invitation_id: String,
    pub to_summoner_id: u64,
    pub state: LolLobbyLobbyInvitationState,
    pub timestamp: String,
    pub to_summoner_name: String,
    pub invitation_type: LolLobbyInvitationType,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyLastQueuedLobby {
    pub queue_id: i32,
    pub was_owner: bool,
    pub can_play_again: bool,
    pub members: Vec<LolLobbyLobbyLastQueuedMember>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyLastQueuedMember {
    pub id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyMatchmakingLowPriorityDataResource {
    pub penalized_summoner_ids: Vec<u64>,
    pub penalty_time: f64,
    pub penalty_time_remaining: f64,
    pub busted_leaver_access_token: String,
    pub reason: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyMatchmakingSearchErrorResource {
    pub id: i32,
    pub error_type: String,
    pub penalized_summoner_id: u64,
    pub penalty_time_remaining: f64,
    pub message: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyMatchmakingSearchResource {
    pub search_state: LolLobbyLobbyMatchmakingSearchState,
    pub low_priority_data: LolLobbyLobbyMatchmakingLowPriorityDataResource,
    pub errors: Vec<LolLobbyLobbyMatchmakingSearchErrorResource>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyMember {
    pub id: u64,
    pub is_owner: bool,
    pub is_spectator: bool,
    pub can_invite_others: bool,
    pub position_preferences: LolLobbyLobbyPositionPreferences,
    pub excluded_position_preference: Option<String>,
    pub summoner_internal_name: String,
    pub show_position_excluder: bool,
    pub auto_fill_eligible: bool,
    pub auto_fill_protected_for_streaking: bool,
    pub auto_fill_protected_for_promos: bool,
    pub auto_fill_protected_for_soloing: bool,
    pub is_bot: bool,
    pub bot_difficulty: LolLobbyLobbyBotDifficulty,
    pub bot_champion_id: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyNotification {
    pub notification_id: String,
    pub notification_reason: String,
    pub summoner_ids: Vec<u64>,
    pub timestamp: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyParticipantDto {
    pub summoner_id: u64,
    pub summoner_icon_id: i32,
    pub summoner_name: String,
    pub summoner_internal_name: String,
    pub puuid: String,
    pub summoner_level: u32,
    pub allowed_start_activity: bool,
    pub allowed_change_activity: bool,
    pub allowed_toggle_invite: bool,
    pub allowed_kick_others: bool,
    pub allowed_invite_others: bool,
    pub is_leader: bool,
    pub is_spectator: bool,
    pub team_id: i32,
    pub first_position_preference: String,
    pub second_position_preference: String,
    pub ready: bool,
    pub show_ghosted_banner: bool,
    pub auto_fill_eligible: bool,
    pub auto_fill_protected_for_streaking: bool,
    pub auto_fill_protected_for_promos: bool,
    pub auto_fill_protected_for_soloing: bool,
    pub is_bot: bool,
    pub bot_id: String,
    pub bot_difficulty: LolLobbyLobbyBotDifficulty,
    pub bot_champion_id: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyPartyRewards {
    pub is_enabled: bool,
    pub queue_id: i32,
    pub is_custom: bool,
    pub party_rewards: Vec<LolLobbyPartyReward>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyPositionPreferences {
    pub first_preference: String,
    pub second_preference: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyStatus {
    pub queue_id: i32,
    pub is_custom: bool,
    pub is_practice_tool: bool,
    pub is_leader: bool,
    pub is_spectator: bool,
    pub allowed_play_again: bool,
    pub member_summoner_ids: Vec<u64>,
    pub invited_summoner_ids: Vec<u64>,
    pub lobby_id: Option<String>,
    pub custom_spectator_policy: LolLobbyQueueCustomGameSpectatorPolicy,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyTimer {
    pub enabled: bool,
    pub countdown: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLoginSession {
    pub state: LolLobbyLoginSessionStates,
    pub username: String,
    pub user_auth_token: String,
    pub account_id: u64,
    pub summoner_id: u64,
    pub id_token: String,
    pub puuid: String,
    pub connected: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyMucJwtDto {
    pub jwt: String,
    pub channel_claim: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyOpenPartyToggleAnalytics {
    pub party_id: String,
    pub platform_id: String,
    pub event_type: String,
    pub event_data: Value,
    pub event_timestamp: u64,
    pub player_id: String,
    pub initial_state: String,
    pub final_state: String,
    pub num_of_toggles: i32,
    pub start_timestamp: u64,
    pub end_timestamp: u64,
    pub elapsed_time: u64,
    pub start_transition: String,
    pub end_transition: String,
    pub game_mode: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyPartiesInvitationAnalytics {
    pub party_id: String,
    pub platform_id: String,
    pub event_type: String,
    pub event_data: Value,
    pub event_timestamp: u64,
    pub players: Vec<LolLobbyPartiesInvitationPlayerAnalytics>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyPartiesInvitationPlayerAnalytics {
    pub puuid: String,
    pub summoner_id: u64,
    pub role: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyPartyChatDto {
    pub jid: String,
    pub multi_user_chat_j_w_t: String,
    pub muc_jwt_dto: LolLobbyMucJwtDto,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyPartyDto {
    pub party_id: String,
    pub platform_id: String,
    pub players: Vec<LolLobbyPartyMemberDto>,
    pub chat: LolLobbyPartyChatDto,
    pub max_party_size: i32,
    pub party_type: String,
    pub game_mode: LolLobbyGameModeDto,
    pub activity_locked: bool,
    pub version: u64,
    pub activity_started_utc_millis: u64,
    pub activity_resume_utc_millis: u64,
    pub active_restrictions: LolLobbyQueueRestrictionDto,
    pub eligibility_hash: i64,
    pub eligibility_restrictions: Option<Vec<LolLobbyGatekeeperRestrictionDto>>,
    pub eligibility_warnings: Option<Vec<LolLobbyGatekeeperRestrictionDto>>,
    pub bot_participants: Option<Vec<LolLobbyBotParticipantDto>>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyPartyInviteAnalytics {
    pub party_id: String,
    pub platform_id: String,
    pub event_type: String,
    pub event_data: Value,
    pub event_timestamp: u64,
    pub from_summoner_id: u64,
    pub to_summoner_id: u64,
    pub party_type: String,
    pub game_mode: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyPartyMemberDto {
    pub platform_id: String,
    pub puuid: String,
    pub account_id: u64,
    pub summoner_id: u64,
    pub party_id: String,
    pub party_version: i64,
    pub role: LolLobbyPartyMemberRoleEnum,
    pub game_mode: Option<LolLobbyGameModeDto>,
    pub ready: Option<bool>,
    pub metadata: LolLobbyPartyMemberMetadataDto,
    pub invited_by_summoner_id: Option<u64>,
    pub invite_timestamp: Option<u64>,
    pub can_invite: Option<bool>,
    pub team: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyPartyMemberMetadataDto {
    pub position_pref: Vec<String>,
    pub champion_selection: Option<i32>,
    pub skin_selection: Option<i32>,
    pub properties: Option<Value>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyPartyNotificationEnvelopeDto {
    pub player: Option<LolLobbyPlayerDto>,
    pub queue_restriction: Option<LolLobbyQueueRestrictionDto>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyPartyPresenceData {
    pub party_id: String,
    pub queue_id: i32,
    pub summoners: Vec<u64>,
    pub max_players: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyPartyQueueEligibilityDto {
    pub party_restrictions: Vec<LolLobbyGatekeeperRestrictionDto>,
    pub available_queue_ids: Vec<i32>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyPartyReward {
    pub premade_size: i32,
    #[serde(rename = "type")]
    pub type_: LolLobbyLobbyPartyRewardType,
    pub value: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyPartyStatusDto {
    pub ready_players: Vec<String>,
    pub left_players: Vec<String>,
    pub eog_players: Vec<String>,
    pub party_size: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyPlayerCollectionDto {
    pub players: Vec<LolLobbyPlayerDto>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyPlayerDto {
    pub puuid: String,
    pub platform_id: String,
    pub account_id: u64,
    pub summoner_id: u64,
    pub eligibility_hash: i64,
    pub server_utc_millis: i64,
    pub parties: Option<Vec<LolLobbyPartyMemberDto>>,
    pub current_party: Option<LolLobbyPartyDto>,
    pub registration: LolLobbyRegistrationCredentials,
    pub created_at: u64,
    pub version: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyPlayerStatus {
    pub current_lobby_status: Option<LolLobbyLobbyStatus>,
    pub last_queued_lobby_status: Option<LolLobbyLobbyStatus>,
    pub can_invite_others_at_eog: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyPremadeMemberDto {
    pub display_name: String,
    pub puuid: String,
    pub party_id: String,
    pub summoner_id: u64,
    pub role: LolLobbyPartyMemberRoleEnum,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyPremadePartyDto {
    pub party_id: String,
    pub comms_enabled: bool,
    pub players: HashMap<String, LolLobbyPremadeMemberDto>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyPremadeRelationshipAnalytics {
    pub puuid: String,
    pub summoner_id: u64,
    pub platform_id: String,
    pub event_type: String,
    pub event_data: Value,
    pub event_timestamp: u64,
    pub premade_players: Vec<String>,
    pub friend_players: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyQueue {
    pub id: i32,
    pub map_id: i32,
    pub name: String,
    pub short_name: String,
    pub description: String,
    pub detailed_description: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub game_mode: String,
    pub asset_mutator: String,
    pub max_tier_for_premade_size2: String,
    pub max_division_for_premade_size2: String,
    pub category: LolLobbyQueueGameCategory,
    pub game_type_config: LolLobbyQueueGameTypeConfig,
    pub num_players_per_team: i32,
    pub minimum_participant_list_size: i32,
    pub maximum_participant_list_size: i32,
    pub min_level: u32,
    pub is_ranked: bool,
    pub are_free_champions_allowed: bool,
    pub is_team_builder_managed: bool,
    pub queue_availability: LolLobbyQueueAvailability,
    pub queue_rewards: LolLobbyQueueReward,
    pub spectator_enabled: bool,
    pub champions_required_to_play: u32,
    pub allowable_premade_sizes: Vec<i32>,
    pub show_position_selector: bool,
    pub last_toggled_off_time: u64,
    pub last_toggled_on_time: u64,
    pub removal_from_game_allowed: bool,
    pub removal_from_game_delay_minutes: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyQueueCustomGame {
    pub queue_availability: LolLobbyQueueAvailability,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyQueueGameTypeConfig {
    pub id: i64,
    pub name: String,
    pub max_allowable_bans: i32,
    pub allow_trades: bool,
    pub exclusive_pick: bool,
    pub duplicate_pick: bool,
    pub team_champion_pool: bool,
    pub cross_team_champion_pool: bool,
    pub advanced_learning_quests: bool,
    pub battle_boost: bool,
    pub death_match: bool,
    pub do_not_remove: bool,
    pub learning_quests: bool,
    pub onboard_coop_beginner: bool,
    pub reroll: bool,
    pub main_pick_timer_duration: i32,
    pub post_pick_timer_duration: i32,
    pub ban_timer_duration: i32,
    pub pick_mode: String,
    pub ban_mode: String,
    pub game_mode_override: Option<String>,
    pub num_players_per_team_override: Option<i32>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyQueueRestrictionDto {
    pub gatekeeper_restrictions: Vec<LolLobbyGatekeeperRestrictionDto>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyQueueReward {
    pub is_ip_enabled: bool,
    pub is_xp_enabled: bool,
    pub is_champion_points_enabled: bool,
    pub party_size_ip_rewards: Vec<i32>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyRankedPositionInfoDTO {
    pub queue: String,
    pub tier: String,
    pub rank: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyRankedStatsUnsignedDTO {
    pub queues: Vec<LolLobbyRankedPositionInfoDTO>,
    pub highest_previous_season_end_tier: String,
    pub highest_previous_season_end_rank: String,
    pub jwt: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyReadyDto {
    pub ready: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyReceivedInvitationDto {
    pub invitation_id: String,
    pub from_summoner_id: u64,
    pub state: LolLobbyLobbyInvitationState,
    pub timestamp: String,
    pub from_summoner_name: String,
    pub can_accept_invitation: bool,
    pub restrictions: Vec<LolLobbyEligibilityRestriction>,
    pub game_config: LolLobbyReceivedInvitationGameConfigDto,
    pub invitation_type: LolLobbyInvitationType,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyReceivedInvitationGameConfigDto {
    pub game_mode: String,
    pub queue_id: i32,
    pub map_id: i32,
    pub invite_game_type: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyRegistrationCredentials {
    pub summoner_token: Option<String>,
    pub inventory_token: Option<String>,
    pub inventory_tokens: Option<Vec<String>>,
    pub simple_inventory_token: Option<String>,
    pub ranked_overview_token: Option<String>,
    pub game_client_version: Option<String>,
    pub player_tokens: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyRegistrationStatus {
    pub complete: bool,
    pub error_codes: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyRiotMessagingServiceMessage {
    pub resource: String,
    pub service: String,
    pub version: String,
    pub timestamp: i64,
    pub payload: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyServiceProxyPayload {
    pub method: String,
    pub url: String,
    pub body: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbySummoner {
    pub summoner_id: u64,
    pub summoner_level: u32,
    pub account_id: u64,
    pub puuid: String,
    pub profile_icon_id: i32,
    pub display_name: String,
    pub internal_name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderActionV1 {
    pub action_id: i32,
    pub actor_cell_id: i32,
    #[serde(rename = "type")]
    pub type_: String,
    pub champion_id: i32,
    pub completed: bool,
    pub duration: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderAfkCheckStateV1 {
    pub max_afk_millis: u32,
    pub remaining_afk_millis: i32,
    pub afk_ready: bool,
    pub inventory_draft: LolLobbyTeamBuilderTbdInventory,
    pub additional_inventory_types: Vec<String>,
    pub compress_afk_check_payload: bool,
    pub multi_user_chat_j_w_t: String,
    pub muc_jwt_dto: LolLobbyTeamBuilderMucJwtDto,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderBackwardsTransitionInfoV1 {
    pub backwards_transition_reason: String,
    pub initiator_summoner_ids: Vec<u64>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderBenchChampion {
    pub champion_id: i32,
    pub is_priority: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderCellV1 {
    pub team_id: i32,
    pub cell_id: i32,
    pub champion_pick_intent: i32,
    pub champion_id: i32,
    pub skin_id: i32,
    pub assigned_position: String,
    pub spell1_id: i32,
    pub spell2_id: i32,
    pub summoner_id: u64,
    pub puuid: String,
    pub entitled_feature_type: String,
    pub name_visibility_type: String,
    pub obfuscated_summoner_id: u64,
    pub obfuscated_puuid: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderCellsV1 {
    pub allied_team: Vec<LolLobbyTeamBuilderCellV1>,
    pub enemy_team: Vec<LolLobbyTeamBuilderCellV1>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderCeremonyV1 {
    pub name: String,
    pub duration: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderChampSelectAction {
    pub id: i64,
    pub actor_cell_id: i64,
    pub champion_id: i32,
    #[serde(rename = "type")]
    pub type_: String,
    pub completed: bool,
    pub is_ally_action: bool,
    pub is_in_progress: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderChampSelectChatRoomDetails {
    pub multi_user_chat_id: String,
    pub multi_user_chat_password: String,
    pub multi_user_chat_j_w_t: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderChampSelectDevPanelData {
    pub counter: i64,
    pub dto_index: i64,
    pub queue_id: i32,
    pub team_id_suffix: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderChampSelectMySelection {
    pub selected_skin_id: Option<i32>,
    pub spell1_id: Option<u64>,
    pub spell2_id: Option<u64>,
    pub ward_skin_id: Option<i64>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderChampSelectPlayerSelection {
    pub cell_id: i64,
    pub champion_id: i32,
    pub selected_skin_id: i32,
    pub ward_skin_id: i64,
    pub spell1_id: u64,
    pub spell2_id: u64,
    pub team: i32,
    pub assigned_position: String,
    pub champion_pick_intent: i32,
    pub player_type: String,
    pub summoner_id: u64,
    pub puuid: String,
    pub entitled_feature_type: String,
    pub name_visibility_type: String,
    pub obfuscated_summoner_id: u64,
    pub obfuscated_puuid: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderChampSelectReport {
    pub offender_puuid: String,
    pub categories: Vec<String>,
    pub location: String,
    pub comment: String,
    pub match_token: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderChampSelectReportV2 {
    pub offender_id: String,
    pub categories: Vec<String>,
    pub location: String,
    pub comment: String,
    pub token_type: String,
    pub token: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderChampSelectSession {
    pub game_id: u64,
    pub timer: LolLobbyTeamBuilderChampSelectTimer,
    pub chat_details: LolLobbyTeamBuilderChampSelectChatRoomDetails,
    pub my_team: Vec<LolLobbyTeamBuilderChampSelectPlayerSelection>,
    pub their_team: Vec<LolLobbyTeamBuilderChampSelectPlayerSelection>,
    pub trades: Vec<LolLobbyTeamBuilderChampSelectTradeContract>,
    pub pick_order_swaps: Vec<LolLobbyTeamBuilderChampSelectSwapContract>,
    pub actions: Vec<Value>,
    pub local_player_cell_id: i64,
    pub allow_skin_selection: bool,
    pub allow_duplicate_picks: bool,
    pub allow_battle_boost: bool,
    pub boostable_skin_count: i32,
    pub allow_rerolling: bool,
    pub rerolls_remaining: u32,
    pub allow_locked_events: bool,
    pub locked_event_index: i32,
    pub bench_enabled: bool,
    pub bench_champions: Vec<LolLobbyTeamBuilderBenchChampion>,
    pub entitled_feature_state: LolLobbyTeamBuilderEntitledFeatureState,
    pub counter: i64,
    pub recovery_counter: i64,
    pub skip_champion_select: bool,
    pub is_spectating: bool,
    pub has_simultaneous_bans: bool,
    pub has_simultaneous_picks: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderChampSelectSwapContract {
    pub id: i64,
    pub cell_id: i64,
    pub state: LolLobbyTeamBuilderChampSelectSwapState,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderChampSelectTimer {
    pub adjusted_time_left_in_phase: i64,
    pub total_time_in_phase: i64,
    pub phase: String,
    pub is_infinite: bool,
    pub internal_now_in_epoch_ms: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderChampSelectTradeContract {
    pub id: i64,
    pub cell_id: i64,
    pub state: LolLobbyTeamBuilderChampSelectTradeState,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderChampionBenchChampionV1 {
    pub champion_id: i32,
    pub is_priority: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderChampionBenchStateV1 {
    pub bench_enabled: bool,
    pub champion_ids: Vec<i32>,
    pub bench_champions: Vec<LolLobbyTeamBuilderChampionBenchChampionV1>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderChampionSelectPreferences {
    pub skins: HashMap<String, i32>,
    pub spells: HashMap<String, Value>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderChampionSelectStateV1 {
    pub team_id: String,
    pub team_chat_room_id: String,
    pub subphase: String,
    pub action_set_list: Vec<Value>,
    pub ceremonies_by_action_set_index: HashMap<String, Value>,
    pub current_action_set_index: i32,
    pub cells: LolLobbyTeamBuilderCellsV1,
    pub local_player_cell_id: i32,
    pub current_total_time_millis: i64,
    pub current_time_remaining_millis: i64,
    pub trades: Vec<LolLobbyTeamBuilderTradeV1>,
    pub pick_order_swaps: Vec<LolLobbyTeamBuilderPickOrderSwapV1>,
    pub pick_intent_cleared_reason: String,
    pub allow_opting_out_of_banning: bool,
    pub allow_skin_selection: bool,
    pub allow_duplicate_picks: bool,
    pub lcu_skips_sending_loadouts_gco: bool,
    pub reroll_state: LolLobbyTeamBuilderRerollStateV1,
    pub locked_events_state: LolLobbyTeamBuilderLockedEventsStateV1,
    pub battle_boost_state: LolLobbyTeamBuilderTeamBuilderBoostInfo,
    pub champion_bench_state: LolLobbyTeamBuilderChampionBenchStateV1,
    pub entitled_feature_state: LolLobbyTeamBuilderEntitledFeatureStateV1,
    pub inventory_draft: LolLobbyTeamBuilderTbdInventory,
    pub skip_champion_select: bool,
    pub is_spectating: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderCountdownTimer {
    pub phase_name: String,
    pub timer: i64,
    pub counter: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderEntitledFeatureState {
    pub additional_rerolls: u32,
    pub unlocked_skin_ids: Vec<i32>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderEntitledFeatureStateV1 {
    pub additional_rerolls: u32,
    pub unlocked_skins_state: LolLobbyTeamBuilderUnlockedSkinsStateV1,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderGameModeSpellList {
    pub spells: Vec<u64>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderGameflowGameClient {
    pub running: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderGameflowGameMap {
    pub per_position_required_summoner_spells:
        HashMap<String, LolLobbyTeamBuilderGameModeSpellList>,
    pub per_position_disallowed_summoner_spells:
        HashMap<String, LolLobbyTeamBuilderGameModeSpellList>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderGameflowSession {
    pub game_client: LolLobbyTeamBuilderGameflowGameClient,
    pub map: LolLobbyTeamBuilderGameflowGameMap,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderGatekeeperRestricted {
    pub gatekeeper_restrictions: Vec<LolLobbyTeamBuilderGatekeeperRestriction>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderGatekeeperRestriction {
    pub summoner_id: u64,
    pub reason: String,
    pub remaining_millis: u32,
    pub payload: String,
    pub queue_id: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderLeaverBusterAbandoned {
    pub abandoner_name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderLobby {
    pub queue_id: i32,
    pub invitation_id: String,
    pub multi_user_chat_id: String,
    pub multi_user_chat_password: String,
    pub muc_jwt_dto: LolLobbyTeamBuilderMucJwtDto,
    pub members: Vec<LolLobbyTeamBuilderLobbyMember>,
    pub local_member: LolLobbyTeamBuilderLobbyMember,
    pub invitations: Vec<LolLobbyTeamBuilderLobbyInvitation>,
    pub was_kicked: bool,
    pub removal_reason: LolLobbyTeamBuilderLobbyRemovedFromGameReason,
    pub can_start_matchmaking: bool,
    pub show_position_selector: bool,
    pub show_position_excluder: bool,
    pub specifiable_position_preferences: Vec<String>,
    pub auto_fill_eligible: bool,
    pub auto_fill_protected_for_streaking: bool,
    pub auto_fill_protected_for_promos: bool,
    pub is_team_builder_managed: bool,
    pub premade_size_allowed: bool,
    pub required_position_coverage_met: bool,
    pub allowable_premade_sizes: Vec<i32>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderLobbyCountdownTimer {
    pub phase_name: String,
    pub timer: i64,
    pub counter: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderLobbyInvitation {
    pub invitation_meta_data: Value,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderLobbyMember {
    pub id: u64,
    pub is_owner: bool,
    pub can_invite_others: bool,
    pub position_preferences: LolLobbyTeamBuilderLobbyPositionPreferences,
    pub excluded_position_preference: Option<String>,
    pub show_position_excluder: bool,
    pub auto_fill_eligible: bool,
    pub auto_fill_protected_for_streaking: bool,
    pub auto_fill_protected_for_promos: bool,
    pub auto_fill_protected_for_soloing: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderLobbyPositionPreferences {
    pub first_preference: String,
    pub second_preference: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderLobbyPositionPreferencesV2 {
    pub first_preference: String,
    pub second_preference: String,
    pub excluded_preference: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderLockedEventsStateV1 {
    pub allow_locked_events: bool,
    pub locked_event_index: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderLoginSession {
    pub state: LolLobbyTeamBuilderLoginSessionState,
    pub summoner_id: u64,
    pub account_id: u64,
    pub connected: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderMatchmakingDodgeData {
    pub state: LolLobbyTeamBuilderMatchmakingDodgeState,
    pub dodger_id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderMatchmakingLowPriorityData {
    pub penalized_summoner_ids: Vec<u64>,
    pub penalty_time: f64,
    pub penalty_time_remaining: f64,
    pub busted_leaver_access_token: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderMatchmakingReadyCheckResource {
    pub state: LolLobbyTeamBuilderMatchmakingReadyCheckState,
    pub player_response: LolLobbyTeamBuilderMatchmakingReadyCheckResponse,
    pub dodge_warning: LolLobbyTeamBuilderMatchmakingDodgeWarning,
    pub timer: f32,
    pub decliner_ids: Vec<u64>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderMatchmakingSearch {
    pub search_state: LolLobbyTeamBuilderMatchmakingSearchState,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderMatchmakingSearchErrorResource {
    pub id: i32,
    pub error_type: String,
    pub penalized_summoner_id: u64,
    pub penalty_time_remaining: f64,
    pub message: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderMatchmakingSearchResource {
    pub queue_id: i32,
    pub is_currently_in_queue: bool,
    pub lobby_id: String,
    pub search_state: LolLobbyTeamBuilderMatchmakingSearchState,
    pub time_in_queue: f32,
    pub estimated_queue_time: f32,
    pub ready_check: LolLobbyTeamBuilderMatchmakingReadyCheckResource,
    pub dodge_data: LolLobbyTeamBuilderMatchmakingDodgeData,
    pub low_priority_data: LolLobbyTeamBuilderMatchmakingLowPriorityData,
    pub errors: Vec<LolLobbyTeamBuilderMatchmakingSearchErrorResource>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderMucJwtDto {
    pub jwt: String,
    pub channel_claim: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderPickOrderSwapV1 {
    pub id: i32,
    pub cell_id: i32,
    pub state: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderQueue {
    pub id: i32,
    pub map_id: i32,
    pub name: String,
    pub short_name: String,
    pub description: String,
    pub detailed_description: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub game_mode: String,
    pub asset_mutator: String,
    pub category: LolLobbyTeamBuilderQueueGameCategory,
    pub game_type_config: LolLobbyTeamBuilderQueueGameTypeConfig,
    pub num_players_per_team: i32,
    pub minimum_participant_list_size: i32,
    pub maximum_participant_list_size: i32,
    pub min_level: u32,
    pub is_ranked: bool,
    pub are_free_champions_allowed: bool,
    pub is_team_builder_managed: bool,
    pub queue_availability: LolLobbyTeamBuilderQueueAvailability,
    pub queue_rewards: LolLobbyTeamBuilderQueueReward,
    pub spectator_enabled: bool,
    pub champions_required_to_play: u32,
    pub allowable_premade_sizes: Vec<i32>,
    pub show_position_selector: bool,
    pub last_toggled_off_time: u64,
    pub last_toggled_on_time: u64,
    pub removal_from_game_allowed: bool,
    pub removal_from_game_delay_minutes: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderQueueGameTypeConfig {
    pub id: i64,
    pub name: String,
    pub max_allowable_bans: i32,
    pub allow_trades: bool,
    pub allow_pick_order_swaps: bool,
    pub exclusive_pick: bool,
    pub duplicate_pick: bool,
    pub team_champion_pool: bool,
    pub cross_team_champion_pool: bool,
    pub advanced_learning_quests: bool,
    pub battle_boost: bool,
    pub death_match: bool,
    pub do_not_remove: bool,
    pub learning_quests: bool,
    pub onboard_coop_beginner: bool,
    pub reroll: bool,
    pub main_pick_timer_duration: i32,
    pub post_pick_timer_duration: i32,
    pub ban_timer_duration: i32,
    pub pick_mode: String,
    pub ban_mode: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderQueueReward {
    pub is_ip_enabled: bool,
    pub is_xp_enabled: bool,
    pub is_champion_points_enabled: bool,
    pub party_size_ip_rewards: Vec<i32>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderRerollStateV1 {
    pub allow_rerolling: bool,
    pub rerolls_remaining: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderSettingCategoryResource {
    pub schema_version: i32,
    pub data: LolLobbyTeamBuilderChampionSelectPreferences,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderTBDMatchmakingState {
    pub estimated_matchmaking_time_millis: i64,
    pub time_in_matchmaking_millis: i64,
    pub backwards_transition_reason: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderTbLobbyStateResource {
    pub counter: i32,
    pub recovery_counter: i32,
    pub phase_name: String,
    pub queue_id: i32,
    pub game_id: u64,
    pub matchmaking_state: Option<LolLobbyTeamBuilderTBDMatchmakingState>,
    pub afk_check_state: Option<LolLobbyTeamBuilderAfkCheckStateV1>,
    pub champion_select_state: Option<LolLobbyTeamBuilderChampionSelectStateV1>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderTbRemovedFromServiceNotification {
    pub reason: String,
    pub backwards_transition_info: LolLobbyTeamBuilderBackwardsTransitionInfoV1,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderTbdInventory {
    pub last_selected_skin_id_by_champion_id: HashMap<String, i32>,
    pub skin_ids: Vec<i32>,
    pub spell_ids: Vec<i32>,
    pub initial_spell_ids: Vec<i32>,
    pub all_champion_ids: Vec<i32>,
    pub disabled_champion_ids: Vec<i32>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderTeamBoost {
    pub summoner_id: i64,
    pub skin_unlock_mode: String,
    pub price: i64,
    pub ip_reward: i64,
    pub ip_reward_for_purchaser: i64,
    pub available_skins: Vec<i64>,
    pub unlocked: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderTeamBuilderBoostInfo {
    pub allow_battle_boost: bool,
    pub boostable_skin_count: i32,
    pub activator_cell_id: i64,
    pub battle_boost_activated: bool,
    pub cost: i64,
    pub unlocked_skin_ids: Vec<i64>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderTeambuilderLeagueEdgeResponse {
    pub payload: LolLobbyTeamBuilderTbLobbyStateResource,
    pub status: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderTradeV1 {
    pub id: i32,
    pub cell_id: i32,
    pub state: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderUnlockedSkinsStateV1 {
    pub unlocked_skin_ids: Vec<i32>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyUserInfoToken {
    pub user_info: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyUserResource {
    pub summoner_id: u64,
    pub lol: HashMap<String, String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLoginAccessToken {
    pub token: String,
    pub scopes: Vec<String>,
    pub expiry: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLoginAccountStateResource {
    pub state: LolLoginAccountStateType,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLoginAuthorization {
    pub current_platform_id: String,
    pub current_account_id: u64,
    pub subject: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLoginConfigStatus {
    pub readiness: LolLoginConfigReadinessEnum,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLoginCrashReportingEnvironment {
    pub environment: String,
    pub user_name: String,
    pub user_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLoginIdToken {
    pub token: String,
    pub expiry: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLoginLcdsResponse {
    pub type_name: String,
    pub body: Value,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLoginLcdsServiceProxyResponse {
    pub service_name: String,
    pub method_name: String,
    pub message_id: String,
    pub status: String,
    pub payload: String,
    pub compressed_payload: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLoginLeagueSessionTokenEnvelope {
    pub token: Option<String>,
    pub logout_on_failure: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLoginLoginConnectionState {
    pub mode: LolLoginLoginConnectionMode,
    pub is_using_developer_auth_token: bool,
    pub is_partner_riot_client: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLoginLoginError {
    pub id: String,
    pub message_id: String,
    pub description: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLoginLoginQueue {
    pub estimated_position_in_queue: u64,
    pub is_position_capped: bool,
    pub approximate_wait_time_seconds: Option<u64>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLoginLoginSession {
    pub state: LolLoginLoginSessionStates,
    pub username: String,
    pub user_auth_token: String,
    pub account_id: u64,
    pub summoner_id: Option<u64>,
    pub is_in_login_queue: bool,
    pub error: Option<LolLoginLoginError>,
    pub id_token: String,
    pub puuid: String,
    pub is_new_player: bool,
    pub connected: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLoginLoginSessionWallet {
    pub ip: i64,
    pub rp: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLoginPlatformGeneratedCredentials {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLoginRSOConfigReadyState {
    pub ready: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLoginRSOPlayerCredentials {
    pub username: String,
    pub password: String,
    pub platform_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLoginSummonerCreatedResource {
    pub summoner_id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLoginSummonerSessionResource {
    pub summoner_id: u64,
    pub display_name: String,
    pub is_new_player: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLoginUsernameAndPassword {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootAccountIdAndSummonerId {
    pub summoner_id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootCollectionsChampionMinimal {
    pub id: i32,
    pub ownership: LolLootCollectionsOwnership,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootCollectionsChampionSkinMinimal {
    pub champion_id: i32,
    pub id: i32,
    pub name: String,
    pub ownership: LolLootCollectionsOwnership,
    pub splash_path: String,
    pub tile_path: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootCollectionsEmote {
    pub item_id: i64,
    pub ownership_type: LolLootInventoryOwnership,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootCollectionsOwnership {
    pub loyalty_reward: bool,
    pub xbox_g_p_reward: bool,
    pub owned: bool,
    pub rental: LolLootCollectionsRental,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootCollectionsRental {
    pub rented: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootCollectionsStatstone {
    pub item_id: i64,
    pub ownership_type: LolLootInventoryOwnership,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootCollectionsSummonerIcon {
    pub item_id: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootCollectionsWardSkin {
    pub id: i64,
    pub name: String,
    pub ownership: LolLootCollectionsOwnership,
    pub ward_image_path: String,
    pub ward_shadow_image_path: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootContextMenu {
    pub name: String,
    pub action_type: String,
    pub recipe_description: String,
    pub recipe_context_menu_action: String,
    pub enabled: bool,
    pub essence_type: String,
    pub essence_quantity: i32,
    pub required_tokens: String,
    pub required_others: String,
    pub required_others_name: String,
    pub required_others_count: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootCosmeticsTFTDamageSkin {
    pub content_id: String,
    pub item_id: i32,
    pub name: String,
    pub description: String,
    pub loadouts_icon: String,
    pub owned: bool,
    pub selected: bool,
    pub loyalty: bool,
    pub f2p: bool,
    pub rarity_value: u32,
    pub purchase_date: String,
    pub level: u32,
    pub group_id: u32,
    pub group_name: String,
    pub upgrades: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootCosmeticsTFTDamageSkinViewModel {
    pub content_id: String,
    pub item_id: i32,
    pub name: String,
    pub description: String,
    pub loadouts_icon: String,
    pub owned: bool,
    pub selected: bool,
    pub loyalty: bool,
    pub f2p: bool,
    pub rarity_value: u32,
    pub purchase_date: String,
    pub is_recent_item: bool,
    pub level: u32,
    pub group_id: u32,
    pub group_name: String,
    pub upgrades: Vec<LolLootCosmeticsTFTDamageSkinViewModel>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootCosmeticsTFTMapSkinViewModel {
    pub item_id: i32,
    pub name: String,
    pub description: String,
    pub loadouts_icon: String,
    pub owned: bool,
    pub selected: bool,
    pub loyalty: bool,
    pub f2p: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootCounter {
    pub id: String,
    pub name: String,
    pub group_id: String,
    pub direction: String,
    pub start_value: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootCounterInstance {
    pub owner_id: String,
    pub product_id: String,
    pub group_id: String,
    pub counter_id: String,
    pub counter_value: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootCurrencyConfiguration {
    pub currencies_using_cap_wallets: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootEntityInstance {
    pub group_id: String,
    pub counters: Vec<LolLootCounterInstance>,
    pub milestones: Vec<LolLootMilestoneInstance>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootGameDataSummonerEmote {
    pub id: i64,
    pub name: String,
    pub inventory_icon: String,
    pub description: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootGameDataSummonerIcon {
    pub id: i32,
    pub title: String,
    pub image_path: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootGameflowSession {
    pub phase: LolLootGameflowPhase,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootGrantorDescription {
    pub app_name: String,
    pub entity_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootGroup {
    pub id: String,
    pub product_id: String,
    pub name: String,
    pub repeat: LolLootRepeat,
    pub counters: Vec<LolLootCounter>,
    pub milestones: Vec<LolLootMilestone>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootItemKey {
    pub inventory_type: String,
    pub item_id: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootLoginDataPacket {
    pub simple_messages: Vec<LolLootLoginSimpleMessage>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootLoginSession {
    pub state: LolLootLoginSessionStates,
    pub summoner_id: u64,
    pub account_id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootLoginSimpleMessage {
    pub account_id: u64,
    pub msg_id: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub params: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootLootBundleContentGdsResource {
    pub localized_description: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootLootBundleGdsResource {
    pub id: String,
    pub description: String,
    pub description_long: String,
    pub image: String,
    pub contents: Vec<LolLootLootBundleContentGdsResource>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootLootDataGdsResource {
    pub loot_items: Vec<LolLootLootItemGdsResource>,
    pub loot_recipes: Vec<LolLootLootRecipeGdsResource>,
    pub loot_tables: Vec<LolLootLootTableGdsResource>,
    pub loot_bundles: Vec<LolLootLootBundleGdsResource>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootLootDescription {
    pub loot_name: String,
    pub localized_description: String,
    pub localized_description_long: String,
    pub image_path: String,
    pub child_loot_table_names: Vec<String>,
    pub children_descriptions: Vec<LolLootLootDescription>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootLootDropTableEntryGdsResource {
    pub loot_id: String,
    pub localized_description: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootLootGrantNotification {
    pub id: i64,
    pub game_id: u64,
    pub player_id: u64,
    pub champion_id: i32,
    pub player_grade: String,
    pub loot_name: String,
    pub message_key: String,
    pub msg_id: String,
    pub account_id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootLootItem {
    pub loot_name: String,
    pub asset: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub rarity: String,
    pub value: i32,
    pub store_item_id: i32,
    pub upgrade_loot_name: String,
    pub expiry_time: i64,
    pub tags: String,
    pub display_categories: String,
    pub rental_seconds: i64,
    pub rental_games: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootLootItemGdsResource {
    pub id: String,
    pub name: String,
    pub description: String,
    pub image: String,
    pub start_date: String,
    pub end_date: String,
    pub recipe_menu_title: String,
    pub recipe_menu_subtitle: String,
    pub mapped_store_id: i32,
    pub lifetime_max: i32,
    pub auto_redeem: bool,
    pub recipe_menu_active: bool,
    pub rarity: LolLootLootRarity,
    #[serde(rename = "type")]
    pub type_: LolLootLootType,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootLootMilestone {
    pub id: String,
    pub threshold: u64,
    pub rewards: Vec<LolLootLootMilestoneReward>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootLootMilestoneRepeat {
    pub repeat_count: i32,
    pub repeat_scope: i32,
    pub multiplier: f32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootLootMilestoneReward {
    pub reward_group_id: String,
    pub reward_type: String,
    pub item_instance_id: String,
    pub inventory_type: String,
    pub item_id: i32,
    pub quantity: i32,
    pub loot_item: Option<LolLootPlayerLoot>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootLootMilestones {
    pub id: String,
    pub progression_config_id: String,
    pub active: bool,
    pub start_date: String,
    pub end_date: String,
    pub store_group_title: String,
    pub repeat: LolLootLootMilestoneRepeat,
    pub loot_items: Vec<String>,
    pub recipes: Vec<String>,
    pub milestones: Vec<LolLootLootMilestone>,
    pub error_caching_milestone_set: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootLootMilestonesClaimResponse {
    pub loot_milestone_set_id: String,
    pub claimed_milestones: Vec<String>,
    pub status: LolLootLootMilestoneClaimStatus,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootLootMilestonesCounter {
    pub loot_milestones_id: String,
    pub counter_value: i64,
    pub completed_loops: i64,
    pub ready_to_claim_milestones: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootLootMilestonesDataGdsResource {
    pub id: String,
    pub store_group_title: String,
    pub start_date: String,
    pub end_date: String,
    pub recipes: Vec<LolLootMilestonesRecipeGdsResource>,
    pub ledger_config: LolLootProgressionConfigGdsResource,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootLootOddsResponse {
    pub loot_id: String,
    pub parent_id: String,
    pub drop_rate: f64,
    pub quantity: i32,
    pub label: String,
    pub query: String,
    pub loot_order: i32,
    pub children: Vec<LolLootLootOddsResponse>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootLootOutputGdsResource {
    pub loot_id: String,
    pub localized_description: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootLootRecipeGdsResource {
    pub id: String,
    pub description: String,
    pub context_menu_text: String,
    pub requirement_text: String,
    pub image_path: String,
    pub intro_video_path: String,
    pub loop_video_path: String,
    pub outro_video_path: String,
    pub has_visible_loot_odds: bool,
    pub outputs: Vec<LolLootLootOutputGdsResource>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootLootTableGdsResource {
    pub id: String,
    pub description: String,
    pub description_long: String,
    pub image: String,
    pub drop_chance: Vec<LolLootLootDropTableEntryGdsResource>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootMilestone {
    pub id: String,
    pub name: String,
    pub group_id: String,
    pub counter_id: String,
    pub trigger_value: i64,
    pub properties: HashMap<String, String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootMilestoneInstance {
    pub milestone_id: String,
    pub owner_id: String,
    pub product_id: String,
    pub group_id: String,
    pub counter_id: String,
    pub trigger_value: i64,
    pub repeat_sequence: u32,
    pub triggered: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootMilestoneLootItemRewardGdsResource {
    pub internal_name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootMilestonesProgressionConfigRepeatGdsResource {
    pub name: LolLootMilestonesProgressionGroupRepeatGdsResource,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootMilestonesProgressionCounterGdsResource {
    pub id: String,
    pub name: String,
    pub direction: String,
    pub start_value: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootMilestonesProgressionGroupRepeatGdsResource {
    pub count: i32,
    pub scope: i32,
    pub multiplier: f32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootMilestonesRecipeGdsResource {
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootPlayerLoot {
    pub loot_name: String,
    pub loot_id: String,
    pub ref_id: String,
    pub localized_name: String,
    pub localized_description: String,
    pub item_desc: String,
    pub display_categories: String,
    pub rarity: String,
    pub tags: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub asset: String,
    pub tile_path: String,
    pub splash_path: String,
    pub shadow_path: String,
    pub upgrade_loot_name: String,
    pub upgrade_essence_name: String,
    pub disenchant_loot_name: String,
    pub localized_recipe_title: String,
    pub localized_recipe_subtitle: String,
    pub item_status: LolLootItemOwnershipStatus,
    pub parent_item_status: LolLootItemOwnershipStatus,
    pub redeemable_status: LolLootRedeemableStatus,
    pub count: i32,
    pub rental_games: i32,
    pub store_item_id: i32,
    pub parent_store_item_id: i32,
    pub value: i32,
    pub upgrade_essence_value: i32,
    pub disenchant_value: i32,
    pub expiry_time: i64,
    pub rental_seconds: i64,
    pub is_new: bool,
    pub is_rental: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootPlayerLootDelta {
    pub delta_count: i32,
    pub player_loot: LolLootPlayerLoot,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootPlayerLootMap {
    pub version: i64,
    pub player_loot: HashMap<String, LolLootPlayerLoot>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootPlayerLootNotification {
    pub id: String,
    pub count: i32,
    pub acknowledged: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootPlayerLootUpdate {
    pub added: Vec<LolLootPlayerLootDelta>,
    pub removed: Vec<LolLootPlayerLootDelta>,
    pub redeemed: Vec<LolLootPlayerLootDelta>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootProgressionConfigGdsResource {
    pub id: String,
    pub name: String,
    pub repeat: LolLootMilestonesProgressionConfigRepeatGdsResource,
    pub counters: Vec<LolLootMilestonesProgressionCounterGdsResource>,
    pub milestones: Vec<LolLootProgressionConfigMilestoneGdsResource>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootProgressionConfigMilestoneGdsResource {
    pub id: String,
    pub name: String,
    pub trigger_value: u64,
    pub properties: Vec<LolLootProgressionConfigMilestonePropertiesGdsResource>,
    pub counter: LolLootMilestonesProgressionCounterGdsResource,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootProgressionConfigMilestonePropertiesGdsResource {
    pub id: String,
    pub name: String,
    pub description: String,
    pub reward_strategy: String,
    pub rewards: Vec<LolLootProgressionConfigMilestoneRewardGdsResource>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootProgressionConfigMilestoneRewardGdsResource {
    pub id: String,
    pub reward_type: String,
    pub quantity: i32,
    pub loot_item_to_grant: Option<LolLootMilestoneLootItemRewardGdsResource>,
    pub legacy_loot_item: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootQueryEvaluatedLootItem {
    pub loot_name: String,
    pub localized_name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootRMSPayload {
    pub product_id: String,
    pub affinities: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootRecipeMenuConfig {
    pub enabled: bool,
    pub loot_items_using_breakout_recipe_menu: Vec<String>,
    pub always_show_loot_ids: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootRecipeMetadata {
    pub guaranteed_descriptions: Vec<LolLootLootDescription>,
    pub bonus_descriptions: Vec<LolLootLootDescription>,
    pub tooltips_disabled: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootRecipeOutput {
    pub loot_name: String,
    pub quantity: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootRecipeSlot {
    pub slot_number: i32,
    pub loot_ids: Vec<String>,
    pub tags: String,
    pub quantity: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootRecipeWithMilestones {
    pub recipe_name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub description: String,
    pub context_menu_text: String,
    pub requirement_text: String,
    pub image_path: String,
    pub intro_video_path: String,
    pub loop_video_path: String,
    pub outro_video_path: String,
    pub display_categories: String,
    pub crafter_name: String,
    pub slots: Vec<LolLootRecipeSlot>,
    pub outputs: Vec<LolLootRecipeOutput>,
    pub metadata: LolLootRecipeMetadata,
    pub loot_milestone_ids: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootRegionLocale {
    pub region: String,
    pub locale: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootRepeat {
    pub count: i32,
    pub scope: u32,
    pub multiplier: f32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootRequestDTOSelectionRequestDTO {
    pub data: LolLootSelectionRequestDTO,
    pub metadata: LolLootRequestMetadataDTO,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootRequestDTOVectorSelectionRequestDTO {
    pub data: Vec<LolLootSelectionRequestDTO>,
    pub metadata: LolLootRequestMetadataDTO,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootRequestMetadataDTO {
    pub transaction_id: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootResponseDTOSvcRewardGrant {
    pub data: LolLootSvcRewardGrant,
    pub metadata: LolLootResponseMetadataDTO,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootResponseDTOMapRewardGroupIdSelectGrantStatus {
    pub data: HashMap<String, LolLootSelectGrantStatusResponse>,
    pub metadata: LolLootResponseMetadataDTO,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootResponseDTOVectorSvcRewardGrant {
    pub data: Vec<LolLootSvcRewardGrant>,
    pub metadata: LolLootResponseMetadataDTO,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootResponseDTOVectorSvcRewardGroup {
    pub data: Vec<LolLootSvcRewardGroup>,
    pub metadata: LolLootResponseMetadataDTO,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootResponseMetadataDTO {}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootReward {
    pub id: String,
    pub item_id: String,
    pub item_type: String,
    pub quantity: i32,
    pub fulfillment_source: String,
    pub media: HashMap<String, String>,
    pub localizations: HashMap<String, String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootRewardGrant {
    pub info: LolLootSvcRewardGrant,
    pub reward_group: LolLootSvcRewardGroup,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootRewardsConfig {
    pub grant_filtering: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootSelectionRequestDTO {
    pub grant_id: String,
    pub reward_group_id: String,
    pub selections: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootSelectionStrategyConfig {
    pub min_selections_allowed: u32,
    pub max_selections_allowed: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootSummoner {
    pub summoner_level: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootSvcRewardGrant {
    pub id: String,
    pub grantee_id: String,
    pub reward_group_id: String,
    pub date_created: String,
    pub status: LolLootGrantStatus,
    pub grant_elements: Vec<LolLootSvcRewardGrantElement>,
    pub selected_ids: Vec<String>,
    pub viewed: bool,
    pub grantor_description: LolLootGrantorDescription,
    pub message_parameters: HashMap<String, Value>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootSvcRewardGrantElement {
    pub element_id: String,
    pub item_id: String,
    pub item_type: String,
    pub fulfillment_source: String,
    pub status: LolLootRewardStatus,
    pub quantity: i32,
    pub media: HashMap<String, String>,
    pub localizations: HashMap<String, String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootSvcRewardGroup {
    pub id: String,
    pub product_id: String,
    pub types: Vec<String>,
    pub rewards: Vec<LolLootReward>,
    pub child_reward_group_ids: Vec<String>,
    pub reward_strategy: LolLootRewardStrategy,
    pub selection_strategy_config: Option<LolLootSelectionStrategyConfig>,
    pub active: bool,
    pub media: HashMap<String, String>,
    pub localizations: HashMap<String, String>,
    pub celebration_type: LolLootCelebrationType,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootTFTDamageSkinGroupViewModel {
    pub group_name: String,
    pub group_id: u32,
    pub num_owned: u32,
    pub num_available: u32,
    pub items: Vec<LolLootCosmeticsTFTDamageSkinViewModel>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootTFTDamageSkinGroupedViewModel {
    pub selected_loadout_item: LolLootCosmeticsTFTDamageSkinViewModel,
    pub default_item_id: i32,
    pub groups: Vec<LolLootTFTDamageSkinGroupViewModel>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootTFTMapSkinGroupViewModel {
    pub items: Vec<LolLootCosmeticsTFTMapSkinViewModel>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootTFTMapSkinGroupedViewModel {
    pub selected_loadout_item: LolLootCosmeticsTFTMapSkinViewModel,
    pub default_item_id: i32,
    pub groups: Vec<LolLootTFTMapSkinGroupViewModel>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLootVerboseLootOddsResponse {
    pub recipe_name: String,
    pub chance_to_contain: Vec<LolLootLootOddsResponse>,
    pub guaranteed_to_contain: Vec<LolLootLootOddsResponse>,
    pub loot_item: LolLootPlayerLoot,
    pub has_pity_rules: bool,
    pub checks_ownership: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLoyaltyAccessToken {
    pub token: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLoyaltyGlobalRewards {
    pub all_champions: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLoyaltyInventoryDTO {
    pub items: HashMap<String, Value>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLoyaltyInventoryItemDTO {
    pub inventory_type: String,
    pub loyalty: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLoyaltyInventoryResponseDTO {
    pub data: LolLoyaltyInventoryDTO,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLoyaltyLoginSession {
    pub state: LolLoyaltyLoginSessionStates,
    pub summoner_id: u64,
    pub account_id: u64,
    pub puuid: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLoyaltyLoyaltyRewards {
    pub free_rewarded_champions_count: i32,
    pub champion_ids: Vec<i32>,
    pub free_rewarded_skins_count: i32,
    pub skin_ids: Vec<i32>,
    pub global: LolLoyaltyGlobalRewards,
    pub ip_boost: i32,
    pub xp_boost: HashMap<String, i32>,
    pub loyalty_t_f_t_map_skin_count: i32,
    pub loyalty_t_f_t_companion_count: i32,
    pub loyalty_t_f_t_damage_skin_count: i32,
    pub loyalty_sources: HashMap<String, bool>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLoyaltyLoyaltyRewardsSimplified {
    pub free_rewarded_champions_count: i32,
    pub champion_ids: Vec<i32>,
    pub free_rewarded_skins_count: i32,
    pub skin_ids: Vec<i32>,
    pub global: LolLoyaltyGlobalRewards,
    pub ip_boost: i32,
    pub xp_boost: i32,
    pub loyalty_t_f_t_map_skin_count: i32,
    pub loyalty_t_f_t_companion_count: i32,
    pub loyalty_t_f_t_damage_skin_count: i32,
    pub loyalty_sources: HashMap<String, bool>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLoyaltyLoyaltyStatusNotification {
    pub status: LolLoyaltyLoyaltyStatus,
    pub rewards: LolLoyaltyLoyaltyRewardsSimplified,
    pub reload_inventory: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLoyaltyPlayerNotification {
    pub critical: bool,
    pub detail_key: String,
    pub source: String,
    pub state: String,
    pub title_key: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub icon_url: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLoyaltyRiotMessagingServiceMessage {
    pub resource: String,
    pub service: String,
    pub version: String,
    pub timestamp: i64,
    pub payload: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLoyaltyRmsEntitlementPayload {
    pub item_id: String,
    pub item_type_id: String,
    pub entitlement_type_id: String,
    pub resource_operation: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMapsGameModeSpellList {
    pub spells: Vec<u64>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMapsMaps {
    pub id: i64,
    pub is_default: bool,
    pub game_mode: String,
    pub game_mode_name: String,
    pub game_mode_short_name: String,
    pub game_mode_description: String,
    pub game_mutator: String,
    pub is_r_g_m: bool,
    pub name: String,
    pub description: String,
    pub map_string_id: String,
    pub platform_id: String,
    pub platform_name: String,
    pub assets: HashMap<String, String>,
    pub loc_strings: HashMap<String, String>,
    pub categorized_content_bundles: Value,
    pub tutorial_cards: Vec<LolMapsTutorialCard>,
    pub properties: Value,
    pub per_position_required_summoner_spells: HashMap<String, LolMapsGameModeSpellList>,
    pub per_position_disallowed_summoner_spells: HashMap<String, LolMapsGameModeSpellList>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMapsTutorialCard {
    pub header: Option<String>,
    pub footer: Option<String>,
    pub description: Option<String>,
    pub image_path: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryAcsEndPoint {
    pub url: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryAcsPlayer {
    pub platform_id: String,
    pub account_id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryGAMHSMatchHistoryData {
    pub metadata: LolMatchHistoryGAMHSMatchHistoryMetadata,
    pub json: Value,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryGAMHSMatchHistoryList {
    pub games: Vec<LolMatchHistoryGAMHSMatchHistoryData>,
    pub active_puuid: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryGAMHSMatchHistoryMetadata {
    pub product: String,
    pub data_version: u8,
    pub info_type: String,
    pub match_id: String,
    pub tags: Vec<String>,
    pub participants: Vec<String>,
    pub timestamp: u64,
    pub private: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryLoginSession {
    pub state: LolMatchHistoryLoginSessionStates,
    pub summoner_id: u64,
    pub account_id: u64,
    pub id_token: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMHSummoner {
    pub summoner_id: u64,
    pub account_id: u64,
    pub display_name: String,
    pub puuid: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryEvent {
    #[serde(rename = "type")]
    pub type_: String,
    pub timestamp: u64,
    pub participant_id: u16,
    pub team_id: u16,
    pub item_id: u16,
    pub killer_id: u16,
    pub victim_id: u16,
    pub skill_slot: u16,
    pub position: LolMatchHistoryMatchHistoryPosition,
    pub assisting_participant_ids: Vec<u16>,
    pub building_type: String,
    pub lane_type: String,
    pub tower_type: String,
    pub monster_type: String,
    pub monster_sub_type: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryGame {
    pub game_id: u64,
    pub platform_id: String,
    pub game_creation: u64,
    pub game_creation_date: String,
    pub game_duration: u32,
    pub queue_id: i32,
    pub map_id: u16,
    pub season_id: u16,
    pub game_version: String,
    pub game_mode: String,
    pub game_type: String,
    pub teams: Vec<LolMatchHistoryMatchHistoryTeam>,
    pub participants: Vec<LolMatchHistoryMatchHistoryParticipant>,
    pub participant_identities: Vec<LolMatchHistoryMatchHistoryParticipantIdentities>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryGameList {
    pub game_index_begin: u64,
    pub game_index_end: u64,
    pub game_begin_date: String,
    pub game_end_date: String,
    pub game_count: u64,
    pub games: Vec<LolMatchHistoryMatchHistoryGame>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryList {
    pub platform_id: String,
    pub account_id: u64,
    pub games: LolMatchHistoryMatchHistoryGameList,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryParticipant {
    pub participant_id: u16,
    pub team_id: u16,
    pub champion_id: i32,
    pub spell1_id: u16,
    pub spell2_id: u16,
    pub highest_achieved_season_tier: String,
    pub stats: LolMatchHistoryMatchHistoryParticipantStatistics,
    pub timeline: LolMatchHistoryMatchHistoryTimeline,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryParticipantFrame {
    pub participant_id: u16,
    pub position: LolMatchHistoryMatchHistoryPosition,
    pub current_gold: i32,
    pub total_gold: i32,
    pub level: u16,
    pub xp: u32,
    pub minions_killed: u16,
    pub jungle_minions_killed: u16,
    pub dominion_score: u16,
    pub team_score: u16,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryParticipantIdentities {
    pub participant_id: u16,
    pub player: LolMatchHistoryMatchHistoryParticipantIdentityPlayer,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryParticipantIdentityPlayer {
    pub platform_id: String,
    pub account_id: u64,
    pub summoner_id: u64,
    pub summoner_name: String,
    pub current_platform_id: String,
    pub current_account_id: u64,
    pub match_history_uri: String,
    pub profile_icon: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryParticipantStatistics {
    pub participant_id: u16,
    pub win: bool,
    pub item0: u16,
    pub item1: u16,
    pub item2: u16,
    pub item3: u16,
    pub item4: u16,
    pub item5: u16,
    pub item6: u16,
    pub kills: i64,
    pub deaths: i64,
    pub assists: i64,
    pub largest_killing_spree: i64,
    pub largest_multi_kill: i64,
    pub killing_sprees: i64,
    pub longest_time_spent_living: i64,
    pub double_kills: i64,
    pub triple_kills: i64,
    pub quadra_kills: i64,
    pub penta_kills: i64,
    pub unreal_kills: i64,
    pub total_damage_dealt: i64,
    pub magic_damage_dealt: i64,
    pub physical_damage_dealt: i64,
    pub true_damage_dealt: i64,
    pub largest_critical_strike: i64,
    pub total_damage_dealt_to_champions: i64,
    pub magic_damage_dealt_to_champions: i64,
    pub physical_damage_dealt_to_champions: i64,
    pub true_damage_dealt_to_champions: i64,
    pub total_heal: i64,
    pub total_units_healed: i64,
    pub total_damage_taken: i64,
    pub magical_damage_taken: i64,
    pub physical_damage_taken: i64,
    pub true_damage_taken: i64,
    pub gold_earned: i64,
    pub gold_spent: i64,
    pub turret_kills: i64,
    pub inhibitor_kills: i64,
    pub total_minions_killed: i64,
    pub neutral_minions_killed: i64,
    pub neutral_minions_killed_team_jungle: i64,
    pub neutral_minions_killed_enemy_jungle: i64,
    pub total_time_crowd_control_dealt: i64,
    pub champ_level: i64,
    pub vision_wards_bought_in_game: i64,
    pub sight_wards_bought_in_game: i64,
    pub wards_placed: i64,
    pub wards_killed: i64,
    pub first_blood_kill: bool,
    pub first_blood_assist: bool,
    pub first_tower_kill: bool,
    pub first_tower_assist: bool,
    pub first_inhibitor_kill: bool,
    pub first_inhibitor_assist: bool,
    pub game_ended_in_early_surrender: bool,
    pub game_ended_in_surrender: bool,
    pub caused_early_surrender: bool,
    pub early_surrender_accomplice: bool,
    pub team_early_surrendered: bool,
    pub combat_player_score: i64,
    pub objective_player_score: i64,
    pub total_player_score: i64,
    pub total_score_rank: i64,
    pub damage_self_mitigated: i64,
    pub damage_dealt_to_objectives: i64,
    pub damage_dealt_to_turrets: i64,
    pub vision_score: i64,
    pub time_c_cing_others: i64,
    pub player_score0: i64,
    pub player_score1: i64,
    pub player_score2: i64,
    pub player_score3: i64,
    pub player_score4: i64,
    pub player_score5: i64,
    pub player_score6: i64,
    pub player_score7: i64,
    pub player_score8: i64,
    pub player_score9: i64,
    pub perk_primary_style: i64,
    pub perk_sub_style: i64,
    pub perk0: i64,
    pub perk0_var1: i64,
    pub perk0_var2: i64,
    pub perk0_var3: i64,
    pub perk1: i64,
    pub perk1_var1: i64,
    pub perk1_var2: i64,
    pub perk1_var3: i64,
    pub perk2: i64,
    pub perk2_var1: i64,
    pub perk2_var2: i64,
    pub perk2_var3: i64,
    pub perk3: i64,
    pub perk3_var1: i64,
    pub perk3_var2: i64,
    pub perk3_var3: i64,
    pub perk4: i64,
    pub perk4_var1: i64,
    pub perk4_var2: i64,
    pub perk4_var3: i64,
    pub perk5: i64,
    pub perk5_var1: i64,
    pub perk5_var2: i64,
    pub perk5_var3: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryPlayerChampMasteryDelta {
    pub grade: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryPlayerDelta {
    pub original_account_id: u64,
    pub original_platform_id: String,
    pub deltas: Vec<LolMatchHistoryMatchHistoryPlayerGameDelta>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryPlayerGameDelta {
    pub game_platform_id: String,
    pub game_id: u64,
    pub platform_delta: LolMatchHistoryMatchHistoryPlayerPlatformDelta,
    pub league_delta: LolMatchHistoryMatchHistoryPlayerLeagueDelta,
    pub champ_mastery: LolMatchHistoryMatchHistoryPlayerChampMasteryDelta,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryPlayerLeagueDelta {
    pub league_point_delta: u64,
    pub reason: String,
    pub mini_series_progress: Vec<String>,
    pub timestamp: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryPlayerPlatformDelta {
    pub xp_delta: u64,
    pub ip_delta: u64,
    pub compensation_mode_enabled: bool,
    pub timestamp: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryPosition {
    pub x: i16,
    pub y: i16,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryTeam {
    pub team_id: u16,
    pub win: String,
    pub first_blood: bool,
    pub first_tower: bool,
    pub first_inhibitor: bool,
    pub first_baron: bool,
    pub first_dargon: bool,
    pub tower_kills: u32,
    pub inhibitor_kills: u32,
    pub baron_kills: u32,
    pub dragon_kills: u32,
    pub vilemaw_kills: u32,
    pub rift_herald_kills: u32,
    pub dominion_victory_score: u32,
    pub bans: Vec<LolMatchHistoryMatchHistoryTeamBan>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryTeamBan {
    pub champion_id: i32,
    pub pick_turn: u16,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryTimeline {
    pub participant_id: u16,
    pub role: String,
    pub lane: String,
    pub creeps_per_min_deltas: HashMap<String, f64>,
    pub xp_per_min_deltas: HashMap<String, f64>,
    pub gold_per_min_deltas: HashMap<String, f64>,
    pub cs_diff_per_min_deltas: HashMap<String, f64>,
    pub xp_diff_per_min_deltas: HashMap<String, f64>,
    pub damage_taken_per_min_deltas: HashMap<String, f64>,
    pub damage_taken_diff_per_min_deltas: HashMap<String, f64>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryTimelineFrame {
    pub participant_frames: HashMap<String, LolMatchHistoryMatchHistoryParticipantFrame>,
    pub events: Vec<LolMatchHistoryMatchHistoryEvent>,
    pub timestamp: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryTimelineFrames {
    pub frames: Vec<LolMatchHistoryMatchHistoryTimelineFrame>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryRecentlyPlayedSummoner {
    pub summoner_id: u64,
    pub summoner_name: String,
    pub game_id: u64,
    pub game_creation_date: String,
    pub champion_id: u64,
    pub team_id: u64,
    pub puuid: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistorySummoner {
    pub display_name: String,
    pub puuid: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchmakingGameflowGameData {
    pub queue: LolMatchmakingGameflowQueue,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchmakingGameflowGameDodge {
    pub state: LolMatchmakingMatchmakingDodgeState,
    pub dodge_ids: Vec<u64>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchmakingGameflowGameTypeConfig {
    pub reroll: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchmakingGameflowQueue {
    pub game_type_config: LolMatchmakingGameflowGameTypeConfig,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchmakingGameflowSession {
    pub phase: LolMatchmakingGameflowPhase,
    pub game_data: LolMatchmakingGameflowGameData,
    pub game_dodge: LolMatchmakingGameflowGameDodge,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchmakingLobbyStatus {
    pub queue_id: i32,
    pub is_custom: bool,
    pub is_leader: bool,
    pub is_spectator: bool,
    pub allowed_play_again: bool,
    pub member_summoner_ids: Vec<u64>,
    pub lobby_id: Option<String>,
    pub custom_spectator_policy: LolMatchmakingQueueCustomGameSpectatorPolicy,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchmakingLoginSession {
    pub state: LolMatchmakingLoginSessionState,
    pub summoner_id: u64,
    pub account_id: u64,
    pub connected: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchmakingMatchmakingDodgeData {
    pub state: LolMatchmakingMatchmakingDodgeState,
    pub dodger_id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchmakingMatchmakingLowPriorityData {
    pub penalized_summoner_ids: Vec<u64>,
    pub penalty_time: f64,
    pub penalty_time_remaining: f64,
    pub busted_leaver_access_token: String,
    pub reason: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchmakingMatchmakingReadyCheckResource {
    pub state: LolMatchmakingMatchmakingReadyCheckState,
    pub player_response: LolMatchmakingMatchmakingReadyCheckResponse,
    pub dodge_warning: LolMatchmakingMatchmakingDodgeWarning,
    pub timer: f32,
    pub decliner_ids: Vec<u64>,
    pub suppress_ux: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchmakingMatchmakingSearchErrorResource {
    pub id: i32,
    pub error_type: String,
    pub penalized_summoner_id: u64,
    pub penalty_time_remaining: f64,
    pub message: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchmakingMatchmakingSearchResource {
    pub queue_id: i32,
    pub is_currently_in_queue: bool,
    pub lobby_id: String,
    pub search_state: LolMatchmakingMatchmakingSearchState,
    pub time_in_queue: f32,
    pub estimated_queue_time: f32,
    pub ready_check: LolMatchmakingMatchmakingReadyCheckResource,
    pub dodge_data: LolMatchmakingMatchmakingDodgeData,
    pub low_priority_data: LolMatchmakingMatchmakingLowPriorityData,
    pub errors: Vec<LolMatchmakingMatchmakingSearchErrorResource>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchmakingPlayerStatus {
    pub current_lobby_status: Option<LolMatchmakingLobbyStatus>,
    pub last_queued_lobby_status: Option<LolMatchmakingLobbyStatus>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchmakingQueue {
    pub id: i32,
    pub is_team_builder_managed: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMissionsCollectionsChampion {
    pub id: i32,
    pub free_to_play: bool,
    pub ownership: LolMissionsCollectionsOwnership,
    pub skins: Vec<LolMissionsCollectionsChampionSkin>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMissionsCollectionsChampionSkin {
    pub champion_id: i32,
    pub id: i32,
    pub ownership: LolMissionsCollectionsOwnership,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMissionsCollectionsOwnership {
    pub loyalty_reward: bool,
    pub xbox_g_p_reward: bool,
    pub owned: bool,
    pub rental: LolMissionsCollectionsRental,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMissionsCollectionsRental {
    pub rented: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMissionsCollectionsSummoner {
    pub summoner_level: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMissionsCollectionsSummonerIcons {
    pub icons: Vec<i32>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMissionsCollectionsWardSkin {
    pub id: i64,
    pub ownership: LolMissionsCollectionsOwnership,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMissionsGameflowSession {
    pub phase: LolMissionsGameflowPhase,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMissionsInventoryItemWithPayload {
    pub item_id: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMissionsLoginSession {
    pub account_id: u64,
    pub summoner_id: u64,
    pub puuid: String,
    pub platform_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMissionsLoyaltyStatusNotification {
    pub status: LolMissionsLoyaltyStatus,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMissionsMissionAsset {
    pub internal_name: String,
    pub path: String,
    pub icon_needs_frame: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMissionsMissionsNotificationResource {
    pub background_url: String,
    pub created: String,
    pub critical: bool,
    pub data: HashMap<String, String>,
    pub detail_key: String,
    pub expires: String,
    pub icon_url: String,
    pub id: u64,
    pub source: String,
    pub state: String,
    pub title_key: String,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMissionsMissionsSettingsDataResource {
    pub selected_series: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMissionsPlayerUpdateResponse {
    pub player_missions: Vec<PlayerMissionDTO>,
    pub player_series: Vec<SeriesDTO>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMissionsPluginRegionLocaleChangedEvent {
    pub locale: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMissionsRewardGrant {
    pub info: LolMissionsRewardGrantInfo,
    pub reward_group: LolMissionsRewardGroup,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMissionsRewardGrantElement {
    pub id: String,
    pub item_id: String,
    pub item_type: String,
    pub fulfillment_source: String,
    pub reward_status: LolMissionsRewardStatus,
    pub quantity: i32,
    pub media: HashMap<String, String>,
    pub localizations: HashMap<String, String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMissionsRewardGrantInfo {
    pub id: String,
    pub grantee_id: String,
    pub reward_group_id: String,
    pub status: LolMissionsGrantStatus,
    pub grant_elements: Vec<LolMissionsRewardGrantElement>,
    pub selected_ids: Vec<String>,
    pub viewed: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMissionsRewardGroup {
    pub id: String,
    pub internal_name: String,
    pub product_id: String,
    pub types: Vec<String>,
    pub rewards: Vec<LolMissionsSvcReward>,
    pub child_reward_group_ids: Vec<String>,
    pub reward_strategy: LolMissionsRewardStrategy,
    pub selection_strategy_config: LolMissionsSelectionStrategyConfig,
    pub active: bool,
    pub media: HashMap<String, String>,
    pub localizations: HashMap<String, String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMissionsRewardGroupsSelection {
    pub reward_groups: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMissionsRewardsProductConfig {
    pub enabled: bool,
    pub service_url: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMissionsSelectionStrategyConfig {
    pub min_selections_allowed: u32,
    pub max_selections_allowed: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMissionsSeriesOpt {
    pub series_id: String,
    pub option: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMissionsSvcReward {
    pub id: String,
    pub item_id: String,
    pub quantity: i32,
    pub fulfillment_source: String,
    pub media: HashMap<String, String>,
    pub localizations: HashMap<String, String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMissionsTftBattlepass {
    pub total_points_earned: i32,
    pub milestones: Vec<LolMissionsTftBattlepassMilestone>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMissionsTftBattlepassMilestone {
    pub mission_id: String,
    pub label: String,
    pub state: String,
    pub points_for_milestone: i32,
    pub percent_complete: i32,
    pub icon_image_url: String,
    pub rewards: Vec<PlayerMissionRewardDTO>,
    pub description: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMissionsTftOrb {
    pub mission_id: String,
    pub status: String,
    pub unlock_time: i64,
    pub reward_level: u8,
    pub rewards: Vec<PlayerMissionRewardDTO>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMissionsTftPaidBattlepass {
    pub total_points_earned: i32,
    pub milestones: Vec<LolMissionsTftPaidBattlepassMilestone>,
    pub bonuses: Vec<LolMissionsTftPaidBattlepassMilestone>,
    pub active_milestone: LolMissionsTftPaidBattlepassMilestone,
    pub info: LolMissionsTftPaidBattlepassInfo,
    pub last_viewed_progress: i32,
    pub last_viewed_milestone: LolMissionsTftPaidBattlepassMilestone,
    pub progress_mission_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMissionsTftPaidBattlepassInfo {
    pub title: String,
    pub description: String,
    pub start_date: u64,
    pub end_date: u64,
    pub premium: bool,
    pub background_image_url: String,
    pub internal_name: String,
    pub pc_purchase_requirement: String,
    pub media: HashMap<String, String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMissionsTftPaidBattlepassMilestone {
    pub mission_id: String,
    pub internal_name: String,
    pub title: String,
    pub description: String,
    pub state: String,
    pub status: String,
    pub points_needed_for_milestone: i32,
    pub points_earned_for_milestone: i32,
    pub total_points_for_milestone: i32,
    pub level: i32,
    pub icon_image_url: String,
    pub icon_needs_frame: bool,
    pub rewards: Vec<PlayerMissionRewardDTO>,
    pub is_paid: bool,
    pub is_locked: bool,
    pub is_keystone: bool,
    pub is_bonus: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMissionsTftWeeklyMissions {
    pub missions: Vec<PlayerMissionDTO>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMissionsUserInfo {
    pub user_info: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolModeProgressionInventoryRewardItem {
    pub item_id: i32,
    pub uuid: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolModeProgressionLoadout {
    pub id: String,
    pub name: String,
    pub scope: String,
    pub loadout: HashMap<String, LolModeProgressionLoadoutsSlot>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolModeProgressionLoadoutsSlot {
    pub content_id: String,
    pub inventory_type: String,
    pub item_id: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeRewardsAccountSettingsData {
    pub login: LolNpeRewardsLoginSeriesSettings,
    pub challenges: LolNpeRewardsChallengesSettings,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeRewardsAccountSettingsPayload {
    pub data: LolNpeRewardsAccountSettingsData,
    pub schema_version: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeRewardsAllRewards {
    pub level: LolNpeRewardsRewardSeries,
    pub login: LolNpeRewardsRewardSeries,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeRewardsChallengesProgress {
    pub progress: LolNpeRewardsProgress,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeRewardsChallengesSettings {
    pub all_missions_completed: bool,
    pub total_count: i8,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeRewardsLoginSeriesSettings {
    pub all_rewards_claimed: bool,
    pub last_completed_mission_internal_name: String,
    pub last_completed_mission_date: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeRewardsMission {
    pub completed_date: i64,
    pub status: String,
    pub series_name: String,
    pub internal_name: String,
    pub metadata: LolNpeRewardsMissionsRewardPackMetaData,
    pub display: LolNpeRewardsMissionDisplay,
    pub objectives: Vec<LolNpeRewardsObjective>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeRewardsMissionDisplay {
    pub locations: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeRewardsMissionSeries {
    pub internal_name: String,
    pub status: String,
    pub id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeRewardsMissionSeriesOptIn {
    pub series_id: String,
    pub option: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeRewardsMissionsRewardPackMetaData {
    pub npe_reward_pack: LolNpeRewardsRewardPack,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeRewardsObjective {
    pub progress: LolNpeRewardsProgress,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeRewardsProgress {
    pub last_viewed_progress: i32,
    pub current_progress: i32,
    pub total_count: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeRewardsRequirements {
    pub level: u32,
    pub day: u32,
    pub mission_internal_name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeRewardsReward {
    pub renderer: String,
    pub data: Value,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeRewardsRewardPack {
    pub index: i32,
    #[serde(rename = "type")]
    pub type_: String,
    pub requirements: LolNpeRewardsRequirements,
    pub state: String,
    pub premium_reward: bool,
    pub reward_key: String,
    pub major_reward: LolNpeRewardsReward,
    pub minor_rewards: Vec<LolNpeRewardsReward>,
    pub delay: i64,
    pub unlock_time: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeRewardsRewardSeries {
    pub reward_packs: Vec<LolNpeRewardsRewardPack>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeRewardsRewardSeriesState {
    pub all_rewards_claimed: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeRewardsSummoner {
    pub summoner_level: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeTutorialPathAccountSettingsCategoryResource {
    pub data: Option<LolNpeTutorialPathAccountSettingsTutorial>,
    pub schema_version: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeTutorialPathAccountSettingsTutorial {
    pub has_seen_tutorial_path: bool,
    pub has_skipped_tutorial_path: bool,
    pub should_see_new_player_experience: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeTutorialPathCollectionsChampion {
    pub alias: String,
    pub ban_vo_path: String,
    pub choose_vo_path: String,
    pub id: i32,
    pub name: String,
    pub roles: Vec<String>,
    pub square_portrait_path: String,
    pub stinger_sfx_path: String,
    pub passive: LolNpeTutorialPathCollectionsChampionSpell,
    pub spells: Vec<LolNpeTutorialPathCollectionsChampionSpell>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeTutorialPathCollectionsChampionSpell {
    pub name: String,
    pub description: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeTutorialPathExpiringWarning {
    #[serde(rename = "type")]
    pub type_: String,
    pub message: String,
    pub alert_time: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeTutorialPathGameflowSession {
    pub phase: LolNpeTutorialPathGameflowPhase,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeTutorialPathIds {
    pub mission_ids: Vec<String>,
    pub series_ids: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeTutorialPathLobbyChangeQueue {
    pub queue_id: i32,
    pub is_custom: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeTutorialPathLobbyDto {
    pub party_id: String,
    pub game_config: LolNpeTutorialPathLobbyGameConfigDto,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeTutorialPathLobbyGameConfigDto {
    pub queue_id: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeTutorialPathMission {
    pub id: String,
    pub title: String,
    pub helper_text: String,
    pub description: String,
    pub background_image_url: String,
    pub icon_image_url: String,
    pub series_name: String,
    pub locale: String,
    pub metadata: LolNpeTutorialPathMissionMetadata,
    pub start_time: i64,
    pub end_time: i64,
    pub last_updated_timestamp: i64,
    pub objectives: Vec<LolNpeTutorialPathObjective>,
    pub rewards: Vec<LolNpeTutorialPathReward>,
    pub expiring_warnings: Vec<LolNpeTutorialPathExpiringWarning>,
    pub requirements: Vec<String>,
    pub reward_strategy: LolNpeTutorialPathRewardStrategy,
    pub display: LolNpeTutorialPathMissionDisplay,
    pub completion_expression: String,
    pub viewed: bool,
    pub is_new: bool,
    pub status: String,
    pub mission_type: String,
    pub display_type: String,
    pub completed_date: i64,
    pub cooldown_time_millis: i64,
    pub celebration_type: String,
    pub client_notify_level: String,
    pub internal_name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeTutorialPathMissionDisplay {
    pub attributes: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeTutorialPathMissionMetadata {
    pub tutorial: LolNpeTutorialPathTutorialMetadata,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeTutorialPathObjective {
    #[serde(rename = "type")]
    pub type_: String,
    pub description: String,
    pub progress: LolNpeTutorialPathProgress,
    pub sequence: i32,
    pub reward_groups: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeTutorialPathProgress {
    pub last_viewed_progress: i32,
    pub current_progress: i32,
    pub total_count: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeTutorialPathRequirement {
    pub expression: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeTutorialPathReward {
    pub reward_type: String,
    pub reward_group: String,
    pub description: String,
    pub icon_url: String,
    pub item_id: String,
    pub unique_name: String,
    pub reward_fulfilled: bool,
    pub reward_group_selected: bool,
    pub sequence: i32,
    pub quantity: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeTutorialPathRewardStrategy {
    pub group_strategy: String,
    pub select_max_group_count: i16,
    pub select_min_group_count: i16,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeTutorialPathSeries {
    pub id: String,
    pub internal_name: String,
    pub status: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeTutorialPathSeriesOpt {
    pub series_id: String,
    pub option: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeTutorialPathSummoner {
    pub summoner_id: u64,
    pub account_id: u64,
    pub display_name: String,
    pub profile_icon_id: i32,
    pub summoner_level: u32,
    pub xp_since_last_level: u64,
    pub xp_until_next_level: u64,
    pub percent_complete_for_next_level: u32,
    pub name_change_flag: bool,
    pub unnamed: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeTutorialPathSummonerIcon {
    pub profile_icon_id: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeTutorialPathTutorial {
    pub id: String,
    pub step_number: i32,
    pub title: String,
    pub description: String,
    pub background_url: String,
    pub queue_id: String,
    pub use_quick_search_matchmaking: bool,
    pub use_chosen_champion: bool,
    pub status: LolNpeTutorialPathTutorialStatus,
    pub is_viewed: bool,
    #[serde(rename = "type")]
    pub type_: LolNpeTutorialPathTutorialType,
    pub rewards: Vec<LolNpeTutorialPathTutorialReward>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeTutorialPathTutorialMetadata {
    pub step_number: i32,
    pub queue_id: String,
    pub display_rewards: HashMap<String, String>,
    pub use_quick_search_matchmaking: bool,
    pub use_chosen_champion: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeTutorialPathTutorialReward {
    pub reward_type: String,
    pub description: String,
    pub reward_fulfilled: bool,
    pub icon_url: String,
    pub item_id: String,
    pub sequence: i32,
    pub unique_name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPatchChunkingPatcherEnvironment {
    pub game_patcher_available: bool,
    pub game_patcher_enabled: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPatchComponentActionProgress {
    pub current_item: String,
    pub total: LolPatchComponentStateProgress,
    pub network: LolPatchComponentStateProgress,
    pub primary_work: LolPatchComponentStateWorkType,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPatchComponentState {
    pub id: String,
    pub action: LolPatchComponentStateAction,
    pub is_up_to_date: bool,
    pub is_update_available: bool,
    pub time_of_last_up_to_date_check_i_s_o8601: Option<String>,
    pub is_corrupted: bool,
    pub progress: Option<LolPatchComponentActionProgress>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPatchComponentStateProgress {
    pub bytes_complete: u64,
    pub bytes_required: u64,
    pub bytes_per_second: f64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPatchEntitlementsTokenResource {
    pub access_token: String,
    pub token: String,
    pub entitlements: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPatchInstallPaths {
    pub game_install_root: String,
    pub game_executable_path: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPatchNotification {
    pub id: String,
    pub notification_id: LolPatchNotificationId,
    pub data: HashMap<String, Value>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPatchPatchSieveCompatVersion {
    pub id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPatchPatchSieveDownload {
    pub url: String,
    pub scd_required: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPatchPatchSieveLabelValue {
    pub values: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPatchPatchSieveQueryResponse {
    pub releases: Vec<LolPatchPatchSieveRelease>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPatchPatchSieveRelease {
    pub release: LolPatchPatchSieveReleaseInfo,
    pub compat_version: LolPatchPatchSieveCompatVersion,
    pub download: LolPatchPatchSieveDownload,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPatchPatchSieveReleaseInfo {
    pub product: String,
    pub id: String,
    pub labels: HashMap<String, LolPatchPatchSieveLabelValue>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPatchPatcherInstallSettings {
    pub game_patcher: Option<String>,
    pub game_patch_url: String,
    pub game_patcher_available: bool,
    pub max_download_speed_mbps: u32,
    pub locales: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPatchPatcherRegionSettings {
    pub patchline: String,
    pub game_patcher: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPatchPatcherSelfUpdateSettings {
    pub restart_delay: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPatchPatcherSettings {
    pub product_refresh_period: f64,
    pub channel: String,
    pub headers: HashMap<String, String>,
    pub self_update: LolPatchPatcherSelfUpdateSettings,
    pub patchsieve_url: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPatchProductState {
    pub id: String,
    pub action: LolPatchComponentStateAction,
    pub is_up_to_date: bool,
    pub is_update_available: bool,
    pub is_corrupted: bool,
    pub is_stopped: bool,
    pub percent_patched: f64,
    pub components: Vec<LolPatchComponentState>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPatchRegionLocale {
    pub region: String,
    pub locale: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPatchScdCookie {
    pub domain: String,
    pub path: String,
    pub cookie: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPatchScdCookies {
    pub cookies: Vec<LolPatchScdCookie>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPatchStatus {
    pub connected_to_patch_server: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPatchSupportedGameRelease {
    pub artifact_id: String,
    pub download: LolPatchPatchSieveDownload,
    pub selected: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPatchSupportedGameReleases {
    pub supported_game_releases: Vec<LolPatchSupportedGameRelease>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPatchUxResource {
    pub visible: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksChampSelectAction {
    pub id: i64,
    pub actor_cell_id: i64,
    pub champion_id: i32,
    #[serde(rename = "type")]
    pub type_: String,
    pub completed: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksChampSelectBannedChampions {
    pub my_team_bans: Vec<i32>,
    pub their_team_bans: Vec<i32>,
    pub num_bans: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksChampSelectChatRoomDetails {
    pub multi_user_chat_j_w_t: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksChampSelectMySelection {
    pub selected_skin_id: Option<i32>,
    pub spell1_id: Option<u64>,
    pub spell2_id: Option<u64>,
    pub ward_skin_id: Option<i64>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksChampSelectPlayerSelection {
    pub cell_id: i64,
    pub champion_id: i32,
    pub selected_skin_id: i32,
    pub ward_skin_id: i64,
    pub spell1_id: u64,
    pub spell2_id: u64,
    pub team: i32,
    pub assigned_position: String,
    pub champion_pick_intent: i32,
    pub summoner_id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksChampSelectSession {
    pub timer: LolPerksChampSelectTimer,
    pub chat_details: LolPerksChampSelectChatRoomDetails,
    pub my_team: Vec<LolPerksChampSelectPlayerSelection>,
    pub their_team: Vec<LolPerksChampSelectPlayerSelection>,
    pub trades: Vec<LolPerksChampSelectTradeContract>,
    pub actions: Vec<Value>,
    pub bans: LolPerksChampSelectBannedChampions,
    pub local_player_cell_id: i64,
    pub is_spectating: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksChampSelectTimer {
    pub adjusted_time_left_in_phase: i64,
    pub total_time_in_phase: i64,
    pub phase: String,
    pub is_infinite: bool,
    pub internal_now_in_epoch_ms: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksChampSelectTradeContract {
    pub id: i64,
    pub cell_id: i64,
    pub state: LolPerksChampSelectTradeState,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksChampionPreferredStyle {
    pub champion_name: String,
    pub style: i32,
    pub champion_id: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksChampionRuneRecommendationsGDSResource {
    pub champion_id: i32,
    pub is_override: bool,
    pub rune_recommendations: Vec<LolPerksRuneRecommendationGDSResource>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksDefaultStatModsPerSubStyle {
    pub id: i32,
    pub perks: Vec<i32>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksGameCustomizationDTO {
    pub category: String,
    pub content: String,
    pub queue_type: u64,
    pub is_teambuilder: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksGameDataChampionSummary {
    pub id: i32,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksGameflowGameData {
    pub queue: LolPerksQueue,
    pub is_custom_game: bool,
    pub game_id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksGameflowSession {
    pub phase: LolPerksGameflowPhase,
    pub game_data: LolPerksGameflowGameData,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksGetGameCustomizationDTO {
    pub queue_type: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksInventoryRunePageCount {
    pub quantity: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksLoginSession {
    pub state: LolPerksLoginSessionState,
    pub summoner_id: u64,
    pub account_id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksNamecheckAuthorization {
    pub subject: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksNamecheckLoginDataPacket {
    pub platform_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksNamecheckPayload {
    pub name: String,
    pub name_validation_context: String,
    pub puuid: String,
    pub shard: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksNamecheckResponse {
    pub errors: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksPerkBook {
    pub current_page_id: i32,
    pub pages: Vec<LolPerksPerkPageResource>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksPerkGDSResource {
    pub icon_path: String,
    pub id: i32,
    pub long_desc: String,
    pub name: String,
    pub short_desc: String,
    pub tooltip: String,
    pub recommendation_descriptor: String,
    pub major_change_patch_version: String,
    pub recommendation_descriptor_attributes: HashMap<String, u32>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksPerkIdListResource {
    pub perk_ids: Vec<i32>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksPerkPageResource {
    pub current: bool,
    pub id: i32,
    pub is_active: bool,
    pub is_valid: bool,
    pub is_editable: bool,
    pub is_deletable: bool,
    pub is_temporary: bool,
    pub name: String,
    pub order: i32,
    pub primary_style_id: i32,
    pub selected_perk_ids: Vec<i32>,
    pub sub_style_id: i32,
    pub auto_modified_selections: Vec<u32>,
    pub last_modified: u64,
    pub rune_recommendation_id: String,
    pub recommendation_index: i32,
    pub is_recommendation_override: bool,
    pub recommendation_champion_id: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksPerkSettingResource {
    pub perk_ids: Vec<i32>,
    pub perk_style: i32,
    pub perk_sub_style: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksPerkSettings {
    pub pages: Vec<LolPerksPerkPageResource>,
    pub per_shard_perk_books: HashMap<String, LolPerksPerkBook>,
    pub settings: LolPerksUISettings,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksPerkStyleResource {
    pub allowed_sub_styles: Vec<i32>,
    pub icon_path: String,
    pub asset_map: HashMap<String, String>,
    pub is_advanced: bool,
    pub id: i32,
    pub name: String,
    pub slots: Vec<LolPerksPerkStyleSlotResource>,
    pub sub_style_bonus: Vec<LolPerksPerkSubStyleBonusResource>,
    pub tooltip: String,
    pub default_sub_style: i32,
    pub default_perks: Vec<i32>,
    pub default_page_name: String,
    pub default_perks_when_splashed: Vec<i32>,
    pub default_stat_mods_per_sub_style: Vec<LolPerksDefaultStatModsPerSubStyle>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksPerkStyleSlotResource {
    pub perks: Vec<i32>,
    #[serde(rename = "type")]
    pub type_: String,
    pub slot_label: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksPerkSubStyleBonusResource {
    pub perk_id: i32,
    pub style_id: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksPerkUIPerk {
    pub icon_path: String,
    pub id: i32,
    pub style_id: i32,
    pub style_id_name: String,
    pub long_desc: String,
    pub name: String,
    pub short_desc: String,
    pub tooltip: String,
    pub recommendation_descriptor: String,
    pub slot_type: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksPerkUIRecommendedPage {
    pub position: String,
    pub is_default_position: bool,
    pub keystone: LolPerksPerkUIPerk,
    pub perks: Vec<LolPerksPerkUIPerk>,
    pub primary_perk_style_id: i32,
    pub secondary_perk_style_id: i32,
    pub primary_recommendation_attribute: String,
    pub secondary_recommendation_attribute: String,
    pub summoner_spell_ids: Vec<i32>,
    pub recommendation_id: String,
    pub is_recommendation_override: bool,
    pub recommendation_champion_id: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksPerkUISlot {
    pub perks: Vec<i32>,
    #[serde(rename = "type")]
    pub type_: String,
    pub slot_label: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksPerkUIStyle {
    pub allowed_sub_styles: Vec<i32>,
    pub icon_path: String,
    pub asset_map: HashMap<String, String>,
    pub id: i32,
    pub name: String,
    pub slots: Vec<LolPerksPerkUISlot>,
    pub sub_style_bonus: Vec<LolPerksPerkSubStyleBonusResource>,
    pub tooltip: String,
    pub default_sub_style: i32,
    pub default_perks: Vec<i32>,
    pub default_page_name: String,
    pub id_name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksPerksConfigDTO {
    pub styles: Vec<LolPerksPerkStyleResource>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksPlatformConfig {
    pub perks_enabled: bool,
    pub auto_repair_pages_enabled: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksPlayerInventory {
    pub owned_page_count: u32,
    pub custom_page_count: u32,
    pub can_add_custom_page: bool,
    pub is_custom_page_creation_unlocked: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksQueue {
    pub id: i32,
    pub map_id: i32,
    pub is_team_builder_managed: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksRuneRecommendationGDSResource {
    pub position: String,
    pub map_id: i32,
    pub is_default_position: bool,
    pub min_summoner_level: u32,
    pub perk_ids: Vec<i32>,
    pub primary_perk_style_id: i32,
    pub secondary_perk_style_id: i32,
    pub summoner_spell_ids: Vec<i32>,
    pub recommendation_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksSettingsStorageContainer {
    pub data: LolPerksPerkSettings,
    pub schema_version: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksSummoner {
    pub summoner_id: u64,
    pub account_id: u64,
    pub display_name: String,
    pub internal_name: String,
    pub profile_icon_id: i32,
    pub summoner_level: u32,
    pub xp_since_last_level: u64,
    pub xp_until_next_level: u64,
    pub percent_complete_for_next_level: u32,
    pub reroll_points: LolPerksSummonerRerollPoints,
    pub puuid: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksSummonerRerollPoints {
    pub points_to_reroll: u32,
    pub current_points: u32,
    pub number_of_rolls: u32,
    pub max_rolls: u32,
    pub points_cost_to_roll: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksUISettings {
    pub show_long_descriptions: bool,
    pub grid_mode_enabled: bool,
    pub show_preset_pages: bool,
    pub gameplay_patch_version_seen: String,
    pub gameplay_updated_perks_seen: Vec<i32>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksUpdatePageOrderRequest {
    pub target_page_id: i32,
    pub destination_page_id: i32,
    pub offset: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksValidateItemSetNameResponse {
    pub success: bool,
    pub name_check_response: LolPerksNamecheckResponse,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksValidatePageNameData {
    pub id: i32,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPftGameClientEndOfGameStats {
    pub game_id: u64,
    pub game_mode: String,
    pub stats_block: Value,
    pub queue_id: i32,
    pub is_ranked: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPftGameflowGameDodge {
    pub state: LolPftGameflowGameDodgeState,
    pub dodge_ids: Vec<u64>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPftGameflowSession {
    pub phase: LolPftGameflowPhase,
    pub game_dodge: LolPftGameflowGameDodge,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPftLoginSession {
    pub state: LolPftLoginSessionStates,
    pub id_token: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPftPFTEndOfGamePlayer {
    pub stats: Value,
    pub items: Vec<i32>,
    pub bot_player: bool,
    pub champion_id: i32,
    pub game_id: u64,
    pub leaver: bool,
    pub leaves: i32,
    pub level: i32,
    pub losses: i32,
    pub profile_icon_id: i32,
    pub spell1_id: i32,
    pub spell2_id: i32,
    pub summoner_name: String,
    pub team_id: i32,
    pub wins: i32,
    pub summoner_id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPftPFTEndOfGamePoints {
    pub point_change_from_champions_owned: i32,
    pub point_change_from_gameplay: i32,
    pub points_used: i32,
    pub previous_points: i32,
    pub points_until_next_reroll: i32,
    pub reroll_count: i32,
    pub total_points: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPftPFTEndOfGameStats {
    pub difficulty: String,
    pub game_id: u64,
    pub game_length: i32,
    pub game_mode: String,
    pub game_mutators: Vec<String>,
    pub game_type: String,
    pub invalid: bool,
    pub queue_type: String,
    pub ranked: bool,
    pub report_game_id: u64,
    pub teams: Vec<LolPftPFTEndOfGameTeam>,
    pub local_player: LolPftPFTEndOfGamePlayer,
    pub my_team_status: String,
    pub leveled_up: bool,
    pub new_spells: Vec<i32>,
    pub previous_level: u64,
    pub rp_earned: i32,
    pub account_id: u64,
    pub base_points: i32,
    pub battle_boost_ip_earned: i32,
    pub boost_ip_earned: i32,
    pub first_win_bonus: i32,
    pub ip_earned: i32,
    pub ip_total: i32,
    pub boost_xp_earned: i32,
    pub experience_earned: i32,
    pub experience_total: i32,
    pub loyalty_boost_xp_earned: i32,
    pub previous_xp_total: u64,
    pub time_until_next_first_win_bonus: i32,
    pub caused_early_surrender: bool,
    pub early_surrender_accomplice: bool,
    pub team_early_surrendered: bool,
    pub game_ended_in_early_surrender: bool,
    pub is_aram_game: bool,
    pub reroll_data: LolPftPFTEndOfGamePoints,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPftPFTEndOfGameTeam {
    pub stats: Value,
    pub players: Vec<LolPftPFTEndOfGamePlayer>,
    pub member_status_string: String,
    pub name: String,
    pub tag: String,
    pub full_id: String,
    pub team_id: i32,
    pub is_bottom_team: bool,
    pub is_player_team: bool,
    pub is_winning_team: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPftPFTEvent {
    pub player_survey_id: u64,
    pub action: String,
    pub data: Vec<Value>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPftPFTMetadata {
    pub account_id: u64,
    pub env: String,
    pub web_region: String,
    pub locale: String,
    pub app_name: String,
    pub app_version: String,
    pub system_os: String,
    pub stats: LolPftPFTEndOfGameStats,
    pub homepage_timer: f64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPftPFTQuestionResponse {
    pub question_id: u64,
    pub response_data: Value,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPftPFTSurvey {
    pub id: u64,
    pub title: String,
    pub caption: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub display: String,
    pub data: HashMap<String, Value>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPftPFTSurveyResults {
    pub question_responses: Vec<LolPftPFTQuestionResponse>,
    pub actions: Vec<LolPftPFTEvent>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPftPFTSurveyV1 {
    pub id: u64,
    pub title: String,
    pub caption: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub url: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPftSummoner {
    pub puuid: String,
    pub summoner_id: u64,
    pub account_id: u64,
    pub summoner_level: u32,
    pub unnamed: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerBehaviorBanNotification {
    pub id: u64,
    pub source: LolPlayerBehaviorNotificationSource,
    pub reason: String,
    pub time_until_ban_expires: u64,
    pub is_perma_ban: bool,
    pub display_reform_card: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerBehaviorCodeOfConductNotification {
    pub message: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerBehaviorPlayerBehaviorConfig {
    pub is_loaded: bool,
    pub code_of_conduct_enabled: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerBehaviorPlayerBehaviorGameflowSessionResource {
    pub phase: LolPlayerBehaviorGameflowPhase,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerBehaviorPlayerBehaviorSimpleMessage {
    pub account_id: u64,
    pub msg_id: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub params: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerBehaviorPlayerNotificationResource {
    pub background_url: String,
    pub created: String,
    pub critical: bool,
    pub data: HashMap<String, String>,
    pub detail_key: String,
    pub expires: String,
    pub icon_url: String,
    pub id: u64,
    pub source: String,
    pub state: String,
    pub title_key: String,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerBehaviorReformCard {
    pub id: u64,
    pub punishment_type: String,
    pub reason: String,
    pub time_when_punishment_expires: u64,
    pub punishment_length_time: u64,
    pub punishment_length_games: i64,
    pub restricted_chat_games_remaining: i64,
    pub chat_logs: Vec<String>,
    pub game_ids: Vec<u64>,
    pub player_facing_message: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerBehaviorReformCardChatLogs {
    pub pre_game_chat_logs: Vec<String>,
    pub in_game_chat_logs: Vec<String>,
    pub post_game_chat_logs: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerBehaviorReformCardV2 {
    pub id: u64,
    pub punishment_type: String,
    pub punishment_reason: String,
    pub punished_until_date_millis: u64,
    pub punishment_length_millis: u64,
    pub punishment_length_games: i64,
    pub punished_for_reform_card_chat_logs: Vec<LolPlayerBehaviorReformCardChatLogs>,
    pub punished_for_game_ids: Vec<u64>,
    pub player_facing_message: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerBehaviorReporterFeedback {
    pub id: u64,
    pub account_id: u64,
    pub message_id: String,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerBehaviorReporterFeedbackMessage {
    pub title: String,
    pub message: String,
    pub category: String,
    pub locale: String,
    pub key: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerBehaviorRestrictionNotification {
    pub id: u64,
    pub source: LolPlayerBehaviorNotificationSource,
    pub games_remaining: i64,
    pub expiration_millis: u64,
    pub display_reform_card: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerBehaviorSettingsResource {
    pub data: Value,
    pub schema_version: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerBehaviorUserInfo {
    pub user_info: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerBehaviorUserInfoBanData {
    pub restrictions: Vec<LolPlayerBehaviorUserInfoRestriction>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerBehaviorUserInfoRestriction {
    #[serde(rename = "type")]
    pub type_: String,
    pub reason: String,
    pub scope: String,
    pub dat: LolPlayerBehaviorUserInfoRestrictionData,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerBehaviorUserInfoRestrictionData {
    pub expiration_millis: u64,
    pub game_data: LolPlayerBehaviorUserInfoRestrictionGameData,
    pub game_location: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerBehaviorUserInfoRestrictionGameData {
    pub product_name: String,
    pub game_location: String,
    pub trigger_game_id: String,
    pub additional_game_ids: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerBehaviorUserInfoToken {
    pub ban: LolPlayerBehaviorUserInfoBanData,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerLevelUpEndOfGameStats {
    pub game_mode: String,
    pub game_mutators: Vec<String>,
    pub game_type: String,
    pub queue_type: String,
    pub leveled_up: bool,
    pub new_spells: Vec<i32>,
    pub previous_level: u32,
    pub rp_earned: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerLevelUpGameDataSummonerSpell {
    pub id: u64,
    pub summoner_level: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerLevelUpLoginSession {
    pub state: LolPlayerLevelUpLoginSessionStates,
    pub summoner_id: u64,
    pub account_id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerLevelUpPlayerLevelUpEvent {
    pub switched_to_standard_free_to_play_champ_rotation: bool,
    pub now_has_access_to_public_chat_rooms: bool,
    pub now_has_access_to_loot: bool,
    pub leveled_up: bool,
    pub new_summoner_level: u32,
    pub new_rune_slot_unlocked: bool,
    pub rp_earned: i32,
    pub new_spells: Vec<u64>,
    pub new_queues: Vec<i32>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerLevelUpPlayerLevelUpEventAck {
    pub seen_this_event: bool,
    pub new_summoner_level: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerLevelUpQueue {
    pub id: i32,
    pub min_level: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerMessagingDynamicCelebrationMessagingNotificationResource {
    pub id: i32,
    pub account_id: u64,
    pub msg_id: String,
    pub celebration_title: String,
    pub celebration_body: String,
    pub celebration_message: String,
    pub inventory_type: String,
    pub item_id: String,
    pub item_quantity: String,
    pub celebration_type: String,
    pub status: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerMessagingLoginDataPacket {
    pub simple_messages: Vec<LolPlayerMessagingSimpleMessage>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerMessagingPlayerMessagingNotificationResource {
    pub id: i32,
    pub account_id: u64,
    pub msg_id: String,
    pub title: String,
    pub body: String,
    pub status: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerMessagingSimpleMessage {
    pub account_id: u64,
    pub msg_id: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub title_code: String,
    pub body_code: String,
    pub params: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerPreferencesLoginSession {
    pub state: LolPlayerPreferencesLoginSessionStates,
    pub summoner_id: u64,
    pub account_id: u64,
    pub id_token: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerPreferencesPlayerPreferences {
    #[serde(rename = "type")]
    pub type_: String,
    pub version: String,
    pub data: String,
    pub hash: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerPreferencesPlayerPreferencesEndpoint {
    pub enabled: bool,
    pub enforce_https: bool,
    pub service_endpoint: String,
    pub version: String,
    pub retries: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerReportSenderChampSelectReport {
    pub offender_puuid: String,
    pub categories: Vec<String>,
    pub location: String,
    pub comment: String,
    pub match_token: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerReportSenderChampSelectSummonerInfo {
    pub puuid: String,
    pub obfuscated_puuid: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerReportSenderEndOfGamePlayerReport {
    pub reported_puuid: String,
    pub game_id: u64,
    pub offense: String,
    pub comment: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerReportSenderGameAgnosticReportPayload {
    pub offender_id: String,
    pub categories: Vec<String>,
    pub location: String,
    pub comment: String,
    pub token_type: String,
    pub token: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPreEndOfGameSequenceEvent {
    pub name: String,
    pub priority: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPremadeVoiceAccountSettingsCategoryDataResource {
    pub auto_join: bool,
    pub mute_on_connect: bool,
    pub input_mode: LolPremadeVoiceInputMode,
    pub push_to_talk_key: String,
    pub show_first_experience_in_l_c_u: bool,
    pub show_first_experience_in_game: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPremadeVoiceAccountSettingsCategoryResource {
    pub data: Option<LolPremadeVoiceAccountSettingsCategoryDataResource>,
    pub schema_version: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPremadeVoiceAudioPropertiesResource {
    pub is_loopback_enabled: bool,
    pub mic_energy: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPremadeVoiceConfigStatus {
    pub readiness: LolPremadeVoiceConfigReadinessEnum,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPremadeVoiceDeviceResource {
    pub handle: String,
    pub name: String,
    pub usable: bool,
    pub is_current_device: bool,
    pub is_default: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPremadeVoiceDeviceResourceRiotClient {
    pub handle: String,
    pub name: String,
    pub is_effective_device: bool,
    pub is_current_device: bool,
    pub is_default: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPremadeVoiceEntitlementsToken {
    pub entitlements: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPremadeVoiceExternalSession {
    pub patchline_full_name: String,
    pub patchline_id: String,
    pub product_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPremadeVoiceFirstExperience {
    pub show_first_experience_in_l_c_u: bool,
    pub show_first_experience_in_game: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPremadeVoiceGameEventHotkeys {
    pub evt_push_to_talk: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPremadeVoiceGameInputSettings {
    pub game_events: LolPremadeVoiceGameEventHotkeys,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPremadeVoiceGameflowGameClient {
    pub running: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPremadeVoiceGameflowSession {
    pub phase: LolPremadeVoiceGameflowPhase,
    pub game_client: LolPremadeVoiceGameflowGameClient,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPremadeVoiceKeyCombo {
    pub key_bindings: Vec<LolPremadeVoicePushToTalkKey>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPremadeVoiceKeycodePushToTalkResource {
    pub enabled: bool,
    pub key_combos: Option<Vec<LolPremadeVoiceKeyCombo>>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPremadeVoiceLocalSettingsCategoryDataResource {
    pub current_capture_device_handle: String,
    pub input_volume: u32,
    pub vad_sensitivity: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPremadeVoiceLocalSettingsCategoryResource {
    pub data: Option<LolPremadeVoiceLocalSettingsCategoryDataResource>,
    pub schema_version: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPremadeVoiceLoginSession {
    pub connected: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPremadeVoiceParticipantResource {
    pub id: String,
    pub name: String,
    pub volume: u32,
    pub energy: u32,
    pub is_muted: bool,
    pub is_speaking: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPremadeVoicePartyDto {
    pub party_id: String,
    pub comms_enabled: bool,
    pub players: HashMap<String, LolPremadeVoicePlayerDto>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPremadeVoicePlayerDto {
    pub display_name: String,
    pub puuid: String,
    pub party_id: String,
    pub summoner_id: u64,
    pub role: LolPremadeVoicePartyMemberRoleEnum,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPremadeVoicePremadeVoiceParticipantDto {
    pub participant_id: String,
    pub summoner_id: u64,
    pub puuid: String,
    pub display_name: String,
    pub volume: u32,
    pub energy: u32,
    pub is_muted: bool,
    pub is_speaking: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPremadeVoicePushToTalkKey {
    pub key: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPremadeVoicePushToTalkResource {
    pub ptt_enabled: bool,
    pub ptt_key_binding: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPremadeVoiceSessionResource {
    pub id: String,
    pub status: LolPremadeVoiceSessionStatus,
    pub participants: Vec<LolPremadeVoiceParticipantResource>,
    pub volume: u32,
    pub is_muted: bool,
    pub is_transmit_enabled: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPremadeVoiceSettingsResource {
    pub current_capture_device_handle: String,
    pub vad_hangover_time: u32,
    pub vad_sensitivity: u32,
    pub mic_level: u32,
    pub local_mic_muted: bool,
    pub loopback_enabled: bool,
    pub auto_join: bool,
    pub mute_on_connect: bool,
    pub vad_active: bool,
    pub ptt_active: bool,
    pub input_mode: LolPremadeVoiceInputMode,
    pub ptt_key: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPremadeVoiceStateResource {
    pub connected: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPremadeVoiceSummoner {
    pub account_id: u64,
    pub puuid: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPremadeVoiceVoiceAvailability {
    pub enabled: bool,
    pub connected_to_voice_server: bool,
    pub voice_channel_available: bool,
    pub disabled_after_login: bool,
    pub show_u_i: bool,
    pub show_disconnected_state: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolProgressionCounter {
    pub id: String,
    pub name: String,
    pub group_id: String,
    pub direction: String,
    pub start_value: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolProgressionCounterInstance {
    pub owner_id: String,
    pub product_id: String,
    pub group_id: String,
    pub counter_id: String,
    pub counter_value: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolProgressionEntityInstance {
    pub group_id: String,
    pub counters: Vec<LolProgressionCounterInstance>,
    pub milestones: Vec<LolProgressionMilestoneInstance>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolProgressionGroup {
    pub id: String,
    pub product_id: String,
    pub name: String,
    pub repeat: LolProgressionRepeat,
    pub counters: Vec<LolProgressionCounter>,
    pub milestones: Vec<LolProgressionMilestone>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolProgressionMilestone {
    pub id: String,
    pub name: String,
    pub group_id: String,
    pub counter_id: String,
    pub trigger_value: i64,
    pub properties: HashMap<String, String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolProgressionMilestoneInstance {
    pub milestone_id: String,
    pub owner_id: String,
    pub product_id: String,
    pub group_id: String,
    pub counter_id: String,
    pub trigger_value: i64,
    pub repeat_sequence: u32,
    pub triggered: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolProgressionRepeat {
    pub count: i32,
    pub scope: u32,
    pub multiplier: f32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPublishingContentAccountData {
    pub puuid: String,
    pub account_id: String,
    pub summoner_level: String,
    pub display_name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPublishingContentBuildInfo {
    pub version: String,
    pub patchline: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPublishingContentClientData {
    pub puuid: String,
    pub account_id: u64,
    pub env: String,
    pub web_region: String,
    pub locale: String,
    pub summoner_level: u16,
    pub summoner_name: String,
    pub app_name: String,
    pub app_version: String,
    pub system_os: String,
    pub protocol: String,
    pub port: u16,
    pub asset_urls: HashMap<String, String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPublishingContentPubHubConfig {
    pub edge: LolPublishingContentPubHubConfigEdge,
    pub app_context: LolPublishingContentPubHubConfigAppContext,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPublishingContentPubHubConfigAppContext {
    pub user_id: String,
    pub user_region: String,
    pub device_category: String,
    pub device_operating_system: String,
    pub device_operating_system_version: String,
    pub app_id: String,
    pub app_version: String,
    pub app_locale: String,
    pub app_language: String,
    pub publishing_locale: String,
    pub app_session_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPublishingContentPubHubConfigEdge {
    pub client_id: String,
    pub client_region: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPublishingContentPublishingLocaleSetting {
    pub data: LolPublishingContentPublishingLocaleSettingData,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPublishingContentPublishingLocaleSettingData {
    pub publishing_locale: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPublishingContentPublishingSettings {
    pub region: String,
    pub locale: String,
    pub web_region: String,
    pub web_locale: String,
    pub publishing_locale: String,
    pub rso_platform_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPublishingContentRegionLocale {
    pub region: String,
    pub locale: String,
    pub web_region: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPublishingContentSummonerInfo {
    pub display_name: String,
    pub summoner_level: u16,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPublishingContentSystemInfo {
    pub operating_system: LolPublishingContentSystemInfoOperatingSystem,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPublishingContentSystemInfoOperatingSystem {
    pub platform: String,
    pub version_major: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetBalance {
    pub currency_type: String,
    pub amount: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetBaseSkinLineDto {
    pub items: Vec<LolPurchaseWidgetSkinLineItemDto>,
    pub localized_name: String,
    pub skin_line_descriptions: Vec<LolPurchaseWidgetSkinLineDescriptionDto>,
    pub pricing_options: Vec<LolPurchaseWidgetPriceOptionDto>,
    pub splash_path: String,
    pub uncentered_splash_path: String,
    pub collection_card_path: String,
    pub collection_description: String,
    pub tile_path: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetBundledItemPricingInfo {
    pub discount_prices: Vec<LolPurchaseWidgetDiscountPricingInfo>,
    pub inventory_type: String,
    pub item_id: i32,
    pub quantity: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetCapOffer {
    pub id: String,
    pub type_id: String,
    pub label: String,
    pub product_id: String,
    pub merchant_id: String,
    pub payload: Vec<LolPurchaseWidgetCapOfferPayloadEntry>,
    pub active: bool,
    pub start_date: String,
    pub created_date: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetCapOfferPayloadEntry {
    pub item_price_map: HashMap<String, i32>,
    pub item_instance_id: String,
    pub fulfillment_type_id: String,
    pub inventory_type_u_u_i_d: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetCapOrdersDataDto {
    pub id: String,
    pub sub_orders: Vec<LolPurchaseWidgetCapOrdersSubOrderDto>,
    pub purchaser: LolPurchaseWidgetCapOrdersTypedIdentifierDto,
    pub location: String,
    pub source: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetCapOrdersMetaDto {
    pub xid: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetCapOrdersOfferContextDto {
    pub quantity: u32,
    pub payment_option: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetCapOrdersOfferDto {
    pub id: String,
    pub product_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetCapOrdersOrderDto {
    pub data: LolPurchaseWidgetCapOrdersDataDto,
    pub meta: LolPurchaseWidgetCapOrdersMetaDto,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetCapOrdersSubOrderDto {
    pub recipient_id: String,
    pub offer_context: LolPurchaseWidgetCapOrdersOfferContextDto,
    pub offer: LolPurchaseWidgetCapOrdersOfferDto,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetCapOrdersTypedIdentifierDto {
    pub id: String,
    pub type_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetCatalogPluginItem {
    pub item_id: i32,
    pub item_instance_id: String,
    pub owned: bool,
    pub inventory_type: String,
    pub sub_inventory_type: String,
    pub name: String,
    pub sub_title: String,
    pub description: String,
    pub image_path: String,
    pub purchase_date: u64,
    pub release_date: u64,
    pub inactive_date: u64,
    pub prices: Vec<LolPurchaseWidgetCatalogPluginPrice>,
    pub tags: Option<Vec<String>>,
    pub metadata: Option<Vec<LolPurchaseWidgetItemMetadataEntry>>,
    pub quest_skin_info: Option<LolPurchaseWidgetSkinLineInfo>,
    pub active: bool,
    pub ownership_type: Option<LolPurchaseWidgetInventoryOwnership>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetCatalogPluginItemAssets {
    pub splash_path: String,
    pub icon_path: String,
    pub tile_path: String,
    pub emblems: Vec<LolPurchaseWidgetChampionSkinEmblem>,
    pub colors: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetCatalogPluginItemWithDetails {
    pub item: LolPurchaseWidgetCatalogPluginItem,
    pub quantity: u32,
    pub required_items: Option<Vec<LolPurchaseWidgetCatalogPluginItemWithDetails>>,
    pub bundled_items: Option<Vec<LolPurchaseWidgetCatalogPluginItemWithDetails>>,
    pub minimum_bundle_prices: Option<Vec<LolPurchaseWidgetCatalogPluginPrice>>,
    pub bundled_discount_prices: Option<Vec<LolPurchaseWidgetCatalogPluginPrice>>,
    pub assets: LolPurchaseWidgetCatalogPluginItemAssets,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetCatalogPluginPrice {
    pub currency: String,
    pub cost: i64,
    pub cost_type: Option<String>,
    pub sale: Option<LolPurchaseWidgetCatalogPluginSale>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetCatalogPluginSale {
    pub start_date: String,
    pub end_date: String,
    pub discount: Option<f32>,
    pub cost: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetChampionSkinEmblem {
    pub name: String,
    pub emblem_path: LolPurchaseWidgetChampionSkinEmblemPath,
    pub emblem_position: LolPurchaseWidgetChampionSkinEmblemPosition,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetChampionSkinEmblemPath {
    pub large: String,
    pub small: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetChampionSkinEmblemPosition {
    pub vertical: String,
    pub horizontal: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetDiscountPricingInfo {
    pub cost: i32,
    pub cost_type: String,
    pub currency: String,
    pub discount: f32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetItemChoiceDetails {
    pub item: LolPurchaseWidgetCatalogPluginItem,
    pub background_image: String,
    pub contents: Vec<LolPurchaseWidgetItemDetails>,
    pub discount: String,
    pub full_price: u32,
    pub display_type: String,
    pub purchase_options: Vec<LolPurchaseWidgetPurchaseOption>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetItemChoices {
    pub choices: Vec<LolPurchaseWidgetItemChoiceDetails>,
    pub validation_errors: Vec<LolPurchaseWidgetValidationErrorEntry>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetItemCost {
    pub currency: String,
    pub cost: i64,
    pub discount: Option<f32>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetItemDefinition {
    pub item_id: i32,
    pub inventory_type: String,
    pub sub_inventory_type: String,
    pub name: String,
    pub description: String,
    pub sub_title: String,
    pub owned: bool,
    pub assets: LolPurchaseWidgetCatalogPluginItemAssets,
    pub tags: Vec<String>,
    pub metadata: Vec<LolPurchaseWidgetItemMetadataEntry>,
    pub bundled_item_price: Option<LolPurchaseWidgetBundledItemPricingInfo>,
    pub loyalty_unlocked: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetItemDetails {
    pub title: String,
    pub sub_title: String,
    pub description: String,
    pub icon_url: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetItemKey {
    pub inventory_type: String,
    pub item_id: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetItemMetadataEntry {
    #[serde(rename = "type")]
    pub type_: String,
    pub value: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetItemOwnership {
    pub item_key: LolPurchaseWidgetItemKey,
    pub quantity: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetItemPrice {
    pub currency_type: String,
    pub price: i64,
    pub purchasable: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetItemSale {
    pub start_date: String,
    pub end_date: String,
    pub discount: Option<f32>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetLoginSession {
    pub puuid: Option<String>,
    pub state: LolPurchaseWidgetLoginSessionStates,
    pub summoner_id: u64,
    pub account_id: u64,
    pub id_token: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetOrderNotificationResource {
    pub event_type_id: String,
    pub event_type: String,
    pub status: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetPriceDetail {
    pub item_key: LolPurchaseWidgetItemKey,
    pub price: LolPurchaseWidgetItemPrice,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetPriceOptionDto {
    pub price: i64,
    pub currency_type: String,
    pub currency_payment_option: Option<String>,
    pub currency_name: Option<String>,
    pub currency_image_path: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetPurchasableItem {
    pub item: LolPurchaseWidgetItemDefinition,
    pub dependencies: Vec<LolPurchaseWidgetItemDefinition>,
    pub bundled_items: Vec<LolPurchaseWidgetItemDefinition>,
    pub sale: Option<LolPurchaseWidgetItemSale>,
    pub purchase_options: Vec<LolPurchaseWidgetPurchaseOption>,
    pub validation_errors: Vec<LolPurchaseWidgetValidationErrorEntry>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetPurchaseItem {
    pub item_key: LolPurchaseWidgetItemKey,
    pub quantity: i32,
    pub source: String,
    pub purchase_currency_info: LolPurchaseWidgetItemPrice,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetPurchaseOfferOrderStatus {
    pub order_state: LolPurchaseWidgetPurchaseOfferOrderStates,
    pub message: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetPurchaseOfferOrderStatuses {
    pub statuses: HashMap<String, LolPurchaseWidgetPurchaseOfferOrderStatus>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetPurchaseOfferRequestV3 {
    pub offer_id: String,
    pub currency_type: String,
    pub quantity: u32,
    pub price: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetPurchaseOfferResponseV3 {
    pub legacy: bool,
    pub order_dto: Option<LolPurchaseWidgetCapOrdersOrderDto>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetPurchaseOption {
    pub price_details: Vec<LolPurchaseWidgetPriceDetail>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetPurchaseRequest {
    pub items: Vec<LolPurchaseWidgetPurchaseItem>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetPurchaseResponse {
    pub items: Vec<LolPurchaseWidgetPurchaseItem>,
    pub transactions: Vec<LolPurchaseWidgetTransaction>,
    pub use_r_m_s_confirmation: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetPurchaseWidgetConfig {
    pub enabled: bool,
    pub non_refundable_disclaimer_enabled: bool,
    pub always_show_purchase_disclaimer: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetRiotMessagingServiceMessage {
    pub resource: String,
    pub service: String,
    pub version: String,
    pub timestamp: i64,
    pub payload: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetSale {
    pub start_date: String,
    pub end_date: String,
    pub prices: Vec<LolPurchaseWidgetItemCost>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetSkinLineDescriptionDto {
    pub title: String,
    pub description: String,
    pub icon_image_path: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetSkinLineDescriptionInfo {
    pub title: String,
    pub description: String,
    pub icon_path: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetSkinLineInfo {
    pub name: String,
    pub description_info: Vec<LolPurchaseWidgetSkinLineDescriptionInfo>,
    pub splash_path: String,
    pub tile_path: String,
    pub collection_card_path: String,
    pub uncentered_splash_path: String,
    pub collection_description: String,
    pub tiers: Vec<LolPurchaseWidgetSkinLineTier>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetSkinLineItemDto {
    pub thumbnail_image_path: String,
    pub large_image_path: Option<String>,
    pub localized_long_name: String,
    pub localized_short_name: String,
    pub large_video_path: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetSkinLineTier {
    pub id: i64,
    pub name: String,
    pub stage: i64,
    pub description: Option<String>,
    pub splash_path: String,
    pub uncentered_splash_path: String,
    pub tile_path: String,
    pub load_screen_path: String,
    pub short_name: String,
    pub splash_video_path: Option<String>,
    pub collection_splash_video_path: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetTransaction {
    pub transaction_id: String,
    pub item_key: LolPurchaseWidgetItemKey,
    pub item_name: String,
    pub icon_url: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetValidateOfferError {
    pub error_key: String,
    pub meta: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetValidateOfferRequestV3 {
    pub offer_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetValidateOfferResponseV3 {
    pub validation_errors: Vec<LolPurchaseWidgetValidateOfferError>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetValidationError {
    pub error_code: String,
    pub message: String,
    pub error_details: HashMap<String, String>,
    pub response_items: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetValidationErrorEntry {
    pub id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetValidationRequest {
    pub items: Vec<LolPurchaseWidgetValidationRequestItem>,
    pub owned_items: Vec<LolPurchaseWidgetItemOwnership>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetValidationRequestItem {
    pub item_key: LolPurchaseWidgetItemKey,
    pub quantity: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetValidationResponse {
    pub items: Vec<LolPurchaseWidgetValidationResponseItem>,
    pub valid: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetValidationResponseItem {
    pub item_key: LolPurchaseWidgetItemKey,
    pub quantity: i32,
    pub validation_currency_info: Vec<LolPurchaseWidgetItemPrice>,
    pub sale: Option<LolPurchaseWidgetSale>,
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetWallet {
    pub account_id: u64,
    pub balances: Vec<LolPurchaseWidgetBalance>,
    pub version: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedAchievedTier {
    pub queue_type: LolRankedLeagueQueueType,
    pub tier: LolRankedLeagueTier,
    pub division: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedEndOfGameStatsBlock {
    pub game_id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedEosNotificationResource {
    pub notification_name: String,
    pub notification_type: String,
    pub season_end_time: i64,
    pub queue: String,
    pub tier: String,
    pub division: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedEosNotificationsConfig {
    pub config: Vec<LolRankedEosNotificationsConfigEntry>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedEosNotificationsConfigEntry {
    pub name: String,
    pub offset_time1: i64,
    pub offset_time2: i64,
    pub offset_time3: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedEosRewardData {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub override_image_path: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedEosRewardGroupsConfig {
    pub reward_groups: HashMap<String, LolRankedEosRewardGroupsRewardsList>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedEosRewardGroupsRewardsList {
    pub rewards: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedEosRewardsConfig {
    pub seasons: HashMap<String, LolRankedEosRewardsConfigEntry>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedEosRewardsConfigEntry {
    pub rewards: HashMap<String, LolRankedEosRewardData>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedEosSettingsData {
    pub notification_shown: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedEosSettingsResource {
    pub data: LolRankedEosSettingsData,
    pub schema_version: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedGameflowGameData {
    pub queue: LolRankedQueue,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedGameflowSession {
    pub phase: LolRankedGameflowPhase,
    pub game_data: LolRankedGameflowGameData,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedGlobalNotification {
    pub notify_reason: String,
    pub participant_id: String,
    pub queue_type: String,
    pub tier: String,
    pub position: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedLcuLeagueNotification {
    pub id: u64,
    pub msg_id: String,
    pub display_type: LolRankedNotificationDisplayType,
    pub notify_reason: String,
    pub change_reason: String,
    pub queue_type: LolRankedLeagueQueueType,
    pub game_id: u64,
    pub provisional_games_remaining: i32,
    pub tier: LolRankedLeagueTier,
    pub division: LolRankedLeagueDivision,
    pub number_of_promotions: u32,
    pub miniseries_progress: String,
    pub league_points: i32,
    pub league_points_delta: i32,
    pub rated_tier: LolRankedRatedTier,
    pub rated_rating: i32,
    pub rated_rating_delta: i32,
    pub eligible_for_promo_helper: bool,
    pub miniseries_wins: i32,
    pub time_until_inactivity_status_changes: i64,
    pub reward_earned_id: String,
    pub reward_earned_type: String,
    pub reward_override_image_path: String,
    pub split_points_notification: Option<LolRankedSplitPointsNotification>,
    pub promo_series_for_ranks_enabled: bool,
    pub consolation_lp_used: i32,
    pub afk_lp_penalty_amount: i32,
    pub afk_lp_penalty_level: i32,
    pub was_afk_or_leaver: bool,
    pub can_demote_from_tier: bool,
    pub win_streak: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedLeagueDivisionInfo {
    pub tier: LolRankedLeagueTier,
    pub division: LolRankedLeagueDivision,
    pub max_league_size: i32,
    pub apex_unlock_time_millis: i64,
    pub min_lp_for_apex_tier: i32,
    pub top_number_of_players: i64,
    pub standings: Vec<LolRankedLeagueStanding>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedLeagueLadderDTO {
    pub queue_type: String,
    pub tier: String,
    pub provisional_game_threshold: i32,
    pub entries: Vec<LolRankedLeagueLadderEntryDTO>,
    pub max_league_size: i32,
    pub next_apex_update: i64,
    pub apex_unlock_time_millis: i64,
    pub min_lp_for_tier: i32,
    pub top_number_of_players: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedLeagueLadderEntryDTO {
    pub summoner_id: u64,
    pub puuid: String,
    pub summoner_name: String,
    pub tier: String,
    pub rank: String,
    pub league_points: i32,
    pub mini_series_progress: String,
    pub wins: i32,
    pub losses: i32,
    pub provisional_games_remaining: i32,
    pub previous_day_league_position: i32,
    pub previous_season_achieved_tier: String,
    pub previous_season_achieved_rank: String,
    pub earned_regalia_reward_ids: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedLeagueLadderInfo {
    pub queue_type: LolRankedLeagueQueueType,
    pub tier: LolRankedLeagueTier,
    pub provisional_game_threshold: i32,
    pub divisions: Vec<LolRankedLeagueDivisionInfo>,
    pub next_apex_update_millis: i64,
    pub requested_ranked_entry: Option<LolRankedLeagueStanding>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedLeagueNotification {
    pub notify_reason: String,
    pub change_reason: String,
    pub queue_type: String,
    pub game_id: u64,
    pub provisional_games_remaining: i32,
    pub tier: String,
    pub rank: String,
    pub miniseries_progress: String,
    pub league_points: i32,
    pub league_points_delta: i32,
    pub split_points: i32,
    pub split_points_breakdown: HashMap<String, i32>,
    pub rated_tier: String,
    pub rated_rating: i32,
    pub rated_rating_delta: i32,
    pub eligible_for_promo_helper: Option<bool>,
    pub promo_series_for_ranks_enabled: bool,
    pub consolation_lp_used: i32,
    pub afk_lp_penalty_amount: i32,
    pub afk_lp_penalty_level: i32,
    pub was_afk_or_leaver: bool,
    pub can_demote_from_tier: bool,
    pub win_streak: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedLeagueNotifications {
    pub league_notifications: Vec<LolRankedLeagueNotification>,
    pub global_notifications: Vec<LolRankedGlobalNotification>,
    pub reward_notifications: Vec<LolRankedRewardNotification>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedLeagueStanding {
    pub summoner_id: u64,
    pub puuid: String,
    pub summoner_name: String,
    pub position: i32,
    pub position_delta: i32,
    pub previous_position: i32,
    pub tier: LolRankedLeagueTier,
    pub division: LolRankedLeagueDivision,
    pub league_points: i64,
    pub miniseries_results: Vec<LolRankedMiniseries>,
    pub wins: u64,
    pub losses: u64,
    pub provisional_games_remaining: i32,
    pub is_provisional: bool,
    pub previous_season_achieved_tier: LolRankedLeagueTier,
    pub previous_season_achieved_division: LolRankedLeagueDivision,
    pub earned_regalia_reward_ids: Vec<String>,
    pub ranked_regalia_level: i32,
    pub pending_promotion: bool,
    pub pending_demotion: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedLeagueTierAndRankDTO {
    pub player_or_team_id: String,
    pub player_or_team_name: String,
    pub queue_type: String,
    pub tier: String,
    pub rank: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedLeaguesSeasonRewardConfig {
    pub qualification_warning_enabled: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedLoginSession {
    pub state: LolRankedLoginSessionStates,
    pub summoner_id: u64,
    pub account_id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedParticipantTiers {
    pub summoner_id: u64,
    pub achieved_tiers: Vec<LolRankedAchievedTier>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedQueue {
    #[serde(rename = "type")]
    pub type_: Value,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedQueuesAndPuuidsPayload {
    pub queue_types: Vec<LolRankedLeagueQueueType>,
    pub summoner_ids: Vec<u64>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedRankedQueueStats {
    pub queue_type: LolRankedLeagueQueueType,
    pub provisional_game_threshold: i32,
    pub provisional_games_remaining: i32,
    pub is_provisional: bool,
    pub tier: LolRankedLeagueTier,
    pub division: LolRankedLeagueDivision,
    pub league_points: i32,
    pub mini_series_progress: String,
    pub rated_tier: LolRankedRatedTier,
    pub rated_rating: i32,
    pub wins: i32,
    pub losses: i32,
    pub highest_tier: LolRankedLeagueTier,
    pub highest_division: LolRankedLeagueDivision,
    pub previous_season_end_tier: LolRankedLeagueTier,
    pub previous_season_end_division: LolRankedLeagueDivision,
    pub previous_season_achieved_tier: LolRankedLeagueTier,
    pub previous_season_achieved_division: LolRankedLeagueDivision,
    pub warnings: Option<LolRankedRankedQueueWarnings>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedRankedQueueStatsDTO {
    pub queue_type: String,
    pub provisional_game_threshold: i32,
    pub provisional_games_remaining: i32,
    pub tier: String,
    pub rank: String,
    pub league_points: i32,
    pub mini_series_progress: String,
    pub rated_tier: String,
    pub rated_rating: i32,
    pub wins: i32,
    pub losses: i32,
    pub highest_tier: String,
    pub highest_rank: String,
    pub previous_season_end_tier: String,
    pub previous_season_end_rank: String,
    pub previous_season_achieved_tier: String,
    pub previous_season_achieved_rank: String,
    pub warnings: Option<LolRankedRankedQueueWarningsDTO>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedRankedQueueWarnings {
    pub display_decay_warning: bool,
    pub time_until_inactivity_status_changes: i64,
    pub demotion_warning: i32,
    pub days_until_decay: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedRankedQueueWarningsDTO {
    pub display_decay_warning: bool,
    pub time_until_inactivity_status_changes: i64,
    pub demotion_warning: i32,
    pub apex_days_until_decay: i32,
    pub days_until_decay: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedRankedStats {
    pub queues: Vec<LolRankedRankedQueueStats>,
    pub queue_map: HashMap<String, LolRankedRankedQueueStats>,
    pub highest_ranked_entry: Option<LolRankedRankedQueueStats>,
    pub highest_ranked_entry_s_r: Option<LolRankedRankedQueueStats>,
    pub earned_regalia_reward_ids: Vec<String>,
    pub ranked_regalia_level: i32,
    pub highest_current_season_reached_tier_s_r: LolRankedLeagueTier,
    pub highest_previous_season_end_tier: LolRankedLeagueTier,
    pub highest_previous_season_end_division: LolRankedLeagueDivision,
    pub highest_previous_season_achieved_tier: LolRankedLeagueTier,
    pub highest_previous_season_achieved_division: LolRankedLeagueDivision,
    pub splits_progress: HashMap<String, i32>,
    pub seasons: HashMap<String, LolRankedSeasonDTO>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedRankedStatsDTO {
    pub queues: Vec<LolRankedRankedQueueStatsDTO>,
    pub earned_regalia_reward_ids: Vec<String>,
    pub highest_previous_season_end_tier: String,
    pub highest_previous_season_end_rank: String,
    pub highest_previous_season_achieved_tier: String,
    pub highest_previous_season_achieved_rank: String,
    pub splits_progress: HashMap<String, i32>,
    pub seasons: HashMap<String, LolRankedSeasonDTO>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedRatedLadderEntryDTO {
    pub summoner_id: u64,
    pub puuid: String,
    pub summoner_name: String,
    pub rated_tier: String,
    pub rated_rating: i32,
    pub wins: i32,
    pub previous_update_ladder_position: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedRatedLadderInfo {
    pub queue_type: LolRankedLeagueQueueType,
    pub standings: Vec<LolRankedRatedLadderStanding>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedRatedLadderStanding {
    pub summoner_id: u64,
    pub puuid: String,
    pub summoner_name: String,
    pub rated_tier: LolRankedRatedTier,
    pub league_points: i32,
    pub wins: i32,
    pub position: i32,
    pub position_delta: i32,
    pub previous_position: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedRewardNotification {
    pub reward_group_id: String,
    pub season_id: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedRewardsInfo {
    pub splits: Vec<LolRankedSeasonSplit>,
    pub current_split: Option<LolRankedSeasonSplit>,
    pub reward_info_by_reward_id: HashMap<String, LolRankedSplitReward>,
    pub current_split_id: i32,
    pub current_season_id: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedSeasonDTO {
    pub current_season_id: i32,
    pub current_season_end: i64,
    pub next_season_start: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedSeasonSplit {
    pub split_id: i32,
    pub season_id: i32,
    pub start_time_millis: u64,
    pub end_time_millis: u64,
    pub reward_track: Vec<LolRankedSplitRewardGroup>,
    pub victorious_skin_reward_group: LolRankedVictoriousSkin,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedSeasonSplitDTO {
    pub split_id: i32,
    pub season_id: i32,
    pub start_time: u64,
    pub end_time: u64,
    pub reward_track: Vec<LolRankedSplitRewardGroupDTO>,
    pub victorious_skin_reward_group: LolRankedVictoriousSkinDTO,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedSequenceEvent {
    pub name: String,
    pub priority: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedSignedRankedStatsDTO {
    pub queues: Vec<LolRankedRankedQueueStatsDTO>,
    pub earned_regalia_reward_ids: Vec<String>,
    pub highest_previous_season_end_tier: String,
    pub highest_previous_season_end_rank: String,
    pub highest_previous_season_achieved_tier: String,
    pub highest_previous_season_achieved_rank: String,
    pub splits_progress: HashMap<String, i32>,
    pub seasons: HashMap<String, LolRankedSeasonDTO>,
    pub jwt: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedSocialLeaderboardRankedQueueStats {
    pub queue_type: LolRankedLeagueQueueType,
    pub provisional_game_threshold: i32,
    pub provisional_games_remaining: i32,
    pub is_provisional: bool,
    pub tier: LolRankedLeagueTier,
    pub division: LolRankedLeagueDivision,
    pub league_points: i32,
    pub mini_series_progress: String,
    pub rated_tier: LolRankedRatedTier,
    pub rated_rating: i32,
    pub wins: i32,
    pub losses: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedSocialLeaderboardRankedQueueStatsDTO {
    pub queue_type: String,
    pub provisional_game_threshold: i32,
    pub provisional_games_remaining: i32,
    pub tier: String,
    pub rank: String,
    pub league_points: i32,
    pub mini_series_progress: String,
    pub rated_tier: String,
    pub rated_rating: i32,
    pub wins: i32,
    pub losses: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedSplitPointsNotification {
    pub split_points_delta: i32,
    pub split_points_before_game: i32,
    pub split_points_after_game: i32,
    pub previous_split_points_required: i32,
    pub split_points_required: i32,
    pub next_reward_id: String,
    pub next_reward_type: String,
    pub split_points_breakdown: HashMap<String, i32>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedSplitReward {
    pub reward_type: String,
    pub quantity: i32,
    pub description: String,
    pub id: String,
    pub regalia_level: Option<i32>,
    pub points_required: i32,
    pub split_id: i32,
    pub champion_id: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedSplitRewardDTO {
    pub reward_type: String,
    pub metadata: LolRankedSplitRewardsMetaData,
    pub default_reward_id: String,
    pub tiered_reward_ids: HashMap<String, String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedSplitRewardGroup {
    pub split_points: i32,
    pub rewards: Vec<LolRankedSplitReward>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedSplitRewardGroupDTO {
    pub split_points: i32,
    pub rewards: Vec<LolRankedSplitRewardDTO>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedSplitRewardsMetaData {
    pub quantity: i32,
    pub description: String,
    pub champion_id: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedSummoner {
    pub summoner_id: u64,
    pub puuid: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedVictoriousSkin {
    pub split_points_by_highest_achieved_tier: HashMap<String, i32>,
    pub item_instance_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedVictoriousSkinDTO {
    pub split_points_by_highest_achieved_tier: HashMap<String, i32>,
    pub item_instance_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRegaliaAccountIdAndSummonerId {
    pub summoner_id: Option<u64>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRegaliaChatPresence {
    pub summoner_id: u64,
    pub icon: i32,
    pub lol: LolRegaliaChatPresenceLolData,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRegaliaChatPresenceExternal {
    pub id: String,
    pub summoner_id: u64,
    pub icon: i32,
    pub lol: HashMap<String, String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRegaliaChatPresenceLolData {
    pub level: u32,
    pub ranked_league_tier: Option<LolRegaliaLeagueTier>,
    pub ranked_league_division: LolRegaliaLeagueDivision,
    pub ranked_league_queue: LolRegaliaLeagueQueueType,
    pub ranked_split_reward_level: i32,
    pub ranked_prev_season_tier: Option<LolRegaliaLeagueTier>,
    pub ranked_prev_season_division: LolRegaliaLeagueDivision,
    pub regalia: Option<LolRegaliaRegaliaSettings>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRegaliaGameDataRegalia {
    pub id: String,
    pub id_secondary: String,
    pub asset_path: String,
    pub is_selectable: bool,
    pub regalia_type: String,
    pub localized_name: String,
    pub localized_description: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRegaliaInventoryItem {
    pub item_id: i32,
    pub uuid: String,
    pub purchase_date: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRegaliaItemKey {
    pub inventory_type: String,
    pub item_id: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRegaliaLoadout {
    pub id: String,
    pub name: String,
    pub scope: String,
    pub loadout: LolRegaliaRegaliaLoadout,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRegaliaRankedQueueStats {
    pub queue_type: LolRegaliaLeagueQueueType,
    pub is_provisional: bool,
    pub tier: LolRegaliaLeagueTier,
    pub division: LolRegaliaLeagueDivision,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRegaliaRankedStats {
    pub queues: Vec<LolRegaliaRankedQueueStats>,
    pub highest_ranked_entry: Option<LolRegaliaRankedQueueStats>,
    pub ranked_regalia_level: i32,
    pub highest_previous_season_achieved_tier: LolRegaliaLeagueTier,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRegaliaRegalia {
    pub profile_icon_id: i32,
    pub crest_type: String,
    pub banner_type: String,
    pub summoner_level: u32,
    pub last_season_highest_rank: Option<LolRegaliaLeagueTier>,
    pub highest_ranked_entry: Option<LolRegaliaRegaliaRankedEntry>,
    pub selected_prestige_crest: u8,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRegaliaRegaliaAsync {
    pub md5: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRegaliaRegaliaFrontendConfig {
    pub hovercard_enabled: bool,
    pub selections_enabled: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRegaliaRegaliaInventoryItem {
    pub items: Vec<LolRegaliaGameDataRegalia>,
    pub is_owned: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRegaliaRegaliaLoadout {
    pub r_e_g_a_l_i_a__c_r_e_s_t__s_l_o_t: LolRegaliaItemKey,
    pub r_e_g_a_l_i_a__b_a_n_n_e_r__s_l_o_t: LolRegaliaItemKey,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRegaliaRegaliaPlatformConfig {
    pub hovercard_enabled: bool,
    pub selections_enabled: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRegaliaRegaliaPreferences {
    pub preferred_crest_type: String,
    pub preferred_banner_type: String,
    pub selected_prestige_crest: u8,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRegaliaRegaliaRankedEntry {
    pub queue_type: LolRegaliaLeagueQueueType,
    pub tier: LolRegaliaLeagueTier,
    pub division: LolRegaliaLeagueDivision,
    pub split_reward_level: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRegaliaRegaliaSettings {
    pub crest_type: LolRegaliaRegaliaCrestType,
    pub banner_type: LolRegaliaRegaliaBannerType,
    pub selected_prestige_crest: u8,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRegaliaRegaliaSettingsExternal {
    pub crest_type: i32,
    pub banner_type: i32,
    pub selected_prestige_crest: u8,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRegaliaRegaliaWithPreferences {
    pub profile_icon_id: i32,
    pub crest_type: String,
    pub banner_type: String,
    pub preferred_crest_type: String,
    pub preferred_banner_type: String,
    pub selected_prestige_crest: u8,
    pub summoner_level: u32,
    pub last_season_highest_rank: Option<LolRegaliaLeagueTier>,
    pub highest_ranked_entry: Option<LolRegaliaRegaliaRankedEntry>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRegaliaSummoner {
    pub summoner_id: u64,
    pub profile_icon_id: i32,
    pub summoner_level: u32,
    pub puuid: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRegaliaSummonerProfile {
    pub regalia: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRegaliaSummonerProfileUpdate {
    pub key: String,
    pub value: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolReplaysClashPlaymodeRestrictedInfo {
    pub is_restricted: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolReplaysGameflowAvailability {
    pub state: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolReplaysGameflowGameClient {
    pub running: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolReplaysGameflowSession {
    pub phase: LolReplaysGameflowPhase,
    pub game_client: LolReplaysGameflowGameClient,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolReplaysInstallPaths {
    pub game_install_root: String,
    pub game_executable_path: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolReplaysReplayContextData {
    pub component_type: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolReplaysReplayCreateMetadata {
    pub game_version: String,
    pub game_type: String,
    pub queue_id: i32,
    pub game_end: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolReplaysReplayMetadata {
    pub state: LolReplaysMetadataState,
    pub game_id: u64,
    pub download_progress: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolReplaysReplaysConfiguration {
    pub is_replays_enabled: bool,
    pub is_replays_for_end_of_game_enabled: bool,
    pub is_replays_for_match_history_enabled: bool,
    pub is_patching: bool,
    pub is_in_tournament: bool,
    pub is_playing_game: bool,
    pub is_playing_replay: bool,
    pub is_logged_in: bool,
    pub game_version: String,
    pub min_server_version: String,
    pub minutes_until_replay_considered_lost: f64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolReplaysReplaysDynamicConfig {
    pub min_supported_game_server_version: String,
    pub minutes_until_replay_considered_lost: f64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolReplaysReplaysSettingsData {
    #[serde(rename = "replays-folder-path")]
    pub replays_folder_path: String,
    #[serde(rename = "highlights-folder-path")]
    pub highlights_folder_path: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolReplaysReplaysSettingsResource {
    pub data: LolReplaysReplaysSettingsData,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolReplaysRoflFileMetadata {
    pub game_length: u32,
    pub game_version: String,
    pub last_game_chunk_id: u32,
    pub last_key_frame_id: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRewardsGrantorDescription {
    pub app_name: String,
    pub entity_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRewardsRMSPayload {
    pub product_id: String,
    pub affinities: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRewardsRegionLocale {
    pub region: String,
    pub locale: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRewardsRequestDTOSelectionRequestDTO {
    pub data: LolRewardsSelectionRequestDTO,
    pub metadata: LolRewardsRequestMetadataDTO,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRewardsRequestDTOVectorSelectionRequestDTO {
    pub data: Vec<LolRewardsSelectionRequestDTO>,
    pub metadata: LolRewardsRequestMetadataDTO,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRewardsRequestMetadataDTO {
    pub transaction_id: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRewardsResponseDTOSvcRewardGrant {
    pub data: LolRewardsSvcRewardGrant,
    pub metadata: LolRewardsResponseMetadataDTO,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRewardsResponseDTOMapRewardGroupIdSelectGrantStatus {
    pub data: HashMap<String, LolRewardsSelectGrantStatusResponse>,
    pub metadata: LolRewardsResponseMetadataDTO,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRewardsResponseDTOVectorSvcRewardGrant {
    pub data: Vec<LolRewardsSvcRewardGrant>,
    pub metadata: LolRewardsResponseMetadataDTO,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRewardsResponseDTOVectorSvcRewardGroup {
    pub data: Vec<LolRewardsSvcRewardGroup>,
    pub metadata: LolRewardsResponseMetadataDTO,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRewardsResponseMetadataDTO {}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRewardsReward {
    pub id: String,
    pub item_id: String,
    pub item_type: String,
    pub quantity: i32,
    pub fulfillment_source: String,
    pub media: HashMap<String, String>,
    pub localizations: HashMap<String, String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRewardsRewardGrant {
    pub info: LolRewardsSvcRewardGrant,
    pub reward_group: LolRewardsSvcRewardGroup,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRewardsRewardsConfig {
    pub grant_filtering: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRewardsSelectionRequestDTO {
    pub grant_id: String,
    pub reward_group_id: String,
    pub selections: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRewardsSelectionStrategyConfig {
    pub min_selections_allowed: u32,
    pub max_selections_allowed: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRewardsSvcRewardGrant {
    pub id: String,
    pub grantee_id: String,
    pub reward_group_id: String,
    pub date_created: String,
    pub status: LolRewardsGrantStatus,
    pub grant_elements: Vec<LolRewardsSvcRewardGrantElement>,
    pub selected_ids: Vec<String>,
    pub viewed: bool,
    pub grantor_description: LolRewardsGrantorDescription,
    pub message_parameters: HashMap<String, Value>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRewardsSvcRewardGrantElement {
    pub element_id: String,
    pub item_id: String,
    pub item_type: String,
    pub fulfillment_source: String,
    pub status: LolRewardsRewardStatus,
    pub quantity: i32,
    pub media: HashMap<String, String>,
    pub localizations: HashMap<String, String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRewardsSvcRewardGroup {
    pub id: String,
    pub product_id: String,
    pub types: Vec<String>,
    pub rewards: Vec<LolRewardsReward>,
    pub child_reward_group_ids: Vec<String>,
    pub reward_strategy: LolRewardsRewardStrategy,
    pub selection_strategy_config: Option<LolRewardsSelectionStrategyConfig>,
    pub active: bool,
    pub media: HashMap<String, String>,
    pub localizations: HashMap<String, String>,
    pub celebration_type: LolRewardsCelebrationType,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRiotMessagingServiceChampionMasteryLevelUp {
    pub id: u64,
    pub player_id: u64,
    pub champion_id: i32,
    pub has_leveled_up: bool,
    pub champion_level: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRiotMessagingServiceGameflowSession {
    pub phase: LolRiotMessagingServiceGameflowPhase,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRiotclientUpgraderGameflowAvailability {
    pub is_available: bool,
    pub state: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRsoAuthAccessToken {
    pub token: String,
    pub scopes: Vec<String>,
    pub expiry: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRsoAuthAuthError {
    pub error: String,
    pub error_description: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRsoAuthAuthHint {
    #[serde(rename = "type")]
    pub type_: LolRsoAuthAuthHintType,
    pub required: bool,
    pub context: Value,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRsoAuthAuthorization {
    pub current_platform_id: String,
    pub current_account_id: u64,
    pub subject: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRsoAuthAuthorizationRequest {
    pub scope: Vec<String>,
    pub trust_levels: Vec<LolRsoAuthRSOAuthorizationTrustLevel>,
    pub client_id: String,
    pub claims: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRsoAuthAuthorizationResponse {
    #[serde(rename = "type")]
    pub type_: String,
    pub authorization: LolRsoAuthImplicitAuthorization,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRsoAuthConfigStatus {
    pub readiness: LolRsoAuthConfigReadinessEnum,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRsoAuthDeviceId {
    pub collector_server_name: String,
    pub merchant_id: String,
    pub session_id: String,
    pub install_id: String,
    pub frame_url: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRsoAuthEntitlementsToken {
    pub token: String,
    pub entitlements: Vec<String>,
    pub expiry: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRsoAuthIdToken {
    pub token: String,
    pub expiry: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRsoAuthImplicitAuthorization {
    pub access_token: LolRsoAuthAccessToken,
    pub id_token: LolRsoAuthIdToken,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRsoAuthPublicClientConfig {
    pub url: String,
    pub client_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRsoAuthRSOConfigReadyState {
    pub ready: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRsoAuthRSOJWTConfig {
    pub token: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRsoAuthRSOPlayerCredentials {
    pub username: String,
    pub password: String,
    pub platform_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRsoAuthRegionStatus {
    pub platform_id: String,
    pub enabled: bool,
    pub is_l_q_fallback_allowed: bool,
    pub is_user_info_enabled: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRsoAuthSessionResponse {
    #[serde(rename = "type")]
    pub type_: String,
    pub error: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolRsoAuthUserInfo {
    pub user_info: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolServiceStatusBroadcastMessage {
    pub content: String,
    pub message_key: String,
    pub severity: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolServiceStatusBroadcastNotification {
    pub broadcast_messages: Vec<LolServiceStatusBroadcastMessage>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolServiceStatusLegacyServiceStatusMessage {
    pub created_at: String,
    pub updated_at: String,
    pub severity: String,
    pub heading: String,
    pub content: String,
    pub translations: Vec<LolServiceStatusLegacyServiceStatusTranslation>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolServiceStatusLegacyServiceStatusResponse {
    pub status: String,
    pub messages: Vec<LolServiceStatusLegacyServiceStatusMessage>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolServiceStatusLegacyServiceStatusTranslation {
    pub locale: String,
    pub heading: Option<String>,
    pub content: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolServiceStatusLoginDataPacket {
    pub broadcast_notification: LolServiceStatusBroadcastNotification,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolServiceStatusRegionLocaleResource {
    pub region: String,
    pub locale: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolServiceStatusRiotStatusIncident {
    pub id: i64,
    pub archive_at: Option<String>,
    pub incident_severity: String,
    pub titles: Vec<LolServiceStatusRiotStatusTitle>,
    pub platforms: Vec<String>,
    pub updated_at: Option<String>,
    pub created_at: String,
    pub updates: Vec<LolServiceStatusRiotStatusUpdate>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolServiceStatusRiotStatusMaintenance {
    pub id: i64,
    pub archive_at: Option<String>,
    pub updates: Vec<LolServiceStatusRiotStatusUpdate>,
    pub created_at: String,
    pub platforms: Vec<String>,
    pub updated_at: Option<String>,
    pub maintenance_status: String,
    pub titles: Vec<LolServiceStatusRiotStatusTitle>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolServiceStatusRiotStatusResource {
    pub id: String,
    pub name: String,
    pub locales: Vec<String>,
    pub maintenances: Vec<LolServiceStatusRiotStatusMaintenance>,
    pub incidents: Vec<LolServiceStatusRiotStatusIncident>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolServiceStatusRiotStatusTitle {
    pub locale: String,
    pub content: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolServiceStatusRiotStatusTranslation {
    pub locale: String,
    pub content: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolServiceStatusRiotStatusUpdate {
    pub id: i64,
    pub updated_at: Option<String>,
    pub publish: bool,
    pub author: String,
    pub created_at: String,
    pub translations: Vec<LolServiceStatusRiotStatusTranslation>,
    pub publish_locations: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolServiceStatusServiceStatusResource {
    pub status: String,
    pub human_readable_url: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolServiceStatusTickerMessage {
    pub severity: String,
    pub created_at: String,
    pub updated_at: String,
    pub heading: String,
    pub message: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolSettingsLCUGameSettingsConfig {
    pub hotkeys_enabled: bool,
    pub sound_enabled: bool,
    pub interface_enabled: bool,
    pub gameplay_enabled: bool,
    pub replays_enabled: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolSettingsLoginSession {
    pub state: LolSettingsLoginSessionStates,
    pub summoner_id: u64,
    pub account_id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolSettingsRegionLocale {
    pub region: String,
    pub locale: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolSettingsReplaysConfig {
    pub replay_service_enabled: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolSettingsSettingCategory {
    pub schema_version: i32,
    pub data: Value,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolSettingsSettingsConfig {
    pub is_hotkeys_enabled: bool,
    pub is_sound_enabled: bool,
    pub is_interface_enabled: bool,
    pub is_gameplay_enabled: bool,
    pub is_replays_enabled: bool,
    pub is_privacy_notice_enabled: bool,
    pub is_terms_enabled: bool,
    pub is_legal_statements_enabled: bool,
    pub localized_licenses_u_r_l: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolShutdownShutdownNotification {
    pub reason: LolShutdownShutdownReason,
    pub countdown: f32,
    pub additional_info: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolSimpleDialogMessagesGameflowSession {
    pub phase: LolSimpleDialogMessagesGameflowPhase,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolSimpleDialogMessagesLocalMessageRequest {
    pub msg_type: String,
    pub msg_body: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolSimpleDialogMessagesLoginDataPacket {
    pub simple_messages: Vec<LolSimpleDialogMessagesSimpleMessage>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolSimpleDialogMessagesMessage {
    pub id: i64,
    #[serde(rename = "type")]
    pub type_: String,
    pub body: Value,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolSimpleDialogMessagesSimpleMessage {
    pub account_id: u64,
    pub msg_id: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub params: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolSocialLeaderboardFriendResource {
    pub summoner_id: u64,
    pub name: String,
    pub puuid: String,
    pub icon: i32,
    pub availability: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolSocialLeaderboardGiftingFriend {
    pub summoner_id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolSocialLeaderboardRankedQueueStats {
    pub queue_type: LolSocialLeaderboardLeagueQueueType,
    pub provisional_games_remaining: i32,
    pub is_provisional: bool,
    pub tier: LolSocialLeaderboardLeagueTier,
    pub division: LolSocialLeaderboardLeagueDivision,
    pub league_points: i32,
    pub wins: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolSocialLeaderboardRankedStats {
    pub queue_map: HashMap<String, LolSocialLeaderboardRankedQueueStats>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolSocialLeaderboardSocialLeaderboardData {
    pub row_data: Vec<LolSocialLeaderboardSocialLeaderboardRowData>,
    pub next_update_time: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolSocialLeaderboardSocialLeaderboardRowData {
    pub puuid: String,
    pub summoner_id: u64,
    pub summoner_name: String,
    pub provisional_games_remaining: i32,
    pub is_provisional: bool,
    pub tier: LolSocialLeaderboardLeagueTier,
    pub division: LolSocialLeaderboardLeagueDivision,
    pub league_points: i32,
    pub wins: i32,
    pub summoner_level: i32,
    pub profile_icon_id: i32,
    pub availability: String,
    pub leaderboard_position: i32,
    pub is_giftable: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolSocialLeaderboardSummoner {
    pub summoner_id: u64,
    pub account_id: u64,
    pub display_name: String,
    pub internal_name: String,
    pub summoner_level: u32,
    pub puuid: String,
    pub profile_icon_id: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolSpectatorSpectateGameInfo {
    pub drop_in_spectate_game_id: String,
    pub game_queue_type: String,
    pub allow_observe_mode: String,
    pub puuid: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolSpectatorSummonerOrTeamAvailabilty {
    pub available_for_watching: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesCatalogBundle {
    pub item: LolStatstonesCatalogItemDetails,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesCatalogBundlePrice {
    pub currency: String,
    pub cost: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesCatalogItemDetails {
    pub item_id: i32,
    pub inventory_type: String,
    pub sub_inventory_type: String,
    pub prices: Vec<LolStatstonesCatalogBundlePrice>,
    pub release_date: String,
    pub item_instance_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesChampionStatstoneSetSummary {
    pub name: String,
    pub stones_available: u32,
    pub stones_owned: u32,
    pub stones_illuminated: u32,
    pub milestones_passed: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesChampionStatstoneSummary {
    pub champion_id: i32,
    pub stones_available: u32,
    pub stones_owned: u32,
    pub stones_illuminated: u32,
    pub milestones_passed: u32,
    pub sets: Vec<LolStatstonesChampionStatstoneSetSummary>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesCollectionsChampion {
    pub name: String,
    pub square_portrait_path: String,
    pub id: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesEogNotificationEnvelope {
    pub self_statstone_progress: Vec<LolStatstonesStatstoneProgress>,
    pub self_personal_bests: Vec<LolStatstonesPersonalBestNotification>,
    pub self_milestone_progress: Vec<LolStatstonesMilestoneProgressNotification>,
    pub others_personal_bests: Vec<LolStatstonesPersonalBestNotification>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesGameDataItemReference {
    pub item_id: i32,
    pub inventory_type: String,
    pub content_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesGameDataStatstone {
    pub name: String,
    pub content_id: String,
    pub item_id: i32,
    pub tracking_type: u32,
    pub is_retired: bool,
    pub is_epic: bool,
    pub bound_champion: LolStatstonesGameDataItemReference,
    pub milestones: Vec<u32>,
    pub category: String,
    pub description: String,
    pub icon_unowned: String,
    pub icon_unlit: String,
    pub icon_lit: String,
    pub icon_full: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesGameDataStatstonePack {
    pub name: String,
    pub description: String,
    pub content_id: String,
    pub item_id: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesGameDataStatstoneSet {
    pub name: String,
    pub item_id: i32,
    pub inventory_type: String,
    pub content_id: String,
    pub statstones: Vec<LolStatstonesGameDataStatstone>,
    pub price: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesGameDataStatstonesInfo {
    pub statstone_data: Vec<LolStatstonesGameDataStatstoneSet>,
    pub pack_data: Vec<LolStatstonesGameDataStatstonePack>,
    pub pack_id_to_stat_stones_ids: Value,
    pub series_id_to_stat_stone_ids: Value,
    pub pack_id_to_sub_pack_ids: Value,
    pub collection_id_to_stat_stone_ids: Value,
    pub pack_id_to_champ_ids: Value,
    pub champ_id_to_pack_ids: Value,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesLoadout {
    pub id: String,
    pub name: String,
    pub scope: String,
    pub item_id: i32,
    pub loadout: HashMap<String, Value>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesLoadoutItem {
    pub inventory_type: String,
    pub content_id: String,
    pub item_id: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesMilestoneNotificationDto {
    pub statstone_id: String,
    pub level: i32,
    pub threshold: i32,
    pub is_completed: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesMilestoneProgressNotification {
    pub statstone_id: String,
    pub statstone_name: String,
    pub threshold: i32,
    pub image_url: String,
    pub level: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesNumberFormattingBehavior {
    pub digits_for_thousands_seperator: u32,
    pub trim_trailing_zeros_after_decimal: bool,
    pub western_number_grouping: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesNumberFormattingData {
    pub decimal_seperator: String,
    pub thousand_seperator: String,
    pub thousand_abbreviation: String,
    pub ten_thousand_abbreviation: String,
    pub million_abbreviation: String,
    pub one_hundred_million_abbreviation: String,
    pub billion_abbreviation: String,
    pub trillion_abbreviation: String,
    pub second_abbreviation: String,
    pub minute_abbreviation: String,
    pub hour_abbreviation: String,
    pub meters_abbreviation: String,
    pub kilometers_abbreviation: String,
    pub percentage_format: String,
    pub number_formatting_behavior: LolStatstonesNumberFormattingBehavior,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesPersonalBestNotification {
    pub summoner: LolStatstonesSummoner,
    pub statstone_id: String,
    pub statstone_name: String,
    pub personal_best: String,
    pub image_url: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesPriceInfo {
    pub currency: String,
    pub price: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesProfileStatstoneSummary {
    pub champion_id: i32,
    pub name: String,
    pub value: String,
    pub image_url: String,
    pub category: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesStatstone {
    pub name: String,
    pub statstone_id: String,
    pub bound_champion_item_id: u32,
    pub next_milestone: String,
    pub completion_value: f32,
    pub is_complete: bool,
    pub is_featured: bool,
    pub is_epic: bool,
    pub is_retired: bool,
    pub category: String,
    pub image_url: String,
    pub description: String,
    pub formatted_value: String,
    pub formatted_personal_best: String,
    pub formatted_milestone_level: String,
    pub player_record: Option<LolStatstonesStatstonePlayerRecord>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesStatstoneCompletion {
    pub statstone_name: String,
    pub category: String,
    pub is_epic: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesStatstoneFeaturedRequest {
    pub index: i32,
    pub existing_featured: Vec<LolStatstonesStatstone>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesStatstoneMasteryVignette {
    pub num_sets_completed: u32,
    pub mastery_level: u32,
    pub completed_set_uuids: Vec<String>,
    pub puuid: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesStatstoneNotificationDto {
    pub statstone_id: String,
    pub puuid: String,
    pub delta: i32,
    pub value: i32,
    pub level: i32,
    pub best: i32,
    pub is_new_best: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesStatstoneNotificationEnvelopeDto {
    pub game_id: u64,
    pub updates: Vec<LolStatstonesStatstoneNotificationDto>,
    pub milestones: Vec<LolStatstonesMilestoneNotificationDto>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesStatstonePlayerRecord {
    pub puuid: String,
    pub statstone_id: String,
    pub value: u32,
    pub personal_best: u32,
    pub milestone_level: u32,
    pub date_acquired: String,
    pub date_modified: String,
    pub date_completed: String,
    pub date_archived: String,
    pub entitled: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesStatstoneProgress {
    pub statstone_id: String,
    pub statstone_name: String,
    pub description: String,
    pub image_url: String,
    pub delta: String,
    pub value: String,
    pub next_milestone: String,
    pub existing_progress_percent: String,
    pub new_progress_percent: String,
    pub new_milestone_difference: String,
    pub total_progress_percent: String,
    pub category: String,
    pub level: i32,
    pub best: i32,
    pub is_new_best: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesStatstoneRekindledVignette {
    pub portrait_path: String,
    pub statstone: LolStatstonesStatstoneCompletion,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesStatstoneSet {
    pub name: String,
    pub statstones: Vec<LolStatstonesStatstone>,
    pub stones_owned: u32,
    pub milestones_passed: u32,
    pub item_id: u32,
    pub inventory_type: String,
    pub sub_inventory_type: String,
    pub item_instance_i_d: String,
    pub prices: Vec<LolStatstonesPriceInfo>,
    pub owned_from_packs: Vec<LolStatstonesGameDataStatstonePack>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesStatstoneSetCompleteVignette {
    pub statstones: Vec<LolStatstonesStatstoneCompletion>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesStatstoneVignetteNotificationEnvelopeDto {
    pub champ_name: String,
    pub champ_id: i32,
    pub mastery_vignette_notifications: Vec<LolStatstonesStatstoneMasteryVignette>,
    pub rekindled_vignette_notifications: Vec<LolStatstonesStatstoneRekindledVignette>,
    pub set_complete_vignette_notifications: Vec<LolStatstonesStatstoneSetCompleteVignette>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesSummoner {
    pub summoner_id: u64,
    pub puuid: String,
    pub display_name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreAccessTokenResource {
    pub token: String,
    pub scopes: Vec<String>,
    pub expiry: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreAllSummonerData {
    pub summoner: LolStoreSummoner,
    pub summoner_level_and_points: LolStoreSummonerLevelAndPoints,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreBundleItemDTO {
    pub inventory_type: String,
    pub item_id: i32,
    pub icon_url: String,
    pub quantity: u32,
    pub name: String,
    pub description: String,
    pub owned: bool,
    pub rp: i64,
    pub discounted_rp: i64,
    pub ip: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreBundled {
    pub flexible: bool,
    pub items: Vec<LolStoreBundledItem>,
    pub minimum_prices: Vec<LolStoreBundledItemCost>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreBundledItem {
    pub inventory_type: String,
    pub item_id: i32,
    pub quantity: u32,
    pub discount_prices: Vec<LolStoreBundledItemCost>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreBundledItemCost {
    pub currency: String,
    pub cost: i64,
    pub discount: Option<f32>,
    pub cost_type: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreCapOffer {
    pub id: String,
    pub type_id: String,
    pub label: String,
    pub product_id: String,
    pub merchant_id: String,
    pub payload: Value,
    pub active: bool,
    pub start_date: String,
    pub created_date: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreCatalogInstanceToItemKeyMap {
    pub platform_ids: HashMap<String, LolStoreItemKey>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreCatalogItem {
    pub item_id: i32,
    pub inventory_type: String,
    pub icon_url: Option<String>,
    pub localizations: Option<HashMap<String, LolStoreItemLocalization>>,
    pub active: Option<bool>,
    pub bundled: Option<LolStoreBundled>,
    pub inactive_date: Option<String>,
    pub max_quantity: Option<i32>,
    pub prices: Option<Vec<LolStoreItemCost>>,
    pub release_date: Option<String>,
    pub sale: Option<LolStoreSale>,
    pub sub_inventory_type: Option<String>,
    pub tags: Option<Vec<String>>,
    pub item_requirements: Option<Vec<LolStoreItemKey>>,
    pub metadata: Option<Vec<LolStoreItemMetadataEntry>>,
    pub item_instance_id: Option<String>,
    pub offer_id: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreFeaturedPageDTO {
    pub player: LolStorePlayer,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreGetPlatformIdsFromInstanceIdsRequest {
    pub instance_ids: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreGiftableResult {
    pub config: LolStoreGiftingConfig,
    pub friends: Vec<LolStoreGiftingFriend>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreGiftingConfig {
    pub recipient_level_limit_item: u32,
    pub recipient_level_limit_rp: u32,
    pub gifting_restriction_flag_rioter: u32,
    pub gifting_item_min_level_send: u32,
    pub gifting_rp_min_level_send: u32,
    pub gifting_rp_max_daily_gifts_send: u32,
    pub gifting_rp_max_daily_gifts_receive: u32,
    pub gifting_item_max_daily_gifts_send: u32,
    pub gifting_item_max_daily_gifts_receive: u32,
    pub gifting_hextech_max_daily_gifts_send: u32,
    pub gifting_hextec_max_daily_gifts_receive: u32,
    pub requires_identity_verification: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreGiftingFriend {
    pub friends_since: String,
    pub old_friends: bool,
    pub summoner_id: u64,
    pub nick: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreItemCost {
    pub currency: String,
    pub cost: i64,
    pub discount: Option<f32>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreItemKey {
    pub inventory_type: String,
    pub item_id: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreItemLocalization {
    pub language: String,
    pub name: String,
    pub description: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreItemMetadataEntry {
    #[serde(rename = "type")]
    pub type_: String,
    pub value: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreItemSale {
    pub id: u64,
    pub active: bool,
    pub item: LolStoreItemKey,
    pub sale: LolStoreSale,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreLoginDataPacket {
    pub all_summoner_data: LolStoreAllSummonerData,
    pub simple_messages: Vec<LolStoreSimpleDialogMessage>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreLoginSession {
    pub state: LolStoreLoginSessionStates,
    pub summoner_id: u64,
    pub account_id: u64,
    pub id_token: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreNotification {
    pub background_url: String,
    pub created: String,
    pub critical: bool,
    pub dismissible: bool,
    pub data: HashMap<String, String>,
    pub detail_key: String,
    pub expires: String,
    pub icon_url: String,
    pub id: u64,
    pub source: String,
    pub state: String,
    pub title_key: String,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreOrderNotificationResource {
    pub id: u64,
    pub event_type_id: String,
    pub event_type: String,
    pub status: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStorePageDTO {
    pub player: LolStorePlayer,
    pub catalog: Vec<LolStoreCatalogItem>,
    pub group_order: Vec<String>,
    pub item_groups: HashMap<String, LolStorePageGroupingDTO>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStorePageGroupingDTO {
    pub items: Vec<LolStoreItemKey>,
    pub hidden: bool,
    pub grouped: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStorePlayer {
    pub account_id: u64,
    pub rp: i64,
    pub ip: i64,
    pub summoner_level: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreRiotMessagingServiceMessage {
    pub resource: String,
    pub service: String,
    pub version: String,
    pub timestamp: i64,
    pub payload: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreSale {
    pub start_date: String,
    pub end_date: String,
    pub prices: Vec<LolStoreItemCost>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreServiceBalance {
    pub currency: String,
    pub amount: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreServiceWallet {
    pub account_id: u64,
    pub balances: Vec<LolStoreServiceBalance>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreSimpleDialogMessage {
    pub account_id: u64,
    pub msg_id: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub params: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreStoreStatus {
    pub storefront_is_running: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreSummoner {
    pub acct_id: u64,
    pub sum_id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreSummonerLevelAndPoints {
    pub summoner_level: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreWallet {
    pub ip: i64,
    pub rp: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolSuggestedPlayersEndOfGamePlayer {
    pub summoner_name: String,
    pub summoner_id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolSuggestedPlayersEndOfGameStats {
    pub teams: Vec<LolSuggestedPlayersEndOfGameTeam>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolSuggestedPlayersEndOfGameTeam {
    pub players: Vec<LolSuggestedPlayersEndOfGamePlayer>,
    pub is_winning_team: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolSuggestedPlayersHonorInteraction {
    pub summoner_id: u64,
    pub display_name: String,
    pub game_id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolSuggestedPlayersSuggestedPlayersConfig {
    pub enabled: bool,
    pub friends_of_friends_enabled: bool,
    pub max_num_suggested_players: u32,
    pub max_num_replacements: u32,
    pub previous_premades_limit: u32,
    pub online_friends_limit: u32,
    pub friends_of_friends_limit: u32,
    pub vicorious_comrades_limit: u32,
    pub max_honor_interaction_players: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolSuggestedPlayersSuggestedPlayersDynamicClientConfig {
    pub suggested_players: LolSuggestedPlayersSuggestedPlayersConfig,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolSuggestedPlayersSuggestedPlayersFriend {
    pub summoner_id: u64,
    pub name: String,
    pub availability: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolSuggestedPlayersSuggestedPlayersKudoedPlayer {
    pub kudoed_summoner_id: u64,
    pub kudoed_summoner_name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolSuggestedPlayersSuggestedPlayersLobbyStatus {
    pub queue_id: i32,
    pub member_summoner_ids: Vec<u64>,
    pub invited_summoner_ids: Vec<u64>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolSuggestedPlayersSuggestedPlayersPlayerStatus {
    pub current_lobby_status: Option<LolSuggestedPlayersSuggestedPlayersLobbyStatus>,
    pub last_queued_lobby_status: Option<LolSuggestedPlayersSuggestedPlayersLobbyStatus>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolSuggestedPlayersSuggestedPlayersQueue {
    pub id: i32,
    pub min_level: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolSuggestedPlayersSuggestedPlayersReportedPlayer {
    pub reported_summoner_id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolSuggestedPlayersSuggestedPlayersSuggestedPlayer {
    pub summoner_name: String,
    pub summoner_id: u64,
    pub common_friend_name: String,
    pub common_friend_id: u64,
    pub reason: LolSuggestedPlayersSuggestedPlayersReason,
    pub game_id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolSuggestedPlayersSuggestedPlayersSummoner {
    pub summoner_id: u64,
    pub display_name: String,
    pub summoner_level: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolSuggestedPlayersSuggestedPlayersVictoriousComrade {
    pub summoner_id: u64,
    pub summoner_name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolSummonerAccountIdAndSummonerId {
    pub account_id: u64,
    pub summoner_id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolSummonerAutoFillQueueDto {
    pub queue_id: i32,
    pub auto_fill_eligible: bool,
    pub auto_fill_protected_for_streaking: bool,
    pub auto_fill_protected_for_promos: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolSummonerGameloopPlayerInfoV2 {
    pub auto_fill_data_bags: Vec<LolSummonerAutoFillQueueDto>,
    pub reroll_data_bags: Vec<LolSummonerRerollDataBagForClientV1>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolSummonerLevelField {
    pub initial_level: u32,
    pub final_level: u32,
    pub progress: LolSummonerLevelProgression,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolSummonerLevelProgression {
    pub initial_xp: u64,
    pub final_xp: u64,
    pub initial_level_boundary: u64,
    pub final_level_boundary: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolSummonerLoginSession {
    pub state: LolSummonerLoginSessionStates,
    pub summoner_id: u64,
    pub account_id: u64,
    pub puuid: String,
    pub connected: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolSummonerProfilePrivacy {
    pub enabled_state: LolSummonerProfilePrivacyEnabledState,
    pub setting: LolSummonerProfilePrivacySetting,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolSummonerRerollDataBagForClientV1 {
    pub queue_id: i32,
    pub points_until_next_reroll: i32,
    pub reroll_count: i32,
    pub total_points: i32,
    pub maximum_rerolls: i32,
    pub point_cost_of_reroll: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolSummonerStatus {
    pub ready: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolSummonerSummoner {
    pub summoner_id: u64,
    pub account_id: u64,
    pub display_name: String,
    pub internal_name: String,
    pub profile_icon_id: i32,
    pub summoner_level: u32,
    pub xp_since_last_level: u64,
    pub xp_until_next_level: u64,
    pub percent_complete_for_next_level: u32,
    pub reroll_points: LolSummonerSummonerRerollPoints,
    pub puuid: String,
    pub name_change_flag: bool,
    pub unnamed: bool,
    pub privacy: LolSummonerProfilePrivacySetting,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolSummonerSummonerCacheData {
    pub summoner: Option<LolSummonerSummoner>,
    pub summoner_icon: Option<i32>,
    pub privacy: Option<LolSummonerProfilePrivacySetting>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolSummonerSummonerCreateRequest {
    pub summoner_name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolSummonerSummonerCreatedId {
    pub summoner_id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolSummonerSummonerDTO {
    pub id: u64,
    pub puuid: String,
    pub account_id: u64,
    pub name: String,
    pub partner_id: String,
    pub profile_icon_id: i32,
    pub level: u32,
    pub exp_points: u32,
    pub exp_to_next_level: u32,
    pub name_change_flag: bool,
    pub unnamed: bool,
    pub privacy: LolSummonerProfilePrivacySetting,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolSummonerSummonerIcon {
    pub profile_icon_id: i32,
    pub inventory_token: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolSummonerSummonerIdAndIcon {
    pub summoner_id: u64,
    pub profile_icon_id: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolSummonerSummonerIdAndName {
    pub summoner_id: u64,
    pub display_name: String,
    pub puuid: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolSummonerSummonerProfileUpdate {
    pub key: String,
    pub value: Value,
    pub inventory: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolSummonerSummonerRequestedName {
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolSummonerSummonerRerollPoints {
    pub points_to_reroll: u32,
    pub current_points: u32,
    pub number_of_rolls: u32,
    pub max_rolls: u32,
    pub points_cost_to_roll: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolSummonerSummonerSession {
    pub summoner_id: u64,
    pub display_name: String,
    pub is_new_player: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolSummonerXpAndLevelMessage {
    pub xp: Value,
    pub level: LolSummonerLevelField,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolTastesDataModelResponse {
    pub response_code: i64,
    pub model_data: Value,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolTftDataModelResponse {
    pub response_code: i64,
    pub model_data: Value,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolTftGameflowGameData {
    pub game_id: u64,
    pub queue: LolTftQueue,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolTftGameflowSession {
    pub phase: LolTftGameflowPhase,
    pub game_data: LolTftGameflowGameData,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolTftLolTftBattlePassHub {
    pub battle_pass_x_p_boosted: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolTftLolTftEvent {
    pub title_translation_key: String,
    pub enabled: bool,
    pub redirect: bool,
    pub url: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolTftLolTftEvents {
    pub events: Vec<LolTftLolTftEvent>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolTftLolTftHomeHub {
    pub enabled: bool,
    pub store_promo_offer_ids: Vec<String>,
    pub tactician_promo_offer_ids: Vec<String>,
    pub battle_pass_offer_ids: Vec<String>,
    pub fallback_store_promo_offer_ids: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolTftLolTftNewsHub {
    pub enabled: bool,
    pub url: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolTftPublishingSettings {
    pub publishing_locale: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolTftQueue {
    pub id: i32,
    pub map_id: i32,
    pub game_mode: String,
    pub category: LolTftQueueGameCategory,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolTftSettingsResource {
    pub data: Value,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTeamPlannerChampion {
    pub champion_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTeamPlannerGameDataTFTSets {
    pub l_c_t_f_t_mode_data: LolTftTeamPlannerTFTModeData,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTeamPlannerSettingsStorageContainer {
    pub data: LolTftTeamPlannerTeamSettings,
    pub schema_version: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTeamPlannerTFTMapSetData {
    pub set_core_name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTeamPlannerTFTModeData {
    pub m_default_set: LolTftTeamPlannerTFTMapSetData,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTeamPlannerTFTTeamPlannerConfig {
    pub enabled: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTeamPlannerTeamPlan {
    pub champions: Vec<LolTftTeamPlannerChampion>,
    pub set_name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTeamPlannerTeamSettings {
    pub teams: Vec<LolTftTeamPlannerTeamPlan>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolTrophiesCapClashTrophyEntitlementPayload {
    pub reward_spec: LolTrophiesClashV2TrophyRewardSpec,
    pub reward_type: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolTrophiesClashV2TrophyRewardSpec {
    pub pedestal: String,
    pub cup: String,
    pub gem: String,
    pub theme: String,
    pub tier: String,
    pub bracket: String,
    pub season_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolTrophiesInventoryItemsByType {
    pub t_o_u_r_n_a_m_e_n_t__t_r_o_p_h_y: Vec<LolTrophiesTournamentTrophyInventoryItem>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolTrophiesInventoryResponse {
    pub items: LolTrophiesInventoryItemsByType,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolTrophiesLoadout {
    pub id: String,
    pub name: String,
    pub scope: String,
    pub loadout: HashMap<String, LolTrophiesLoadoutItem>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolTrophiesLoadoutItem {
    pub item_id: i32,
    pub inventory_type: String,
    pub data: HashMap<String, String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolTrophiesOtherPlayerTrophyInventoryItem {
    pub uuid: String,
    pub item_id: i32,
    pub inventory_type: String,
    pub purchase_date: String,
    pub payload: LolTrophiesCapClashTrophyEntitlementPayload,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolTrophiesTournamentTrophyInventoryItem {
    pub payload: LolTrophiesCapClashTrophyEntitlementPayload,
    pub purchase_date: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolTrophiesTrophyProfileData {
    pub theme: String,
    pub tier: i64,
    pub bracket: i64,
    pub season_id: i64,
    pub pedestal: String,
    pub cup: String,
    pub gem: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolUserExperienceGameflowSession {
    pub phase: LolUserExperienceGameflowPhase,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopAccessTokenResource {
    pub token: String,
    pub scopes: Vec<String>,
    pub expiry: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopCatalogItem {
    pub item_id: i32,
    pub inventory_type: String,
    pub item_instance_id: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopClientCacheClearMessageDTO {
    pub regions: Vec<String>,
    pub clear_all: bool,
    pub inventory_types: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopCurrencyDTO {
    pub amount: i32,
    pub sub_currencies: HashMap<String, i32>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopInventoryCacheEntry {
    pub signed_inventory_jwt: String,
    pub expiration_m_s: u64,
    pub issued_at_m_s: u64,
    pub received_at_m_s: u64,
    pub valid: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopInventoryDTO {
    pub puuid: String,
    pub account_id: u64,
    pub summoner_id: u64,
    pub items: HashMap<String, Value>,
    pub expires: String,
    pub items_jwt: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopInventoryItem {
    pub uuid: String,
    pub item_id: i32,
    pub inventory_type: String,
    pub purchase_date: String,
    pub quantity: u64,
    pub ownership_type: LolYourshopItemOwnershipType,
    pub expiration_date: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopInventoryItemDTO {
    pub item_id: i32,
    pub inventory_type: String,
    pub expiration_date: String,
    pub purchase_date: String,
    pub quantity: u64,
    pub owned_quantity: u64,
    pub used_in_game_date: String,
    pub entitlement_id: String,
    pub entitlement_type_id: String,
    pub instance_id: String,
    pub instance_type_id: String,
    pub payload: Value,
    pub f2p: bool,
    pub rental: bool,
    pub loyalty: bool,
    pub loyalty_sources: Vec<String>,
    pub lsb: bool,
    pub wins: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopInventoryItemWithPayload {
    pub uuid: String,
    pub item_id: i32,
    pub inventory_type: String,
    pub purchase_date: String,
    pub quantity: u64,
    pub ownership_type: LolYourshopItemOwnershipType,
    pub expiration_date: String,
    pub f2p: bool,
    pub rental: bool,
    pub loyalty: bool,
    pub loyalty_sources: Vec<String>,
    pub owned: bool,
    pub payload: Value,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopInventoryNotification {
    pub id: i64,
    pub item_id: i32,
    pub inventory_type: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub acknowledged: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopInventoryResponseDTO {
    pub data: LolYourshopInventoryDTO,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopItemKey {
    pub inventory_type: String,
    pub item_id: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopLoginSession {
    pub state: LolYourshopLoginSessionStates,
    pub summoner_id: u64,
    pub account_id: u64,
    pub id_token: String,
    pub puuid: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopLoyaltyRewards {
    pub free_rewarded_champions_count: i32,
    pub champion_ids: Vec<i32>,
    pub free_rewarded_skins_count: i32,
    pub skin_ids: Vec<i32>,
    pub ip_boost: i32,
    pub xp_boost: HashMap<String, i32>,
    pub loyalty_t_f_t_map_skin_count: i32,
    pub loyalty_t_f_t_companion_count: i32,
    pub loyalty_t_f_t_damage_skin_count: i32,
    pub loyalty_sources: HashMap<String, bool>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopLoyaltyRewardsSimplified {
    pub free_rewarded_champions_count: i32,
    pub champion_ids: Vec<i32>,
    pub free_rewarded_skins_count: i32,
    pub skin_ids: Vec<i32>,
    pub ip_boost: i32,
    pub xp_boost: i32,
    pub loyalty_t_f_t_map_skin_count: i32,
    pub loyalty_t_f_t_companion_count: i32,
    pub loyalty_t_f_t_damage_skin_count: i32,
    pub loyalty_sources: HashMap<String, bool>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopLoyaltyStatusNotification {
    pub status: LolYourshopLoyaltyStatus,
    pub rewards: LolYourshopLoyaltyRewardsSimplified,
    pub reload_inventory: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopOffer {
    pub offer_id: String,
    pub inventory_type: String,
    pub item_id: i32,
    pub original_price: i64,
    pub discount_price: i64,
    pub name: String,
    pub owned: bool,
    pub revealed: bool,
    pub expiration_date: String,
    pub spot_index: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopOfferData {
    pub offers: Vec<LolYourshopOffer>,
    pub promotion: LolYourshopPromotion,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopOfferId {
    pub offer_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopOfferIds {
    pub offers: Vec<LolYourshopOfferId>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopOfferRequest {
    pub offer_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopOfferRequests {
    pub offers: Vec<LolYourshopOfferRequest>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopPlayerNotification {
    pub critical: bool,
    pub detail_key: String,
    pub source: String,
    pub state: String,
    pub title_key: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub icon_url: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopPlayerPermissions {
    pub use_data: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopPromotion {
    pub name: String,
    pub start_time: String,
    pub end_time: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopPurchaseItem {
    pub inventory_type: String,
    pub item_id: i32,
    pub price_paid: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopPurchaseResponse {
    pub items: Vec<LolYourshopPurchaseItem>,
    pub wallet: LolYourshopWallet,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopRiotMessagingServiceMessage {
    pub resource: String,
    pub service: String,
    pub version: String,
    pub timestamp: i64,
    pub payload: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopRmsEntitlementPayload {
    pub item_id: String,
    pub item_type_id: String,
    pub entitlement_type_id: String,
    pub resource_operation: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopRmsStoreEntitlementItem {
    pub inventory_type: String,
    pub item_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopRmsStoreEntitlementPayload {
    pub transaction_id: String,
    pub items: Vec<LolYourshopRmsStoreEntitlementItem>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopRmsWalletPayload {}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopRmsXboxSubscriptionChange {
    pub puuid: String,
    pub subscription_id: String,
    pub active: String,
    pub identity_provider: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopSimpleInventoryDTO {
    pub items: HashMap<String, Value>,
    pub items_jwt: String,
    pub expires: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopSimpleInventoryResponseDTO {
    pub data: LolYourshopSimpleInventoryDTO,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopSummonerIcon {
    pub item_id: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopUIOffer {
    pub id: String,
    pub original_price: i64,
    pub discount_price: i64,
    #[serde(rename = "type")]
    pub type_: String,
    pub skin_name: String,
    pub champion_id: i32,
    pub skin_id: i32,
    pub owned: bool,
    pub revealed: bool,
    pub purchasing: bool,
    pub expiration_date: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopUIStatus {
    pub hub_enabled: bool,
    pub name: String,
    pub start_time: String,
    pub end_time: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopWallet {
    pub rp: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopWalletCacheEntry {
    pub signed_balances_jwt: String,
    pub expiration_m_s: u64,
    pub issued_at_m_s: u64,
    pub received_at_m_s: u64,
    pub valid: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopWalletDTO {
    pub puuid: String,
    pub account_id: i64,
    pub expires: String,
    pub balances: HashMap<String, i32>,
    pub balances_jwt: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopWalletResponseDTO {
    pub data: LolYourshopWalletDTO,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopXboxSubscriptionStatus {
    pub active: String,
    pub subscription_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopYourshopConfig {
    pub active: bool,
    pub themed_background: bool,
    pub promotion_name: String,
    pub promotion_start_date: String,
    pub promotion_end_date: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LootItemClientDTO {
    pub loot_name: String,
    pub asset: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub rarity: String,
    pub value: i32,
    pub store_item_id: i32,
    pub upgrade_loot_name: String,
    pub expiry_time: i64,
    pub tags: String,
    pub display_categories: String,
    pub rental_seconds: i64,
    pub rental_games: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LootItemListClientDTO {
    pub loot_items: Vec<LootItemClientDTO>,
    pub last_update: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LootLcdsClientConfigurationDTO {
    pub loot_items_using_breakout_recipe_menu: Vec<String>,
    pub loot_materials_to_always_render: Vec<String>,
    pub currencies_using_cap_wallets: Vec<String>,
    pub disabled_redemptions: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LootLcdsLootDescriptionDTO {
    pub loot_name: String,
    pub child_loot_table_names: Vec<String>,
    pub localization_map: HashMap<String, String>,
    pub localization_long_description_map: HashMap<String, String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LootLcdsRecipeClientDTO {
    pub recipe_name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub display_categories: String,
    pub crafter_name: String,
    pub slots: Vec<LootLcdsRecipeSlotClientDTO>,
    pub outputs: Vec<LootLcdsRecipeOutputDTO>,
    pub metadata: LootLcdsRecipeMetadata,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LootLcdsRecipeListClientDTO {
    pub recipes: Vec<LootLcdsRecipeClientDTO>,
    pub last_update: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LootLcdsRecipeMetadata {
    pub guaranteed_descriptions: Vec<LootLcdsLootDescriptionDTO>,
    pub bonus_descriptions: Vec<LootLcdsLootDescriptionDTO>,
    pub tooltips_disabled: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LootLcdsRecipeOutputDTO {
    pub loot_name: String,
    pub quantity_expression: String,
    pub probability: f64,
    pub allow_duplicates: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LootLcdsRecipeSlotClientDTO {
    pub slot_number: i32,
    pub query: String,
    pub quantity_expression: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LootNameRefId {
    pub loot_name: String,
    pub ref_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LootOddsDTO {
    pub loot_id: String,
    pub parent_id: String,
    pub drop_rate: f64,
    pub quantity: i32,
    pub label: String,
    pub loot_order: i32,
    pub children: Vec<LootOddsDTO>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LootProgressionRecipeConfigMap {
    pub last_update: i64,
    pub loot_progression_recipe_d_t_o_map: HashMap<String, LootProgressionRecipeConfiguration>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LootProgressionRecipeConfiguration {
    pub active: bool,
    pub progression_uuid: String,
    pub recipe_name: String,
    pub activation_time: String,
    pub deactivation_time: String,
    pub counter_uuids: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LootSimpleDialogMessageResponse {
    pub account_id: u64,
    pub msg_id: String,
    pub command: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct MatchedPlayerDTO {
    pub roster_id: String,
    pub captain: PlayerInfoDTO,
    pub players: Vec<PlayerInfoDTO>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct MatchmakingLcdsBustedLeaver {
    pub summoner: MatchmakingLcdsSummoner,
    pub reason_failed: String,
    pub access_token: String,
    pub leaver_penalty_millis_remaining: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct MatchmakingLcdsFailedJoinPlayer {
    pub summoner: MatchmakingLcdsSummoner,
    pub reason_failed: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct MatchmakingLcdsGameDTO {
    pub game_state: String,
    pub status_of_participants: String,
    pub terminated_condition: String,
    pub team_one: Vec<MatchmakingLcdsPlayerParticipant>,
    pub team_two: Vec<MatchmakingLcdsPlayerParticipant>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct MatchmakingLcdsGameNotification {
    #[serde(rename = "type")]
    pub type_: String,
    pub message_code: String,
    pub message_argument: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct MatchmakingLcdsMatchMakerParams {
    pub queue_ids: Vec<i32>,
    pub team: Vec<u64>,
    pub invitation_id: Option<String>,
    pub last_maestro_message: String,
    pub languages: Option<String>,
    pub bot_difficulty: String,
    pub team_id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct MatchmakingLcdsMatchMakerPayload {
    pub l_e_a_v_e_r__b_u_s_t_e_r__a_c_c_e_s_s__t_o_k_e_n: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct MatchmakingLcdsPlayerParticipant {
    pub summoner_id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct MatchmakingLcdsQueueDisabled {
    pub summoner: MatchmakingLcdsSummoner,
    pub reason_failed: String,
    pub queue_id: i32,
    pub message: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct MatchmakingLcdsQueueDodger {
    pub summoner: MatchmakingLcdsSummoner,
    pub reason_failed: String,
    pub dodge_penalty_remaining_time: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct MatchmakingLcdsQueueInfo {
    pub queue_id: i32,
    pub wait_time: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct MatchmakingLcdsQueueRestricted {
    pub summoner: MatchmakingLcdsSummoner,
    pub reason_failed: String,
    pub queue_id: i32,
    pub message: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct MatchmakingLcdsQueueThrottled {
    pub summoner: MatchmakingLcdsSummoner,
    pub reason_failed: String,
    pub queue_id: i32,
    pub message: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct MatchmakingLcdsSearchingForMatchNotification {
    pub joined_queues: Vec<MatchmakingLcdsQueueInfo>,
    pub player_join_failures: Vec<Value>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct MatchmakingLcdsSummoner {
    pub sum_id: u64,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct MemberBanUnbanTournament {
    pub tournament_id: i64,
    pub tournamentname_loc_key: String,
    pub tournamentname_loc_key_secondary: String,
    pub tournament_phase_lock_in_time: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct MetricMetadata {
    pub sample_window_ms: u32,
    #[serde(rename = "type")]
    pub type_: MetricType,
    pub data_type: MetricDataType,
    pub alerts: Vec<MetricMetadataAlert>,
    pub notify: MetricMetadataNotify,
    pub units: String,
    pub pretty_name: String,
    pub category: String,
    pub description: String,
    pub destination: String,
    pub info: String,
    pub transient_aggregation: AggregationType,
    pub priority: MetricPriority,
    pub period: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct MetricMetadataAlert {
    pub level: String,
    pub min: f64,
    pub max: f64,
    pub notify: MetricMetadataNotify,
    pub pretty_name: String,
    pub description: String,
    pub info: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct MetricMetadataHipchatNotification {
    pub roomid: String,
    pub tags: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct MetricMetadataNotify {
    pub email: Vec<String>,
    pub hipchat: Vec<MetricMetadataHipchatNotification>,
    pub pagerduty: Vec<MetricMetadataPagerDutyNotification>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct MetricMetadataPagerDutyNotification {
    pub apikey: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct MissionAlertDTO {
    #[serde(rename = "type")]
    pub type_: String,
    pub message: String,
    pub alert_time: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct MissionDisplay {
    pub attributes: Vec<String>,
    pub locations: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct MissionMetadata {
    pub tutorial: TutorialMetadata,
    pub npe_reward_pack: NpeRewardPackMetadata,
    pub level: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct MissionProgressDTO {
    pub last_viewed_progress: i32,
    pub current_progress: i32,
    pub total_count: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct MissionRequirementDTO {
    pub expression: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct MissionsCompressedPayloadDTO {
    pub missions_compressed: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct MucJwtDto {
    pub jwt: String,
    pub channel_claim: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct MultipleReplayMetadataRequestV2 {
    pub platform_id: String,
    pub game_ids: Vec<u64>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct MultipleReplayMetadataResponseItemV2 {
    pub game_id: u64,
    pub status: ReplayResponseStatus,
    pub metadata: ReplayMetadataV2,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct MultipleReplayMetadataResponseV2 {
    pub metadata_responses: Vec<MultipleReplayMetadataResponseItemV2>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct NonTimingEventV1 {
    pub when: u64,
    pub event_name: String,
    pub value: String,
    pub unit: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct NotifyFailureRequest {
    pub availability_item_name: String,
    pub failure_info: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct NotifySuccessRequest {
    pub availability_item_name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct NpeReward {
    pub renderer: String,
    pub data: Value,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct NpeRewardPackMetadata {
    pub index: i32,
    pub premium_reward: bool,
    pub reward_key: String,
    pub major_reward: NpeReward,
    pub minor_rewards: Vec<NpeReward>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct OpenedTeamDTO {
    pub name: String,
    pub short_name: String,
    pub logo: i32,
    pub logo_color: i32,
    pub invitation_id: String,
    pub captain_id: u64,
    pub tier: i32,
    pub members: Vec<OpenedTeamMemberDTO>,
    pub invitees: Vec<PendingRosterInviteeDTO>,
    pub open_positions: Vec<Position>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct OpenedTeamMemberDTO {
    pub player_id: i64,
    pub position: Position,
    pub tier: i32,
    pub friendship: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PartiesVoiceDTO {
    pub jwt: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PatcherChunkingPatcherEnvironment {
    pub game_patcher_available: bool,
    pub game_patcher_enabled: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PatcherComponentActionProgress {
    pub current_item: String,
    pub total: PatcherComponentStateProgress,
    pub network: PatcherComponentStateProgress,
    pub primary_work: PatcherComponentStateWorkType,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PatcherComponentState {
    pub id: String,
    pub action: PatcherComponentStateAction,
    pub is_up_to_date: bool,
    pub is_update_available: bool,
    pub time_of_last_up_to_date_check_i_s_o8601: Option<String>,
    pub is_corrupted: bool,
    pub progress: Option<PatcherComponentActionProgress>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PatcherComponentStateProgress {
    pub bytes_complete: u64,
    pub bytes_required: u64,
    pub bytes_per_second: f64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PatcherNotification {
    pub id: String,
    pub notification_id: PatcherNotificationId,
    pub data: HashMap<String, Value>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PatcherP2PStatus {
    pub is_enabled_for_patchline: bool,
    pub is_allowed_by_user: bool,
    pub requires_restart: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PatcherP2PStatusUpdate {
    pub is_allowed_by_user: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PatcherProductResource {
    pub id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PatcherProductState {
    pub id: String,
    pub action: PatcherComponentStateAction,
    pub is_up_to_date: bool,
    pub is_update_available: bool,
    pub is_corrupted: bool,
    pub is_stopped: bool,
    pub percent_patched: f64,
    pub components: Vec<PatcherComponentState>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PatcherStatus {
    pub connected_to_patch_server: bool,
    pub successfully_installed_version: Option<u32>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PatcherUxResource {
    pub visible: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PaymentsFrontEndRequest {
    pub is_prepaid: bool,
    pub locale_id: String,
    pub summoner_level: i16,
    pub giftee_account_id: String,
    pub giftee_message: String,
    pub rso_token: String,
    pub use_pmc_sessions: bool,
    pub game: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PaymentsFrontEndResult {
    pub url: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PaymentsPMCStartUrlRequest {
    pub is_prepaid: bool,
    pub locale_id: String,
    pub summoner_level: i16,
    pub giftee_account_id: String,
    pub giftee_message: String,
    pub game: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PaymentsPMCStartUrlResult {
    pub id: String,
    pub locale_id: String,
    pub user_id: String,
    pub summoner_level: i16,
    pub player_facing_id: String,
    pub pmc_start_url: String,
    pub created_at: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PendingOpenedTeamDTO {
    pub invitation_id: String,
    pub name: String,
    pub short_name: String,
    pub logo: i32,
    pub logo_color: i32,
    pub tier: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PendingRosterDTO {
    pub version: i32,
    pub tournament_id: i64,
    pub invitation_id: String,
    pub name: String,
    pub short_name: String,
    pub logo: i32,
    pub logo_color: i32,
    pub reward_logos: Vec<RewardLogo>,
    pub tier: i32,
    pub captain_id: u64,
    pub high_tier_variance: bool,
    pub members: Vec<PendingRosterMemberDTO>,
    pub invitees: Vec<PendingRosterInviteeDTO>,
    pub ticket_offers: Vec<TicketOfferDTO>,
    pub invite_faileds: Vec<FailedInvite>,
    pub lft: bool,
    pub multi_user_chat_j_w_t: String,
    pub muc_jwt_dto: MucJwtDto,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PendingRosterInviteeDTO {
    pub invitee_id: u64,
    pub invitee_state: PendingRosterInviteeState,
    pub inviter: u64,
    pub invite_time: i64,
    pub invite_type: InviteType,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PendingRosterMemberDTO {
    pub player_id: u64,
    pub member_state: PendingRosterMemberState,
    pub bet: i32,
    pub bet_type: TicketType,
    pub position: Position,
    pub join_time: i64,
    pub pending_pay: i32,
    pub pending_premium_pay: i32,
    pub self_bet: i32,
    pub tier: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PhaseInMember {
    pub player_id: u64,
    pub bet: i32,
    pub pending_pay: i32,
    pub self_bet: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PhaseRosterDTO {
    pub phase_id: i64,
    pub period: i32,
    pub bracket_id: i64,
    pub checkin_time: i64,
    pub bracket_d_t_o: Option<Bracket>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlayerBehaviorLcdsForcedClientShutdown {
    pub reason: String,
    pub additional_info: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlayerBehaviorLcdsSimpleMessage {
    pub account_id: u64,
    pub msg_id: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub params: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlayerBehaviorSimpleMessageResponse {
    pub account_id: u64,
    pub msg_id: String,
    pub command: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlayerDTO {
    pub banned: bool,
    pub tmnt_wins: i32,
    pub tmnt_losses: i32,
    pub notifications: Vec<ClashOfflineNotification>,
    pub lft: bool,
    pub primary_pos: String,
    pub secondary_pos: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlayerFinderDTO {
    pub player_id: u64,
    pub tier: i32,
    pub primary_pos: Position,
    pub secondary_pos: Position,
    #[serde(rename = "type")]
    pub type_: PlayerFinderEnum,
    pub friend_id: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlayerInfoDTO {
    pub player_id: u64,
    pub team_id: String,
    pub position: String,
    pub role: Role,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlayerInventory {
    pub ward_skins: Vec<i64>,
    pub champions: Vec<i32>,
    pub skins: Vec<i32>,
    pub icons: Vec<i32>,
    pub inventory_jwts: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlayerLootDTO {
    pub loot_name: String,
    pub ref_id: String,
    pub count: i32,
    pub expiry_time: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlayerLootDefinitionsDTO {
    pub loot_item_list: LootItemListClientDTO,
    pub recipe_list: LootLcdsRecipeListClientDTO,
    pub query_result: QueryResultDTO,
    pub player_loot: Vec<PlayerLootDTO>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlayerLootResultDTO {
    pub status: String,
    pub details: String,
    pub added: Vec<PlayerLootDTO>,
    pub removed: Vec<PlayerLootDTO>,
    pub redeemed: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlayerMissionDTO {
    pub id: String,
    pub title: String,
    pub helper_text: String,
    pub description: String,
    pub background_image_url: String,
    pub icon_image_url: String,
    pub series_name: String,
    pub locale: String,
    pub sequence: i32,
    pub metadata: MissionMetadata,
    pub start_time: i64,
    pub end_time: i64,
    pub last_updated_timestamp: i64,
    pub objectives: Vec<PlayerMissionObjectiveDTO>,
    pub rewards: Vec<PlayerMissionRewardDTO>,
    pub expiring_warnings: Vec<MissionAlertDTO>,
    pub requirements: Vec<String>,
    pub reward_strategy: RewardStrategy,
    pub display: MissionDisplay,
    pub completion_expression: String,
    pub viewed: bool,
    pub is_new: bool,
    pub status: String,
    pub mission_type: String,
    pub display_type: String,
    pub earned_date: i64,
    pub completed_date: i64,
    pub cooldown_time_millis: i64,
    pub celebration_type: String,
    pub client_notify_level: String,
    pub internal_name: String,
    pub media: HashMap<String, String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlayerMissionEligibilityData {
    pub level: i32,
    pub loyalty_enabled: bool,
    pub player_inventory: PlayerInventory,
    pub user_info_token: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlayerMissionObjectiveDTO {
    #[serde(rename = "type")]
    pub type_: String,
    pub description: String,
    pub progress: MissionProgressDTO,
    pub sequence: i32,
    pub reward_groups: Vec<String>,
    pub has_objective_based_reward: bool,
    pub status: String,
    pub requirements: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlayerMissionRewardDTO {
    pub reward_type: String,
    pub reward_group: String,
    pub description: String,
    pub icon_url: String,
    pub small_icon_url: String,
    pub item_id: String,
    pub unique_name: String,
    pub reward_fulfilled: bool,
    pub reward_group_selected: bool,
    pub sequence: i32,
    pub quantity: i32,
    pub is_objective_based_reward: bool,
    pub media: HashMap<String, String>,
    pub icon_needs_frame: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlayerNotificationsPlayerNotificationConfigResource {
    pub expiration_check_frequency: Option<u64>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlayerNotificationsPlayerNotificationResource {
    pub background_url: String,
    pub created: String,
    pub critical: bool,
    pub data: HashMap<String, String>,
    pub detail_key: String,
    pub expires: String,
    pub icon_url: String,
    pub id: u64,
    pub source: String,
    pub state: String,
    pub title_key: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub dismissible: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlayerNotificationsRiotMessagingServiceMessage {
    pub resource: String,
    pub service: String,
    pub version: String,
    pub timestamp: i64,
    pub payload: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlayerParticipant {
    pub summoner_id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlayerRosterHistoryDTO {
    pub rosters: Vec<RosterMemberDTO>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlayerTierDTO {
    pub player_id: u64,
    pub tier: i32,
    pub primary_pos: Position,
    pub second_pos: Position,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PluginDescriptionResource {
    pub name: String,
    pub version: String,
    pub riot_meta: PluginMetadataResource,
    pub plugin_dependencies: HashMap<String, String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PluginLcdsEvent {
    pub client_id: String,
    pub subtopic: String,
    pub type_name: String,
    pub body: Value,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PluginLcdsResponse {
    pub type_name: String,
    pub body: Value,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PluginManagerResource {
    pub state: PluginManagerState,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PluginMetadataResource {
    #[serde(rename = "type")]
    pub type_: String,
    pub subtype: String,
    pub app: String,
    pub feature: String,
    pub mock: String,
    pub has_bundled_assets: bool,
    pub global_asset_bundles: Vec<String>,
    pub per_locale_asset_bundles: HashMap<String, Value>,
    pub implements: HashMap<String, String>,
    pub threading: PluginThreadingModel,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PluginRegionLocaleChangedEvent {
    pub region: String,
    pub locale: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PluginResource {
    pub full_name: String,
    pub short_name: String,
    pub version: String,
    pub supertype: String,
    pub subtype: String,
    pub app: String,
    pub feature: String,
    pub threading_model: String,
    pub asset_bundle_names: Vec<String>,
    pub mounted_asset_bundles: HashMap<String, String>,
    pub order_wad_file_mounted: i32,
    pub dependencies: Vec<PluginResourceContract>,
    pub implemented_contracts: Vec<PluginResourceContract>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PluginResourceContract {
    pub full_name: String,
    pub version: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PluginResourceEvent {
    pub uri: String,
    pub event_type: PluginResourceEventType,
    pub data: Value,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PluginServiceProxyResponse {
    pub uuid: String,
    pub service_name: String,
    pub method_name: String,
    pub payload: String,
    pub status: String,
    pub error: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProcessControlProcess {
    pub status: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PublicRosterDTO {
    pub id: i64,
    pub tournament_id: i64,
    pub name: String,
    pub short_name: String,
    pub logo: i32,
    pub logo_color: i32,
    pub member_ids: Vec<u64>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Punishment {
    pub punished_for_chat_logs: Vec<String>,
    pub punished_for_game_ids: Vec<u64>,
    pub punishment_reason: String,
    pub punishment_type: String,
    pub punished_until_date_millis: u64,
    pub punishment_length_millis: u64,
    pub perma_ban: bool,
    pub punishment_length_games: i64,
    pub player_facing_message: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct QueryEvaluationRequestDTO {
    pub query: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct QueryEvaluationResultDTO {
    pub loot_item_names: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct QueryResultDTO {
    pub query_to_loot_names: Value,
    pub last_update: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct RankedScoutingDTO {
    pub player_id: u64,
    pub total_mastery_score: u64,
    pub top_masteries: Vec<ChampionMasteryPublicDTO>,
    pub top_season_champions: Vec<ChampionScoutingDTO>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct RankedScoutingMemberDTO {
    pub player_id: u64,
    pub champion_scouting_data: Vec<RankedScoutingTopChampionDTO>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct RankedScoutingTopChampionDTO {
    pub champion_id: i32,
    pub rank: i32,
    pub win_count: i32,
    pub game_count: i32,
    pub kda: f32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct RedeemLootTransactionDTO {
    pub client_id: String,
    pub puuid: String,
    pub player_id: u64,
    pub account_id: i64,
    pub loot_name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct RegionLocale {
    pub region: String,
    pub locale: String,
    pub web_region: String,
    pub web_language: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReplayMetadataV2 {
    pub game_version: String,
    pub file_size: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct RewardDetails {
    pub tournament_id: i64,
    pub roster_id: i64,
    pub team_member_ids: Vec<i64>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct RewardLogo {
    pub logo: i32,
    pub member_owned_count: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct RewardStrategy {
    pub group_strategy: String,
    pub select_max_group_count: u16,
    pub select_min_group_count: u16,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct RgaClientConfigConfigStatus {
    pub readiness: String,
    pub update_id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct RiotMessagingServiceAcknowledgeBody {
    pub id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct RiotMessagingServiceEntitlementsToken {
    pub access_token: String,
    pub token: String,
    pub subject: String,
    pub issuer: String,
    pub entitlements: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct RiotMessagingServicePluginRegionLocaleChangedEvent {
    pub region: String,
    pub locale: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct RiotMessagingServiceSession {
    pub state: RiotMessagingServiceState,
    pub token: String,
    pub token_type: RiotMessagingServiceTokenType,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct RmsMessage {
    pub id: String,
    pub resource: String,
    pub service: String,
    pub version: String,
    pub timestamp: i64,
    pub payload: String,
    pub ack_required: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct RosterAggregatedStatsDTO {
    pub period_stats: Vec<RosterPeriodAggregatedStatsDTO>,
    pub player_stats: HashMap<String, RosterPlayerAggregatedStatsDTO>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct RosterDTO {
    pub version: i32,
    pub id: i64,
    pub tournament_id: i64,
    pub invitation_id: String,
    pub name: String,
    pub short_name: String,
    pub logo: i32,
    pub logo_color: i32,
    pub tier: i32,
    pub points: i32,
    pub wins: i32,
    pub losses: i32,
    pub eliminated: bool,
    pub captain_id: u64,
    pub members: Vec<RosterMemberDTO>,
    pub phases: Vec<PhaseRosterDTO>,
    pub dynamic_state: RosterDynamicStateDTO,
    pub banned: bool,
    pub multi_user_chat_j_w_t: String,
    pub muc_jwt_dto: MucJwtDto,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct RosterDynamicStateDTO {
    pub version: i32,
    pub tournament_id: i64,
    pub roster_id: i64,
    pub phase_checkin_states: Vec<u64>,
    pub members: Vec<PhaseInMember>,
    pub ticket_offers: Vec<TicketOfferDTO>,
    pub withdraw: RosterWithdraw,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct RosterMatchAggregatedStatsDTO {
    pub round: i32,
    pub duration: i64,
    pub opponent_short_name: String,
    pub opponent_logo: i32,
    pub opponent_logo_color: i32,
    pub win: bool,
    pub loser_bracket: bool,
    pub game_id: i64,
    pub kills: i32,
    pub opponent_kills: i32,
    pub player_champion_ids: HashMap<String, i32>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct RosterMemberDTO {
    pub roster_id: i64,
    pub player_id: u64,
    pub tournament_id: i64,
    pub current_bid: i32,
    pub bid_type: TicketType,
    pub pending_spend: i32,
    pub pending_premium_spend: i32,
    pub position: Position,
    pub join_time: i64,
    pub tier: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct RosterPeriodAggregatedStatsDTO {
    pub period: i32,
    pub bracket_size: i32,
    pub time: i64,
    pub match_stats: Vec<RosterMatchAggregatedStatsDTO>,
    pub player_bids: HashMap<String, i32>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct RosterPlayerAggregatedStatsDTO {
    pub raw_stats_sum: HashMap<String, i32>,
    pub raw_stats_max: HashMap<String, i32>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct RosterStatsDTO {
    pub roster_id: i64,
    pub tournament_theme_id: i32,
    pub tournament_name_loc_key: String,
    pub tournament_name_loc_key_secondary: String,
    pub schedule_time: i64,
    pub schedule_end_time: i64,
    pub tournament_periods: i32,
    pub tier: i32,
    pub roster_name: String,
    pub roster_short_name: String,
    pub roster_logo: i32,
    pub roster_logo_color: i32,
    pub stats: RosterAggregatedStatsDTO,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct RosterWithdraw {
    pub init_vote_time: i64,
    pub init_vote_member: u64,
    pub vote_timeout_ms: i64,
    pub lockout_time_ms: i64,
    pub game_start_buffer_ms: i64,
    pub vote_withdraw_members: Vec<i64>,
    pub decline_withdraw_members: Vec<i64>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SLIBoolDiagnostic {
    pub key: String,
    pub value: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SLICount {
    pub sli_name: String,
    pub idempotency_key: String,
    pub successes: f64,
    pub failures: f64,
    pub start_time_epoch_ms: i64,
    pub end_time_epoch_ms: i64,
    pub labels: HashMap<String, String>,
    pub bool_diagnostics: HashMap<String, bool>,
    pub double_diagnostics: HashMap<String, f64>,
    pub int_diagnostics: HashMap<String, i64>,
    pub string_diagnostics: HashMap<String, String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SLIDoubleDiagnostic {
    pub key: String,
    pub value: f64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SLIIntDiagnostic {
    pub key: String,
    pub value: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SLILabel {
    pub key: String,
    pub value: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SLIStringDiagnostic {
    pub key: String,
    pub value: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SanitizerContainsSanitizedRequest {
    pub text: String,
    pub level: Option<u32>,
    pub aggressive_scan: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SanitizerContainsSanitizedResponse {
    pub contains: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SanitizerSanitizeRequest {
    pub texts: Option<Vec<String>>,
    pub text: Option<String>,
    pub level: Option<u32>,
    pub aggressive_scan: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SanitizerSanitizeResponse {
    pub texts: Option<Vec<String>>,
    pub text: Option<String>,
    pub modified: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SanitizerSanitizerStatus {
    pub ready: bool,
    pub region: String,
    pub locale: String,
    pub filtered_word_counts_by_level: HashMap<String, u32>,
    pub whitelisted_word_counts_by_level: HashMap<String, u32>,
    pub breaking_chars_count: u32,
    pub projected_chars_count: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SeriesDTO {
    pub id: String,
    pub internal_name: String,
    pub parent_internal_name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub eligibility_type: String,
    pub display_type: String,
    pub title: String,
    pub description: String,
    pub opt_in_button_text: String,
    pub opt_out_button_text: String,
    pub status: String,
    pub start_date: i64,
    pub end_date: i64,
    pub created_date: i64,
    pub last_updated_timestamp: i64,
    pub viewed: bool,
    pub media: SeriesMediaDTO,
    pub tags: Vec<String>,
    pub warnings: Vec<AlertDTO>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SeriesMediaDTO {
    pub background_url: String,
    pub background_image_large_url: String,
    pub background_image_small_url: String,
    pub tracker_icon_url: String,
    pub tracker_icon: String,
    pub accent_color: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ShutdownLcdsForcedClientShutdown {
    pub reason: String,
    pub additional_info: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SimpleDialogMessage {
    pub account_id: u64,
    pub msg_id: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub params: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SimpleDialogMessageResponse {
    pub account_id: u64,
    pub msg_id: String,
    pub command: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SpectatorLcdsSpectateAvailabilityDto {
    pub team_or_summoner_ids: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SpectatorLcdsSpectateAvailabilityResponseDto {
    pub available_for_watching: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct StoreLcdsChampionDTO {
    pub end_date: u64,
    pub purchase_date: u64,
    pub win_count_remaining: u32,
    pub sources: Vec<String>,
    pub active: bool,
    pub bot_enabled: bool,
    pub champion_id: i32,
    pub champion_skins: Vec<StoreLcdsChampionSkinDTO>,
    pub free_to_play: bool,
    pub free_to_play_reward: bool,
    pub f2p_reward_sources: Vec<String>,
    pub owned: bool,
    pub ranked_play_enabled: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct StoreLcdsChampionSkinDTO {
    pub end_date: u64,
    pub purchase_date: u64,
    pub win_count_remaining: i32,
    pub sources: Vec<String>,
    pub champion_id: i32,
    pub free_to_play_reward: bool,
    pub f2p_reward_sources: Vec<String>,
    pub last_selected: bool,
    pub owned: bool,
    pub skin_id: i32,
    pub still_obtainable: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct StoreLcdsSimpleDialogMessage {
    pub account_id: u64,
    pub msg_id: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub params: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct StoreLcdsSimpleDialogMessageResponse {
    pub account_id: u64,
    pub msg_id: String,
    pub command: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct StoreLcdsStoreAccountBalanceNotification {
    pub ip: i64,
    pub rp: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct StoreLcdsStoreFulfillmentNotification {
    pub inventory_type: String,
    pub rp: i64,
    pub ip: i64,
    pub data: Value,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ThemeVp {
    pub theme_id: i32,
    pub theme_vp: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct TicketOfferDTO {
    pub offer_player_id: u64,
    pub receive_player_id: u64,
    pub count: i32,
    #[serde(rename = "type")]
    pub type_: TicketType,
    pub ticket_offer_state: TicketOfferState,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct TierConfig {
    pub tier: i32,
    pub delay_time: i64,
    pub estimate_time: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct TimeSeriesEventBeginV1 {
    pub when: u64,
    pub event_name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct TimeSeriesEventEndV1 {
    pub when: u64,
    pub event_name: String,
    pub suffix: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct TimeSeriesEventMarkerV1 {
    pub when: u64,
    pub event_name: String,
    pub marker_name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct TournamentDTO {
    pub id: i64,
    pub schedule_time: i64,
    pub schedule_end_time: i64,
    pub roster_create_deadline: i64,
    pub entry_fee: i32,
    pub roster_size: i32,
    pub theme_id: i32,
    pub name_loc_key: String,
    pub name_loc_key_secondary: String,
    pub buy_in_options: Vec<i32>,
    pub buy_in_options_premium: Vec<i32>,
    pub queue_id: i32,
    pub scouting_time_ms: i64,
    pub last_theme_of_season: bool,
    pub bracket_size: String,
    pub min_games: i32,
    pub sms_restriction: bool,
    pub honor_restriction: bool,
    pub rank_restriction: bool,
    pub voice_enabled: bool,
    pub phases: Vec<TournamentPhaseDTO>,
    pub reward_config: Vec<ClashRewardConfigClient>,
    pub tier_configs: Vec<TierConfig>,
    pub bracket_formation_init_delay_ms: i64,
    pub bracket_formation_interval_ms: i64,
    pub status: TournamentStatusEnum,
    pub resume_time: i64,
    pub lft: bool,
    pub max_invites: i32,
    pub max_suggestions_per_player: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct TournamentHistoryAndWinnersDTO {
    pub tournament_history: Vec<TournamentDTO>,
    pub tournament_winners_compressed: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct TournamentInfoDTO {
    pub tournament: TournamentDTO,
    pub roster: Option<RosterDTO>,
    pub pending_roster: Option<PendingRosterDTO>,
    pub invitee_pending_rosters: Vec<PendingRosterDTO>,
    pub theme_vp: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct TournamentInfoMinimalDTO {
    pub tournament_info: Vec<TournamentInfoDTO>,
    pub time: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct TournamentPhaseDTO {
    pub id: i64,
    pub tournament_id: i64,
    pub registration_time: i64,
    pub start_time: i64,
    pub period: i32,
    pub cancelled: bool,
    pub limit_tiers: Vec<i32>,
    pub capacity_status: CapacityEnum,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct TournamentPlayerInfoDTO {
    pub tournament_info: Vec<TournamentInfoDTO>,
    pub roster_stats: Vec<RosterStatsDTO>,
    pub player: PlayerDTO,
    pub season_vp: i32,
    pub theme_vps: Vec<ThemeVp>,
    pub time: i64,
    pub tier: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct TracingCriticalFlowEventV1 {
    pub when: u64,
    pub event_id: String,
    pub succeeded: bool,
    pub payload_string: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct TracingEventV1 {
    pub when: u64,
    pub name: String,
    pub src: String,
    pub dest: String,
    pub tags: String,
    pub details: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct TracingModuleV1 {
    pub module_id: u32,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: TracingModuleTypeV1,
    pub threading_model: TracingModuleThreadingModelV1,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct TracingPhaseBeginV1 {
    pub when: u64,
    pub name: String,
    pub importance: TracingPhaseImportanceV1,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct TracingPhaseEndV1 {
    pub when: u64,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct TutorialMetadata {
    pub step_number: i32,
    pub queue_id: String,
    pub display_rewards: HashMap<String, String>,
    pub use_quick_search_matchmaking: bool,
    pub use_chosen_champion: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct VerboseLootOddsDTO {
    pub recipe_name: String,
    pub last_updated: String,
    pub chance_to_contain: Vec<LootOddsDTO>,
    pub guaranteed_to_contain: Vec<LootOddsDTO>,
    pub has_pity_rules: bool,
    pub checks_ownership: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct YourshopLcdsChampionDTO {
    pub champion_id: i32,
    pub owned: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct YourshopLcdsChampionSkinDTO {
    pub champion_id: i32,
    pub owned: bool,
    pub skin_id: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct YourshopLcdsClientDynamicConfigurationNotification {
    pub configs: String,
    pub delta: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct YourshopStoreFulfillmentNotification {
    pub inventory_type: String,
    pub data: Value,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct BasicOperatingSystemInfo {
    pub edition: String,
    pub platform: String,
    pub version_major: String,
    pub version_minor: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct BasicSystemInfo {
    pub operating_system: BasicOperatingSystemInfo,
    pub physical_memory: u64,
    pub physical_processor_cores: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Cookie {
    pub url: String,
    pub name: String,
    pub value: String,
    pub domain: String,
    pub path: String,
    pub secure: bool,
    pub httponly: bool,
    pub expires: Option<i64>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum AggregationType {
    none = 0,
    sum = 1,
    average = 2,
}

impl Default for AggregationType {
    fn default() -> Self {
        AggregationType::none
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum BindingAsyncState {
    None = 0,
    Running = 1,
    Cancelling = 2,
    Cancelled = 3,
    Succeeded = 4,
    Failed = 5,
}

impl Default for BindingAsyncState {
    fn default() -> Self {
        BindingAsyncState::None
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum BindingHelpFormat {
    Full = 1,
    Epytext = 2,
}

impl Default for BindingHelpFormat {
    fn default() -> Self {
        BindingHelpFormat::Full
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum CapacityEnum {
    LOW = 0,
    MEDIUM = 1,
    HIGH = 2,
    FULL = 3,
}

impl Default for CapacityEnum {
    fn default() -> Self {
        CapacityEnum::LOW
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum ClashRewardKeyType {
    TIER = 0,
    CUP = 1,
    TICKET_COUNT = 2,
    TICKET_TYPE = 3,
    LOWEST_POSITION = 4,
    TOURNAMENT_WIN_POS = 5,
    WINS = 6,
    POINTS = 7,
    THEME_VP = 8,
    SEASON_VP = 9,
    SEASON_FLAG_COUNT = 10,
    TOC_STATE = 11,
}

impl Default for ClashRewardKeyType {
    fn default() -> Self {
        ClashRewardKeyType::TIER
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum ClashRewardTime {
    NONE = 0,
    EOG = 1,
    EOB = 2,
    EOT = 3,
}

impl Default for ClashRewardTime {
    fn default() -> Self {
        ClashRewardTime::NONE
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum ClashRewardType {
    TROPHY = 0,
    FLAG = 1,
    FRAME = 2,
    LOGO = 3,
    LOOT = 4,
    VP = 5,
    TOC = 6,
}

impl Default for ClashRewardType {
    fn default() -> Self {
        ClashRewardType::TROPHY
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum ClientBracketMatchStatus {
    UPCOMING = 0,
    STARTED = 1,
    COMPLETED = 2,
}

impl Default for ClientBracketMatchStatus {
    fn default() -> Self {
        ClientBracketMatchStatus::UPCOMING
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum ClientConfigDepInjectorEntitlementsUpdateType {
    Create = 0,
    Update = 1,
    Delete = 2,
}

impl Default for ClientConfigDepInjectorEntitlementsUpdateType {
    fn default() -> Self {
        ClientConfigDepInjectorEntitlementsUpdateType::Create
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum ClientRequestError {
    ALREADY_IN_GAME = 0,
    ALREADY_MEMBER = 1,
    ALREADY_INVITED = 2,
    ALREADY_SUGGESTED = 3,
    ALREADY_DECLINE_WITHDRAW = 4,
    ALREADY_DECLINED = 5,
    ALREADY_VOTE_WITHDRAW = 6,
    ALREADY_IN_PHASE = 7,
    CAPTAIN_NOT_ALLOWED = 8,
    CLASH_BANNED = 9,
    CLASH_BANNED_INVITEE = 10,
    ELIGIBILITY_SERVER_ERROR = 11,
    FAIL_INVITE = 12,
    FAIL_SUGGESTINVITE = 13,
    HONOR_INELIGIBILITY = 14,
    LOGO_NOT_ALLOWED = 15,
    LOGOCOLOR_NOT_ALLOWED = 16,
    IN_OTHER_ROSTER = 17,
    IN_OTHER_PENDINGROSTER = 18,
    IN_OTHER_PHASE_OF_PERIOD = 19,
    INACTIVE_REGISTRATION = 20,
    INACTIVE_PHASE = 21,
    INTERNAL_ERROR = 22,
    INVALID_ROSTER = 23,
    INVALID_BUY_BACK = 24,
    INVALID_TOURNAMENT = 25,
    INVALID_PHASE = 26,
    INVALID_INVITEE = 27,
    INVALID_ROSTER_MEMBER_SIZE = 28,
    INVALID_POSITION = 29,
    INVALID_LOGO = 30,
    INVALID_LOGOCOLOR = 31,
    INVALID_NAME = 32,
    INVALID_SHORTNAME = 33,
    INVALID_CHECKELIGIBILITY_SIZE = 34,
    INVALID_BRACKET = 35,
    INVALID_MATCHID = 36,
    INVALID_PLAYER = 37,
    INVALID_Tier = 38,
    INVALID_WITHDRAW = 39,
    INVALID_MATCHSTATUS_FORGAMEEND = 40,
    INVALID_REWARD_CONFIG_NAME = 41,
    INVALID_SEASON = 42,
    MAX_INVITED = 43,
    MAX_ROSTER_FETCHSIZE = 44,
    NO_SAME_PLAYER = 45,
    NO_AVAILABLE_PHASE = 46,
    NOT_CAPTAIN = 47,
    NOT_MEMBER = 48,
    NOT_INVITEE = 49,
    NOT_SEED_INTO_LEAGUE = 50,
    NOT_ENOUGH_TICKETS = 51,
    NOT_ALLOWED_DELETE_TOURNAMENT = 52,
    NOT_ALLOWED_DELETE_TOURNAMENT_REWARD_CONFIG = 53,
    NO_PERMISSION = 54,
    NO_MORE_RECOMMEND = 55,
    OVER_SUGGESTION_INVITE = 56,
    OVER_INVITE = 57,
    PENDING_ROSTER_NOT_READY = 58,
    PENDING_ROSTER_FULL = 59,
    PENDING_ROSTER_CLOSE = 60,
    PHASE_CANCELLED = 61,
    PHASE_FULL = 62,
    ROSTER_ELIMINATED = 63,
    ROSTER_DISBAND_NOT_ALLOWED = 64,
    SUGGEST_INVITEE_NOT_EXIST = 65,
    SUMMONER_LEVEL_REQUIREMENT_NOT_MET = 66,
    SMS_NOT_VERIFIED = 67,
    TICKET_ALREADY_SET = 68,
    TICKET_OFFER_NOT_EXIST = 69,
    TICKET_OFFER_INVALID_COUNT = 70,
    TICKET_NOT_SET = 71,
    VOICE_NOT_AVAILABLE = 72,
    WITHDRAW_NOT_ALLOWED = 73,
    WITHDRAW_CANCEL_NOT_ALLOWED = 74,
    WITHDRAW_LOCKOUT = 75,
}

impl Default for ClientRequestError {
    fn default() -> Self {
        ClientRequestError::ALREADY_IN_GAME
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum ElevationAction {
    FixBrokenPermissions = 1,
}

impl Default for ElevationAction {
    fn default() -> Self {
        ElevationAction::FixBrokenPermissions
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum ExternalPluginsAvailability {
    NotAvailable = 0,
    Preparing = 1,
    Connected = 2,
    Recovering = 3,
    Error = 4,
}

impl Default for ExternalPluginsAvailability {
    fn default() -> Self {
        ExternalPluginsAvailability::NotAvailable
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum GameQueuesLcdsAllowSpectators {
    NONE = 0,
    LOBBYONLY = 1,
    DROPINONLY = 2,
    ALL = 3,
}

impl Default for GameQueuesLcdsAllowSpectators {
    fn default() -> Self {
        GameQueuesLcdsAllowSpectators::NONE
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum InviteType {
    FREEAGENT = 0,
    FRIEND = 1,
    SUGGEST = 2,
    SELFJOIN = 3,
    NONE = 4,
}

impl Default for InviteType {
    fn default() -> Self {
        InviteType::FREEAGENT
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LCDSLoyaltyStateChangeNotificationCategory {
    legacy = 0,
    grant = 1,
    expiry = 2,
    change = 3,
    revoke = 4,
    disabled = 5,
}

impl Default for LCDSLoyaltyStateChangeNotificationCategory {
    fn default() -> Self {
        LCDSLoyaltyStateChangeNotificationCategory::legacy
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LcdsInvitationState {
    ACTIVE = 0,
    ON_HOLD = 1,
    REVOKED = 2,
}

impl Default for LcdsInvitationState {
    fn default() -> Self {
        LcdsInvitationState::ACTIVE
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LcdsInviteeState {
    CREATOR = 0,
    PENDING = 1,
    DECLINED = 2,
    ACCEPTED = 3,
    ACCEPT_FAILED = 4,
    JOINED = 5,
    QUIT = 6,
    KICKED = 7,
    BANNED = 8,
}

impl Default for LcdsInviteeState {
    fn default() -> Self {
        LcdsInviteeState::CREATOR
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LcdsRemovalReason {
    KICKED = 0,
    DESTROYED = 1,
    PROGRESSED = 2,
}

impl Default for LcdsRemovalReason {
    fn default() -> Self {
        LcdsRemovalReason::KICKED
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LogSeverityLevels {
    Okay = 0,
    Warning = 1,
    Error = 2,
    Always = 3,
}

impl Default for LogSeverityLevels {
    fn default() -> Self {
        LogSeverityLevels::Okay
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolAccountVerificationLoginSessionState {
    IN_PROGRESS = 0,
    SUCCEEDED = 1,
    LOGGING_OUT = 2,
    ERROR = 3,
}

impl Default for LolAccountVerificationLoginSessionState {
    fn default() -> Self {
        LolAccountVerificationLoginSessionState::IN_PROGRESS
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolAntiAddictionPolicyType {
    antiAddictionWarning = 0,
    antiAddictionShutdown = 1,
    antiAddictionHeartbeat = 2,
}

impl Default for LolAntiAddictionPolicyType {
    fn default() -> Self {
        LolAntiAddictionPolicyType::antiAddictionWarning
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolCareerStatsCareerStatsQueueType {
    draft5 = 0,
    rank5flex = 1,
    rank5solo = 2,
    blind5 = 3,
    aram = 4,
    blind3 = 5,
    rank3flex = 6,
    other = 7,
}

impl Default for LolCareerStatsCareerStatsQueueType {
    fn default() -> Self {
        LolCareerStatsCareerStatsQueueType::draft5
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolCareerStatsRankedTier {
    ALL = 0,
    UNRANKED = 1,
    IRON = 2,
    BRONZE = 3,
    SILVER = 4,
    GOLD = 5,
    PLATINUM = 6,
    DIAMOND = 7,
    MASTER = 8,
    GRANDMASTER = 9,
    CHALLENGER = 10,
}

impl Default for LolCareerStatsRankedTier {
    fn default() -> Self {
        LolCareerStatsRankedTier::ALL
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolCareerStatsSummonersRiftPosition {
    ALL = 0,
    UNKNOWN = 1,
    TOP = 2,
    JUNGLE = 3,
    MID = 4,
    BOTTOM = 5,
    SUPPORT = 6,
}

impl Default for LolCareerStatsSummonersRiftPosition {
    fn default() -> Self {
        LolCareerStatsSummonersRiftPosition::ALL
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolCatalogInventoryOwnership {
    OWNED = 0,
    RENTED = 1,
    LOYALTY = 2,
    F2P = 3,
}

impl Default for LolCatalogInventoryOwnership {
    fn default() -> Self {
        LolCatalogInventoryOwnership::OWNED
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolChallengesChallengeRequirementMappingName {
    NONE = 0,
    CHAMPION = 1,
    CHAMPION_SKIN = 2,
    ITEM = 3,
}

impl Default for LolChallengesChallengeRequirementMappingName {
    fn default() -> Self {
        LolChallengesChallengeRequirementMappingName::NONE
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolChallengesClientState {
    Hidden = 0,
    Disabled = 1,
    DarkHidden = 2,
    DarkDisabled = 3,
    Enabled = 4,
}

impl Default for LolChallengesClientState {
    fn default() -> Self {
        LolChallengesClientState::Hidden
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolChallengesGameflowPhase {
    None = 0,
    Lobby = 1,
    Matchmaking = 2,
    CheckedIntoTournament = 3,
    ReadyCheck = 4,
    ChampSelect = 5,
    GameStart = 6,
    FailedToLaunch = 7,
    InProgress = 8,
    Reconnect = 9,
    WaitingForStats = 10,
    PreEndOfGame = 11,
    EndOfGame = 12,
    TerminatedInError = 13,
}

impl Default for LolChallengesGameflowPhase {
    fn default() -> Self {
        LolChallengesGameflowPhase::None
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolChallengesNotificationDisplayType {
    NONE = 0,
    TOAST = 1,
    VIGNETTE = 2,
}

impl Default for LolChallengesNotificationDisplayType {
    fn default() -> Self {
        LolChallengesNotificationDisplayType::NONE
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolChallengesSource {
    CHALLENGES = 0,
    EOGD = 1,
    CAP_INVENTORY = 2,
    HONOR = 3,
    CHAMPION_MASTERY = 4,
    RANKED_LEAGUES = 5,
    CLASH = 6,
    LOOT = 7,
    ETERNALS = 8,
    SUMMONER_SERVICE = 9,
    NONE = 10,
}

impl Default for LolChallengesSource {
    fn default() -> Self {
        LolChallengesSource::CHALLENGES
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolChampSelectChampSelectSwapState {
    AVAILABLE = 1,
    BUSY = 2,
    INVALID = 3,
    RECEIVED = 4,
    SENT = 5,
    DECLINED = 6,
    CANCELLED = 7,
    ACCEPTED = 8,
}

impl Default for LolChampSelectChampSelectSwapState {
    fn default() -> Self {
        LolChampSelectChampSelectSwapState::AVAILABLE
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolChampSelectChampSelectTradeState {
    AVAILABLE = 1,
    BUSY = 2,
    INVALID = 3,
    RECEIVED = 4,
    SENT = 5,
    DECLINED = 6,
    CANCELLED = 7,
    ACCEPTED = 8,
}

impl Default for LolChampSelectChampSelectTradeState {
    fn default() -> Self {
        LolChampSelectChampSelectTradeState::AVAILABLE
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolChampSelectLegacyChampSelectTradeState {
    AVAILABLE = 1,
    BUSY = 2,
    INVALID = 3,
    RECEIVED = 4,
    SENT = 5,
}

impl Default for LolChampSelectLegacyChampSelectTradeState {
    fn default() -> Self {
        LolChampSelectLegacyChampSelectTradeState::AVAILABLE
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolChampSelectLegacyGameflowGameDodgeState {
    Invalid = 48,
    PartyDodged = 49,
    StrangerDodged = 50,
    TournamentDodged = 51,
}

impl Default for LolChampSelectLegacyGameflowGameDodgeState {
    fn default() -> Self {
        LolChampSelectLegacyGameflowGameDodgeState::Invalid
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolChampSelectLegacyGameflowPhase {
    None = 0,
    Lobby = 1,
    Matchmaking = 2,
    CheckedIntoTournament = 3,
    ReadyCheck = 4,
    ChampSelect = 5,
    GameStart = 6,
    FailedToLaunch = 7,
    InProgress = 8,
    Reconnect = 9,
    WaitingForStats = 10,
    PreEndOfGame = 11,
    EndOfGame = 12,
    TerminatedInError = 13,
}

impl Default for LolChampSelectLegacyGameflowPhase {
    fn default() -> Self {
        LolChampSelectLegacyGameflowPhase::None
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolChampSelectLegacyLoginSessionStates {
    IN_PROGRESS = 0,
    SUCCEEDED = 1,
    LOGGING_OUT = 2,
    ERROR = 3,
}

impl Default for LolChampSelectLegacyLoginSessionStates {
    fn default() -> Self {
        LolChampSelectLegacyLoginSessionStates::IN_PROGRESS
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolChampionsLoginSessionStates {
    IN_PROGRESS = 0,
    SUCCEEDED = 1,
    LOGGING_OUT = 2,
    ERROR = 3,
}

impl Default for LolChampionsLoginSessionStates {
    fn default() -> Self {
        LolChampionsLoginSessionStates::IN_PROGRESS
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolChampionsLoyaltyStatus {
    LEGACY = 0,
    REWARDS_GRANT = 1,
    EXPIRY = 2,
    CHANGE = 3,
    REVOKE = 4,
    DISABLED = 5,
}

impl Default for LolChampionsLoyaltyStatus {
    fn default() -> Self {
        LolChampionsLoyaltyStatus::LEGACY
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolChatAccountState {
    offline = 1,
    mobile = 2,
    away = 3,
    chat = 4,
    dnd = 5,
}

impl Default for LolChatAccountState {
    fn default() -> Self {
        LolChatAccountState::offline
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolChatAuthType {
    rsoCreate = 0,
    rsoRefresh = 1,
}

impl Default for LolChatAuthType {
    fn default() -> Self {
        LolChatAuthType::rsoCreate
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolChatChatPlatformLoginSessionState {
    IN_PROGRESS = 0,
    SUCCEEDED = 1,
    LOGGING_OUT = 2,
    ERROR = 3,
}

impl Default for LolChatChatPlatformLoginSessionState {
    fn default() -> Self {
        LolChatChatPlatformLoginSessionState::IN_PROGRESS
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolChatChatSessionState {
    disconnected = 0,
    connecting = 1,
    connected = 2,
}

impl Default for LolChatChatSessionState {
    fn default() -> Self {
        LolChatChatSessionState::disconnected
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolChatConfigReadinessEnum {
    NotReady = 0,
    Ready = 1,
    Disabled = 2,
}

impl Default for LolChatConfigReadinessEnum {
    fn default() -> Self {
        LolChatConfigReadinessEnum::NotReady
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolChatConfigType {
    public = 0,
    player = 1,
}

impl Default for LolChatConfigType {
    fn default() -> Self {
        LolChatConfigType::public
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolChatFriendRequestDirection {
    in_ = 0,
    out = 1,
    both = 2,
}

impl Default for LolChatFriendRequestDirection {
    fn default() -> Self {
        LolChatFriendRequestDirection::in_
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolChatFriendSubscriptionType {
    pending_out = 0,
    pending_in = 1,
}

impl Default for LolChatFriendSubscriptionType {
    fn default() -> Self {
        LolChatFriendSubscriptionType::pending_out
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolChatGameflowPhase {
    None = 0,
    Lobby = 1,
    Matchmaking = 2,
    CheckedIntoTournament = 3,
    ReadyCheck = 4,
    ChampSelect = 5,
    GameStart = 6,
    FailedToLaunch = 7,
    InProgress = 8,
    Reconnect = 9,
    WaitingForStats = 10,
    PreEndOfGame = 11,
    EndOfGame = 12,
    TerminatedInError = 13,
}

impl Default for LolChatGameflowPhase {
    fn default() -> Self {
        LolChatGameflowPhase::None
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolChatLeagueDivision {
    I = 0,
    II = 1,
    III = 2,
    IV = 3,
    V = 4,
    NA = 5,
}

impl Default for LolChatLeagueDivision {
    fn default() -> Self {
        LolChatLeagueDivision::I
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolChatLeagueQueueType {
    NONE = 0,
    RANKED_SOLO_5x5 = 1,
    RANKED_FLEX_SR = 2,
    RANKED_FLEX_TT = 3,
    RANKED_TFT = 4,
}

impl Default for LolChatLeagueQueueType {
    fn default() -> Self {
        LolChatLeagueQueueType::NONE
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolChatLeagueTier {
    NONE = 0,
    IRON = 1,
    BRONZE = 2,
    SILVER = 3,
    GOLD = 4,
    PLATINUM = 5,
    DIAMOND = 6,
    MASTER = 7,
    GRANDMASTER = 8,
    CHALLENGER = 9,
}

impl Default for LolChatLeagueTier {
    fn default() -> Self {
        LolChatLeagueTier::NONE
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolChatMessageType {
    chat = 0,
    groupchat = 1,
    dm = 2,
    system = 3,
}

impl Default for LolChatMessageType {
    fn default() -> Self {
        LolChatMessageType::chat
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolChatMuteType {
    PLAYER_MUTE = 0,
    SETTINGS_MUTE = 1,
    SYSTEM_MUTE = 2,
}

impl Default for LolChatMuteType {
    fn default() -> Self {
        LolChatMuteType::PLAYER_MUTE
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolChatQueueCustomGameSpectatorPolicy {
    NotAllowed = 0,
    LobbyAllowed = 1,
    FriendsAllowed = 2,
    AllAllowed = 3,
}

impl Default for LolChatQueueCustomGameSpectatorPolicy {
    fn default() -> Self {
        LolChatQueueCustomGameSpectatorPolicy::NotAllowed
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolChatSessionState {
    initializing = 0,
    connected = 1,
    loaded = 2,
    disconnected = 3,
    shuttingdown = 4,
}

impl Default for LolChatSessionState {
    fn default() -> Self {
        LolChatSessionState::initializing
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolClashClashState {
    Disabled = 0,
    Enabled = 1,
}

impl Default for LolClashClashState {
    fn default() -> Self {
        LolClashClashState::Disabled
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolClashClashVisibility {
    Hidden = 0,
    Visible = 1,
}

impl Default for LolClashClashVisibility {
    fn default() -> Self {
        LolClashClashVisibility::Hidden
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolClashFoundationError {
    CLASH_NOT_INITIALIZED = 0,
    CLASH_DISABLED = 1,
    DESERIALIZATION_FAILED = 2,
    GAMEFLOW_UNAVAILABLE = 3,
    LOL_INVENTORY_NOT_READY = 4,
    INVALID_SIMPLE_STATE_FLAG = 5,
}

impl Default for LolClashFoundationError {
    fn default() -> Self {
        LolClashFoundationError::CLASH_NOT_INITIALIZED
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolClashGameflowPhase {
    None = 0,
    Lobby = 1,
    Matchmaking = 2,
    CheckedIntoTournament = 3,
    ReadyCheck = 4,
    ChampSelect = 5,
    GameStart = 6,
    FailedToLaunch = 7,
    InProgress = 8,
    Reconnect = 9,
    WaitingForStats = 10,
    PreEndOfGame = 11,
    EndOfGame = 12,
    TerminatedInError = 13,
}

impl Default for LolClashGameflowPhase {
    fn default() -> Self {
        LolClashGameflowPhase::None
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolClashKdaClassification {
    LOW = 0,
    AVERAGE = 1,
    HIGH = 2,
}

impl Default for LolClashKdaClassification {
    fn default() -> Self {
        LolClashKdaClassification::LOW
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolClashLoginSessionState {
    IN_PROGRESS = 0,
    SUCCEEDED = 1,
    LOGGING_OUT = 2,
    ERROR = 3,
}

impl Default for LolClashLoginSessionState {
    fn default() -> Self {
        LolClashLoginSessionState::IN_PROGRESS
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolClashMatchmakingDodgeState {
    Invalid = 48,
    PartyDodged = 49,
    StrangerDodged = 50,
    TournamentDodged = 51,
}

impl Default for LolClashMatchmakingDodgeState {
    fn default() -> Self {
        LolClashMatchmakingDodgeState::Invalid
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolClashMatchmakingDodgeWarning {
    None = 0,
    Warning = 1,
    Penalty = 2,
}

impl Default for LolClashMatchmakingDodgeWarning {
    fn default() -> Self {
        LolClashMatchmakingDodgeWarning::None
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolClashMatchmakingReadyCheckResponse {
    None = 48,
    Accepted = 49,
    Declined = 50,
}

impl Default for LolClashMatchmakingReadyCheckResponse {
    fn default() -> Self {
        LolClashMatchmakingReadyCheckResponse::None
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolClashMatchmakingReadyCheckState {
    Invalid = 0,
    InProgress = 1,
    EveryoneReady = 2,
    StrangerNotReady = 3,
    PartyNotReady = 4,
    Error = 5,
}

impl Default for LolClashMatchmakingReadyCheckState {
    fn default() -> Self {
        LolClashMatchmakingReadyCheckState::Invalid
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolClashNotifyReason {
    SUGGESTION = 0,
    DECLINE_SUGGESTION = 1,
    ACCEPT_SUGGESTION = 2,
    REVOKE_SUGGESTION = 3,
    DECLINE_SELFJOIN = 4,
    ACCEPT_SELFJOIN = 5,
    REVOKE_SELFJOIN = 6,
    SELFJOIN = 7,
    READY = 8,
    UNREADY = 9,
    OWNER_CLOSE = 10,
    DISMISS = 11,
    ROSTER_DELETE = 12,
    OWNER_TRANSFER = 13,
    CHANGE_LOGO = 14,
    CHANGE_NAME = 15,
    CHANGE_SHORTNAME = 16,
    CHANGE_POSITION = 17,
    CHANGE_NAMETAGLOGO = 18,
    CHANGE_LFT = 19,
    INVITE = 20,
    RESENT_INVITE = 21,
    DECLINE_INVITE = 22,
    ACCEPT_INVITE = 23,
    REVOKE_INVITE = 24,
    LEAVE = 25,
    CAPTAIN_LEAVE = 26,
    KICK = 27,
    SET_TICKET = 28,
    OFFER_TICKET = 29,
    REVOKED_TICKET = 30,
    DECLINE_TICKET = 31,
    ACCEPT_TICKET = 32,
    REWARD_GRANT_FAILED = 33,
    REWARD_GRANT_RETRY = 34,
    REVERTED_REGISTRATION = 35,
    BAN = 36,
    UNBAN = 37,
    MEMBER_BAN = 38,
    TEAMMATE_BAN = 39,
    TEAMMATE_UNBAN = 40,
}

impl Default for LolClashNotifyReason {
    fn default() -> Self {
        LolClashNotifyReason::SUGGESTION
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolClashPlayerState {
    NO_ROSTER = 0,
    PENDING_ROSTER = 1,
    REGISTERED_ROSTER = 2,
    BRACKET_ROSTER = 3,
    ELIMINATED = 4,
}

impl Default for LolClashPlayerState {
    fn default() -> Self {
        LolClashPlayerState::NO_ROSTER
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolClashPresenceState {
    NONE = 0,
    LOCKED_IN = 1,
    SCOUTING = 2,
}

impl Default for LolClashPresenceState {
    fn default() -> Self {
        LolClashPresenceState::NONE
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolClashQueueAvailability {
    Available = 0,
    PlatformDisabled = 1,
    DoesntMeetRequirements = 2,
}

impl Default for LolClashQueueAvailability {
    fn default() -> Self {
        LolClashQueueAvailability::Available
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolClashQueueGameCategory {
    None = 0,
    Custom = 1,
    PvP = 2,
    VersusAi = 3,
    Alpha = 4,
}

impl Default for LolClashQueueGameCategory {
    fn default() -> Self {
        LolClashQueueGameCategory::None
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolClashRosterMemberState {
    DECLINED = 0,
    PENDING = 1,
    NOT_READY = 2,
    FORCED_NOT_READY = 3,
    READY = 4,
}

impl Default for LolClashRosterMemberState {
    fn default() -> Self {
        LolClashRosterMemberState::DECLINED
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolClashRosterNotifyReason {
    ROSTER_SET_TICKET = 0,
    ROSTER_OFFER_TICKET = 1,
    ROSTER_ACCEPT_TICKET = 2,
    ROSTER_DECLINE_TICKET = 3,
    ROSTER_REVOKED_TICKET = 4,
    BYE_AUTO_WIN = 5,
    BRACKET_READY = 6,
    CHANGE_POSITION = 7,
    EOG_PLAYER_UPDATE = 8,
    REGISTERED = 9,
    RESTRICTION_AUTO_WIN = 10,
    PHASE_UNREADY = 11,
    PHASE_READY = 12,
    PHASE_CHECKIN = 13,
    PHASE_BACKOUT = 14,
    PERIOD_CANCEL = 15,
    PERIOD_SPLIT = 16,
    GAME_COMPLETED = 17,
    GAME_SCHEDULED = 18,
    GAME_STARTED = 19,
    GAME_STARTED_ERROR = 20,
    GAME_END_ERROR = 21,
    QUEUE_DODGE = 22,
    OWNER_TRANSFER = 23,
    VOTE_WITHDRAW_UPDATE = 24,
    VOTE_WITHDRAW_DISMISS = 25,
    WITHDRAW = 26,
    ROUND_COMPLETE = 27,
    NO_SHOW_PING = 28,
    TIER_CHANGED = 29,
    BRACKET_ROSTER_REMOVED = 30,
    BRACKET_ROSTER_REPLACED = 31,
    CANNOT_FIND_MATCH = 32,
    BANNED_SMURF = 33,
    BANNED_SMURF_TEAMMATE = 34,
    BANNED_SMURF_OPPONENT = 35,
    TICKET_CHARGED = 36,
    TICKET_REFUNDED = 37,
    TICKET_COULD_NOT_BE_CHARGED = 38,
    GAME_START_RETRY = 39,
    GAME_START_RETRY_SUMMONERS = 40,
    GAME_START_RETRY_OPPONENT = 41,
    GAME_START_FAILED = 42,
    GAME_START_FAILED_SUMMONERS = 43,
    GAME_START_FAILED_OPPONENT = 44,
    GAME_RESCHEDULED = 45,
}

impl Default for LolClashRosterNotifyReason {
    fn default() -> Self {
        LolClashRosterNotifyReason::ROSTER_SET_TICKET
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolClashSimpleStateStatus {
    UNACKNOWLEDGED = 0,
    ACKNOWLEDGED = 1,
}

impl Default for LolClashSimpleStateStatus {
    fn default() -> Self {
        LolClashSimpleStateStatus::UNACKNOWLEDGED
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolClashTournamentNotifyReason {
    NEW_TOURNAMENT = 0,
    UPDATE_TOURNAMENT = 1,
    CANCEL_TOURNAMENT = 2,
    CANCEL_PERIOD = 3,
    ADD_PHASE = 4,
    UPDATE_PHASE = 5,
    REVERT_PHASE = 6,
    UPDATE_STATUS = 7,
}

impl Default for LolClashTournamentNotifyReason {
    fn default() -> Self {
        LolClashTournamentNotifyReason::NEW_TOURNAMENT
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolClashTournamentState {
    UPCOMING = 0,
    IDLE = 1,
    LOCK_IN = 2,
    SCOUTING = 3,
    IN_GAME = 4,
    RESULTS = 5,
}

impl Default for LolClashTournamentState {
    fn default() -> Self {
        LolClashTournamentState::UPCOMING
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolCollectionsCollectionsSummonerBackdropType {
    default = 0,
    summoner_icon = 1,
    highest_mastery = 2,
    specified_skin = 3,
}

impl Default for LolCollectionsCollectionsSummonerBackdropType {
    fn default() -> Self {
        LolCollectionsCollectionsSummonerBackdropType::default
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolCollectionsItemOwnershipType {
    OWNED = 0,
    RENTED = 1,
    LOYALTY = 2,
    F2P = 3,
}

impl Default for LolCollectionsItemOwnershipType {
    fn default() -> Self {
        LolCollectionsItemOwnershipType::OWNED
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolCollectionsLoginSessionStates {
    IN_PROGRESS = 0,
    SUCCEEDED = 1,
    LOGGING_OUT = 2,
    ERROR = 3,
}

impl Default for LolCollectionsLoginSessionStates {
    fn default() -> Self {
        LolCollectionsLoginSessionStates::IN_PROGRESS
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolContentTargetingGameflowPhase {
    None = 0,
    Lobby = 1,
    Matchmaking = 2,
    CheckedIntoTournament = 3,
    ReadyCheck = 4,
    ChampSelect = 5,
    GameStart = 6,
    FailedToLaunch = 7,
    InProgress = 8,
    Reconnect = 9,
    WaitingForStats = 10,
    PreEndOfGame = 11,
    EndOfGame = 12,
    TerminatedInError = 13,
}

impl Default for LolContentTargetingGameflowPhase {
    fn default() -> Self {
        LolContentTargetingGameflowPhase::None
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolContentTargetingLoginSessionState {
    IN_PROGRESS = 0,
    SUCCEEDED = 1,
    LOGGING_OUT = 2,
    ERROR = 3,
}

impl Default for LolContentTargetingLoginSessionState {
    fn default() -> Self {
        LolContentTargetingLoginSessionState::IN_PROGRESS
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolContentTargetingQueueGameCategory {
    None = 0,
    Custom = 1,
    PvP = 2,
    VersusAi = 3,
    Alpha = 4,
}

impl Default for LolContentTargetingQueueGameCategory {
    fn default() -> Self {
        LolContentTargetingQueueGameCategory::None
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolContentTargetingRankedDivision {
    NA = 0,
    I = 1,
    II = 2,
    III = 3,
    IV = 4,
    V = 5,
}

impl Default for LolContentTargetingRankedDivision {
    fn default() -> Self {
        LolContentTargetingRankedDivision::NA
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolContentTargetingRankedQueue {
    NONE = 0,
    RANKED_SOLO_5x5 = 1,
    RANKED_FLEX_SR = 2,
    RANKED_FLEX_TT = 3,
    RANKED_TFT = 4,
}

impl Default for LolContentTargetingRankedQueue {
    fn default() -> Self {
        LolContentTargetingRankedQueue::NONE
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolContentTargetingRankedTier {
    NONE = 0,
    IRON = 1,
    BRONZE = 2,
    SILVER = 3,
    GOLD = 4,
    PLATINUM = 5,
    DIAMOND = 6,
    MASTER = 7,
    GRANDMASTER = 8,
    CHALLENGER = 9,
}

impl Default for LolContentTargetingRankedTier {
    fn default() -> Self {
        LolContentTargetingRankedTier::NONE
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolEndOfGameGameflowPhase {
    None = 0,
    Lobby = 1,
    Matchmaking = 2,
    CheckedIntoTournament = 3,
    ReadyCheck = 4,
    ChampSelect = 5,
    GameStart = 6,
    FailedToLaunch = 7,
    InProgress = 8,
    Reconnect = 9,
    WaitingForStats = 10,
    PreEndOfGame = 11,
    EndOfGame = 12,
    TerminatedInError = 13,
}

impl Default for LolEndOfGameGameflowPhase {
    fn default() -> Self {
        LolEndOfGameGameflowPhase::None
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolEndOfGameLoginSessionStates {
    IN_PROGRESS = 0,
    SUCCEEDED = 1,
    LOGGING_OUT = 2,
    ERROR = 3,
}

impl Default for LolEndOfGameLoginSessionStates {
    fn default() -> Self {
        LolEndOfGameLoginSessionStates::IN_PROGRESS
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolEndOfGameReportRecipientMode {
    Legacy = 0,
    Game_Agnostic = 1,
    Combined = 2,
}

impl Default for LolEndOfGameReportRecipientMode {
    fn default() -> Self {
        LolEndOfGameReportRecipientMode::Legacy
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolEsportStreamNotificationsGameflowPhase {
    None = 0,
    Lobby = 1,
    Matchmaking = 2,
    CheckedIntoTournament = 3,
    ReadyCheck = 4,
    ChampSelect = 5,
    GameStart = 6,
    FailedToLaunch = 7,
    InProgress = 8,
    Reconnect = 9,
    WaitingForStats = 10,
    PreEndOfGame = 11,
    EndOfGame = 12,
    TerminatedInError = 13,
}

impl Default for LolEsportStreamNotificationsGameflowPhase {
    fn default() -> Self {
        LolEsportStreamNotificationsGameflowPhase::None
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolEventShopCelebrationType {
    NONE = 0,
    TOAST = 1,
    FULLSCREEN = 2,
}

impl Default for LolEventShopCelebrationType {
    fn default() -> Self {
        LolEventShopCelebrationType::NONE
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolEventShopGrantStatus {
    PENDING_FULFILLMENT = 0,
    PENDING_SELECTION = 1,
    FULFILLED = 2,
    FAILED = 3,
}

impl Default for LolEventShopGrantStatus {
    fn default() -> Self {
        LolEventShopGrantStatus::PENDING_FULFILLMENT
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolEventShopInventoryOwnership {
    OWNED = 0,
    RENTED = 1,
    LOYALTY = 2,
    F2P = 3,
}

impl Default for LolEventShopInventoryOwnership {
    fn default() -> Self {
        LolEventShopInventoryOwnership::OWNED
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolEventShopItemOwnershipType {
    OWNED = 0,
    RENTED = 1,
    LOYALTY = 2,
    F2P = 3,
}

impl Default for LolEventShopItemOwnershipType {
    fn default() -> Self {
        LolEventShopItemOwnershipType::OWNED
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolEventShopLoginSessionStates {
    IN_PROGRESS = 0,
    SUCCEEDED = 1,
    LOGGING_OUT = 2,
    ERROR = 3,
}

impl Default for LolEventShopLoginSessionStates {
    fn default() -> Self {
        LolEventShopLoginSessionStates::IN_PROGRESS
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolEventShopLoyaltyStatus {
    LEGACY = 0,
    REWARDS_GRANT = 1,
    EXPIRY = 2,
    CHANGE = 3,
    REVOKE = 4,
    DISABLED = 5,
}

impl Default for LolEventShopLoyaltyStatus {
    fn default() -> Self {
        LolEventShopLoyaltyStatus::LEGACY
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolEventShopOfferCategory {
    Featured = 0,
    Chromas = 1,
    Skins = 2,
    Borders = 3,
    Loot = 4,
    Tft = 5,
    Currencies = 6,
}

impl Default for LolEventShopOfferCategory {
    fn default() -> Self {
        LolEventShopOfferCategory::Featured
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolEventShopOfferPromotionType {
    kNone = 0,
    kFeatured = 1,
    kFeaturedHighlighted = 2,
}

impl Default for LolEventShopOfferPromotionType {
    fn default() -> Self {
        LolEventShopOfferPromotionType::kNone
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolEventShopOfferStates {
    kOwned = 0,
    kAvailable = 1,
    kUnavailable = 2,
    kUnrevealed = 3,
    kPurchasing = 4,
}

impl Default for LolEventShopOfferStates {
    fn default() -> Self {
        LolEventShopOfferStates::kOwned
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolEventShopPassOwnershipTypes {
    Unowned = 0,
    Purchased = 1,
}

impl Default for LolEventShopPassOwnershipTypes {
    fn default() -> Self {
        LolEventShopPassOwnershipTypes::Unowned
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolEventShopPurchaseOfferOrderStates {
    NOT_STARTED = 0,
    IN_PROGRESS = 1,
    FAIL = 2,
    SUCCESS = 3,
}

impl Default for LolEventShopPurchaseOfferOrderStates {
    fn default() -> Self {
        LolEventShopPurchaseOfferOrderStates::NOT_STARTED
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolEventShopRewardStatus {
    PENDING = 0,
    FULFILLED = 1,
    FAILED = 2,
}

impl Default for LolEventShopRewardStatus {
    fn default() -> Self {
        LolEventShopRewardStatus::PENDING
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolEventShopRewardStrategy {
    ALL = 0,
    RANDOM = 1,
    SELECTION = 2,
}

impl Default for LolEventShopRewardStrategy {
    fn default() -> Self {
        LolEventShopRewardStrategy::ALL
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolEventShopRewardTrackItemHeaderType {
    PREMIUM = 0,
    FREE = 1,
    NONE = 2,
}

impl Default for LolEventShopRewardTrackItemHeaderType {
    fn default() -> Self {
        LolEventShopRewardTrackItemHeaderType::PREMIUM
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolEventShopRewardTrackItemStates {
    Locked = 0,
    Unlocked = 1,
    Unselected = 2,
    Selected = 3,
}

impl Default for LolEventShopRewardTrackItemStates {
    fn default() -> Self {
        LolEventShopRewardTrackItemStates::Locked
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolEventShopRewardTrackItemTag {
    Rare = 0,
    Free = 1,
    Instant = 2,
    Choice = 3,
    Multiple = 4,
}

impl Default for LolEventShopRewardTrackItemTag {
    fn default() -> Self {
        LolEventShopRewardTrackItemTag::Rare
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolEventShopSelectGrantStatusResponse {
    SELECTED = 0,
    FAILED = 1,
}

impl Default for LolEventShopSelectGrantStatusResponse {
    fn default() -> Self {
        LolEventShopSelectGrantStatusResponse::SELECTED
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolEventShopTokenUpsellLockedType {
    UNASSIGNED = 0,
    LOCKED = 1,
    UNLOCKED = 2,
}

impl Default for LolEventShopTokenUpsellLockedType {
    fn default() -> Self {
        LolEventShopTokenUpsellLockedType::UNASSIGNED
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolFeaturedModesEligibilityRestrictionCode {
    QueueDisabled = 0,
    QueueUnsupported = 1,
    PlayerLevelRestriction = 2,
    PlayerTimedRestriction = 3,
    PlayerBannedRestriction = 4,
    PlayerAvailableChampionRestriction = 5,
    TeamDivisionRestriction = 6,
    TeamSkillRestriction = 7,
    TeamMaxSizeRestriction = 8,
    TeamMinSizeRestriction = 9,
    PlayerBingeRestriction = 10,
    PlayerDodgeRestriction = 11,
    PlayerInGameRestriction = 12,
    PlayerLeaverBustedRestriction = 13,
    PlayerLeaverQueueLockoutRestriction = 14,
    PlayerLeaverTaintedWarningRestriction = 15,
    PlayerMaxLevelRestriction = 16,
    PlayerMinLevelRestriction = 17,
    PlayerMinorRestriction = 18,
    PlayerRankedSuspensionRestriction = 19,
    TeamHighMMRMaxSizeRestriction = 20,
    TeamSizeRestriction = 21,
    PrerequisiteQueuesNotPlayedRestriction = 22,
    GameVersionMismatch = 23,
    GameVersionMissing = 24,
    GameVersionNotSupported = 25,
    QueueEntryNotEntitledRestriction = 26,
    UnknownRestriction = 27,
}

impl Default for LolFeaturedModesEligibilityRestrictionCode {
    fn default() -> Self {
        LolFeaturedModesEligibilityRestrictionCode::QueueDisabled
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolFeaturedModesGameflowAvailabilityState {
    Available = 0,
    Initializing = 1,
    Patching = 2,
    EligibilityInfoMissing = 3,
    PlayerBanned = 4,
    InGameFlow = 5,
    Configuration = 6,
}

impl Default for LolFeaturedModesGameflowAvailabilityState {
    fn default() -> Self {
        LolFeaturedModesGameflowAvailabilityState::Available
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolFeaturedModesGameflowPhase {
    None = 0,
    Lobby = 1,
    Matchmaking = 2,
    CheckedIntoTournament = 3,
    ReadyCheck = 4,
    ChampSelect = 5,
    GameStart = 6,
    FailedToLaunch = 7,
    InProgress = 8,
    Reconnect = 9,
    WaitingForStats = 10,
    PreEndOfGame = 11,
    EndOfGame = 12,
    TerminatedInError = 13,
}

impl Default for LolFeaturedModesGameflowPhase {
    fn default() -> Self {
        LolFeaturedModesGameflowPhase::None
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolFeaturedModesLoginSessionStates {
    IN_PROGRESS = 0,
    SUCCEEDED = 1,
    LOGGING_OUT = 2,
    ERROR = 3,
}

impl Default for LolFeaturedModesLoginSessionStates {
    fn default() -> Self {
        LolFeaturedModesLoginSessionStates::IN_PROGRESS
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolFeaturedModesQueueAvailability {
    Available = 0,
    PlatformDisabled = 1,
    DoesntMeetRequirements = 2,
}

impl Default for LolFeaturedModesQueueAvailability {
    fn default() -> Self {
        LolFeaturedModesQueueAvailability::Available
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolFeaturedModesQueueGameCategory {
    None = 0,
    Custom = 1,
    PvP = 2,
    VersusAi = 3,
    Alpha = 4,
}

impl Default for LolFeaturedModesQueueGameCategory {
    fn default() -> Self {
        LolFeaturedModesQueueGameCategory::None
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolGameQueuesLoginSessionStates {
    IN_PROGRESS = 0,
    SUCCEEDED = 1,
    LOGGING_OUT = 2,
    ERROR = 3,
}

impl Default for LolGameQueuesLoginSessionStates {
    fn default() -> Self {
        LolGameQueuesLoginSessionStates::IN_PROGRESS
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolGameQueuesQueueAvailability {
    Available = 0,
    PlatformDisabled = 1,
    DoesntMeetRequirements = 2,
}

impl Default for LolGameQueuesQueueAvailability {
    fn default() -> Self {
        LolGameQueuesQueueAvailability::Available
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolGameQueuesQueueCustomGameSpectatorPolicy {
    NotAllowed = 0,
    LobbyAllowed = 1,
    FriendsAllowed = 2,
    AllAllowed = 3,
}

impl Default for LolGameQueuesQueueCustomGameSpectatorPolicy {
    fn default() -> Self {
        LolGameQueuesQueueCustomGameSpectatorPolicy::NotAllowed
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolGameQueuesQueueGameCategory {
    None = 0,
    Custom = 1,
    PvP = 2,
    VersusAi = 3,
    Alpha = 4,
}

impl Default for LolGameQueuesQueueGameCategory {
    fn default() -> Self {
        LolGameQueuesQueueGameCategory::None
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolGameSettingsLoginSessionStates {
    IN_PROGRESS = 0,
    SUCCEEDED = 1,
    LOGGING_OUT = 2,
    ERROR = 3,
}

impl Default for LolGameSettingsLoginSessionStates {
    fn default() -> Self {
        LolGameSettingsLoginSessionStates::IN_PROGRESS
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolGameflowGameflowAvailabilityState {
    Available = 0,
    Initializing = 1,
    Patching = 2,
    PlayerBanned = 3,
    InGameFlow = 4,
    Configuration = 5,
    EligibilityInfoMissing = 6,
}

impl Default for LolGameflowGameflowAvailabilityState {
    fn default() -> Self {
        LolGameflowGameflowAvailabilityState::Available
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolGameflowGameflowGameDodgeState {
    Invalid = 48,
    PartyDodged = 49,
    StrangerDodged = 50,
    TournamentDodged = 51,
}

impl Default for LolGameflowGameflowGameDodgeState {
    fn default() -> Self {
        LolGameflowGameflowGameDodgeState::Invalid
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolGameflowGameflowPhase {
    None = 0,
    Lobby = 1,
    Matchmaking = 2,
    CheckedIntoTournament = 3,
    ReadyCheck = 4,
    ChampSelect = 5,
    GameStart = 6,
    FailedToLaunch = 7,
    InProgress = 8,
    Reconnect = 9,
    WaitingForStats = 10,
    PreEndOfGame = 11,
    EndOfGame = 12,
    TerminatedInError = 13,
}

impl Default for LolGameflowGameflowPhase {
    fn default() -> Self {
        LolGameflowGameflowPhase::None
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolGameflowGameflowWatchPhase {
    None = 0,
    WatchStarted = 1,
    WatchInProgress = 2,
    WatchFailedToLaunch = 3,
}

impl Default for LolGameflowGameflowWatchPhase {
    fn default() -> Self {
        LolGameflowGameflowWatchPhase::None
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolGameflowLoginSessionStates {
    IN_PROGRESS = 0,
    SUCCEEDED = 1,
    LOGGING_OUT = 2,
    ERROR = 3,
}

impl Default for LolGameflowLoginSessionStates {
    fn default() -> Self {
        LolGameflowLoginSessionStates::IN_PROGRESS
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolGameflowPatcherProductStateAction {
    Idle = 0,
    CheckingForUpdates = 1,
    Patching = 2,
    Repairing = 3,
    Migrating = 4,
}

impl Default for LolGameflowPatcherProductStateAction {
    fn default() -> Self {
        LolGameflowPatcherProductStateAction::Idle
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolGameflowQueueAvailability {
    Available = 0,
    PlatformDisabled = 1,
    DoesntMeetRequirements = 2,
}

impl Default for LolGameflowQueueAvailability {
    fn default() -> Self {
        LolGameflowQueueAvailability::Available
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolGameflowQueueCustomGameSpectatorPolicy {
    NotAllowed = 0,
    LobbyAllowed = 1,
    FriendsAllowed = 2,
    AllAllowed = 3,
}

impl Default for LolGameflowQueueCustomGameSpectatorPolicy {
    fn default() -> Self {
        LolGameflowQueueCustomGameSpectatorPolicy::NotAllowed
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolGameflowQueueGameCategory {
    None = 0,
    Custom = 1,
    PvP = 2,
    VersusAi = 3,
    Alpha = 4,
}

impl Default for LolGameflowQueueGameCategory {
    fn default() -> Self {
        LolGameflowQueueGameCategory::None
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolGeoinfoLoginSessionState {
    IN_PROGRESS = 0,
    SUCCEEDED = 1,
    LOGGING_OUT = 2,
    ERROR = 3,
}

impl Default for LolGeoinfoLoginSessionState {
    fn default() -> Self {
        LolGeoinfoLoginSessionState::IN_PROGRESS
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolHeartbeatLoginSessionStates {
    IN_PROGRESS = 0,
    SUCCEEDED = 1,
    LOGGING_OUT = 2,
    ERROR = 3,
}

impl Default for LolHeartbeatLoginSessionStates {
    fn default() -> Self {
        LolHeartbeatLoginSessionStates::IN_PROGRESS
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolHoneyfruitGameflowPhase {
    None = 0,
    Lobby = 1,
    Matchmaking = 2,
    CheckedIntoTournament = 3,
    ReadyCheck = 4,
    ChampSelect = 5,
    GameStart = 6,
    FailedToLaunch = 7,
    InProgress = 8,
    Reconnect = 9,
    WaitingForStats = 10,
    PreEndOfGame = 11,
    EndOfGame = 12,
    TerminatedInError = 13,
}

impl Default for LolHoneyfruitGameflowPhase {
    fn default() -> Self {
        LolHoneyfruitGameflowPhase::None
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolHoneyfruitHoneyfruitActionType {
    dismiss = 0,
    dismiss_temporarily = 1,
    dismiss_permanently = 2,
    link = 3,
}

impl Default for LolHoneyfruitHoneyfruitActionType {
    fn default() -> Self {
        LolHoneyfruitHoneyfruitActionType::dismiss
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolHoneyfruitHoneyfruitLinkingFailureReason {
    ALREADY_LINKED = 0,
    ACCESS_TOKEN_EXPIRED = 1,
    BAD_AUTHORIZATION_PARAM = 2,
    DEGRADED = 3,
    DISABLED = 4,
    NOT_LINKED = 5,
    REQUEST_FAILURE = 6,
    UNHANDLED_SERVER_SIDE_ERROR = 7,
}

impl Default for LolHoneyfruitHoneyfruitLinkingFailureReason {
    fn default() -> Self {
        LolHoneyfruitHoneyfruitLinkingFailureReason::ALREADY_LINKED
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolHoneyfruitHoneyfruitLinkingState {
    hidden = 0,
    snoozed = 1,
    prompt = 2,
    confirm_snooze = 3,
    error = 4,
    in_progress = 5,
    linking_complete = 6,
    linked = 7,
}

impl Default for LolHoneyfruitHoneyfruitLinkingState {
    fn default() -> Self {
        LolHoneyfruitHoneyfruitLinkingState::hidden
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolHoneyfruitHoneyfruitLinkingStatusError {
    no_error = 0,
    not_signed_in = 1,
    service_unavailable = 2,
    unknown_error = 3,
}

impl Default for LolHoneyfruitHoneyfruitLinkingStatusError {
    fn default() -> Self {
        LolHoneyfruitHoneyfruitLinkingStatusError::no_error
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolHoneyfruitHoneyfruitPublisher {
    garena = 0,
    riot = 1,
    tencent = 2,
    twm = 3,
    vng = 4,
}

impl Default for LolHoneyfruitHoneyfruitPublisher {
    fn default() -> Self {
        LolHoneyfruitHoneyfruitPublisher::garena
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolHoneyfruitV1ResponseType {
    auth = 0,
    healup = 1,
    signup = 2,
    multifactor = 3,
    success = 4,
    error = 5,
}

impl Default for LolHoneyfruitV1ResponseType {
    fn default() -> Self {
        LolHoneyfruitV1ResponseType::auth
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolHonorV2GameflowPhase {
    None = 0,
    Lobby = 1,
    Matchmaking = 2,
    CheckedIntoTournament = 3,
    ReadyCheck = 4,
    ChampSelect = 5,
    GameStart = 6,
    FailedToLaunch = 7,
    InProgress = 8,
    Reconnect = 9,
    WaitingForStats = 10,
    PreEndOfGame = 11,
    EndOfGame = 12,
    TerminatedInError = 13,
}

impl Default for LolHonorV2GameflowPhase {
    fn default() -> Self {
        LolHonorV2GameflowPhase::None
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolHonorV2LoginSessionStates {
    IN_PROGRESS = 0,
    SUCCEEDED = 1,
    LOGGING_OUT = 2,
    ERROR = 3,
}

impl Default for LolHonorV2LoginSessionStates {
    fn default() -> Self {
        LolHonorV2LoginSessionStates::IN_PROGRESS
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolInventoryItemOwnershipType {
    OWNED = 0,
    RENTED = 1,
    LOYALTY = 2,
    F2P = 3,
}

impl Default for LolInventoryItemOwnershipType {
    fn default() -> Self {
        LolInventoryItemOwnershipType::OWNED
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolInventoryLoginSessionStates {
    IN_PROGRESS = 0,
    SUCCEEDED = 1,
    LOGGING_OUT = 2,
    ERROR = 3,
}

impl Default for LolInventoryLoginSessionStates {
    fn default() -> Self {
        LolInventoryLoginSessionStates::IN_PROGRESS
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolInventoryLoyaltyStatus {
    LEGACY = 0,
    REWARDS_GRANT = 1,
    EXPIRY = 2,
    CHANGE = 3,
    REVOKE = 4,
    DISABLED = 5,
}

impl Default for LolInventoryLoyaltyStatus {
    fn default() -> Self {
        LolInventoryLoyaltyStatus::LEGACY
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolItemSetsLoginSessionStates {
    IN_PROGRESS = 0,
    SUCCEEDED = 1,
    LOGGING_OUT = 2,
    ERROR = 3,
}

impl Default for LolItemSetsLoginSessionStates {
    fn default() -> Self {
        LolItemSetsLoginSessionStates::IN_PROGRESS
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolKrShutdownLawPolicyType {
    shutdown = 0,
    playTime = 1,
    warningMessage = 2,
    disableMatchMaking = 3,
}

impl Default for LolKrShutdownLawPolicyType {
    fn default() -> Self {
        LolKrShutdownLawPolicyType::shutdown
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolKrShutdownLawShutdownLawStatus {
    NONE = 0,
    WARNING = 1,
    CUT_OFF = 2,
}

impl Default for LolKrShutdownLawShutdownLawStatus {
    fn default() -> Self {
        LolKrShutdownLawShutdownLawStatus::NONE
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolLeagueSessionLeagueSessionStatus {
    UNINITIALIZED = 0,
    INITIALIZED = 1,
    EXPIRED = 2,
    DUPLICATED = 3,
    ANTI_ADDICTION_EXPIRED = 4,
}

impl Default for LolLeagueSessionLeagueSessionStatus {
    fn default() -> Self {
        LolLeagueSessionLeagueSessionStatus::UNINITIALIZED
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolLeaverBusterLeaverBusterNotificationType {
    Invalid = 0,
    TaintedWarning = 1,
    PunishmentIncurred = 2,
    PunishedGamesRemaining = 3,
    Reforming = 4,
    PreLockoutWarning = 5,
    OnLockoutWarning = 6,
}

impl Default for LolLeaverBusterLeaverBusterNotificationType {
    fn default() -> Self {
        LolLeaverBusterLeaverBusterNotificationType::Invalid
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolLicenseAgreementLicenseAgreementType {
    PrivacyNotice = 0,
    TermsOfUse = 1,
}

impl Default for LolLicenseAgreementLicenseAgreementType {
    fn default() -> Self {
        LolLicenseAgreementLicenseAgreementType::PrivacyNotice
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolLicenseAgreementLicenseServeLocation {
    Preparing = 0,
    Local = 1,
    External = 2,
}

impl Default for LolLicenseAgreementLicenseServeLocation {
    fn default() -> Self {
        LolLicenseAgreementLicenseServeLocation::Preparing
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolLoadoutsGameflowPhase {
    None = 0,
    Lobby = 1,
    Matchmaking = 2,
    CheckedIntoTournament = 3,
    ReadyCheck = 4,
    ChampSelect = 5,
    GameStart = 6,
    FailedToLaunch = 7,
    InProgress = 8,
    Reconnect = 9,
    WaitingForStats = 10,
    PreEndOfGame = 11,
    EndOfGame = 12,
    TerminatedInError = 13,
}

impl Default for LolLoadoutsGameflowPhase {
    fn default() -> Self {
        LolLoadoutsGameflowPhase::None
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolLoadoutsLoginSessionStates {
    IN_PROGRESS = 0,
    SUCCEEDED = 1,
    LOGGING_OUT = 2,
    ERROR = 3,
}

impl Default for LolLoadoutsLoginSessionStates {
    fn default() -> Self {
        LolLoadoutsLoginSessionStates::IN_PROGRESS
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolLobbyEligibilityRestrictionCode {
    QueueDisabled = 0,
    QueueUnsupported = 1,
    PlayerLevelRestriction = 2,
    PlayerTimedRestriction = 3,
    PlayerBannedRestriction = 4,
    PlayerAvailableChampionRestriction = 5,
    TeamDivisionRestriction = 6,
    TeamSkillRestriction = 7,
    TeamMaxSizeRestriction = 8,
    TeamMinSizeRestriction = 9,
    PlayerBingeRestriction = 10,
    PlayerDodgeRestriction = 11,
    PlayerInGameRestriction = 12,
    PlayerLeaverBustedRestriction = 13,
    PlayerLeaverQueueLockoutRestriction = 14,
    PlayerLeaverTaintedWarningRestriction = 15,
    PlayerMaxLevelRestriction = 16,
    PlayerMinLevelRestriction = 17,
    PlayerMinorRestriction = 18,
    PlayerTimePlayedRestriction = 19,
    PlayerRankSoloOnlyRestriction = 20,
    PlayerRankedSuspensionRestriction = 21,
    TeamHighMMRMaxSizeRestriction = 22,
    TeamSizeRestriction = 23,
    PrerequisiteQueuesNotPlayedRestriction = 24,
    GameVersionMismatch = 25,
    GameVersionMissing = 26,
    GameVersionNotSupported = 27,
    QueueEntryNotEntitledRestriction = 28,
    UnknownRestriction = 29,
    BanInfoNotAvailable = 30,
    MinorInfoNotAvailable = 31,
    SummonerInfoNotAvailable = 32,
    LeaguesInfoNotAvailable = 33,
    InventoryChampsInfoNotAvailable = 34,
    InventoryQueuesInfoNotAvailable = 35,
    MmrStandardDeviationTooLarge = 36,
}

impl Default for LolLobbyEligibilityRestrictionCode {
    fn default() -> Self {
        LolLobbyEligibilityRestrictionCode::QueueDisabled
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolLobbyGameflowPhase {
    None = 0,
    Lobby = 1,
    Matchmaking = 2,
    CheckedIntoTournament = 3,
    ReadyCheck = 4,
    ChampSelect = 5,
    GameStart = 6,
    FailedToLaunch = 7,
    InProgress = 8,
    Reconnect = 9,
    WaitingForStats = 10,
    PreEndOfGame = 11,
    EndOfGame = 12,
    TerminatedInError = 13,
}

impl Default for LolLobbyGameflowPhase {
    fn default() -> Self {
        LolLobbyGameflowPhase::None
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolLobbyGameflowSampleTag {
    Unregistered = 0,
    GameVersionMissing = 1,
    UserInfoTokenMissing = 2,
    SummonerTokenMissing = 3,
    RankedTokenMissing = 4,
    InventoryTokenMissing = 5,
}

impl Default for LolLobbyGameflowSampleTag {
    fn default() -> Self {
        LolLobbyGameflowSampleTag::Unregistered
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolLobbyInvitationType {
    invalid = 0,
    lobby = 1,
    party = 2,
}

impl Default for LolLobbyInvitationType {
    fn default() -> Self {
        LolLobbyInvitationType::invalid
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolLobbyLobbyBotDifficulty {
    NONE = -1,
    EASY = 0,
    MEDIUM = 1,
    HARD = 2,
    UBER = 3,
    TUTORIAL = 4,
    INTRO = 5,
}

impl Default for LolLobbyLobbyBotDifficulty {
    fn default() -> Self {
        LolLobbyLobbyBotDifficulty::NONE
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolLobbyLobbyInvitationState {
    Requested = 0,
    Pending = 1,
    Accepted = 2,
    Joined = 3,
    Declined = 4,
    Kicked = 5,
    OnHold = 6,
    Error = 7,
}

impl Default for LolLobbyLobbyInvitationState {
    fn default() -> Self {
        LolLobbyLobbyInvitationState::Requested
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolLobbyLobbyMatchmakingSearchState {
    Invalid = 0,
    AbandonedLowPriorityQueue = 1,
    Canceled = 2,
    Searching = 3,
    Found = 4,
    Error = 5,
    ServiceError = 6,
    ServiceShutdown = 7,
}

impl Default for LolLobbyLobbyMatchmakingSearchState {
    fn default() -> Self {
        LolLobbyLobbyMatchmakingSearchState::Invalid
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolLobbyLobbyPartyRewardType {
    Ip = 1,
    Icon = 2,
    None = 9999,
}

impl Default for LolLobbyLobbyPartyRewardType {
    fn default() -> Self {
        LolLobbyLobbyPartyRewardType::Ip
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolLobbyLobbyRemovedFromGameReason {
    None = 0,
    Kicked = 1,
    Disbanded = 2,
    Left = 3,
    ServiceError = 4,
    Other = 5,
    Timeout = 6,
    GameStartError = 7,
    ServiceShutdown = 8,
}

impl Default for LolLobbyLobbyRemovedFromGameReason {
    fn default() -> Self {
        LolLobbyLobbyRemovedFromGameReason::None
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolLobbyLoginSessionStates {
    IN_PROGRESS = 0,
    SUCCEEDED = 1,
    LOGGING_OUT = 2,
    ERROR = 3,
}

impl Default for LolLobbyLoginSessionStates {
    fn default() -> Self {
        LolLobbyLoginSessionStates::IN_PROGRESS
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolLobbyMatchmakingDodgeState {
    Invalid = 48,
    PartyDodged = 49,
    StrangerDodged = 50,
}

impl Default for LolLobbyMatchmakingDodgeState {
    fn default() -> Self {
        LolLobbyMatchmakingDodgeState::Invalid
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolLobbyPartyEogStatusCategory {
    kLeft = 0,
    kPlayAgain = 1,
    kOnEog = 2,
}

impl Default for LolLobbyPartyEogStatusCategory {
    fn default() -> Self {
        LolLobbyPartyEogStatusCategory::kLeft
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolLobbyPartyMemberRoleEnum {
    LEADER = 0,
    MEMBER = 1,
    INVITED = 2,
    HOLD = 3,
    KICKED = 4,
    DECLINED = 5,
    NONE = 6,
}

impl Default for LolLobbyPartyMemberRoleEnum {
    fn default() -> Self {
        LolLobbyPartyMemberRoleEnum::LEADER
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolLobbyPlayerUpdateType {
    None = 0,
    Direct = 1,
    ServiceProxy = 2,
    RMS = 3,
}

impl Default for LolLobbyPlayerUpdateType {
    fn default() -> Self {
        LolLobbyPlayerUpdateType::None
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolLobbyQueueAvailability {
    Available = 0,
    PlatformDisabled = 1,
    DoesntMeetRequirements = 2,
}

impl Default for LolLobbyQueueAvailability {
    fn default() -> Self {
        LolLobbyQueueAvailability::Available
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolLobbyQueueCustomGameSpectatorPolicy {
    NotAllowed = 0,
    LobbyAllowed = 1,
    FriendsAllowed = 2,
    AllAllowed = 3,
}

impl Default for LolLobbyQueueCustomGameSpectatorPolicy {
    fn default() -> Self {
        LolLobbyQueueCustomGameSpectatorPolicy::NotAllowed
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolLobbyQueueGameCategory {
    None = 0,
    Custom = 1,
    PvP = 2,
    VersusAi = 3,
    Alpha = 4,
}

impl Default for LolLobbyQueueGameCategory {
    fn default() -> Self {
        LolLobbyQueueGameCategory::None
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolLobbyTeamBuilderChampSelectSwapState {
    AVAILABLE = 1,
    BUSY = 2,
    INVALID = 3,
    RECEIVED = 4,
    SENT = 5,
}

impl Default for LolLobbyTeamBuilderChampSelectSwapState {
    fn default() -> Self {
        LolLobbyTeamBuilderChampSelectSwapState::AVAILABLE
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolLobbyTeamBuilderChampSelectTradeState {
    AVAILABLE = 1,
    BUSY = 2,
    INVALID = 3,
    RECEIVED = 4,
    SENT = 5,
}

impl Default for LolLobbyTeamBuilderChampSelectTradeState {
    fn default() -> Self {
        LolLobbyTeamBuilderChampSelectTradeState::AVAILABLE
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolLobbyTeamBuilderLobbyRemovedFromGameReason {
    None = 0,
    Kicked = 1,
    Disbanded = 2,
    Left = 3,
    ServiceError = 4,
    Other = 5,
    Timeout = 6,
    GameStartError = 7,
    ServiceShutdown = 8,
}

impl Default for LolLobbyTeamBuilderLobbyRemovedFromGameReason {
    fn default() -> Self {
        LolLobbyTeamBuilderLobbyRemovedFromGameReason::None
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolLobbyTeamBuilderLoginSessionState {
    IN_PROGRESS = 0,
    SUCCEEDED = 1,
    LOGGING_OUT = 2,
    ERROR = 3,
}

impl Default for LolLobbyTeamBuilderLoginSessionState {
    fn default() -> Self {
        LolLobbyTeamBuilderLoginSessionState::IN_PROGRESS
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolLobbyTeamBuilderMatchmakingDodgeState {
    Invalid = 48,
    PartyDodged = 49,
    StrangerDodged = 50,
    TournamentDodged = 51,
}

impl Default for LolLobbyTeamBuilderMatchmakingDodgeState {
    fn default() -> Self {
        LolLobbyTeamBuilderMatchmakingDodgeState::Invalid
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolLobbyTeamBuilderMatchmakingDodgeWarning {
    None = 0,
    Warning = 1,
    Penalty = 2,
}

impl Default for LolLobbyTeamBuilderMatchmakingDodgeWarning {
    fn default() -> Self {
        LolLobbyTeamBuilderMatchmakingDodgeWarning::None
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolLobbyTeamBuilderMatchmakingReadyCheckResponse {
    None = 48,
    Accepted = 49,
    Declined = 50,
}

impl Default for LolLobbyTeamBuilderMatchmakingReadyCheckResponse {
    fn default() -> Self {
        LolLobbyTeamBuilderMatchmakingReadyCheckResponse::None
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolLobbyTeamBuilderMatchmakingReadyCheckState {
    Invalid = 0,
    InProgress = 1,
    EveryoneReady = 2,
    StrangerNotReady = 3,
    PartyNotReady = 4,
    Error = 5,
}

impl Default for LolLobbyTeamBuilderMatchmakingReadyCheckState {
    fn default() -> Self {
        LolLobbyTeamBuilderMatchmakingReadyCheckState::Invalid
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolLobbyTeamBuilderMatchmakingSearchState {
    Invalid = 0,
    AbandonedLowPriorityQueue = 1,
    Canceled = 2,
    Searching = 3,
    Found = 4,
    Error = 5,
    ServiceError = 6,
    ServiceShutdown = 7,
}

impl Default for LolLobbyTeamBuilderMatchmakingSearchState {
    fn default() -> Self {
        LolLobbyTeamBuilderMatchmakingSearchState::Invalid
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolLobbyTeamBuilderQueueAvailability {
    Available = 0,
    PlatformDisabled = 1,
    DoesntMeetRequirements = 2,
}

impl Default for LolLobbyTeamBuilderQueueAvailability {
    fn default() -> Self {
        LolLobbyTeamBuilderQueueAvailability::Available
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolLobbyTeamBuilderQueueGameCategory {
    None = 0,
    Custom = 1,
    PvP = 2,
    VersusAi = 3,
    Alpha = 4,
}

impl Default for LolLobbyTeamBuilderQueueGameCategory {
    fn default() -> Self {
        LolLobbyTeamBuilderQueueGameCategory::None
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolLoginAccountStateType {
    CREATING = 1,
    ENABLED = 2,
    TRANSFERRING_OUT = 3,
    TRANSFERRING_IN = 4,
    TRANSFERRED_OUT = 5,
    GENERATING = 6,
}

impl Default for LolLoginAccountStateType {
    fn default() -> Self {
        LolLoginAccountStateType::CREATING
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolLoginConfigReadinessEnum {
    NotReady = 0,
    Ready = 1,
    Disabled = 2,
}

impl Default for LolLoginConfigReadinessEnum {
    fn default() -> Self {
        LolLoginConfigReadinessEnum::NotReady
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolLoginConfigType {
    public = 0,
    player = 1,
}

impl Default for LolLoginConfigType {
    fn default() -> Self {
        LolLoginConfigType::public
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolLoginLeagueSessionStatus {
    UNINITIALIZED = 0,
    INITIALIZED = 1,
    EXPIRED = 2,
    DUPLICATED = 3,
    ANTI_ADDICTION_EXPIRED = 4,
}

impl Default for LolLoginLeagueSessionStatus {
    fn default() -> Self {
        LolLoginLeagueSessionStatus::UNINITIALIZED
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolLoginLoginConnectionMode {
    Preparing = 0,
    Legacy = 1,
    Partner = 2,
    RiotClient = 3,
}

impl Default for LolLoginLoginConnectionMode {
    fn default() -> Self {
        LolLoginLoginConnectionMode::Preparing
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolLoginLoginSessionStates {
    IN_PROGRESS = 0,
    SUCCEEDED = 1,
    LOGGING_OUT = 2,
    ERROR = 3,
}

impl Default for LolLoginLoginSessionStates {
    fn default() -> Self {
        LolLoginLoginSessionStates::IN_PROGRESS
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolLootCelebrationType {
    NONE = 0,
    TOAST = 1,
    FULLSCREEN = 2,
}

impl Default for LolLootCelebrationType {
    fn default() -> Self {
        LolLootCelebrationType::NONE
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolLootGameflowPhase {
    None = 0,
    Lobby = 1,
    Matchmaking = 2,
    CheckedIntoTournament = 3,
    ReadyCheck = 4,
    ChampSelect = 5,
    GameStart = 6,
    FailedToLaunch = 7,
    InProgress = 8,
    Reconnect = 9,
    WaitingForStats = 10,
    PreEndOfGame = 11,
    EndOfGame = 12,
    TerminatedInError = 13,
}

impl Default for LolLootGameflowPhase {
    fn default() -> Self {
        LolLootGameflowPhase::None
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolLootGrantStatus {
    PENDING_FULFILLMENT = 0,
    PENDING_SELECTION = 1,
    FULFILLED = 2,
    FAILED = 3,
}

impl Default for LolLootGrantStatus {
    fn default() -> Self {
        LolLootGrantStatus::PENDING_FULFILLMENT
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolLootInventoryOwnership {
    OWNED = 0,
    RENTED = 1,
    F2P = 2,
}

impl Default for LolLootInventoryOwnership {
    fn default() -> Self {
        LolLootInventoryOwnership::OWNED
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolLootItemOwnershipStatus {
    NONE = 0,
    FREE = 1,
    RENTAL = 2,
    OWNED = 3,
}

impl Default for LolLootItemOwnershipStatus {
    fn default() -> Self {
        LolLootItemOwnershipStatus::NONE
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolLootLoginSessionStates {
    IN_PROGRESS = 0,
    SUCCEEDED = 1,
    LOGGING_OUT = 2,
    ERROR = 3,
}

impl Default for LolLootLoginSessionStates {
    fn default() -> Self {
        LolLootLoginSessionStates::IN_PROGRESS
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolLootLootMilestoneClaimStatus {
    NOT_STARTED = 0,
    IN_PROGRESS = 1,
    COMPLETED = 2,
    FAILED = 3,
}

impl Default for LolLootLootMilestoneClaimStatus {
    fn default() -> Self {
        LolLootLootMilestoneClaimStatus::NOT_STARTED
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolLootLootRarity {
    Default = 0,
    Epic = 1,
    Legendary = 2,
    Mythic = 3,
    Ultimate = 4,
}

impl Default for LolLootLootRarity {
    fn default() -> Self {
        LolLootLootRarity::Default
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolLootLootType {
    Chest = 0,
    Currency = 1,
    Material = 2,
    WardSkin = 3,
    Skin = 4,
    Skin_Rental = 5,
    SummonerIcon = 6,
    Companion = 7,
    Egg = 8,
    Egg_Color = 9,
    Statstone = 10,
    Statstone_Shard = 11,
    Boost = 12,
    SkinBorder = 13,
    TFT_Map_Skin = 14,
    TFT_Damage_Skin = 15,
}

impl Default for LolLootLootType {
    fn default() -> Self {
        LolLootLootType::Chest
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolLootRedeemableStatus {
    UNKNOWN = 0,
    REDEEMABLE = 1,
    REDEEMABLE_RENTAL = 2,
    NOT_REDEEMABLE = 3,
    NOT_REDEEMABLE_RENTAL = 4,
    ALREADY_OWNED = 5,
    ALREADY_RENTED = 6,
    CHAMPION_NOT_OWNED = 7,
    SKIN_NOT_OWNED = 8,
}

impl Default for LolLootRedeemableStatus {
    fn default() -> Self {
        LolLootRedeemableStatus::UNKNOWN
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolLootRewardStatus {
    PENDING = 0,
    FULFILLED = 1,
    FAILED = 2,
}

impl Default for LolLootRewardStatus {
    fn default() -> Self {
        LolLootRewardStatus::PENDING
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolLootRewardStrategy {
    ALL = 0,
    RANDOM = 1,
    SELECTION = 2,
}

impl Default for LolLootRewardStrategy {
    fn default() -> Self {
        LolLootRewardStrategy::ALL
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolLootSelectGrantStatusResponse {
    SELECTED = 0,
    FAILED = 1,
}

impl Default for LolLootSelectGrantStatusResponse {
    fn default() -> Self {
        LolLootSelectGrantStatusResponse::SELECTED
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolLoyaltyLoginSessionStates {
    IN_PROGRESS = 0,
    SUCCEEDED = 1,
    LOGGING_OUT = 2,
    ERROR = 3,
}

impl Default for LolLoyaltyLoginSessionStates {
    fn default() -> Self {
        LolLoyaltyLoginSessionStates::IN_PROGRESS
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolLoyaltyLoyaltyStatus {
    LEGACY = 0,
    REWARDS_GRANT = 1,
    EXPIRY = 2,
    CHANGE = 3,
    REVOKE = 4,
    DISABLED = 5,
}

impl Default for LolLoyaltyLoyaltyStatus {
    fn default() -> Self {
        LolLoyaltyLoyaltyStatus::LEGACY
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolMatchHistoryLoginSessionStates {
    IN_PROGRESS = 0,
    SUCCEEDED = 1,
    LOGGING_OUT = 2,
    ERROR = 3,
}

impl Default for LolMatchHistoryLoginSessionStates {
    fn default() -> Self {
        LolMatchHistoryLoginSessionStates::IN_PROGRESS
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolMatchmakingGameflowPhase {
    None = 0,
    Lobby = 1,
    Matchmaking = 2,
    CheckedIntoTournament = 3,
    ReadyCheck = 4,
    ChampSelect = 5,
    GameStart = 6,
    FailedToLaunch = 7,
    InProgress = 8,
    Reconnect = 9,
    WaitingForStats = 10,
    PreEndOfGame = 11,
    EndOfGame = 12,
    TerminatedInError = 13,
}

impl Default for LolMatchmakingGameflowPhase {
    fn default() -> Self {
        LolMatchmakingGameflowPhase::None
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolMatchmakingLoginSessionState {
    IN_PROGRESS = 0,
    SUCCEEDED = 1,
    LOGGING_OUT = 2,
    ERROR = 3,
}

impl Default for LolMatchmakingLoginSessionState {
    fn default() -> Self {
        LolMatchmakingLoginSessionState::IN_PROGRESS
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolMatchmakingMatchmakingDodgeState {
    Invalid = 48,
    PartyDodged = 49,
    StrangerDodged = 50,
    TournamentDodged = 51,
}

impl Default for LolMatchmakingMatchmakingDodgeState {
    fn default() -> Self {
        LolMatchmakingMatchmakingDodgeState::Invalid
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolMatchmakingMatchmakingDodgeWarning {
    None = 0,
    Warning = 1,
    Penalty = 2,
}

impl Default for LolMatchmakingMatchmakingDodgeWarning {
    fn default() -> Self {
        LolMatchmakingMatchmakingDodgeWarning::None
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolMatchmakingMatchmakingReadyCheckResponse {
    None = 48,
    Accepted = 49,
    Declined = 50,
}

impl Default for LolMatchmakingMatchmakingReadyCheckResponse {
    fn default() -> Self {
        LolMatchmakingMatchmakingReadyCheckResponse::None
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolMatchmakingMatchmakingReadyCheckState {
    Invalid = 0,
    InProgress = 1,
    EveryoneReady = 2,
    StrangerNotReady = 3,
    PartyNotReady = 4,
    Error = 5,
}

impl Default for LolMatchmakingMatchmakingReadyCheckState {
    fn default() -> Self {
        LolMatchmakingMatchmakingReadyCheckState::Invalid
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolMatchmakingMatchmakingSearchState {
    Invalid = 0,
    AbandonedLowPriorityQueue = 1,
    Canceled = 2,
    Searching = 3,
    Found = 4,
    Error = 5,
    ServiceError = 6,
    ServiceShutdown = 7,
}

impl Default for LolMatchmakingMatchmakingSearchState {
    fn default() -> Self {
        LolMatchmakingMatchmakingSearchState::Invalid
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolMatchmakingQueueCustomGameSpectatorPolicy {
    NotAllowed = 0,
    LobbyAllowed = 1,
    FriendsAllowed = 2,
    AllAllowed = 3,
}

impl Default for LolMatchmakingQueueCustomGameSpectatorPolicy {
    fn default() -> Self {
        LolMatchmakingQueueCustomGameSpectatorPolicy::NotAllowed
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolMissionsGameflowPhase {
    None = 0,
    Lobby = 1,
    Matchmaking = 2,
    CheckedIntoTournament = 3,
    ReadyCheck = 4,
    ChampSelect = 5,
    GameStart = 6,
    FailedToLaunch = 7,
    InProgress = 8,
    Reconnect = 9,
    WaitingForStats = 10,
    PreEndOfGame = 11,
    EndOfGame = 12,
    TerminatedInError = 13,
}

impl Default for LolMissionsGameflowPhase {
    fn default() -> Self {
        LolMissionsGameflowPhase::None
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolMissionsGrantStatus {
    PENDING_FULFILLMENT = 0,
    PENDING_SELECTION = 1,
    FULFILLED = 2,
}

impl Default for LolMissionsGrantStatus {
    fn default() -> Self {
        LolMissionsGrantStatus::PENDING_FULFILLMENT
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolMissionsLoyaltyStatus {
    LEGACY = 0,
    REWARDS_GRANT = 1,
    EXPIRY = 2,
    CHANGE = 3,
    REVOKE = 4,
    DISABLED = 5,
}

impl Default for LolMissionsLoyaltyStatus {
    fn default() -> Self {
        LolMissionsLoyaltyStatus::LEGACY
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolMissionsRewardStatus {
    PENDING = 0,
    FULFILLED = 1,
}

impl Default for LolMissionsRewardStatus {
    fn default() -> Self {
        LolMissionsRewardStatus::PENDING
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolMissionsRewardStrategy {
    ALL = 0,
    RANDOM = 1,
    SELECTION = 2,
}

impl Default for LolMissionsRewardStrategy {
    fn default() -> Self {
        LolMissionsRewardStrategy::ALL
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolNpeTutorialPathGameflowPhase {
    None = 0,
    Lobby = 1,
    Matchmaking = 2,
    CheckedIntoTournament = 3,
    ReadyCheck = 4,
    ChampSelect = 5,
    GameStart = 6,
    FailedToLaunch = 7,
    InProgress = 8,
    Reconnect = 9,
    WaitingForStats = 10,
    PreEndOfGame = 11,
    EndOfGame = 12,
    TerminatedInError = 13,
}

impl Default for LolNpeTutorialPathGameflowPhase {
    fn default() -> Self {
        LolNpeTutorialPathGameflowPhase::None
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolNpeTutorialPathTutorialStatus {
    LOCKED = 0,
    UNLOCKED = 1,
    COMPLETED = 2,
}

impl Default for LolNpeTutorialPathTutorialStatus {
    fn default() -> Self {
        LolNpeTutorialPathTutorialStatus::LOCKED
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolNpeTutorialPathTutorialType {
    CARD = 0,
    REWARD = 1,
}

impl Default for LolNpeTutorialPathTutorialType {
    fn default() -> Self {
        LolNpeTutorialPathTutorialType::CARD
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolPatchComponentStateAction {
    Idle = 0,
    CheckingForUpdates = 1,
    Patching = 2,
    Repairing = 3,
    Migrating = 4,
}

impl Default for LolPatchComponentStateAction {
    fn default() -> Self {
        LolPatchComponentStateAction::Idle
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolPatchComponentStateWorkType {
    Scanning = 0,
    Network = 1,
    Disk = 2,
}

impl Default for LolPatchComponentStateWorkType {
    fn default() -> Self {
        LolPatchComponentStateWorkType::Scanning
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolPatchNotificationId {
    UnspecifiedError = 0,
    ConnectionError = 1,
    MissingFilesError = 2,
    FailedToWriteError = 3,
    DidRestoreClientBackup = 4,
    NotEnoughDiskSpace = 5,
    BrokenPermissions = 6,
}

impl Default for LolPatchNotificationId {
    fn default() -> Self {
        LolPatchNotificationId::UnspecifiedError
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolPatchScdEnabled {
    Off = 0,
    On = 1,
}

impl Default for LolPatchScdEnabled {
    fn default() -> Self {
        LolPatchScdEnabled::Off
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolPerksChampSelectTradeState {
    AVAILABLE = 1,
    BUSY = 2,
    INVALID = 3,
    RECEIVED = 4,
    SENT = 5,
}

impl Default for LolPerksChampSelectTradeState {
    fn default() -> Self {
        LolPerksChampSelectTradeState::AVAILABLE
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolPerksGameflowPhase {
    None = 0,
    Lobby = 1,
    Matchmaking = 2,
    CheckedIntoTournament = 3,
    ReadyCheck = 4,
    ChampSelect = 5,
    GameStart = 6,
    FailedToLaunch = 7,
    InProgress = 8,
    Reconnect = 9,
    WaitingForStats = 10,
    PreEndOfGame = 11,
    EndOfGame = 12,
    TerminatedInError = 13,
}

impl Default for LolPerksGameflowPhase {
    fn default() -> Self {
        LolPerksGameflowPhase::None
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolPerksLoginSessionState {
    IN_PROGRESS = 0,
    SUCCEEDED = 1,
    LOGGING_OUT = 2,
    ERROR = 3,
}

impl Default for LolPerksLoginSessionState {
    fn default() -> Self {
        LolPerksLoginSessionState::IN_PROGRESS
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolPftGameflowGameDodgeState {
    Invalid = 48,
    PartyDodged = 49,
    StrangerDodged = 50,
    TournamentDodged = 51,
}

impl Default for LolPftGameflowGameDodgeState {
    fn default() -> Self {
        LolPftGameflowGameDodgeState::Invalid
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolPftGameflowPhase {
    None = 0,
    Lobby = 1,
    Matchmaking = 2,
    CheckedIntoTournament = 3,
    ReadyCheck = 4,
    ChampSelect = 5,
    GameStart = 6,
    FailedToLaunch = 7,
    InProgress = 8,
    Reconnect = 9,
    WaitingForStats = 10,
    PreEndOfGame = 11,
    EndOfGame = 12,
    TerminatedInError = 13,
}

impl Default for LolPftGameflowPhase {
    fn default() -> Self {
        LolPftGameflowPhase::None
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolPftLoginSessionStates {
    IN_PROGRESS = 0,
    SUCCEEDED = 1,
    LOGGING_OUT = 2,
    ERROR = 3,
}

impl Default for LolPftLoginSessionStates {
    fn default() -> Self {
        LolPftLoginSessionStates::IN_PROGRESS
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolPlayerBehaviorGameflowPhase {
    None = 0,
    Lobby = 1,
    Matchmaking = 2,
    CheckedIntoTournament = 3,
    ReadyCheck = 4,
    ChampSelect = 5,
    GameStart = 6,
    FailedToLaunch = 7,
    InProgress = 8,
    Reconnect = 9,
    WaitingForStats = 10,
    PreEndOfGame = 11,
    EndOfGame = 12,
    TerminatedInError = 13,
}

impl Default for LolPlayerBehaviorGameflowPhase {
    fn default() -> Self {
        LolPlayerBehaviorGameflowPhase::None
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolPlayerBehaviorNotificationSource {
    Invalid = 0,
    Login = 1,
    ForcedShutdown = 2,
    Message = 3,
}

impl Default for LolPlayerBehaviorNotificationSource {
    fn default() -> Self {
        LolPlayerBehaviorNotificationSource::Invalid
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolPlayerLevelUpLoginSessionStates {
    IN_PROGRESS = 0,
    SUCCEEDED = 1,
    LOGGING_OUT = 2,
    ERROR = 3,
}

impl Default for LolPlayerLevelUpLoginSessionStates {
    fn default() -> Self {
        LolPlayerLevelUpLoginSessionStates::IN_PROGRESS
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolPlayerPreferencesLoginSessionStates {
    IN_PROGRESS = 0,
    SUCCEEDED = 1,
    LOGGING_OUT = 2,
    ERROR = 3,
}

impl Default for LolPlayerPreferencesLoginSessionStates {
    fn default() -> Self {
        LolPlayerPreferencesLoginSessionStates::IN_PROGRESS
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolPreEndOfGameGameflowPhase {
    None = 0,
    Lobby = 1,
    Matchmaking = 2,
    CheckedIntoTournament = 3,
    ReadyCheck = 4,
    ChampSelect = 5,
    GameStart = 6,
    FailedToLaunch = 7,
    InProgress = 8,
    Reconnect = 9,
    WaitingForStats = 10,
    PreEndOfGame = 11,
    EndOfGame = 12,
    TerminatedInError = 13,
}

impl Default for LolPreEndOfGameGameflowPhase {
    fn default() -> Self {
        LolPreEndOfGameGameflowPhase::None
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolPremadeVoiceConfigReadinessEnum {
    NotReady = 0,
    Ready = 1,
    Disabled = 2,
}

impl Default for LolPremadeVoiceConfigReadinessEnum {
    fn default() -> Self {
        LolPremadeVoiceConfigReadinessEnum::NotReady
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolPremadeVoiceConfigType {
    public = 0,
    player = 1,
}

impl Default for LolPremadeVoiceConfigType {
    fn default() -> Self {
        LolPremadeVoiceConfigType::public
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolPremadeVoiceGameflowPhase {
    None = 0,
    Lobby = 1,
    Matchmaking = 2,
    CheckedIntoTournament = 3,
    ReadyCheck = 4,
    ChampSelect = 5,
    GameStart = 6,
    FailedToLaunch = 7,
    InProgress = 8,
    Reconnect = 9,
    WaitingForStats = 10,
    PreEndOfGame = 11,
    EndOfGame = 12,
    TerminatedInError = 13,
}

impl Default for LolPremadeVoiceGameflowPhase {
    fn default() -> Self {
        LolPremadeVoiceGameflowPhase::None
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolPremadeVoiceInputMode {
    voiceActivity = 0,
    pushToTalk = 1,
}

impl Default for LolPremadeVoiceInputMode {
    fn default() -> Self {
        LolPremadeVoiceInputMode::voiceActivity
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolPremadeVoicePartyMemberRoleEnum {
    LEADER = 0,
    MEMBER = 1,
    INVITED = 2,
    HOLD = 3,
    KICKED = 4,
    DECLINED = 5,
}

impl Default for LolPremadeVoicePartyMemberRoleEnum {
    fn default() -> Self {
        LolPremadeVoicePartyMemberRoleEnum::LEADER
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolPremadeVoiceSessionStatus {
    active = 0,
    onHold = 1,
}

impl Default for LolPremadeVoiceSessionStatus {
    fn default() -> Self {
        LolPremadeVoiceSessionStatus::active
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolPurchaseWidgetInventoryOwnership {
    OWNED = 0,
    RENTED = 1,
    LOYALTY = 2,
    F2P = 3,
}

impl Default for LolPurchaseWidgetInventoryOwnership {
    fn default() -> Self {
        LolPurchaseWidgetInventoryOwnership::OWNED
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolPurchaseWidgetLoginSessionStates {
    IN_PROGRESS = 0,
    SUCCEEDED = 1,
    LOGGING_OUT = 2,
    ERROR = 3,
}

impl Default for LolPurchaseWidgetLoginSessionStates {
    fn default() -> Self {
        LolPurchaseWidgetLoginSessionStates::IN_PROGRESS
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolPurchaseWidgetPurchaseOfferOrderStates {
    NOT_STARTED = 0,
    IN_PROGRESS = 1,
    FAIL = 2,
    SUCCESS = 3,
}

impl Default for LolPurchaseWidgetPurchaseOfferOrderStates {
    fn default() -> Self {
        LolPurchaseWidgetPurchaseOfferOrderStates::NOT_STARTED
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolRankedEosNotificationType {
    FIRST_WARNING = 0,
    SECOND_WARNING = 1,
    SEASON_ENDED = 2,
}

impl Default for LolRankedEosNotificationType {
    fn default() -> Self {
        LolRankedEosNotificationType::FIRST_WARNING
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolRankedGameflowPhase {
    None = 0,
    Lobby = 1,
    Matchmaking = 2,
    CheckedIntoTournament = 3,
    ReadyCheck = 4,
    ChampSelect = 5,
    GameStart = 6,
    FailedToLaunch = 7,
    InProgress = 8,
    Reconnect = 9,
    WaitingForStats = 10,
    PreEndOfGame = 11,
    EndOfGame = 12,
    TerminatedInError = 13,
}

impl Default for LolRankedGameflowPhase {
    fn default() -> Self {
        LolRankedGameflowPhase::None
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolRankedLeagueDivision {
    I = 0,
    II = 1,
    III = 2,
    IV = 3,
    V = 4,
    NA = 5,
}

impl Default for LolRankedLeagueDivision {
    fn default() -> Self {
        LolRankedLeagueDivision::I
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolRankedLeagueQueueType {
    NONE = 0,
    RANKED_SOLO_5x5 = 1,
    RANKED_FLEX_SR = 2,
    RANKED_FLEX_TT = 3,
    RANKED_TFT = 4,
    RANKED_TFT_TURBO = 5,
    RANKED_TFT_PAIRS = 6,
    RANKED_TFT_DOUBLE_UP = 7,
}

impl Default for LolRankedLeagueQueueType {
    fn default() -> Self {
        LolRankedLeagueQueueType::NONE
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolRankedLeagueTier {
    NONE = 0,
    IRON = 1,
    BRONZE = 2,
    SILVER = 3,
    GOLD = 4,
    PLATINUM = 5,
    DIAMOND = 6,
    MASTER = 7,
    GRANDMASTER = 8,
    CHALLENGER = 9,
}

impl Default for LolRankedLeagueTier {
    fn default() -> Self {
        LolRankedLeagueTier::NONE
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolRankedLoginSessionStates {
    IN_PROGRESS = 0,
    SUCCEEDED = 1,
    LOGGING_OUT = 2,
    ERROR = 3,
}

impl Default for LolRankedLoginSessionStates {
    fn default() -> Self {
        LolRankedLoginSessionStates::IN_PROGRESS
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolRankedMiniseries {
    W = 0,
    L = 1,
    N = 2,
}

impl Default for LolRankedMiniseries {
    fn default() -> Self {
        LolRankedMiniseries::W
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolRankedNotificationDisplayType {
    NONE = 0,
    TOAST = 1,
    MODAL = 2,
    VIGNETTE = 3,
}

impl Default for LolRankedNotificationDisplayType {
    fn default() -> Self {
        LolRankedNotificationDisplayType::NONE
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolRankedRatedTier {
    NONE = 0,
    GRAY = 1,
    GREEN = 2,
    BLUE = 3,
    PURPLE = 4,
    ORANGE = 5,
}

impl Default for LolRankedRatedTier {
    fn default() -> Self {
        LolRankedRatedTier::NONE
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolRankedSeverity {
    WARNING = 0,
    ALERT = 1,
}

impl Default for LolRankedSeverity {
    fn default() -> Self {
        LolRankedSeverity::WARNING
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolRegaliaLeagueDivision {
    I = 0,
    II = 1,
    III = 2,
    IV = 3,
    V = 4,
    NA = 5,
}

impl Default for LolRegaliaLeagueDivision {
    fn default() -> Self {
        LolRegaliaLeagueDivision::I
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolRegaliaLeagueQueueType {
    NONE = 0,
    RANKED_SOLO_5x5 = 1,
    RANKED_FLEX_SR = 2,
    RANKED_FLEX_TT = 3,
    RANKED_TFT = 4,
    RANKED_TFT_TURBO = 5,
    RANKED_TFT_PAIRS = 6,
    RANKED_TFT_DOUBLE_UP = 7,
}

impl Default for LolRegaliaLeagueQueueType {
    fn default() -> Self {
        LolRegaliaLeagueQueueType::NONE
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolRegaliaLeagueTier {
    NONE = 0,
    IRON = 1,
    BRONZE = 2,
    SILVER = 3,
    GOLD = 4,
    PLATINUM = 5,
    DIAMOND = 6,
    MASTER = 7,
    GRANDMASTER = 8,
    CHALLENGER = 9,
}

impl Default for LolRegaliaLeagueTier {
    fn default() -> Self {
        LolRegaliaLeagueTier::NONE
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolRegaliaRegaliaBannerType {
    blank = 1,
    lastSeasonHighestRank = 2,
}

impl Default for LolRegaliaRegaliaBannerType {
    fn default() -> Self {
        LolRegaliaRegaliaBannerType::blank
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolRegaliaRegaliaCrestType {
    none = 0,
    prestige = 1,
    ranked = 2,
}

impl Default for LolRegaliaRegaliaCrestType {
    fn default() -> Self {
        LolRegaliaRegaliaCrestType::none
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolReplaysGameflowPhase {
    None = 0,
    Lobby = 1,
    Matchmaking = 2,
    CheckedIntoTournament = 3,
    ReadyCheck = 4,
    ChampSelect = 5,
    GameStart = 6,
    FailedToLaunch = 7,
    InProgress = 8,
    Reconnect = 9,
    WaitingForStats = 10,
    PreEndOfGame = 11,
    EndOfGame = 12,
    TerminatedInError = 13,
}

impl Default for LolReplaysGameflowPhase {
    fn default() -> Self {
        LolReplaysGameflowPhase::None
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolReplaysGameflowWatchPhase {
    None = 0,
    WatchStarted = 1,
    WatchInProgress = 2,
    WatchFailedToLaunch = 3,
}

impl Default for LolReplaysGameflowWatchPhase {
    fn default() -> Self {
        LolReplaysGameflowWatchPhase::None
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolReplaysMetadataState {
    checking = 0,
    found = 1,
    watch = 2,
    download = 3,
    downloading = 4,
    incompatible = 5,
    missingOrExpired = 6,
    retryDownload = 7,
    lost = 8,
    unsupported = 9,
    error = 10,
}

impl Default for LolReplaysMetadataState {
    fn default() -> Self {
        LolReplaysMetadataState::checking
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolRewardsCelebrationType {
    NONE = 0,
    TOAST = 1,
    FULLSCREEN = 2,
}

impl Default for LolRewardsCelebrationType {
    fn default() -> Self {
        LolRewardsCelebrationType::NONE
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolRewardsGrantStatus {
    PENDING_FULFILLMENT = 0,
    PENDING_SELECTION = 1,
    FULFILLED = 2,
    FAILED = 3,
}

impl Default for LolRewardsGrantStatus {
    fn default() -> Self {
        LolRewardsGrantStatus::PENDING_FULFILLMENT
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolRewardsRewardStatus {
    PENDING = 0,
    FULFILLED = 1,
    FAILED = 2,
}

impl Default for LolRewardsRewardStatus {
    fn default() -> Self {
        LolRewardsRewardStatus::PENDING
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolRewardsRewardStrategy {
    ALL = 0,
    RANDOM = 1,
    SELECTION = 2,
}

impl Default for LolRewardsRewardStrategy {
    fn default() -> Self {
        LolRewardsRewardStrategy::ALL
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolRewardsSelectGrantStatusResponse {
    SELECTED = 0,
    FAILED = 1,
}

impl Default for LolRewardsSelectGrantStatusResponse {
    fn default() -> Self {
        LolRewardsSelectGrantStatusResponse::SELECTED
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolRiotMessagingServiceGameflowPhase {
    None = 0,
    Lobby = 1,
    Matchmaking = 2,
    CheckedIntoTournament = 3,
    ReadyCheck = 4,
    ChampSelect = 5,
    GameStart = 6,
    FailedToLaunch = 7,
    InProgress = 8,
    Reconnect = 9,
    WaitingForStats = 10,
    PreEndOfGame = 11,
    EndOfGame = 12,
    TerminatedInError = 13,
}

impl Default for LolRiotMessagingServiceGameflowPhase {
    fn default() -> Self {
        LolRiotMessagingServiceGameflowPhase::None
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolRsoAuthAuthHintType {
    email_verification = 0,
    password_reset = 1,
    parental_consent = 2,
    ambiguous_username = 3,
}

impl Default for LolRsoAuthAuthHintType {
    fn default() -> Self {
        LolRsoAuthAuthHintType::email_verification
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolRsoAuthConfigReadinessEnum {
    NotReady = 0,
    Ready = 1,
    Disabled = 2,
}

impl Default for LolRsoAuthConfigReadinessEnum {
    fn default() -> Self {
        LolRsoAuthConfigReadinessEnum::NotReady
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolRsoAuthConfigType {
    public = 0,
    player = 1,
}

impl Default for LolRsoAuthConfigType {
    fn default() -> Self {
        LolRsoAuthConfigType::public
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolRsoAuthRSOAuthorizationTrustLevel {
    always_trusted = 0,
    trusted_device = 1,
    always_verify = 2,
}

impl Default for LolRsoAuthRSOAuthorizationTrustLevel {
    fn default() -> Self {
        LolRsoAuthRSOAuthorizationTrustLevel::always_trusted
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolSettingsLoginSessionStates {
    IN_PROGRESS = 0,
    SUCCEEDED = 1,
    LOGGING_OUT = 2,
    ERROR = 3,
}

impl Default for LolSettingsLoginSessionStates {
    fn default() -> Self {
        LolSettingsLoginSessionStates::IN_PROGRESS
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolSettingsPublisher {
    riot = 0,
    garena = 1,
    tencent = 2,
    twm = 3,
    vng = 4,
}

impl Default for LolSettingsPublisher {
    fn default() -> Self {
        LolSettingsPublisher::riot
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolShutdownShutdownReason {
    Invalid = 0,
    PlatformMaintenance = 1,
    LcuAlphaDisabled = 2,
    PlayerBanned = 3,
}

impl Default for LolShutdownShutdownReason {
    fn default() -> Self {
        LolShutdownShutdownReason::Invalid
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolSimpleDialogMessagesGameflowPhase {
    None = 0,
    Lobby = 1,
    Matchmaking = 2,
    CheckedIntoTournament = 3,
    ReadyCheck = 4,
    ChampSelect = 5,
    GameStart = 6,
    FailedToLaunch = 7,
    InProgress = 8,
    Reconnect = 9,
    WaitingForStats = 10,
    PreEndOfGame = 11,
    EndOfGame = 12,
    TerminatedInError = 13,
}

impl Default for LolSimpleDialogMessagesGameflowPhase {
    fn default() -> Self {
        LolSimpleDialogMessagesGameflowPhase::None
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolSocialLeaderboardLeagueDivision {
    I = 0,
    II = 1,
    III = 2,
    IV = 3,
    V = 4,
    NA = 5,
}

impl Default for LolSocialLeaderboardLeagueDivision {
    fn default() -> Self {
        LolSocialLeaderboardLeagueDivision::I
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolSocialLeaderboardLeagueQueueType {
    NONE = 0,
    RANKED_SOLO_5x5 = 1,
    RANKED_FLEX_SR = 2,
    RANKED_FLEX_TT = 3,
    RANKED_TFT = 4,
    RANKED_TFT_TURBO = 5,
    RANKED_TFT_PAIRS = 6,
    RANKED_TFT_DOUBLE_UP = 7,
}

impl Default for LolSocialLeaderboardLeagueQueueType {
    fn default() -> Self {
        LolSocialLeaderboardLeagueQueueType::NONE
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolSocialLeaderboardLeagueTier {
    NONE = 0,
    IRON = 1,
    BRONZE = 2,
    SILVER = 3,
    GOLD = 4,
    PLATINUM = 5,
    DIAMOND = 6,
    MASTER = 7,
    GRANDMASTER = 8,
    CHALLENGER = 9,
}

impl Default for LolSocialLeaderboardLeagueTier {
    fn default() -> Self {
        LolSocialLeaderboardLeagueTier::NONE
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolStoreLoginSessionStates {
    IN_PROGRESS = 0,
    SUCCEEDED = 1,
    LOGGING_OUT = 2,
    ERROR = 3,
}

impl Default for LolStoreLoginSessionStates {
    fn default() -> Self {
        LolStoreLoginSessionStates::IN_PROGRESS
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolSuggestedPlayersGameflowPhase {
    None = 0,
    Lobby = 1,
    Matchmaking = 2,
    CheckedIntoTournament = 3,
    ReadyCheck = 4,
    ChampSelect = 5,
    GameStart = 6,
    FailedToLaunch = 7,
    InProgress = 8,
    Reconnect = 9,
    WaitingForStats = 10,
    PreEndOfGame = 11,
    EndOfGame = 12,
    TerminatedInError = 13,
}

impl Default for LolSuggestedPlayersGameflowPhase {
    fn default() -> Self {
        LolSuggestedPlayersGameflowPhase::None
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolSuggestedPlayersSuggestedPlayersReason {
    PreviousPremade = 1,
    OnlineFriend = 2,
    FriendOfFriend = 3,
    VictoriousComrade = 5,
    HonorInteractions = 6,
    LegacyPlayAgain = 9999,
}

impl Default for LolSuggestedPlayersSuggestedPlayersReason {
    fn default() -> Self {
        LolSuggestedPlayersSuggestedPlayersReason::PreviousPremade
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolSummonerLoginSessionStates {
    IN_PROGRESS = 0,
    SUCCEEDED = 1,
    LOGGING_OUT = 2,
    ERROR = 3,
}

impl Default for LolSummonerLoginSessionStates {
    fn default() -> Self {
        LolSummonerLoginSessionStates::IN_PROGRESS
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolSummonerProfilePrivacyEnabledState {
    UNKNOWN = 0,
    ENABLED = 1,
    DISABLED = 2,
}

impl Default for LolSummonerProfilePrivacyEnabledState {
    fn default() -> Self {
        LolSummonerProfilePrivacyEnabledState::UNKNOWN
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolSummonerProfilePrivacySetting {
    PRIVATE = 0,
    PUBLIC = 1,
}

impl Default for LolSummonerProfilePrivacySetting {
    fn default() -> Self {
        LolSummonerProfilePrivacySetting::PRIVATE
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolTftGameflowPhase {
    None = 0,
    Lobby = 1,
    Matchmaking = 2,
    CheckedIntoTournament = 3,
    ReadyCheck = 4,
    ChampSelect = 5,
    GameStart = 6,
    FailedToLaunch = 7,
    InProgress = 8,
    Reconnect = 9,
    WaitingForStats = 10,
    PreEndOfGame = 11,
    EndOfGame = 12,
    TerminatedInError = 13,
}

impl Default for LolTftGameflowPhase {
    fn default() -> Self {
        LolTftGameflowPhase::None
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolTftQueueGameCategory {
    None = 0,
    Custom = 1,
    PvP = 2,
    VersusAi = 3,
    Alpha = 4,
}

impl Default for LolTftQueueGameCategory {
    fn default() -> Self {
        LolTftQueueGameCategory::None
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolUserExperienceGameflowPhase {
    None = 0,
    Lobby = 1,
    Matchmaking = 2,
    CheckedIntoTournament = 3,
    ReadyCheck = 4,
    ChampSelect = 5,
    GameStart = 6,
    FailedToLaunch = 7,
    InProgress = 8,
    Reconnect = 9,
    WaitingForStats = 10,
    PreEndOfGame = 11,
    EndOfGame = 12,
    TerminatedInError = 13,
}

impl Default for LolUserExperienceGameflowPhase {
    fn default() -> Self {
        LolUserExperienceGameflowPhase::None
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolYourshopItemOwnershipType {
    OWNED = 0,
    RENTED = 1,
    LOYALTY = 2,
    F2P = 3,
}

impl Default for LolYourshopItemOwnershipType {
    fn default() -> Self {
        LolYourshopItemOwnershipType::OWNED
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolYourshopLoginSessionStates {
    IN_PROGRESS = 0,
    SUCCEEDED = 1,
    LOGGING_OUT = 2,
    ERROR = 3,
}

impl Default for LolYourshopLoginSessionStates {
    fn default() -> Self {
        LolYourshopLoginSessionStates::IN_PROGRESS
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LolYourshopLoyaltyStatus {
    LEGACY = 0,
    REWARDS_GRANT = 1,
    EXPIRY = 2,
    CHANGE = 3,
    REVOKE = 4,
    DISABLED = 5,
}

impl Default for LolYourshopLoyaltyStatus {
    fn default() -> Self {
        LolYourshopLoyaltyStatus::LEGACY
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum MetricDataType {
    unknown = 0,
    int = 1,
    uint = 2,
    float = 3,
    bool = 4,
    string = 5,
}

impl Default for MetricDataType {
    fn default() -> Self {
        MetricDataType::unknown
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum MetricPriority {
    low = 0,
    medium = 1,
    high = 2,
}

impl Default for MetricPriority {
    fn default() -> Self {
        MetricPriority::low
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum MetricType {
    unknown = 0,
    percentage = 1,
    count = 2,
    duration = 3,
    rate = 4,
}

impl Default for MetricType {
    fn default() -> Self {
        MetricType::unknown
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum PatcherComponentStateAction {
    Idle = 0,
    CheckingForUpdates = 1,
    Patching = 2,
    Repairing = 3,
    Migrating = 4,
}

impl Default for PatcherComponentStateAction {
    fn default() -> Self {
        PatcherComponentStateAction::Idle
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum PatcherComponentStateWorkType {
    Scanning = 0,
    Network = 1,
    Disk = 2,
}

impl Default for PatcherComponentStateWorkType {
    fn default() -> Self {
        PatcherComponentStateWorkType::Scanning
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum PatcherNotificationId {
    UnspecifiedError = 0,
    ConnectionError = 1,
    MissingFilesError = 2,
    FailedToWriteError = 3,
    DidRestoreClientBackup = 4,
    NotEnoughDiskSpace = 5,
    BrokenPermissions = 6,
}

impl Default for PatcherNotificationId {
    fn default() -> Self {
        PatcherNotificationId::UnspecifiedError
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum PendingRosterInviteeState {
    SUGGESTED = 0,
    PENDING = 1,
    DECLINED = 2,
    REVOKED = 3,
    SUGGEST_DECLINED = 4,
    SUGGEST_REVOKED = 5,
    ACCEPTED = 6,
    SELFJOIN = 7,
    SELFJOIN_DECLINED = 8,
    SELFJOIN_REVOKED = 9,
}

impl Default for PendingRosterInviteeState {
    fn default() -> Self {
        PendingRosterInviteeState::SUGGESTED
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum PendingRosterMemberState {
    NOT_READY = 0,
    FORCED_NOT_READY = 1,
    READY = 2,
    LEFT = 3,
    KICK = 4,
}

impl Default for PendingRosterMemberState {
    fn default() -> Self {
        PendingRosterMemberState::NOT_READY
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum PickModes {
    NOT_PICKING = 0,
    IN_PROGRESS = 1,
    DONE = 2,
}

impl Default for PickModes {
    fn default() -> Self {
        PickModes::NOT_PICKING
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum PlayerFinderEnum {
    FREEAGENT = 0,
    FRIEND = 1,
}

impl Default for PlayerFinderEnum {
    fn default() -> Self {
        PlayerFinderEnum::FREEAGENT
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum PluginManagerState {
    NotReady = 0,
    PluginsInitialized = 1,
}

impl Default for PluginManagerState {
    fn default() -> Self {
        PluginManagerState::NotReady
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum PluginResourceEventType {
    Create = 0,
    Update = 1,
    Delete = 2,
}

impl Default for PluginResourceEventType {
    fn default() -> Self {
        PluginResourceEventType::Create
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum PluginThreadingModel {
    dedicated = 1,
    sequential = 2,
    concurrent = 3,
    parallel = 4,
}

impl Default for PluginThreadingModel {
    fn default() -> Self {
        PluginThreadingModel::dedicated
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum Position {
    TOP = 0,
    MIDDLE = 1,
    BOTTOM = 2,
    JUNGLE = 3,
    UTILITY = 4,
    FILL = 5,
    UNSELECTED = 6,
}

impl Default for Position {
    fn default() -> Self {
        Position::TOP
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum RemotingHelpFormat {
    Full = 1,
    Epytext = 2,
    Brief = 4,
    Console = 5,
}

impl Default for RemotingHelpFormat {
    fn default() -> Self {
        RemotingHelpFormat::Full
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum RemotingPrivilege {
    None = 0,
    User = 127,
    Admin = 254,
    Local = 255,
}

impl Default for RemotingPrivilege {
    fn default() -> Self {
        RemotingPrivilege::None
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum RemotingSerializedFormat {
    JSON = 1,
    YAML = 2,
    MsgPack = 3,
}

impl Default for RemotingSerializedFormat {
    fn default() -> Self {
        RemotingSerializedFormat::JSON
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum ReplayResponseStatus {
    OK = 0,
    NOT_FOUND = 1,
    EXPIRED = 2,
    BAD_REQUEST = 3,
    INTERNAL_SERVER_ERROR = 4,
}

impl Default for ReplayResponseStatus {
    fn default() -> Self {
        ReplayResponseStatus::OK
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum RiotMessagingServiceState {
    Disconnecting = 0,
    Disconnected = 1,
    Connecting = 3,
    Connected = 5,
}

impl Default for RiotMessagingServiceState {
    fn default() -> Self {
        RiotMessagingServiceState::Disconnecting
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum RiotMessagingServiceTokenType {
    Unavailable = 0,
    Access = 1,
    Identity = 2,
}

impl Default for RiotMessagingServiceTokenType {
    fn default() -> Self {
        RiotMessagingServiceTokenType::Unavailable
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum Role {
    CAPTAIN = 0,
    MEMBER = 1,
    NONE = 2,
}

impl Default for Role {
    fn default() -> Self {
        Role::CAPTAIN
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum TicketOfferState {
    ACTIVE = 0,
    ACCEPTED = 1,
    REJECTED = 2,
    REVOKED = 3,
}

impl Default for TicketOfferState {
    fn default() -> Self {
        TicketOfferState::ACTIVE
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum TicketType {
    BASIC = 0,
    PREMIUM = 1,
}

impl Default for TicketType {
    fn default() -> Self {
        TicketType::BASIC
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum TournamentStatusEnum {
    DEFAULT = 0,
    CANCELLED = 1,
    PAUSED = 2,
    PRERESUME = 3,
}

impl Default for TournamentStatusEnum {
    fn default() -> Self {
        TournamentStatusEnum::DEFAULT
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum TracingModuleThreadingModelV1 {
    kNone = 0,
    kMainThread = 1,
    kDedicated = 2,
    kSequential = 3,
    kConcurrent = 4,
    kParallel = 5,
}

impl Default for TracingModuleThreadingModelV1 {
    fn default() -> Self {
        TracingModuleThreadingModelV1::kNone
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum TracingModuleTypeV1 {
    kUnknown = 0,
    kRemoteAppModule = 1,
    kBackEndPlugin = 2,
    kBackendOther = 3,
    kFrontEndPlugin = 4,
    kRemotingSource = 5,
}

impl Default for TracingModuleTypeV1 {
    fn default() -> Self {
        TracingModuleTypeV1::kUnknown
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum TracingPhaseImportanceV1 {
    trivial = 0,
    minor = 1,
    major = 2,
}

impl Default for TracingPhaseImportanceV1 {
    fn default() -> Self {
        TracingPhaseImportanceV1::trivial
    }
}
