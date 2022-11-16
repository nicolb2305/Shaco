#![allow(dead_code)]
use std::error::Error;
use std::collections::HashMap;
use handlebars::Handlebars;
use serde_json::json;
use serde_json::value::Value;
use crate::rest::RESTClient;
use super::api_types::*; 

pub async fn delete_lol_chat_v1_blocked_players_by_id(client: RESTClient, id: String) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-chat/v1/blocked-players/{{id}}", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.delete(url.to_owned()).await?;
    Ok(value)
}

pub async fn delete_lol_chat_v1_conversations_active(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/lol-chat/v1/conversations/active";
    let value = client.delete(url.to_owned()).await?;
    Ok(value)
}

pub async fn delete_lol_chat_v1_conversations_by_id(client: RESTClient, id: String) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-chat/v1/conversations/{{id}}", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.delete(url.to_owned()).await?;
    Ok(value)
}

pub async fn delete_lol_chat_v1_conversations_by_id_messages(client: RESTClient, id: String) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-chat/v1/conversations/{{id}}/messages", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.delete(url.to_owned()).await?;
    Ok(value)
}

pub async fn delete_lol_chat_v1_errors_by_id(client: RESTClient, id: u64) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-chat/v1/errors/{{id}}", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.delete(url.to_owned()).await?;
    Ok(value)
}

pub async fn delete_lol_chat_v1_friend_groups_by_id(client: RESTClient, id: u32) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-chat/v1/friend-groups/{{id}}", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.delete(url.to_owned()).await?;
    Ok(value)
}

pub async fn delete_lol_chat_v1_friend_requests_by_id(client: RESTClient, id: String) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-chat/v1/friend-requests/{{id}}", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.delete(url.to_owned()).await?;
    Ok(value)
}

pub async fn delete_lol_chat_v1_friends_by_id(client: RESTClient, id: String) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-chat/v1/friends/{{id}}", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.delete(url.to_owned()).await?;
    Ok(value)
}

pub async fn delete_lol_chat_v1_player_mutes(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/lol-chat/v1/player-mutes";
    let value = client.delete(url.to_owned()).await?;
    Ok(value)
}

pub async fn delete_lol_chat_v1_session(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/lol-chat/v1/session";
    let value = client.delete(url.to_owned()).await?;
    Ok(value)
}

pub async fn delete_lol_chat_v1_settings_by_key(client: RESTClient, key: String, do_async: Option<bool>) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-chat/v1/settings/{{key}}?doAsync={{do_async}}", &json!({"key": key, "do_async": do_async}))?;
    let url = format!("{}", template_url);
    let value = client.delete(url.to_owned()).await?;
    Ok(value)
}

pub async fn delete_lol_clash_v1_voice(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/lol-clash/v1/voice";
    let value = client.delete(url.to_owned()).await?;
    Ok(value)
}

pub async fn delete_lol_clash_v1_voice_delay_by_delay_seconds(client: RESTClient, delay_seconds: f64) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-clash/v1/voice-delay/{{delay_seconds}}", &json!({"delay_seconds": delay_seconds}))?;
    let url = format!("{}", template_url);
    let value = client.delete(url.to_owned()).await?;
    Ok(value)
}

pub async fn delete_lol_cosmetics_v1_selection_companion(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/lol-cosmetics/v1/selection/companion";
    let value = client.delete(url.to_owned()).await?;
    Ok(value)
}

pub async fn delete_lol_cosmetics_v1_selection_tft_damage_skin(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/lol-cosmetics/v1/selection/tft-damage-skin";
    let value = client.delete(url.to_owned()).await?;
    Ok(value)
}

pub async fn delete_lol_cosmetics_v1_selection_tft_map_skin(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/lol-cosmetics/v1/selection/tft-map-skin";
    let value = client.delete(url.to_owned()).await?;
    Ok(value)
}

pub async fn delete_lol_gameflow_v1_early_exit_notifications_eog(client: RESTClient) -> Result<(), Box<dyn Error>> {
    let url = "/lol-gameflow/v1/early-exit-notifications/eog";
    client.delete(url.to_owned()).await?;
    Ok(())
}

pub async fn delete_lol_gameflow_v1_early_exit_notifications_eog_by_key(client: RESTClient, key: i32) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-gameflow/v1/early-exit-notifications/eog/{{key}}", &json!({"key": key}))?;
    let url = format!("{}", template_url);
    client.delete(url.to_owned()).await?;
    Ok(())
}

pub async fn delete_lol_gameflow_v1_early_exit_notifications_missions(client: RESTClient) -> Result<(), Box<dyn Error>> {
    let url = "/lol-gameflow/v1/early-exit-notifications/missions";
    client.delete(url.to_owned()).await?;
    Ok(())
}

pub async fn delete_lol_gameflow_v1_early_exit_notifications_missions_by_key(client: RESTClient, key: i32) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-gameflow/v1/early-exit-notifications/missions/{{key}}", &json!({"key": key}))?;
    let url = format!("{}", template_url);
    client.delete(url.to_owned()).await?;
    Ok(())
}

pub async fn delete_lol_highlights_v1_highlights_by_id(client: RESTClient, id: u64) -> Result<LolHighlightsHighlight, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-highlights/v1/highlights/{{id}}", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.delete(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolHighlightsHighlight>(value)?)
}

pub async fn delete_lol_honeyfruit_v1_debug_linking_status(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/lol-honeyfruit/v1/debug-linking-status";
    let value = client.delete(url.to_owned()).await?;
    Ok(value)
}

pub async fn delete_lol_honeyfruit_v1_debug_vng_publisher_settings(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/lol-honeyfruit/v1/debug-vng-publisher-settings";
    let value = client.delete(url.to_owned()).await?;
    Ok(value)
}

pub async fn delete_lol_leaver_buster_v1_notifications_by_id(client: RESTClient, id: u32) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-leaver-buster/v1/notifications/{{id}}", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.delete(url.to_owned()).await?;
    Ok(value)
}

pub async fn delete_lol_loadouts_v4_loadouts_by_id(client: RESTClient, id: String) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-loadouts/v4/loadouts/{{id}}", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    client.delete(url.to_owned()).await?;
    Ok(())
}

pub async fn delete_lol_lobby_v1_clash(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/lol-lobby/v1/clash";
    let value = client.delete(url.to_owned()).await?;
    Ok(value)
}

pub async fn delete_lol_lobby_v1_lobby_custom_bots_by_summoner_internal_name(client: RESTClient, summoner_internal_name: String) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-lobby/v1/lobby/custom/bots/{{summoner_internal_name}}", &json!({"summoner_internal_name": summoner_internal_name}))?;
    let url = format!("{}", template_url);
    let value = client.delete(url.to_owned()).await?;
    Ok(value)
}

pub async fn delete_lol_lobby_v2_lobby(client: RESTClient) -> Result<(), Box<dyn Error>> {
    let url = "/lol-lobby/v2/lobby";
    client.delete(url.to_owned()).await?;
    Ok(())
}

pub async fn delete_lol_lobby_v2_lobby_matchmaking_search(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/lol-lobby/v2/lobby/matchmaking/search";
    let value = client.delete(url.to_owned()).await?;
    Ok(value)
}

pub async fn delete_lol_lobby_v2_notifications_by_notification_id(client: RESTClient, notification_id: String) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-lobby/v2/notifications/{{notification_id}}", &json!({"notification_id": notification_id}))?;
    let url = format!("{}", template_url);
    let value = client.delete(url.to_owned()).await?;
    Ok(value)
}

pub async fn delete_lol_login_v1_service_proxy_async_requests_by_service_name_by_method_name(client: RESTClient, service_name: String, method_name: String, plugin_id: u32) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-login/v1/service-proxy-async-requests/{serviceName}/{{method_name}}?pluginId={{plugin_id}}", &json!({"service_name": service_name, "method_name": method_name, "plugin_id": plugin_id}))?;
    let url = format!("{}", template_url);
    client.delete(url.to_owned()).await?;
    Ok(())
}

pub async fn delete_lol_login_v1_session(client: RESTClient) -> Result<(), Box<dyn Error>> {
    let url = "/lol-login/v1/session";
    client.delete(url.to_owned()).await?;
    Ok(())
}

pub async fn delete_lol_login_v1_shutdown_locks_by_lock_name(client: RESTClient, lock_name: String) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-login/v1/shutdown-locks/{{lock_name}}", &json!({"lock_name": lock_name}))?;
    let url = format!("{}", template_url);
    client.delete(url.to_owned()).await?;
    Ok(())
}

pub async fn delete_lol_loot_v1_loot_grants_by_id(client: RESTClient, id: i64) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-loot/v1/loot-grants/{{id}}", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.delete(url.to_owned()).await?;
    Ok(value)
}

pub async fn delete_lol_loot_v1_player_loot_by_loot_id_new_notification(client: RESTClient, loot_id: String) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-loot/v1/player-loot/{{loot_id}}/new-notification", &json!({"loot_id": loot_id}))?;
    let url = format!("{}", template_url);
    let value = client.delete(url.to_owned()).await?;
    Ok(value)
}

pub async fn delete_lol_matchmaking_v1_search(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/lol-matchmaking/v1/search";
    let value = client.delete(url.to_owned()).await?;
    Ok(value)
}

pub async fn delete_lol_patch_v1_notifications_by_id(client: RESTClient, id: String) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-patch/v1/notifications/{{id}}", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    client.delete(url.to_owned()).await?;
    Ok(())
}

pub async fn delete_lol_perks_v1_pages(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/lol-perks/v1/pages";
    let value = client.delete(url.to_owned()).await?;
    Ok(value)
}

pub async fn delete_lol_perks_v1_pages_by_id(client: RESTClient, id: i32) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-perks/v1/pages/{{id}}", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.delete(url.to_owned()).await?;
    Ok(value)
}

pub async fn delete_lol_perks_v1_pages_by_id_auto_modified_selections(client: RESTClient, id: i32) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-perks/v1/pages/{{id}}/auto-modified-selections", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.delete(url.to_owned()).await?;
    Ok(value)
}

pub async fn delete_lol_player_behavior_v1_code_of_conduct_notification(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/lol-player-behavior/v1/code-of-conduct-notification";
    let value = client.delete(url.to_owned()).await?;
    Ok(value)
}

pub async fn delete_lol_player_behavior_v1_reporter_feedback_by_id(client: RESTClient, id: String) -> Result<LolPlayerBehaviorReporterFeedback, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-player-behavior/v1/reporter-feedback/{{id}}", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.delete(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolPlayerBehaviorReporterFeedback>(value)?)
}

pub async fn delete_lol_player_messaging_v1_celebration_notification_by_id_acknowledge(client: RESTClient, id: u32) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-player-messaging/v1/celebration/notification/{{id}}/acknowledge", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.delete(url.to_owned()).await?;
    Ok(value)
}

pub async fn delete_lol_player_messaging_v1_notification_by_id_acknowledge(client: RESTClient, id: u32) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-player-messaging/v1/notification/{{id}}/acknowledge", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.delete(url.to_owned()).await?;
    Ok(value)
}

pub async fn delete_lol_pre_end_of_game_v1_registration_by_sequence_event_name(client: RESTClient, sequence_event_name: String) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-pre-end-of-game/v1/registration/{{sequence_event_name}}", &json!({"sequence_event_name": sequence_event_name}))?;
    let url = format!("{}", template_url);
    client.delete(url.to_owned()).await?;
    Ok(())
}

pub async fn delete_lol_premade_voice_v1_mic_test(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/lol-premade-voice/v1/mic-test";
    let value = client.delete(url.to_owned()).await?;
    Ok(value)
}

pub async fn delete_lol_premade_voice_v1_session(client: RESTClient) -> Result<(), Box<dyn Error>> {
    let url = "/lol-premade-voice/v1/session";
    client.delete(url.to_owned()).await?;
    Ok(())
}

pub async fn delete_lol_rms_v1_champion_mastery_leaveup_update_by_id(client: RESTClient, id: u64) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-rms/v1/champion-mastery-leaveup-update/{{id}}", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.delete(url.to_owned()).await?;
    Ok(value)
}

pub async fn delete_lol_rso_auth_v1_auth_hints_hint(client: RESTClient) -> Result<(), Box<dyn Error>> {
    let url = "/lol-rso-auth/v1/auth-hints/hint";
    client.delete(url.to_owned()).await?;
    Ok(())
}

pub async fn delete_lol_rso_auth_v1_authorization(client: RESTClient) -> Result<(), Box<dyn Error>> {
    let url = "/lol-rso-auth/v1/authorization";
    client.delete(url.to_owned()).await?;
    Ok(())
}

pub async fn delete_lol_rso_auth_v1_session(client: RESTClient) -> Result<(), Box<dyn Error>> {
    let url = "/lol-rso-auth/v1/session";
    client.delete(url.to_owned()).await?;
    Ok(())
}

pub async fn delete_lol_rso_auth_v2_config(client: RESTClient) -> Result<(), Box<dyn Error>> {
    let url = "/lol-rso-auth/v2/config";
    client.delete(url.to_owned()).await?;
    Ok(())
}

pub async fn delete_lol_simple_dialog_messages_v1_messages_by_message_id(client: RESTClient, message_id: i64) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-simple-dialog-messages/v1/messages/{{message_id}}", &json!({"message_id": message_id}))?;
    let url = format!("{}", template_url);
    let value = client.delete(url.to_owned()).await?;
    Ok(value)
}

pub async fn delete_lol_statstones_v1_vignette_notifications(client: RESTClient) -> Result<(), Box<dyn Error>> {
    let url = "/lol-statstones/v1/vignette-notifications";
    client.delete(url.to_owned()).await?;
    Ok(())
}

pub async fn delete_lol_statstones_v1_vignette_notifications_by_key(client: RESTClient, key: i32) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-statstones/v1/vignette-notifications/{{key}}", &json!({"key": key}))?;
    let url = format!("{}", template_url);
    client.delete(url.to_owned()).await?;
    Ok(())
}

pub async fn delete_lol_suggested_players_v1_suggested_players_by_summoner_id(client: RESTClient, summoner_id: u64) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-suggested-players/v1/suggested-players/{{summoner_id}}", &json!({"summoner_id": summoner_id}))?;
    let url = format!("{}", template_url);
    let value = client.delete(url.to_owned()).await?;
    Ok(value)
}

pub async fn delete_patcher_v1_notifications_by_id(client: RESTClient, id: String) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/patcher/v1/notifications/{{id}}", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.delete(url.to_owned()).await?;
    Ok(value)
}

pub async fn delete_patcher_v1_products_by_product_id(client: RESTClient, product_id: String) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/patcher/v1/products/{{product_id}}", &json!({"product_id": product_id}))?;
    let url = format!("{}", template_url);
    let value = client.delete(url.to_owned()).await?;
    Ok(value)
}

pub async fn delete_player_notifications_v1_notifications_by_id(client: RESTClient, id: u64) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/player-notifications/v1/notifications/{{id}}", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.delete(url.to_owned()).await?;
    Ok(value)
}

pub async fn delete_riot_messaging_service_v1_connect(client: RESTClient) -> Result<(), Box<dyn Error>> {
    let url = "/riot-messaging-service/v1/connect";
    client.delete(url.to_owned()).await?;
    Ok(())
}

pub async fn delete_riot_messaging_service_v1_entitlements(client: RESTClient) -> Result<(), Box<dyn Error>> {
    let url = "/riot-messaging-service/v1/entitlements";
    client.delete(url.to_owned()).await?;
    Ok(())
}

pub async fn delete_riot_messaging_service_v1_session(client: RESTClient) -> Result<(), Box<dyn Error>> {
    let url = "/riot-messaging-service/v1/session";
    client.delete(url.to_owned()).await?;
    Ok(())
}

pub async fn delete_riotclient_affinity(client: RESTClient) -> Result<(), Box<dyn Error>> {
    let url = "/riotclient/affinity";
    client.delete(url.to_owned()).await?;
    Ok(())
}

pub async fn delete_riotclient_splash(client: RESTClient) -> Result<(), Box<dyn Error>> {
    let url = "/riotclient/splash";
    client.delete(url.to_owned()).await?;
    Ok(())
}

pub async fn delete_riotclient_v1_auth_tokens_by_auth_token(client: RESTClient, auth_token: String) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/riotclient/v1/auth-tokens/{{auth_token}}", &json!({"auth_token": auth_token}))?;
    let url = format!("{}", template_url);
    let value = client.delete(url.to_owned()).await?;
    Ok(value)
}

pub async fn delete_tracing_v1_performance_by_name(client: RESTClient, name: String) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/tracing/v1/performance/{{name}}", &json!({"name": name}))?;
    let url = format!("{}", template_url);
    client.delete(url.to_owned()).await?;
    Ok(())
}

pub async fn delete_tracing_v1_trace_time_series_event_by_event_name(client: RESTClient, event_name: String, when: u64, suffix: Option<String>) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/tracing/v1/trace/time-series-event/{{event_name}}?when={{when}}&suffix={{suffix}}", &json!({"event_name": event_name, "when": when, "suffix": suffix}))?;
    let url = format!("{}", template_url);
    client.delete(url.to_owned()).await?;
    Ok(())
}

pub async fn get_anti_addiction_v1_policies_by_policy_type_anti_addiction_state(client: RESTClient, policy_type: LolAntiAddictionPolicyType) -> Result<LolAntiAddictionAntiAddictionState, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/anti-addiction/v1/policies/{{policy_type}}/anti-addiction-state", &json!({"policy_type": policy_type}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolAntiAddictionAntiAddictionState>(value)?)
}

pub async fn get_by_plugin_assets_by_path(client: RESTClient, plugin: String, path: String, if_none_match: Option<String>) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/{{plugin}}/assets/{+path}?path={{path}}&if-none-match={{if_none_match}}", &json!({"plugin": plugin, "path": path, "if_none_match": if_none_match}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(value)
}

pub async fn get_client_config_v1_config(client: RESTClient, type_: ClientConfigConfigType, app: Option<String>, version: Option<String>, patchline: Option<String>, region: Option<String>, namespace: Option<String>) -> Result<HashMap<String, Value>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/client-config/v1/config?type={{type_}}&app={{app}}&version={{version}}&patchline={{patchline}}&region={{region}}&namespace={{namespace}}", &json!({"type_": type_, "app": app, "version": version, "patchline": patchline, "region": region, "namespace": namespace}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<HashMap<String, Value>>(value)?)
}

pub async fn get_client_config_v1_config_by_name(client: RESTClient, name: String, type_: ClientConfigConfigType, app: Option<String>, version: Option<String>, patchline: Option<String>, region: Option<String>) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/client-config/v1/config/{{name}}?type={{type_}}&app={{app}}&version={{version}}&patchline={{patchline}}&region={{region}}", &json!({"name": name, "type_": type_, "app": app, "version": version, "patchline": patchline, "region": region}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(value)
}

pub async fn get_client_config_v1_status_by_type(client: RESTClient, type_: ClientConfigConfigType) -> Result<ClientConfigConfigStatus, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/client-config/v1/status/{{type_}}", &json!({"type_": type_}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<ClientConfigConfigStatus>(value)?)
}

pub async fn get_client_config_v2_config_by_name(client: RESTClient, name: String) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/client-config/v2/config/{{name}}", &json!({"name": name}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(value)
}

pub async fn get_client_config_v2_namespace_by_namespace(client: RESTClient, namespace: String) -> Result<HashMap<String, Value>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/client-config/v2/namespace/{{namespace}}", &json!({"namespace": namespace}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<HashMap<String, Value>>(value)?)
}

pub async fn get_client_config_v2_namespace_by_namespace_player(client: RESTClient, namespace: String) -> Result<HashMap<String, Value>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/client-config/v2/namespace/{{namespace}}/player", &json!({"namespace": namespace}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<HashMap<String, Value>>(value)?)
}

pub async fn get_client_config_v2_namespace_by_namespace_public(client: RESTClient, namespace: String) -> Result<HashMap<String, Value>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/client-config/v2/namespace/{{namespace}}/public", &json!({"namespace": namespace}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<HashMap<String, Value>>(value)?)
}

pub async fn get_config_v1_config(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/config/v1/config";
    let value = client.get(url.to_owned()).await?;
    Ok(value)
}

pub async fn get_cookie_jar_v1_cookies(client: RESTClient) -> Result<Vec<Cookie>, Box<dyn Error>> {
    let url = "/cookie-jar/v1/cookies";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<Cookie>>(value)?)
}

pub async fn get_crash_reporting_v1_crash_status(client: RESTClient) -> Result<bool, Box<dyn Error>> {
    let url = "/crash-reporting/v1/crash-status";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<bool>(value)?)
}

pub async fn get_data_store_v1_install_dir(client: RESTClient) -> Result<String, Box<dyn Error>> {
    let url = "/data-store/v1/install-dir";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<String>(value)?)
}

pub async fn get_data_store_v1_install_settings_by_path(client: RESTClient, path: String) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/data-store/v1/install-settings/{+path}?path={{path}}", &json!({"path": path}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(value)
}

pub async fn get_data_store_v1_system_settings_by_path(client: RESTClient, path: String) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/data-store/v1/system-settings/{+path}?path={{path}}", &json!({"path": path}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(value)
}

pub async fn get_entitlements_v1_token(client: RESTClient) -> Result<EntitlementsToken, Box<dyn Error>> {
    let url = "/entitlements/v1/token";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<EntitlementsToken>(value)?)
}

pub async fn get_lol_account_verification_v1_activation_pin(client: RESTClient) -> Result<LolAccountVerificationSendActivationPinResponse, Box<dyn Error>> {
    let url = "/lol-account-verification/v1/activationPin";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolAccountVerificationSendActivationPinResponse>(value)?)
}

pub async fn get_lol_account_verification_v1_device(client: RESTClient) -> Result<LolAccountVerificationDeviceResponse, Box<dyn Error>> {
    let url = "/lol-account-verification/v1/device";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolAccountVerificationDeviceResponse>(value)?)
}

pub async fn get_lol_account_verification_v1_get_phone_number(client: RESTClient) -> Result<LolAccountVerificationPhoneNumberResponse, Box<dyn Error>> {
    let url = "/lol-account-verification/v1/get-phone-number";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolAccountVerificationPhoneNumberResponse>(value)?)
}

pub async fn get_lol_account_verification_v1_is_verified(client: RESTClient) -> Result<LolAccountVerificationIsVerifiedResponse, Box<dyn Error>> {
    let url = "/lol-account-verification/v1/is-verified";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolAccountVerificationIsVerifiedResponse>(value)?)
}

pub async fn get_lol_active_boosts_v1_active_boosts(client: RESTClient) -> Result<LolActiveBoostsActiveBoosts, Box<dyn Error>> {
    let url = "/lol-active-boosts/v1/active-boosts";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolActiveBoostsActiveBoosts>(value)?)
}

pub async fn get_lol_anti_addiction_v1_anti_addiction_token(client: RESTClient) -> Result<LolAntiAddictionAntiAddictionToken, Box<dyn Error>> {
    let url = "/lol-anti-addiction/v1/anti-addiction-token";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolAntiAddictionAntiAddictionToken>(value)?)
}

pub async fn get_lol_banners_v1_current_summoner_flags(client: RESTClient) -> Result<Vec<LolBannersBannerFlag>, Box<dyn Error>> {
    let url = "/lol-banners/v1/current-summoner/flags";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolBannersBannerFlag>>(value)?)
}

pub async fn get_lol_banners_v1_current_summoner_flags_equipped(client: RESTClient) -> Result<LolBannersBannerFlag, Box<dyn Error>> {
    let url = "/lol-banners/v1/current-summoner/flags/equipped";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolBannersBannerFlag>(value)?)
}

pub async fn get_lol_banners_v1_current_summoner_frames_equipped(client: RESTClient) -> Result<LolBannersBannerFrame, Box<dyn Error>> {
    let url = "/lol-banners/v1/current-summoner/frames/equipped";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolBannersBannerFrame>(value)?)
}

pub async fn get_lol_banners_v1_players_by_puuid_flags_equipped(client: RESTClient, puuid: String) -> Result<LolBannersBannerFlag, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-banners/v1/players/{{puuid}}/flags/equipped", &json!({"puuid": puuid}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolBannersBannerFlag>(value)?)
}

pub async fn get_lol_career_stats_v1_champion_averages_by_champion_id_by_position_by_tier_by_queue(client: RESTClient, champion_id: i32, position: LolCareerStatsSummonersRiftPosition, tier: LolCareerStatsRankedTier, queue: LolCareerStatsCareerStatsQueueType) -> Result<LolCareerStatsChampionQueueStatsResponse, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-career-stats/v1/champion-averages/{championId}/{position}/{tier}/{{queue}}", &json!({"champion_id": champion_id, "position": position, "tier": tier, "queue": queue}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolCareerStatsChampionQueueStatsResponse>(value)?)
}

pub async fn get_lol_career_stats_v1_champion_averages_season_by_season_by_champion_id_by_position_by_tier_by_queue(client: RESTClient, season: u32, champion_id: i32, position: LolCareerStatsSummonersRiftPosition, tier: LolCareerStatsRankedTier, queue: LolCareerStatsCareerStatsQueueType) -> Result<LolCareerStatsChampionQueueStatsResponse, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-career-stats/v1/champion-averages/season/{season}/{championId}/{position}/{tier}/{{queue}}", &json!({"season": season, "champion_id": champion_id, "position": position, "tier": tier, "queue": queue}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolCareerStatsChampionQueueStatsResponse>(value)?)
}

pub async fn get_lol_career_stats_v1_champion_experts_by_champion_id_by_position(client: RESTClient, champion_id: i32, position: LolCareerStatsSummonersRiftPosition) -> Result<Vec<LolCareerStatsExpertPlayer>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-career-stats/v1/champion-experts/{championId}/{{position}}", &json!({"champion_id": champion_id, "position": position}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolCareerStatsExpertPlayer>>(value)?)
}

pub async fn get_lol_career_stats_v1_champion_experts_season_by_season_by_champion_id_by_position(client: RESTClient, season: u32, champion_id: i32, position: LolCareerStatsSummonersRiftPosition) -> Result<Vec<LolCareerStatsExpertPlayer>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-career-stats/v1/champion-experts/season/{season}/{championId}/{{position}}", &json!({"season": season, "champion_id": champion_id, "position": position}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolCareerStatsExpertPlayer>>(value)?)
}

pub async fn get_lol_career_stats_v1_position_averages_by_position_by_tier_by_queue(client: RESTClient, position: LolCareerStatsSummonersRiftPosition, tier: LolCareerStatsRankedTier, queue: LolCareerStatsCareerStatsQueueType) -> Result<LolCareerStatsChampionQueueStatsResponse, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-career-stats/v1/position-averages/{position}/{tier}/{{queue}}", &json!({"position": position, "tier": tier, "queue": queue}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolCareerStatsChampionQueueStatsResponse>(value)?)
}

pub async fn get_lol_career_stats_v1_position_averages_season_by_season_by_position_by_tier_by_queue(client: RESTClient, season: u32, position: LolCareerStatsSummonersRiftPosition, tier: LolCareerStatsRankedTier, queue: LolCareerStatsCareerStatsQueueType) -> Result<LolCareerStatsChampionQueueStatsResponse, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-career-stats/v1/position-averages/season/{season}/{position}/{tier}/{{queue}}", &json!({"season": season, "position": position, "tier": tier, "queue": queue}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolCareerStatsChampionQueueStatsResponse>(value)?)
}

pub async fn get_lol_career_stats_v1_position_experts_by_position(client: RESTClient, position: LolCareerStatsSummonersRiftPosition) -> Result<Vec<LolCareerStatsExpertPlayer>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-career-stats/v1/position-experts/{{position}}", &json!({"position": position}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolCareerStatsExpertPlayer>>(value)?)
}

pub async fn get_lol_career_stats_v1_position_experts_season_by_season_by_position(client: RESTClient, season: u32, position: LolCareerStatsSummonersRiftPosition) -> Result<Vec<LolCareerStatsExpertPlayer>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-career-stats/v1/position-experts/season/{season}/{{position}}", &json!({"season": season, "position": position}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolCareerStatsExpertPlayer>>(value)?)
}

pub async fn get_lol_career_stats_v1_summoner_games_by_puuid(client: RESTClient, puuid: String) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-career-stats/v1/summoner-games/{{puuid}}", &json!({"puuid": puuid}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(value)
}

pub async fn get_lol_career_stats_v1_summoner_games_by_puuid_season_by_season(client: RESTClient, puuid: String, season: u32) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-career-stats/v1/summoner-games/{puuid}/season/{{season}}", &json!({"puuid": puuid, "season": season}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(value)
}

pub async fn get_lol_career_stats_v1_summoner_stats_by_puuid_by_season_by_queue_by_position(client: RESTClient, puuid: String, season: u32, queue: LolCareerStatsCareerStatsQueueType, position: LolCareerStatsSummonersRiftPosition, champion_id: Option<i32>) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-career-stats/v1/summoner-stats/{puuid}/{season}/{queue}/{{position}}?championId={{champion_id}}", &json!({"puuid": puuid, "season": season, "queue": queue, "position": position, "champion_id": champion_id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(value)
}

pub async fn get_lol_catalog_v1_item_details(client: RESTClient, inventory_type: String, item_id: i64) -> Result<LolCatalogCatalogPluginItemWithDetails, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-catalog/v1/item-details?inventoryType={{inventory_type}}&itemId={{item_id}}", &json!({"inventory_type": inventory_type, "item_id": item_id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolCatalogCatalogPluginItemWithDetails>(value)?)
}

pub async fn get_lol_catalog_v1_items(client: RESTClient, inventory_type: String, item_ids: Vec<i64>) -> Result<Vec<LolCatalogItemChoiceDetails>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-catalog/v1/items?inventoryType={{inventory_type}}&itemIds={{item_ids}}", &json!({"inventory_type": inventory_type, "item_ids": item_ids}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolCatalogItemChoiceDetails>>(value)?)
}

pub async fn get_lol_catalog_v1_items_by_inventory_type(client: RESTClient, inventory_type: String) -> Result<Vec<LolCatalogCatalogPluginItem>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-catalog/v1/items/{{inventory_type}}", &json!({"inventory_type": inventory_type}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolCatalogCatalogPluginItem>>(value)?)
}

pub async fn get_lol_challenges_v1_available_queue_ids(client: RESTClient) -> Result<Vec<i32>, Box<dyn Error>> {
    let url = "/lol-challenges/v1/available-queue-ids";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<i32>>(value)?)
}

pub async fn get_lol_challenges_v1_challenges_category_data(client: RESTClient) -> Result<Vec<LolChallengesUIChallenge>, Box<dyn Error>> {
    let url = "/lol-challenges/v1/challenges/category-data";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolChallengesUIChallenge>>(value)?)
}

pub async fn get_lol_challenges_v1_challenges_local_player(client: RESTClient) -> Result<Vec<LolChallengesUIChallenge>, Box<dyn Error>> {
    let url = "/lol-challenges/v1/challenges/local-player";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolChallengesUIChallenge>>(value)?)
}

pub async fn get_lol_challenges_v1_client_state(client: RESTClient) -> Result<LolChallengesClientState, Box<dyn Error>> {
    let url = "/lol-challenges/v1/client-state";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolChallengesClientState>(value)?)
}

pub async fn get_lol_challenges_v1_level_points(client: RESTClient) -> Result<HashMap<String, i64>, Box<dyn Error>> {
    let url = "/lol-challenges/v1/level-points";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<HashMap<String, i64>>(value)?)
}

pub async fn get_lol_challenges_v1_my_updated_challenges_by_game_id(client: RESTClient, game_id: u64) -> Result<Vec<LolChallengesUIChallenge>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-challenges/v1/my-updated-challenges/{{game_id}}", &json!({"game_id": game_id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolChallengesUIChallenge>>(value)?)
}

pub async fn get_lol_challenges_v1_penalty(client: RESTClient) -> Result<LolChallengesUIChallengePenalty, Box<dyn Error>> {
    let url = "/lol-challenges/v1/penalty";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolChallengesUIChallengePenalty>(value)?)
}

pub async fn get_lol_challenges_v1_seasons(client: RESTClient) -> Result<Vec<LolChallengesChallengeSeason>, Box<dyn Error>> {
    let url = "/lol-challenges/v1/seasons";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolChallengesChallengeSeason>>(value)?)
}

pub async fn get_lol_challenges_v1_suggested_challenges_local_player(client: RESTClient) -> Result<Vec<LolChallengesUIChallenge>, Box<dyn Error>> {
    let url = "/lol-challenges/v1/suggested-challenges/local-player";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolChallengesUIChallenge>>(value)?)
}

pub async fn get_lol_challenges_v1_summary_player_data_local_player(client: RESTClient) -> Result<LolChallengesUIPlayerSummary, Box<dyn Error>> {
    let url = "/lol-challenges/v1/summary-player-data/local-player";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolChallengesUIPlayerSummary>(value)?)
}

pub async fn get_lol_challenges_v1_summary_player_data_player_by_puuid(client: RESTClient, puuid: String) -> Result<LolChallengesUIPlayerSummary, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-challenges/v1/summary-player-data/player/{{puuid}}", &json!({"puuid": puuid}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolChallengesUIPlayerSummary>(value)?)
}

pub async fn get_lol_challenges_v1_summary_players_data_players(client: RESTClient, puuids: Vec<String>) -> Result<HashMap<String, LolChallengesUIPlayerSummary>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-challenges/v1/summary-players-data/players?puuids={{puuids}}", &json!({"puuids": puuids}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<HashMap<String, LolChallengesUIPlayerSummary>>(value)?)
}

pub async fn get_lol_challenges_v1_titles(client: RESTClient) -> Result<HashMap<String, LolChallengesUITitle>, Box<dyn Error>> {
    let url = "/lol-challenges/v1/titles";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<HashMap<String, LolChallengesUITitle>>(value)?)
}

pub async fn get_lol_challenges_v1_titles_local_player(client: RESTClient) -> Result<Vec<LolChallengesUITitle>, Box<dyn Error>> {
    let url = "/lol-challenges/v1/titles/local-player";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolChallengesUITitle>>(value)?)
}

pub async fn get_lol_challenges_v1_updated_challenges_by_game_id_by_puuid(client: RESTClient, game_id: u64, puuid: String) -> Result<Vec<LolChallengesUIChallenge>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-challenges/v1/updated-challenges/{gameId}/{{puuid}}", &json!({"game_id": game_id, "puuid": puuid}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolChallengesUIChallenge>>(value)?)
}

pub async fn get_lol_champ_select_legacy_v1_bannable_champion_ids(client: RESTClient) -> Result<Vec<i32>, Box<dyn Error>> {
    let url = "/lol-champ-select-legacy/v1/bannable-champion-ids";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<i32>>(value)?)
}

pub async fn get_lol_champ_select_legacy_v1_current_champion(client: RESTClient) -> Result<i32, Box<dyn Error>> {
    let url = "/lol-champ-select-legacy/v1/current-champion";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<i32>(value)?)
}

pub async fn get_lol_champ_select_legacy_v1_disabled_champion_ids(client: RESTClient) -> Result<Vec<i32>, Box<dyn Error>> {
    let url = "/lol-champ-select-legacy/v1/disabled-champion-ids";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<i32>>(value)?)
}

pub async fn get_lol_champ_select_legacy_v1_implementation_active(client: RESTClient) -> Result<bool, Box<dyn Error>> {
    let url = "/lol-champ-select-legacy/v1/implementation-active";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<bool>(value)?)
}

pub async fn get_lol_champ_select_legacy_v1_pickable_champion_ids(client: RESTClient) -> Result<Vec<i32>, Box<dyn Error>> {
    let url = "/lol-champ-select-legacy/v1/pickable-champion-ids";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<i32>>(value)?)
}

pub async fn get_lol_champ_select_legacy_v1_pickable_skin_ids(client: RESTClient) -> Result<Vec<i32>, Box<dyn Error>> {
    let url = "/lol-champ-select-legacy/v1/pickable-skin-ids";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<i32>>(value)?)
}

pub async fn get_lol_champ_select_legacy_v1_session(client: RESTClient) -> Result<LolChampSelectLegacyChampSelectSession, Box<dyn Error>> {
    let url = "/lol-champ-select-legacy/v1/session";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolChampSelectLegacyChampSelectSession>(value)?)
}

pub async fn get_lol_champ_select_legacy_v1_session_my_selection(client: RESTClient) -> Result<LolChampSelectLegacyChampSelectPlayerSelection, Box<dyn Error>> {
    let url = "/lol-champ-select-legacy/v1/session/my-selection";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolChampSelectLegacyChampSelectPlayerSelection>(value)?)
}

pub async fn get_lol_champ_select_legacy_v1_session_timer(client: RESTClient) -> Result<LolChampSelectLegacyChampSelectTimer, Box<dyn Error>> {
    let url = "/lol-champ-select-legacy/v1/session/timer";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolChampSelectLegacyChampSelectTimer>(value)?)
}

pub async fn get_lol_champ_select_legacy_v1_session_trades(client: RESTClient) -> Result<Vec<LolChampSelectLegacyChampSelectTradeContract>, Box<dyn Error>> {
    let url = "/lol-champ-select-legacy/v1/session/trades";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolChampSelectLegacyChampSelectTradeContract>>(value)?)
}

pub async fn get_lol_champ_select_legacy_v1_session_trades_by_id(client: RESTClient, id: i64) -> Result<LolChampSelectLegacyChampSelectTradeContract, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-champ-select-legacy/v1/session/trades/{{id}}", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolChampSelectLegacyChampSelectTradeContract>(value)?)
}

pub async fn get_lol_champ_select_legacy_v1_team_boost(client: RESTClient) -> Result<LolChampSelectLegacyTeamBoost, Box<dyn Error>> {
    let url = "/lol-champ-select-legacy/v1/team-boost";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolChampSelectLegacyTeamBoost>(value)?)
}

pub async fn get_lol_champ_select_v1_all_grid_champions(client: RESTClient) -> Result<Vec<LolChampSelectChampGridChampion>, Box<dyn Error>> {
    let url = "/lol-champ-select/v1/all-grid-champions";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolChampSelectChampGridChampion>>(value)?)
}

pub async fn get_lol_champ_select_v1_bannable_champion_ids(client: RESTClient) -> Result<Vec<i32>, Box<dyn Error>> {
    let url = "/lol-champ-select/v1/bannable-champion-ids";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<i32>>(value)?)
}

pub async fn get_lol_champ_select_v1_current_champion(client: RESTClient) -> Result<i32, Box<dyn Error>> {
    let url = "/lol-champ-select/v1/current-champion";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<i32>(value)?)
}

pub async fn get_lol_champ_select_v1_disabled_champion_ids(client: RESTClient) -> Result<Vec<i32>, Box<dyn Error>> {
    let url = "/lol-champ-select/v1/disabled-champion-ids";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<i32>>(value)?)
}

pub async fn get_lol_champ_select_v1_grid_champions_by_champion_id(client: RESTClient, champion_id: i32) -> Result<LolChampSelectChampGridChampion, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-champ-select/v1/grid-champions/{{champion_id}}", &json!({"champion_id": champion_id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolChampSelectChampGridChampion>(value)?)
}

pub async fn get_lol_champ_select_v1_muted_players(client: RESTClient) -> Result<Vec<LolChampSelectMutedPlayerInfo>, Box<dyn Error>> {
    let url = "/lol-champ-select/v1/muted-players";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolChampSelectMutedPlayerInfo>>(value)?)
}

pub async fn get_lol_champ_select_v1_ongoing_swap(client: RESTClient) -> Result<LolChampSelectChampSelectSwapNotification, Box<dyn Error>> {
    let url = "/lol-champ-select/v1/ongoing-swap";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolChampSelectChampSelectSwapNotification>(value)?)
}

pub async fn get_lol_champ_select_v1_ongoing_trade(client: RESTClient) -> Result<LolChampSelectChampSelectTradeNotification, Box<dyn Error>> {
    let url = "/lol-champ-select/v1/ongoing-trade";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolChampSelectChampSelectTradeNotification>(value)?)
}

pub async fn get_lol_champ_select_v1_pickable_champion_ids(client: RESTClient) -> Result<Vec<i32>, Box<dyn Error>> {
    let url = "/lol-champ-select/v1/pickable-champion-ids";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<i32>>(value)?)
}

pub async fn get_lol_champ_select_v1_pickable_skin_ids(client: RESTClient) -> Result<Vec<i32>, Box<dyn Error>> {
    let url = "/lol-champ-select/v1/pickable-skin-ids";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<i32>>(value)?)
}

pub async fn get_lol_champ_select_v1_pin_drop_notification(client: RESTClient) -> Result<LolChampSelectChampSelectPinDropNotification, Box<dyn Error>> {
    let url = "/lol-champ-select/v1/pin-drop-notification";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolChampSelectChampSelectPinDropNotification>(value)?)
}

pub async fn get_lol_champ_select_v1_session(client: RESTClient) -> Result<LolChampSelectChampSelectSession, Box<dyn Error>> {
    let url = "/lol-champ-select/v1/session";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolChampSelectChampSelectSession>(value)?)
}

pub async fn get_lol_champ_select_v1_session_my_selection(client: RESTClient) -> Result<LolChampSelectChampSelectPlayerSelection, Box<dyn Error>> {
    let url = "/lol-champ-select/v1/session/my-selection";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolChampSelectChampSelectPlayerSelection>(value)?)
}

pub async fn get_lol_champ_select_v1_session_swaps(client: RESTClient) -> Result<Vec<LolChampSelectChampSelectSwapContract>, Box<dyn Error>> {
    let url = "/lol-champ-select/v1/session/swaps";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolChampSelectChampSelectSwapContract>>(value)?)
}

pub async fn get_lol_champ_select_v1_session_swaps_by_id(client: RESTClient, id: i64) -> Result<LolChampSelectChampSelectSwapContract, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-champ-select/v1/session/swaps/{{id}}", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolChampSelectChampSelectSwapContract>(value)?)
}

pub async fn get_lol_champ_select_v1_session_timer(client: RESTClient) -> Result<LolChampSelectChampSelectTimer, Box<dyn Error>> {
    let url = "/lol-champ-select/v1/session/timer";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolChampSelectChampSelectTimer>(value)?)
}

pub async fn get_lol_champ_select_v1_session_trades(client: RESTClient) -> Result<Vec<LolChampSelectChampSelectTradeContract>, Box<dyn Error>> {
    let url = "/lol-champ-select/v1/session/trades";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolChampSelectChampSelectTradeContract>>(value)?)
}

pub async fn get_lol_champ_select_v1_session_trades_by_id(client: RESTClient, id: i64) -> Result<LolChampSelectChampSelectTradeContract, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-champ-select/v1/session/trades/{{id}}", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolChampSelectChampSelectTradeContract>(value)?)
}

pub async fn get_lol_champ_select_v1_sfx_notifications(client: RESTClient) -> Result<Vec<LolChampSelectSfxNotification>, Box<dyn Error>> {
    let url = "/lol-champ-select/v1/sfx-notifications";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolChampSelectSfxNotification>>(value)?)
}

pub async fn get_lol_champ_select_v1_skin_carousel_skins(client: RESTClient) -> Result<Vec<LolChampSelectSkinSelectorSkin>, Box<dyn Error>> {
    let url = "/lol-champ-select/v1/skin-carousel-skins";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolChampSelectSkinSelectorSkin>>(value)?)
}

pub async fn get_lol_champ_select_v1_skin_selector_info(client: RESTClient) -> Result<LolChampSelectSkinSelectorInfo, Box<dyn Error>> {
    let url = "/lol-champ-select/v1/skin-selector-info";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolChampSelectSkinSelectorInfo>(value)?)
}

pub async fn get_lol_champ_select_v1_summoners_by_slot_id(client: RESTClient, slot_id: u32) -> Result<LolChampSelectChampSelectSummoner, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-champ-select/v1/summoners/{{slot_id}}", &json!({"slot_id": slot_id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolChampSelectChampSelectSummoner>(value)?)
}

pub async fn get_lol_champ_select_v1_team_boost(client: RESTClient) -> Result<LolChampSelectTeamBoost, Box<dyn Error>> {
    let url = "/lol-champ-select/v1/team-boost";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolChampSelectTeamBoost>(value)?)
}

pub async fn get_lol_champions_v1_inventories_by_summoner_id_champions(client: RESTClient, summoner_id: u64) -> Result<Vec<LolChampionsCollectionsChampion>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-champions/v1/inventories/{{summoner_id}}/champions", &json!({"summoner_id": summoner_id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolChampionsCollectionsChampion>>(value)?)
}

pub async fn get_lol_champions_v1_inventories_by_summoner_id_champions_by_champion_id(client: RESTClient, summoner_id: u64, champion_id: i32) -> Result<LolChampionsCollectionsChampion, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-champions/v1/inventories/{summonerId}/champions/{{champion_id}}", &json!({"summoner_id": summoner_id, "champion_id": champion_id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolChampionsCollectionsChampion>(value)?)
}

pub async fn get_lol_champions_v1_inventories_by_summoner_id_champions_by_champion_id_skins(client: RESTClient, summoner_id: u64, champion_id: i32) -> Result<Vec<LolChampionsCollectionsChampionSkin>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-champions/v1/inventories/{summonerId}/champions/{{champion_id}}/skins", &json!({"summoner_id": summoner_id, "champion_id": champion_id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolChampionsCollectionsChampionSkin>>(value)?)
}

pub async fn get_lol_champions_v1_inventories_by_summoner_id_champions_by_champion_id_skins_by_champion_skin_id(client: RESTClient, summoner_id: u64, champion_id: i32, champion_skin_id: i32) -> Result<LolChampionsCollectionsChampionSkin, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-champions/v1/inventories/{summonerId}/champions/{championId}/skins/{{champion_skin_id}}", &json!({"summoner_id": summoner_id, "champion_id": champion_id, "champion_skin_id": champion_skin_id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolChampionsCollectionsChampionSkin>(value)?)
}

pub async fn get_lol_champions_v1_inventories_by_summoner_id_champions_by_champion_id_skins_by_skin_id_chromas(client: RESTClient, summoner_id: u64, champion_id: i32, skin_id: i32) -> Result<Vec<LolChampionsCollectionsChampionChroma>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-champions/v1/inventories/{summonerId}/champions/{championId}/skins/{{skin_id}}/chromas", &json!({"summoner_id": summoner_id, "champion_id": champion_id, "skin_id": skin_id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolChampionsCollectionsChampionChroma>>(value)?)
}

pub async fn get_lol_champions_v1_inventories_by_summoner_id_champions_minimal(client: RESTClient, summoner_id: u64) -> Result<Vec<LolChampionsCollectionsChampionMinimal>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-champions/v1/inventories/{{summoner_id}}/champions-minimal", &json!({"summoner_id": summoner_id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolChampionsCollectionsChampionMinimal>>(value)?)
}

pub async fn get_lol_champions_v1_inventories_by_summoner_id_champions_playable_count(client: RESTClient, summoner_id: u64) -> Result<LolChampionsCollectionsChampionPlayableCounts, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-champions/v1/inventories/{{summoner_id}}/champions-playable-count", &json!({"summoner_id": summoner_id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolChampionsCollectionsChampionPlayableCounts>(value)?)
}

pub async fn get_lol_champions_v1_inventories_by_summoner_id_skins_minimal(client: RESTClient, summoner_id: u64) -> Result<Vec<LolChampionsCollectionsChampionSkinMinimal>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-champions/v1/inventories/{{summoner_id}}/skins-minimal", &json!({"summoner_id": summoner_id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolChampionsCollectionsChampionSkinMinimal>>(value)?)
}

pub async fn get_lol_champions_v1_owned_champions_minimal(client: RESTClient) -> Result<Vec<LolChampionsCollectionsChampionMinimal>, Box<dyn Error>> {
    let url = "/lol-champions/v1/owned-champions-minimal";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolChampionsCollectionsChampionMinimal>>(value)?)
}

pub async fn get_lol_chat_v1_blocked_players(client: RESTClient) -> Result<Vec<LolChatBlockedPlayerResource>, Box<dyn Error>> {
    let url = "/lol-chat/v1/blocked-players";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolChatBlockedPlayerResource>>(value)?)
}

pub async fn get_lol_chat_v1_blocked_players_by_id(client: RESTClient, id: String) -> Result<LolChatBlockedPlayerResource, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-chat/v1/blocked-players/{{id}}", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolChatBlockedPlayerResource>(value)?)
}

pub async fn get_lol_chat_v1_config(client: RESTClient) -> Result<LolChatChatServiceDynamicClientConfig, Box<dyn Error>> {
    let url = "/lol-chat/v1/config";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolChatChatServiceDynamicClientConfig>(value)?)
}

pub async fn get_lol_chat_v1_conversations(client: RESTClient) -> Result<Vec<LolChatConversationResource>, Box<dyn Error>> {
    let url = "/lol-chat/v1/conversations";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolChatConversationResource>>(value)?)
}

pub async fn get_lol_chat_v1_conversations_active(client: RESTClient) -> Result<LolChatActiveConversationResource, Box<dyn Error>> {
    let url = "/lol-chat/v1/conversations/active";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolChatActiveConversationResource>(value)?)
}

pub async fn get_lol_chat_v1_conversations_by_id(client: RESTClient, id: String) -> Result<LolChatConversationResource, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-chat/v1/conversations/{{id}}", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolChatConversationResource>(value)?)
}

pub async fn get_lol_chat_v1_conversations_by_id_messages(client: RESTClient, id: String) -> Result<Vec<LolChatConversationMessageResource>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-chat/v1/conversations/{{id}}/messages", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolChatConversationMessageResource>>(value)?)
}

pub async fn get_lol_chat_v1_conversations_by_id_participants(client: RESTClient, id: String) -> Result<Vec<LolChatUserResource>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-chat/v1/conversations/{{id}}/participants", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolChatUserResource>>(value)?)
}

pub async fn get_lol_chat_v1_conversations_by_id_participants_by_pid(client: RESTClient, id: String, pid: String) -> Result<LolChatUserResource, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-chat/v1/conversations/{id}/participants/{{pid}}", &json!({"id": id, "pid": pid}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolChatUserResource>(value)?)
}

pub async fn get_lol_chat_v1_conversations_notify(client: RESTClient) -> Result<String, Box<dyn Error>> {
    let url = "/lol-chat/v1/conversations/notify";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<String>(value)?)
}

pub async fn get_lol_chat_v1_errors(client: RESTClient) -> Result<Vec<LolChatErrorResource>, Box<dyn Error>> {
    let url = "/lol-chat/v1/errors";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolChatErrorResource>>(value)?)
}

pub async fn get_lol_chat_v1_friend_counts(client: RESTClient) -> Result<LolChatFriendCountsResource, Box<dyn Error>> {
    let url = "/lol-chat/v1/friend-counts";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolChatFriendCountsResource>(value)?)
}

pub async fn get_lol_chat_v1_friend_exists_by_summoner_id(client: RESTClient, summoner_id: u64) -> Result<bool, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-chat/v1/friend-exists/{{summoner_id}}", &json!({"summoner_id": summoner_id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<bool>(value)?)
}

pub async fn get_lol_chat_v1_friend_groups(client: RESTClient) -> Result<Vec<LolChatGroupResource>, Box<dyn Error>> {
    let url = "/lol-chat/v1/friend-groups";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolChatGroupResource>>(value)?)
}

pub async fn get_lol_chat_v1_friend_groups_by_id(client: RESTClient, id: u32) -> Result<LolChatGroupResource, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-chat/v1/friend-groups/{{id}}", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolChatGroupResource>(value)?)
}

pub async fn get_lol_chat_v1_friend_groups_by_id_friends(client: RESTClient, id: u32) -> Result<Vec<LolChatFriendResource>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-chat/v1/friend-groups/{{id}}/friends", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolChatFriendResource>>(value)?)
}

pub async fn get_lol_chat_v1_friend_requests(client: RESTClient) -> Result<Vec<LolChatFriendRequestResource>, Box<dyn Error>> {
    let url = "/lol-chat/v1/friend-requests";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolChatFriendRequestResource>>(value)?)
}

pub async fn get_lol_chat_v1_friends(client: RESTClient) -> Result<Vec<LolChatFriendResource>, Box<dyn Error>> {
    let url = "/lol-chat/v1/friends";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolChatFriendResource>>(value)?)
}

pub async fn get_lol_chat_v1_friends_by_id(client: RESTClient, id: String) -> Result<LolChatFriendResource, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-chat/v1/friends/{{id}}", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolChatFriendResource>(value)?)
}

pub async fn get_lol_chat_v1_me(client: RESTClient) -> Result<LolChatUserResource, Box<dyn Error>> {
    let url = "/lol-chat/v1/me";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolChatUserResource>(value)?)
}

pub async fn get_lol_chat_v1_player_mutes(client: RESTClient) -> Result<HashMap<String, LolChatPlayerMuteStatus>, Box<dyn Error>> {
    let url = "/lol-chat/v1/player-mutes";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<HashMap<String, LolChatPlayerMuteStatus>>(value)?)
}

pub async fn get_lol_chat_v1_resources(client: RESTClient) -> Result<LolChatProductMetadataMap, Box<dyn Error>> {
    let url = "/lol-chat/v1/resources";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolChatProductMetadataMap>(value)?)
}

pub async fn get_lol_chat_v1_session(client: RESTClient) -> Result<LolChatSessionResource, Box<dyn Error>> {
    let url = "/lol-chat/v1/session";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolChatSessionResource>(value)?)
}

pub async fn get_lol_chat_v1_settings(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/lol-chat/v1/settings";
    let value = client.get(url.to_owned()).await?;
    Ok(value)
}

pub async fn get_lol_chat_v1_settings_by_key(client: RESTClient, key: String) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-chat/v1/settings/{{key}}", &json!({"key": key}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(value)
}

pub async fn get_lol_clash_v1_all_tournaments(client: RESTClient) -> Result<Vec<TournamentDTO>, Box<dyn Error>> {
    let url = "/lol-clash/v1/all-tournaments";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<TournamentDTO>>(value)?)
}

pub async fn get_lol_clash_v1_awaiting_resent_eog(client: RESTClient) -> Result<bool, Box<dyn Error>> {
    let url = "/lol-clash/v1/awaiting-resent-eog";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<bool>(value)?)
}

pub async fn get_lol_clash_v1_bracket_by_bracket_id(client: RESTClient, bracket_id: i64) -> Result<LolClashBracket, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-clash/v1/bracket/{{bracket_id}}", &json!({"bracket_id": bracket_id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolClashBracket>(value)?)
}

pub async fn get_lol_clash_v1_checkin_allowed(client: RESTClient) -> Result<bool, Box<dyn Error>> {
    let url = "/lol-clash/v1/checkin-allowed";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<bool>(value)?)
}

pub async fn get_lol_clash_v1_current_tournament_ids(client: RESTClient) -> Result<Vec<i64>, Box<dyn Error>> {
    let url = "/lol-clash/v1/currentTournamentIds";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<i64>>(value)?)
}

pub async fn get_lol_clash_v1_disabled_config(client: RESTClient) -> Result<LolClashClashDisabledConfig, Box<dyn Error>> {
    let url = "/lol-clash/v1/disabled-config";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolClashClashDisabledConfig>(value)?)
}

pub async fn get_lol_clash_v1_enabled(client: RESTClient) -> Result<bool, Box<dyn Error>> {
    let url = "/lol-clash/v1/enabled";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<bool>(value)?)
}

pub async fn get_lol_clash_v1_eog_player_update(client: RESTClient) -> Result<LolClashEogPlayerUpdateDTO, Box<dyn Error>> {
    let url = "/lol-clash/v1/eog-player-update";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolClashEogPlayerUpdateDTO>(value)?)
}

pub async fn get_lol_clash_v1_event_by_uuid(client: RESTClient) -> Result<ClashEventData, Box<dyn Error>> {
    let url = "/lol-clash/v1/event/{uuid}";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<ClashEventData>(value)?)
}

pub async fn get_lol_clash_v1_game_end(client: RESTClient) -> Result<LolClashTournamentGameEnd, Box<dyn Error>> {
    let url = "/lol-clash/v1/game-end";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolClashTournamentGameEnd>(value)?)
}

pub async fn get_lol_clash_v1_historyandwinners(client: RESTClient) -> Result<LolClashTournamentHistoryAndWinners, Box<dyn Error>> {
    let url = "/lol-clash/v1/historyandwinners";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolClashTournamentHistoryAndWinners>(value)?)
}

pub async fn get_lol_clash_v1_iconconfig(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/lol-clash/v1/iconconfig";
    let value = client.get(url.to_owned()).await?;
    Ok(value)
}

pub async fn get_lol_clash_v1_invited_roster_ids(client: RESTClient) -> Result<Vec<String>, Box<dyn Error>> {
    let url = "/lol-clash/v1/invited-roster-ids";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<String>>(value)?)
}

pub async fn get_lol_clash_v1_lft_team_requests(client: RESTClient) -> Result<Vec<PendingOpenedTeamDTO>, Box<dyn Error>> {
    let url = "/lol-clash/v1/lft/team/requests";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<PendingOpenedTeamDTO>>(value)?)
}

pub async fn get_lol_clash_v1_notifications(client: RESTClient) -> Result<LolClashPlayerNotificationData, Box<dyn Error>> {
    let url = "/lol-clash/v1/notifications";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolClashPlayerNotificationData>(value)?)
}

pub async fn get_lol_clash_v1_ping(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/lol-clash/v1/ping";
    let value = client.get(url.to_owned()).await?;
    Ok(value)
}

pub async fn get_lol_clash_v1_player(client: RESTClient) -> Result<LolClashPlayerData, Box<dyn Error>> {
    let url = "/lol-clash/v1/player";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolClashPlayerData>(value)?)
}

pub async fn get_lol_clash_v1_player_chat_rosters(client: RESTClient) -> Result<Vec<LolClashPlayerChatRoster>, Box<dyn Error>> {
    let url = "/lol-clash/v1/player/chat-rosters";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolClashPlayerChatRoster>>(value)?)
}

pub async fn get_lol_clash_v1_player_history(client: RESTClient) -> Result<Vec<LolClashRosterStats>, Box<dyn Error>> {
    let url = "/lol-clash/v1/player/history";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolClashRosterStats>>(value)?)
}

pub async fn get_lol_clash_v1_playmode_restricted(client: RESTClient) -> Result<bool, Box<dyn Error>> {
    let url = "/lol-clash/v1/playmode-restricted";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<bool>(value)?)
}

pub async fn get_lol_clash_v1_ready(client: RESTClient) -> Result<bool, Box<dyn Error>> {
    let url = "/lol-clash/v1/ready";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<bool>(value)?)
}

pub async fn get_lol_clash_v1_rewards(client: RESTClient) -> Result<LolClashPlayerRewards, Box<dyn Error>> {
    let url = "/lol-clash/v1/rewards";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolClashPlayerRewards>(value)?)
}

pub async fn get_lol_clash_v1_roster_by_roster_id(client: RESTClient, roster_id: String) -> Result<LolClashRoster, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-clash/v1/roster/{{roster_id}}", &json!({"roster_id": roster_id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolClashRoster>(value)?)
}

pub async fn get_lol_clash_v1_roster_by_roster_id_stats(client: RESTClient, roster_id: i64) -> Result<LolClashRosterStats, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-clash/v1/roster/{{roster_id}}/stats", &json!({"roster_id": roster_id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolClashRosterStats>(value)?)
}

pub async fn get_lol_clash_v1_scouting_champions(client: RESTClient, summoner_ids: Vec<u64>) -> Result<Vec<LolClashScoutingChampions>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-clash/v1/scouting/champions?summonerIds={{summoner_ids}}", &json!({"summoner_ids": summoner_ids}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolClashScoutingChampions>>(value)?)
}

pub async fn get_lol_clash_v1_scouting_matchhistory(client: RESTClient, summoner_ids: Vec<u64>) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-clash/v1/scouting/matchhistory?summonerIds={{summoner_ids}}", &json!({"summoner_ids": summoner_ids}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(value)
}

pub async fn get_lol_clash_v1_season_rewards_by_season_id(client: RESTClient, season_id: i32) -> Result<ClashSeasonRewardResult, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-clash/v1/season-rewards/{{season_id}}", &json!({"season_id": season_id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<ClashSeasonRewardResult>(value)?)
}

pub async fn get_lol_clash_v1_simple_state_flags(client: RESTClient) -> Result<Vec<LolClashSimpleStateFlag>, Box<dyn Error>> {
    let url = "/lol-clash/v1/simple-state-flags";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolClashSimpleStateFlag>>(value)?)
}

pub async fn get_lol_clash_v1_thirdparty_team_data(client: RESTClient) -> Result<LolClashThirdPartyApiRoster, Box<dyn Error>> {
    let url = "/lol-clash/v1/thirdparty/team-data";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolClashThirdPartyApiRoster>(value)?)
}

pub async fn get_lol_clash_v1_time(client: RESTClient) -> Result<i64, Box<dyn Error>> {
    let url = "/lol-clash/v1/time";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<i64>(value)?)
}

pub async fn get_lol_clash_v1_tournament_by_tournament_id(client: RESTClient, tournament_id: i64) -> Result<LolClashTournament, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-clash/v1/tournament/{{tournament_id}}", &json!({"tournament_id": tournament_id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolClashTournament>(value)?)
}

pub async fn get_lol_clash_v1_tournament_by_tournament_id_get_player_tiers(client: RESTClient, tournament_id: i64, summoner_ids: Vec<u64>) -> Result<Vec<PlayerTierDTO>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-clash/v1/tournament/{{tournament_id}}/get-player-tiers?summonerIds={{summoner_ids}}", &json!({"tournament_id": tournament_id, "summoner_ids": summoner_ids}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<PlayerTierDTO>>(value)?)
}

pub async fn get_lol_clash_v1_tournament_by_tournament_id_player(client: RESTClient, tournament_id: i64) -> Result<LolClashPlayerTournamentData, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-clash/v1/tournament/{{tournament_id}}/player", &json!({"tournament_id": tournament_id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolClashPlayerTournamentData>(value)?)
}

pub async fn get_lol_clash_v1_tournament_by_tournament_id_player_honor_restricted(client: RESTClient, tournament_id: Option<i64>) -> Result<bool, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-clash/v1/tournament/{{tournament_id}}/player-honor-restricted", &json!({"tournament_id": tournament_id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<bool>(value)?)
}

pub async fn get_lol_clash_v1_tournament_by_tournament_id_state_info(client: RESTClient, tournament_id: i64) -> Result<LolClashTournamentStateInfo, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-clash/v1/tournament/{{tournament_id}}/stateInfo", &json!({"tournament_id": tournament_id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolClashTournamentStateInfo>(value)?)
}

pub async fn get_lol_clash_v1_tournament_by_tournament_id_winners(client: RESTClient, tournament_id: i64) -> Result<LolClashTournamentWinnerHistory, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-clash/v1/tournament/{{tournament_id}}/winners", &json!({"tournament_id": tournament_id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolClashTournamentWinnerHistory>(value)?)
}

pub async fn get_lol_clash_v1_tournament_cancelled(client: RESTClient) -> Result<Vec<i64>, Box<dyn Error>> {
    let url = "/lol-clash/v1/tournament/cancelled";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<i64>>(value)?)
}

pub async fn get_lol_clash_v1_tournament_get_all_player_tiers(client: RESTClient) -> Result<Vec<PlayerTierDTO>, Box<dyn Error>> {
    let url = "/lol-clash/v1/tournament/get-all-player-tiers";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<PlayerTierDTO>>(value)?)
}

pub async fn get_lol_clash_v1_tournament_state_info(client: RESTClient) -> Result<Vec<LolClashTournamentStateInfo>, Box<dyn Error>> {
    let url = "/lol-clash/v1/tournament-state-info";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolClashTournamentStateInfo>>(value)?)
}

pub async fn get_lol_clash_v1_tournament_summary(client: RESTClient) -> Result<Vec<LolClashTournamentSummary>, Box<dyn Error>> {
    let url = "/lol-clash/v1/tournament-summary";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolClashTournamentSummary>>(value)?)
}

pub async fn get_lol_clash_v1_visible(client: RESTClient) -> Result<bool, Box<dyn Error>> {
    let url = "/lol-clash/v1/visible";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<bool>(value)?)
}

pub async fn get_lol_clash_v1_voice_enabled(client: RESTClient) -> Result<bool, Box<dyn Error>> {
    let url = "/lol-clash/v1/voice-enabled";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<bool>(value)?)
}

pub async fn get_lol_clash_v2_playmode_restricted(client: RESTClient) -> Result<LolClashPlaymodeRestrictedInfo, Box<dyn Error>> {
    let url = "/lol-clash/v2/playmode-restricted";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolClashPlaymodeRestrictedInfo>(value)?)
}

pub async fn get_lol_client_config_v3_client_config_by_name(client: RESTClient, name: String) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-client-config/v3/client-config/{{name}}", &json!({"name": name}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(value)
}

pub async fn get_lol_collections_v1_inventories_by_summoner_id_backdrop(client: RESTClient, summoner_id: u64) -> Result<LolCollectionsCollectionsSummonerBackdrop, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-collections/v1/inventories/{{summoner_id}}/backdrop", &json!({"summoner_id": summoner_id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolCollectionsCollectionsSummonerBackdrop>(value)?)
}

pub async fn get_lol_collections_v1_inventories_by_summoner_id_champion_mastery(client: RESTClient, summoner_id: u64) -> Result<Vec<LolCollectionsCollectionsChampionMastery>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-collections/v1/inventories/{{summoner_id}}/champion-mastery", &json!({"summoner_id": summoner_id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolCollectionsCollectionsChampionMastery>>(value)?)
}

pub async fn get_lol_collections_v1_inventories_by_summoner_id_champion_mastery_top(client: RESTClient, summoner_id: u64, limit: u64, sort_rule: Option<String>) -> Result<LolCollectionsCollectionsTopChampionMasteries, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-collections/v1/inventories/{{summoner_id}}/champion-mastery/top?limit={{limit}}&sortRule={{sort_rule}}", &json!({"summoner_id": summoner_id, "limit": limit, "sort_rule": sort_rule}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolCollectionsCollectionsTopChampionMasteries>(value)?)
}

pub async fn get_lol_collections_v1_inventories_by_summoner_id_spells(client: RESTClient, summoner_id: u64) -> Result<LolCollectionsCollectionsSummonerSpells, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-collections/v1/inventories/{{summoner_id}}/spells", &json!({"summoner_id": summoner_id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolCollectionsCollectionsSummonerSpells>(value)?)
}

pub async fn get_lol_collections_v1_inventories_by_summoner_id_ward_skins(client: RESTClient, summoner_id: u64) -> Result<Vec<LolCollectionsCollectionsWardSkin>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-collections/v1/inventories/{{summoner_id}}/ward-skins", &json!({"summoner_id": summoner_id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolCollectionsCollectionsWardSkin>>(value)?)
}

pub async fn get_lol_collections_v1_inventories_by_summoner_id_ward_skins_by_ward_skin_id(client: RESTClient, summoner_id: u64, ward_skin_id: i64) -> Result<LolCollectionsCollectionsWardSkin, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-collections/v1/inventories/{summonerId}/ward-skins/{{ward_skin_id}}", &json!({"summoner_id": summoner_id, "ward_skin_id": ward_skin_id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolCollectionsCollectionsWardSkin>(value)?)
}

pub async fn get_lol_collections_v1_inventories_chest_eligibility(client: RESTClient) -> Result<LolCollectionsCollectionsChestEligibility, Box<dyn Error>> {
    let url = "/lol-collections/v1/inventories/chest-eligibility";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolCollectionsCollectionsChestEligibility>(value)?)
}

pub async fn get_lol_collections_v1_inventories_local_player_champion_mastery_score(client: RESTClient) -> Result<u64, Box<dyn Error>> {
    let url = "/lol-collections/v1/inventories/local-player/champion-mastery-score";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<u64>(value)?)
}

pub async fn get_lol_collections_v1_inventories_scouting(client: RESTClient, summoner_ids: Vec<u64>) -> Result<Vec<RankedScoutingDTO>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-collections/v1/inventories/scouting?summonerIds={{summoner_ids}}", &json!({"summoner_ids": summoner_ids}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<RankedScoutingDTO>>(value)?)
}

pub async fn get_lol_content_targeting_v1_filters(client: RESTClient) -> Result<LolContentTargetingContentTargetingFilterResponse, Box<dyn Error>> {
    let url = "/lol-content-targeting/v1/filters";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolContentTargetingContentTargetingFilterResponse>(value)?)
}

pub async fn get_lol_content_targeting_v1_locale(client: RESTClient) -> Result<LolContentTargetingContentTargetingLocaleResponse, Box<dyn Error>> {
    let url = "/lol-content-targeting/v1/locale";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolContentTargetingContentTargetingLocaleResponse>(value)?)
}

pub async fn get_lol_content_targeting_v1_protected_filters(client: RESTClient) -> Result<LolContentTargetingContentTargetingFilterResponse, Box<dyn Error>> {
    let url = "/lol-content-targeting/v1/protected_filters";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolContentTargetingContentTargetingFilterResponse>(value)?)
}

pub async fn get_lol_cosmetics_v1_inventories_by_set_name_companions(client: RESTClient, set_name: String) -> Result<LolCosmeticsCompanionsGroupedViewModel, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-cosmetics/v1/inventories/{{set_name}}/companions", &json!({"set_name": set_name}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolCosmeticsCompanionsGroupedViewModel>(value)?)
}

pub async fn get_lol_cosmetics_v1_inventories_by_set_name_damage_skins(client: RESTClient, set_name: String) -> Result<LolCosmeticsTFTDamageSkinGroupedViewModel, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-cosmetics/v1/inventories/{{set_name}}/damage-skins", &json!({"set_name": set_name}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolCosmeticsTFTDamageSkinGroupedViewModel>(value)?)
}

pub async fn get_lol_cosmetics_v1_inventories_by_set_name_map_skins(client: RESTClient, set_name: String) -> Result<LolCosmeticsTFTMapSkinGroupedViewModel, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-cosmetics/v1/inventories/{{set_name}}/map-skins", &json!({"set_name": set_name}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolCosmeticsTFTMapSkinGroupedViewModel>(value)?)
}

pub async fn get_lol_email_verification_v1_email(client: RESTClient) -> Result<LolEmailVerificationEmailVerificationSession, Box<dyn Error>> {
    let url = "/lol-email-verification/v1/email";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolEmailVerificationEmailVerificationSession>(value)?)
}

pub async fn get_lol_end_of_game_v1_champion_mastery_updates(client: RESTClient) -> Result<LolEndOfGameChampionMasteryUpdate, Box<dyn Error>> {
    let url = "/lol-end-of-game/v1/champion-mastery-updates";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolEndOfGameChampionMasteryUpdate>(value)?)
}

pub async fn get_lol_end_of_game_v1_eog_stats_block(client: RESTClient) -> Result<LolEndOfGameEndOfGameStats, Box<dyn Error>> {
    let url = "/lol-end-of-game/v1/eog-stats-block";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolEndOfGameEndOfGameStats>(value)?)
}

pub async fn get_lol_end_of_game_v1_gameclient_eog_stats_block(client: RESTClient) -> Result<LolEndOfGameGameClientEndOfGameStats, Box<dyn Error>> {
    let url = "/lol-end-of-game/v1/gameclient-eog-stats-block";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolEndOfGameGameClientEndOfGameStats>(value)?)
}

pub async fn get_lol_end_of_game_v1_reported_players(client: RESTClient) -> Result<Vec<u64>, Box<dyn Error>> {
    let url = "/lol-end-of-game/v1/reported-players";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<u64>>(value)?)
}

pub async fn get_lol_end_of_game_v1_tft_eog_stats(client: RESTClient) -> Result<LolEndOfGameTFTEndOfGameViewModel, Box<dyn Error>> {
    let url = "/lol-end-of-game/v1/tft-eog-stats";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolEndOfGameTFTEndOfGameViewModel>(value)?)
}

pub async fn get_lol_esport_stream_notifications_v1_live_streams(client: RESTClient) -> Result<LolEsportStreamNotificationsESportsLiveStreams, Box<dyn Error>> {
    let url = "/lol-esport-stream-notifications/v1/live-streams";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolEsportStreamNotificationsESportsLiveStreams>(value)?)
}

pub async fn get_lol_esport_stream_notifications_v1_stream_url(client: RESTClient) -> Result<String, Box<dyn Error>> {
    let url = "/lol-esport-stream-notifications/v1/stream-url";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<String>(value)?)
}

pub async fn get_lol_event_shop_v1_categories_offers(client: RESTClient) -> Result<Vec<LolEventShopCategoryOffersUIData>, Box<dyn Error>> {
    let url = "/lol-event-shop/v1/categories-offers";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolEventShopCategoryOffersUIData>>(value)?)
}

pub async fn get_lol_event_shop_v1_event_header_data(client: RESTClient) -> Result<LolEventShopEventHeaderUIData, Box<dyn Error>> {
    let url = "/lol-event-shop/v1/event-header-data";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolEventShopEventHeaderUIData>(value)?)
}

pub async fn get_lol_event_shop_v1_failure_loading_reward_track(client: RESTClient) -> Result<LolEventShopEventShopError, Box<dyn Error>> {
    let url = "/lol-event-shop/v1/failure-loading-reward-track";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolEventShopEventShopError>(value)?)
}

pub async fn get_lol_event_shop_v1_failure_loading_token_shop(client: RESTClient) -> Result<LolEventShopEventShopError, Box<dyn Error>> {
    let url = "/lol-event-shop/v1/failure-loading-token-shop";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolEventShopEventShopError>(value)?)
}

pub async fn get_lol_event_shop_v1_info(client: RESTClient) -> Result<LolEventShopEventShopInfoUIData, Box<dyn Error>> {
    let url = "/lol-event-shop/v1/info";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolEventShopEventShopInfoUIData>(value)?)
}

pub async fn get_lol_event_shop_v1_is_grace_period(client: RESTClient) -> Result<bool, Box<dyn Error>> {
    let url = "/lol-event-shop/v1/is-grace-period";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<bool>(value)?)
}

pub async fn get_lol_event_shop_v1_navigation_button_data(client: RESTClient) -> Result<LolEventShopNavigationButtonUIData, Box<dyn Error>> {
    let url = "/lol-event-shop/v1/navigation-button-data";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolEventShopNavigationButtonUIData>(value)?)
}

pub async fn get_lol_event_shop_v1_pass_background_data(client: RESTClient) -> Result<LolEventShopEventBackgroundUIData, Box<dyn Error>> {
    let url = "/lol-event-shop/v1/pass-background-data";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolEventShopEventBackgroundUIData>(value)?)
}

pub async fn get_lol_event_shop_v1_progress_info_data(client: RESTClient) -> Result<LolEventShopProgressInfoUIData, Box<dyn Error>> {
    let url = "/lol-event-shop/v1/progress-info-data";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolEventShopProgressInfoUIData>(value)?)
}

pub async fn get_lol_event_shop_v1_reward_track_bonus_items(client: RESTClient) -> Result<Vec<LolEventShopRewardTrackItem>, Box<dyn Error>> {
    let url = "/lol-event-shop/v1/reward-track-bonus-items";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolEventShopRewardTrackItem>>(value)?)
}

pub async fn get_lol_event_shop_v1_reward_track_bonus_progress(client: RESTClient) -> Result<LolEventShopRewardTrackProgress, Box<dyn Error>> {
    let url = "/lol-event-shop/v1/reward-track-bonus-progress";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolEventShopRewardTrackProgress>(value)?)
}

pub async fn get_lol_event_shop_v1_reward_track_items(client: RESTClient) -> Result<Vec<LolEventShopRewardTrackItem>, Box<dyn Error>> {
    let url = "/lol-event-shop/v1/reward-track-items";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolEventShopRewardTrackItem>>(value)?)
}

pub async fn get_lol_event_shop_v1_reward_track_progress(client: RESTClient) -> Result<LolEventShopRewardTrackProgress, Box<dyn Error>> {
    let url = "/lol-event-shop/v1/reward-track-progress";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolEventShopRewardTrackProgress>(value)?)
}

pub async fn get_lol_event_shop_v1_reward_track_xp(client: RESTClient) -> Result<LolEventShopRewardTrackXP, Box<dyn Error>> {
    let url = "/lol-event-shop/v1/reward-track-xp";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolEventShopRewardTrackXP>(value)?)
}

pub async fn get_lol_event_shop_v1_token_balance(client: RESTClient) -> Result<u32, Box<dyn Error>> {
    let url = "/lol-event-shop/v1/token-balance";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<u32>(value)?)
}

pub async fn get_lol_event_shop_v1_token_shop_data(client: RESTClient) -> Result<LolEventShopTokenShopUIData, Box<dyn Error>> {
    let url = "/lol-event-shop/v1/token-shop-data";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolEventShopTokenShopUIData>(value)?)
}

pub async fn get_lol_event_shop_v1_token_upsell(client: RESTClient) -> Result<Vec<LolEventShopTokenUpsell>, Box<dyn Error>> {
    let url = "/lol-event-shop/v1/token-upsell";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolEventShopTokenUpsell>>(value)?)
}

pub async fn get_lol_event_shop_v1_unclaimed_rewards(client: RESTClient) -> Result<LolEventShopUnclaimedRewardsUIData, Box<dyn Error>> {
    let url = "/lol-event-shop/v1/unclaimed-rewards";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolEventShopUnclaimedRewardsUIData>(value)?)
}

pub async fn get_lol_game_client_chat_v1_buddies(client: RESTClient) -> Result<Vec<String>, Box<dyn Error>> {
    let url = "/lol-game-client-chat/v1/buddies";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<String>>(value)?)
}

pub async fn get_lol_game_client_chat_v1_ignored_summoners(client: RESTClient) -> Result<Vec<u64>, Box<dyn Error>> {
    let url = "/lol-game-client-chat/v1/ignored-summoners";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<u64>>(value)?)
}

pub async fn get_lol_game_client_chat_v1_muted_summoners(client: RESTClient) -> Result<Vec<u64>, Box<dyn Error>> {
    let url = "/lol-game-client-chat/v1/muted-summoners";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<u64>>(value)?)
}

pub async fn get_lol_game_queues_v1_custom(client: RESTClient) -> Result<LolGameQueuesQueueCustomGame, Box<dyn Error>> {
    let url = "/lol-game-queues/v1/custom";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolGameQueuesQueueCustomGame>(value)?)
}

pub async fn get_lol_game_queues_v1_custom_non_default(client: RESTClient) -> Result<LolGameQueuesQueueCustomGame, Box<dyn Error>> {
    let url = "/lol-game-queues/v1/custom-non-default";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolGameQueuesQueueCustomGame>(value)?)
}

pub async fn get_lol_game_queues_v1_game_type_config_by_game_type_config_id(client: RESTClient, game_type_config_id: u32) -> Result<LolGameQueuesQueueGameTypeConfig, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-game-queues/v1/game-type-config/{{game_type_config_id}}", &json!({"game_type_config_id": game_type_config_id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolGameQueuesQueueGameTypeConfig>(value)?)
}

pub async fn get_lol_game_queues_v1_game_type_config_by_game_type_config_id_map_by_map_id(client: RESTClient, game_type_config_id: u32, map_id: i32) -> Result<LolGameQueuesQueueGameTypeConfig, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-game-queues/v1/game-type-config/{gameTypeConfigId}/map/{{map_id}}", &json!({"game_type_config_id": game_type_config_id, "map_id": map_id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolGameQueuesQueueGameTypeConfig>(value)?)
}

pub async fn get_lol_game_queues_v1_queues(client: RESTClient) -> Result<Vec<LolGameQueuesQueue>, Box<dyn Error>> {
    let url = "/lol-game-queues/v1/queues";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolGameQueuesQueue>>(value)?)
}

pub async fn get_lol_game_queues_v1_queues_by_id(client: RESTClient, id: i32) -> Result<LolGameQueuesQueue, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-game-queues/v1/queues/{{id}}", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolGameQueuesQueue>(value)?)
}

pub async fn get_lol_game_queues_v1_queues_type_by_queue_type(client: RESTClient, queue_type: String) -> Result<LolGameQueuesQueue, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-game-queues/v1/queues/type/{{queue_type}}", &json!({"queue_type": queue_type}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolGameQueuesQueue>(value)?)
}

pub async fn get_lol_game_settings_v1_didreset(client: RESTClient) -> Result<bool, Box<dyn Error>> {
    let url = "/lol-game-settings/v1/didreset";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<bool>(value)?)
}

pub async fn get_lol_game_settings_v1_game_settings(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/lol-game-settings/v1/game-settings";
    let value = client.get(url.to_owned()).await?;
    Ok(value)
}

pub async fn get_lol_game_settings_v1_game_settings_schema(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/lol-game-settings/v1/game-settings-schema";
    let value = client.get(url.to_owned()).await?;
    Ok(value)
}

pub async fn get_lol_game_settings_v1_input_settings(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/lol-game-settings/v1/input-settings";
    let value = client.get(url.to_owned()).await?;
    Ok(value)
}

pub async fn get_lol_game_settings_v1_input_settings_schema(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/lol-game-settings/v1/input-settings-schema";
    let value = client.get(url.to_owned()).await?;
    Ok(value)
}

pub async fn get_lol_game_settings_v1_ready(client: RESTClient) -> Result<bool, Box<dyn Error>> {
    let url = "/lol-game-settings/v1/ready";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<bool>(value)?)
}

pub async fn get_lol_gameflow_v1_active_patcher_lock(client: RESTClient) -> Result<bool, Box<dyn Error>> {
    let url = "/lol-gameflow/v1/active-patcher-lock";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<bool>(value)?)
}

pub async fn get_lol_gameflow_v1_availability(client: RESTClient) -> Result<LolGameflowGameflowAvailability, Box<dyn Error>> {
    let url = "/lol-gameflow/v1/availability";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolGameflowGameflowAvailability>(value)?)
}

pub async fn get_lol_gameflow_v1_basic_tutorial(client: RESTClient) -> Result<bool, Box<dyn Error>> {
    let url = "/lol-gameflow/v1/basic-tutorial";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<bool>(value)?)
}

pub async fn get_lol_gameflow_v1_battle_training(client: RESTClient) -> Result<bool, Box<dyn Error>> {
    let url = "/lol-gameflow/v1/battle-training";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<bool>(value)?)
}

pub async fn get_lol_gameflow_v1_early_exit_notifications_eog(client: RESTClient) -> Result<Vec<Value>, Box<dyn Error>> {
    let url = "/lol-gameflow/v1/early-exit-notifications/eog";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<Value>>(value)?)
}

pub async fn get_lol_gameflow_v1_early_exit_notifications_missions(client: RESTClient) -> Result<Vec<Value>, Box<dyn Error>> {
    let url = "/lol-gameflow/v1/early-exit-notifications/missions";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<Value>>(value)?)
}

pub async fn get_lol_gameflow_v1_extra_game_client_args(client: RESTClient) -> Result<Vec<String>, Box<dyn Error>> {
    let url = "/lol-gameflow/v1/extra-game-client-args";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<String>>(value)?)
}

pub async fn get_lol_gameflow_v1_gameflow_metadata_player_status(client: RESTClient) -> Result<LolGameflowPlayerStatus, Box<dyn Error>> {
    let url = "/lol-gameflow/v1/gameflow-metadata/player-status";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolGameflowPlayerStatus>(value)?)
}

pub async fn get_lol_gameflow_v1_gameflow_metadata_registration_status(client: RESTClient) -> Result<LolGameflowRegistrationStatus, Box<dyn Error>> {
    let url = "/lol-gameflow/v1/gameflow-metadata/registration-status";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolGameflowRegistrationStatus>(value)?)
}

pub async fn get_lol_gameflow_v1_gameflow_phase(client: RESTClient) -> Result<LolGameflowGameflowPhase, Box<dyn Error>> {
    let url = "/lol-gameflow/v1/gameflow-phase";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolGameflowGameflowPhase>(value)?)
}

pub async fn get_lol_gameflow_v1_session(client: RESTClient) -> Result<LolGameflowGameflowSession, Box<dyn Error>> {
    let url = "/lol-gameflow/v1/session";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolGameflowGameflowSession>(value)?)
}

pub async fn get_lol_gameflow_v1_session_per_position_summoner_spells_disallowed(client: RESTClient) -> Result<HashMap<String, LolGameflowGameModeSpellList>, Box<dyn Error>> {
    let url = "/lol-gameflow/v1/session/per-position-summoner-spells/disallowed";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<HashMap<String, LolGameflowGameModeSpellList>>(value)?)
}

pub async fn get_lol_gameflow_v1_session_per_position_summoner_spells_disallowed_as_string(client: RESTClient) -> Result<String, Box<dyn Error>> {
    let url = "/lol-gameflow/v1/session/per-position-summoner-spells/disallowed/as-string";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<String>(value)?)
}

pub async fn get_lol_gameflow_v1_session_per_position_summoner_spells_required(client: RESTClient) -> Result<HashMap<String, LolGameflowGameModeSpellList>, Box<dyn Error>> {
    let url = "/lol-gameflow/v1/session/per-position-summoner-spells/required";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<HashMap<String, LolGameflowGameModeSpellList>>(value)?)
}

pub async fn get_lol_gameflow_v1_session_per_position_summoner_spells_required_as_string(client: RESTClient) -> Result<String, Box<dyn Error>> {
    let url = "/lol-gameflow/v1/session/per-position-summoner-spells/required/as-string";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<String>(value)?)
}

pub async fn get_lol_gameflow_v1_spectate(client: RESTClient) -> Result<bool, Box<dyn Error>> {
    let url = "/lol-gameflow/v1/spectate";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<bool>(value)?)
}

pub async fn get_lol_gameflow_v1_spectate_delayed_launch(client: RESTClient) -> Result<(), Box<dyn Error>> {
    let url = "/lol-gameflow/v1/spectate/delayed-launch";
    client.get(url.to_owned()).await?;
    Ok(())
}

pub async fn get_lol_gameflow_v1_watch(client: RESTClient) -> Result<LolGameflowGameflowWatchPhase, Box<dyn Error>> {
    let url = "/lol-gameflow/v1/watch";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolGameflowGameflowWatchPhase>(value)?)
}

pub async fn get_lol_geoinfo_v1_getlocation(client: RESTClient, ip_address: String) -> Result<LolGeoinfoGeoInfo, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-geoinfo/v1/getlocation?ip_address={{ip_address}}", &json!({"ip_address": ip_address}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolGeoinfoGeoInfo>(value)?)
}

pub async fn get_lol_geoinfo_v1_whereami(client: RESTClient) -> Result<LolGeoinfoGeoInfoResponse, Box<dyn Error>> {
    let url = "/lol-geoinfo/v1/whereami";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolGeoinfoGeoInfoResponse>(value)?)
}

pub async fn get_lol_highlights_v1_config(client: RESTClient) -> Result<LolHighlightsHighlightsConfig, Box<dyn Error>> {
    let url = "/lol-highlights/v1/config";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolHighlightsHighlightsConfig>(value)?)
}

pub async fn get_lol_highlights_v1_highlights(client: RESTClient) -> Result<Vec<LolHighlightsHighlight>, Box<dyn Error>> {
    let url = "/lol-highlights/v1/highlights";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolHighlightsHighlight>>(value)?)
}

pub async fn get_lol_highlights_v1_highlights_by_id(client: RESTClient, id: u64) -> Result<LolHighlightsHighlight, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-highlights/v1/highlights/{{id}}", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolHighlightsHighlight>(value)?)
}

pub async fn get_lol_highlights_v1_highlights_folder_path(client: RESTClient) -> Result<String, Box<dyn Error>> {
    let url = "/lol-highlights/v1/highlights-folder-path";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<String>(value)?)
}

pub async fn get_lol_highlights_v1_highlights_folder_path_default(client: RESTClient) -> Result<String, Box<dyn Error>> {
    let url = "/lol-highlights/v1/highlights-folder-path/default";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<String>(value)?)
}

pub async fn get_lol_honeyfruit_v1_linking_status(client: RESTClient) -> Result<LolHoneyfruitHoneyfruitLinkingStatus, Box<dyn Error>> {
    let url = "/lol-honeyfruit/v1/linking-status";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolHoneyfruitHoneyfruitLinkingStatus>(value)?)
}

pub async fn get_lol_honeyfruit_v1_vng_publisher_settings(client: RESTClient) -> Result<LolHoneyfruitHoneyfruitVNGPublisherSettings, Box<dyn Error>> {
    let url = "/lol-honeyfruit/v1/vng-publisher-settings";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolHoneyfruitHoneyfruitVNGPublisherSettings>(value)?)
}

pub async fn get_lol_honor_v2_v1_ballot(client: RESTClient) -> Result<LolHonorV2Ballot, Box<dyn Error>> {
    let url = "/lol-honor-v2/v1/ballot";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolHonorV2Ballot>(value)?)
}

pub async fn get_lol_honor_v2_v1_config(client: RESTClient) -> Result<LolHonorV2HonorConfig, Box<dyn Error>> {
    let url = "/lol-honor-v2/v1/config";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolHonorV2HonorConfig>(value)?)
}

pub async fn get_lol_honor_v2_v1_late_recognition(client: RESTClient) -> Result<Vec<LolHonorV2Honor>, Box<dyn Error>> {
    let url = "/lol-honor-v2/v1/late-recognition";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolHonorV2Honor>>(value)?)
}

pub async fn get_lol_honor_v2_v1_latest_eligible_game(client: RESTClient) -> Result<u64, Box<dyn Error>> {
    let url = "/lol-honor-v2/v1/latest-eligible-game";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<u64>(value)?)
}

pub async fn get_lol_honor_v2_v1_level_change(client: RESTClient) -> Result<LolHonorV2VendedHonorChange, Box<dyn Error>> {
    let url = "/lol-honor-v2/v1/level-change";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolHonorV2VendedHonorChange>(value)?)
}

pub async fn get_lol_honor_v2_v1_mutual_honor(client: RESTClient) -> Result<LolHonorV2MutualHonor, Box<dyn Error>> {
    let url = "/lol-honor-v2/v1/mutual-honor";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolHonorV2MutualHonor>(value)?)
}

pub async fn get_lol_honor_v2_v1_profile(client: RESTClient) -> Result<LolHonorV2ProfileInfo, Box<dyn Error>> {
    let url = "/lol-honor-v2/v1/profile";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolHonorV2ProfileInfo>(value)?)
}

pub async fn get_lol_honor_v2_v1_recognition(client: RESTClient) -> Result<Vec<LolHonorV2Honor>, Box<dyn Error>> {
    let url = "/lol-honor-v2/v1/recognition";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolHonorV2Honor>>(value)?)
}

pub async fn get_lol_honor_v2_v1_recognition_history(client: RESTClient) -> Result<Vec<LolHonorV2HonorInteraction>, Box<dyn Error>> {
    let url = "/lol-honor-v2/v1/recognition-history";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolHonorV2HonorInteraction>>(value)?)
}

pub async fn get_lol_honor_v2_v1_reward_granted(client: RESTClient) -> Result<LolHonorV2VendedReward, Box<dyn Error>> {
    let url = "/lol-honor-v2/v1/reward-granted";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolHonorV2VendedReward>(value)?)
}

pub async fn get_lol_honor_v2_v1_team_choices(client: RESTClient) -> Result<Vec<u64>, Box<dyn Error>> {
    let url = "/lol-honor-v2/v1/team-choices";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<u64>>(value)?)
}

pub async fn get_lol_honor_v2_v1_vote_completion(client: RESTClient) -> Result<LolHonorV2VoteCompletion, Box<dyn Error>> {
    let url = "/lol-honor-v2/v1/vote-completion";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolHonorV2VoteCompletion>(value)?)
}

pub async fn get_lol_hovercard_v1_friend_info_by_puuid(client: RESTClient, puuid: String) -> Result<LolHovercardHovercardUserInfo, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-hovercard/v1/friend-info/{{puuid}}", &json!({"puuid": puuid}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolHovercardHovercardUserInfo>(value)?)
}

pub async fn get_lol_hovercard_v1_friend_info_by_summoner_by_summoner_id(client: RESTClient, summoner_id: u64) -> Result<LolHovercardHovercardUserInfo, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-hovercard/v1/friend-info-by-summoner/{{summoner_id}}", &json!({"summoner_id": summoner_id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolHovercardHovercardUserInfo>(value)?)
}

pub async fn get_lol_inventory_v1_champ_select_inventory(client: RESTClient) -> Result<String, Box<dyn Error>> {
    let url = "/lol-inventory/v1/champSelectInventory";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<String>(value)?)
}

pub async fn get_lol_inventory_v1_initial_configuration_complete(client: RESTClient) -> Result<bool, Box<dyn Error>> {
    let url = "/lol-inventory/v1/initial-configuration-complete";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<bool>(value)?)
}

pub async fn get_lol_inventory_v1_inventory(client: RESTClient, inventory_types: Vec<String>) -> Result<Vec<LolInventoryInventoryItemWithPayload>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-inventory/v1/inventory?inventoryTypes={{inventory_types}}", &json!({"inventory_types": inventory_types}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolInventoryInventoryItemWithPayload>>(value)?)
}

pub async fn get_lol_inventory_v1_inventory_emotes(client: RESTClient) -> Result<Vec<LolInventoryInventoryItemWithPayload>, Box<dyn Error>> {
    let url = "/lol-inventory/v1/inventory/emotes";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolInventoryInventoryItemWithPayload>>(value)?)
}

pub async fn get_lol_inventory_v1_inventory_with_f2_p(client: RESTClient, inventory_types: Vec<String>) -> Result<Vec<LolInventoryInventoryItemWithPayload>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-inventory/v1/inventoryWithF2P?inventoryTypes={{inventory_types}}", &json!({"inventory_types": inventory_types}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolInventoryInventoryItemWithPayload>>(value)?)
}

pub async fn get_lol_inventory_v1_notifications_by_inventory_type(client: RESTClient, inventory_type: String) -> Result<Vec<LolInventoryInventoryNotification>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-inventory/v1/notifications/{{inventory_type}}", &json!({"inventory_type": inventory_type}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolInventoryInventoryNotification>>(value)?)
}

pub async fn get_lol_inventory_v1_players_by_puuid_inventory(client: RESTClient, puuid: String, inventory_types: Vec<String>) -> Result<Vec<LolInventoryInventoryItemWithPayload>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-inventory/v1/players/{{puuid}}/inventory?inventoryTypes={{inventory_types}}", &json!({"puuid": puuid, "inventory_types": inventory_types}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolInventoryInventoryItemWithPayload>>(value)?)
}

pub async fn get_lol_inventory_v1_signed_inventory(client: RESTClient, inventory_types: Vec<String>) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-inventory/v1/signedInventory?inventoryTypes={{inventory_types}}", &json!({"inventory_types": inventory_types}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<HashMap<String, String>>(value)?)
}

pub async fn get_lol_inventory_v1_signed_inventory_cache(client: RESTClient) -> Result<HashMap<String, LolInventoryInventoryCacheEntry>, Box<dyn Error>> {
    let url = "/lol-inventory/v1/signedInventoryCache";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<HashMap<String, LolInventoryInventoryCacheEntry>>(value)?)
}

pub async fn get_lol_inventory_v1_signed_inventory_simple(client: RESTClient, inventory_types: Vec<String>, query_params: Option<HashMap<String, String>>) -> Result<String, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-inventory/v1/signedInventory/simple?inventoryTypes={{inventory_types}}&queryParams={{query_params}}", &json!({"inventory_types": inventory_types, "query_params": query_params}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<String>(value)?)
}

pub async fn get_lol_inventory_v1_signed_inventory_tournamentlogos(client: RESTClient) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let url = "/lol-inventory/v1/signedInventory/tournamentlogos";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<HashMap<String, String>>(value)?)
}

pub async fn get_lol_inventory_v1_signed_wallet(client: RESTClient, currency_types: Vec<String>) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-inventory/v1/signedWallet?currencyTypes={{currency_types}}", &json!({"currency_types": currency_types}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<HashMap<String, String>>(value)?)
}

pub async fn get_lol_inventory_v1_signed_wallet_by_currency_type(client: RESTClient, currency_type: String) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-inventory/v1/signedWallet/{{currency_type}}", &json!({"currency_type": currency_type}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<HashMap<String, String>>(value)?)
}

pub async fn get_lol_inventory_v1_wallet(client: RESTClient, currency_types: Vec<String>) -> Result<HashMap<String, i32>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-inventory/v1/wallet?currencyTypes={{currency_types}}", &json!({"currency_types": currency_types}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<HashMap<String, i32>>(value)?)
}

pub async fn get_lol_inventory_v1_wallet_by_currency_type(client: RESTClient, currency_type: String) -> Result<HashMap<String, i32>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-inventory/v1/wallet/{{currency_type}}", &json!({"currency_type": currency_type}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<HashMap<String, i32>>(value)?)
}

pub async fn get_lol_inventory_v1_xbox_subscription_status(client: RESTClient) -> Result<LolInventoryXboxSubscriptionStatus, Box<dyn Error>> {
    let url = "/lol-inventory/v1/xbox-subscription-status";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolInventoryXboxSubscriptionStatus>(value)?)
}

pub async fn get_lol_inventory_v2_inventory_by_inventory_type(client: RESTClient, inventory_type: String) -> Result<Vec<LolInventoryInventoryItemWithPayload>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-inventory/v2/inventory/{{inventory_type}}", &json!({"inventory_type": inventory_type}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolInventoryInventoryItemWithPayload>>(value)?)
}

pub async fn get_lol_item_sets_v1_item_sets_by_summoner_id_sets(client: RESTClient, summoner_id: u64) -> Result<LolItemSetsItemSets, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-item-sets/v1/item-sets/{{summoner_id}}/sets", &json!({"summoner_id": summoner_id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolItemSetsItemSets>(value)?)
}

pub async fn get_lol_kickout_v1_notification(client: RESTClient) -> Result<LolKickoutKickoutMessage, Box<dyn Error>> {
    let url = "/lol-kickout/v1/notification";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolKickoutKickoutMessage>(value)?)
}

pub async fn get_lol_kr_playtime_reminder_v1_message(client: RESTClient) -> Result<String, Box<dyn Error>> {
    let url = "/lol-kr-playtime-reminder/v1/message";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<String>(value)?)
}

pub async fn get_lol_kr_playtime_reminder_v1_playtime(client: RESTClient) -> Result<LolKrPlaytimeReminderPlaytimeReminder, Box<dyn Error>> {
    let url = "/lol-kr-playtime-reminder/v1/playtime";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolKrPlaytimeReminderPlaytimeReminder>(value)?)
}

pub async fn get_lol_kr_shutdown_law_v1_custom_status(client: RESTClient) -> Result<LolKrShutdownLawQueueShutdownStatus, Box<dyn Error>> {
    let url = "/lol-kr-shutdown-law/v1/custom-status";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolKrShutdownLawQueueShutdownStatus>(value)?)
}

pub async fn get_lol_kr_shutdown_law_v1_disabled_queues(client: RESTClient) -> Result<Vec<i32>, Box<dyn Error>> {
    let url = "/lol-kr-shutdown-law/v1/disabled-queues";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<i32>>(value)?)
}

pub async fn get_lol_kr_shutdown_law_v1_notification(client: RESTClient) -> Result<LolKrShutdownLawShutdownLawNotification, Box<dyn Error>> {
    let url = "/lol-kr-shutdown-law/v1/notification";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolKrShutdownLawShutdownLawNotification>(value)?)
}

pub async fn get_lol_kr_shutdown_law_v1_queue_status_by_queue_id(client: RESTClient, queue_id: i32) -> Result<LolKrShutdownLawQueueShutdownStatus, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-kr-shutdown-law/v1/queue-status/{{queue_id}}", &json!({"queue_id": queue_id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolKrShutdownLawQueueShutdownStatus>(value)?)
}

pub async fn get_lol_kr_shutdown_law_v1_rating_screen(client: RESTClient) -> Result<LolKrShutdownLawRatingScreenInfo, Box<dyn Error>> {
    let url = "/lol-kr-shutdown-law/v1/rating-screen";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolKrShutdownLawRatingScreenInfo>(value)?)
}

pub async fn get_lol_kr_shutdown_law_v1_status(client: RESTClient) -> Result<LolKrShutdownLawAllQueueShutdownStatus, Box<dyn Error>> {
    let url = "/lol-kr-shutdown-law/v1/status";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolKrShutdownLawAllQueueShutdownStatus>(value)?)
}

pub async fn get_lol_league_session_v1_league_session_token(client: RESTClient) -> Result<String, Box<dyn Error>> {
    let url = "/lol-league-session/v1/league-session-token";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<String>(value)?)
}

pub async fn get_lol_leaver_buster_v1_notifications(client: RESTClient) -> Result<Vec<LolLeaverBusterLeaverBusterNotificationResource>, Box<dyn Error>> {
    let url = "/lol-leaver-buster/v1/notifications";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolLeaverBusterLeaverBusterNotificationResource>>(value)?)
}

pub async fn get_lol_leaver_buster_v1_notifications_by_id(client: RESTClient, id: u32) -> Result<LolLeaverBusterLeaverBusterNotificationResource, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-leaver-buster/v1/notifications/{{id}}", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolLeaverBusterLeaverBusterNotificationResource>(value)?)
}

pub async fn get_lol_license_agreement_v1_agreements(client: RESTClient) -> Result<Vec<LolLicenseAgreementLicenseAgreement>, Box<dyn Error>> {
    let url = "/lol-license-agreement/v1/agreements";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolLicenseAgreementLicenseAgreement>>(value)?)
}

pub async fn get_lol_license_agreement_v1_all_agreements(client: RESTClient) -> Result<Vec<LolLicenseAgreementLicenseAgreement>, Box<dyn Error>> {
    let url = "/lol-license-agreement/v1/all-agreements";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolLicenseAgreementLicenseAgreement>>(value)?)
}

pub async fn get_lol_license_agreement_v1_serve_location(client: RESTClient) -> Result<LolLicenseAgreementLicenseServeLocation, Box<dyn Error>> {
    let url = "/lol-license-agreement/v1/serve-location";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolLicenseAgreementLicenseServeLocation>(value)?)
}

pub async fn get_lol_loadouts_v1_loadouts_ready(client: RESTClient) -> Result<bool, Box<dyn Error>> {
    let url = "/lol-loadouts/v1/loadouts-ready";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<bool>(value)?)
}

pub async fn get_lol_loadouts_v4_loadouts_by_loadout_id(client: RESTClient, loadout_id: String) -> Result<LolLoadoutsScopedLoadout, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-loadouts/v4/loadouts/{{loadout_id}}", &json!({"loadout_id": loadout_id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolLoadoutsScopedLoadout>(value)?)
}

pub async fn get_lol_loadouts_v4_loadouts_scope_account(client: RESTClient) -> Result<Vec<LolLoadoutsScopedLoadout>, Box<dyn Error>> {
    let url = "/lol-loadouts/v4/loadouts/scope/account";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolLoadoutsScopedLoadout>>(value)?)
}

pub async fn get_lol_loadouts_v4_loadouts_scope_by_scope_by_scope_item_id(client: RESTClient, scope: String, scope_item_id: Option<u32>) -> Result<Vec<LolLoadoutsScopedLoadout>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-loadouts/v4/loadouts/scope/{scope}/{{scope_item_id}}", &json!({"scope": scope, "scope_item_id": scope_item_id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolLoadoutsScopedLoadout>>(value)?)
}

pub async fn get_lol_lobby_team_builder_champ_select_v1_bannable_champion_ids(client: RESTClient) -> Result<Vec<i32>, Box<dyn Error>> {
    let url = "/lol-lobby-team-builder/champ-select/v1/bannable-champion-ids";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<i32>>(value)?)
}

pub async fn get_lol_lobby_team_builder_champ_select_v1_current_champion(client: RESTClient) -> Result<i32, Box<dyn Error>> {
    let url = "/lol-lobby-team-builder/champ-select/v1/current-champion";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<i32>(value)?)
}

pub async fn get_lol_lobby_team_builder_champ_select_v1_disabled_champion_ids(client: RESTClient) -> Result<Vec<i32>, Box<dyn Error>> {
    let url = "/lol-lobby-team-builder/champ-select/v1/disabled-champion-ids";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<i32>>(value)?)
}

pub async fn get_lol_lobby_team_builder_champ_select_v1_has_auto_assigned_smite(client: RESTClient) -> Result<bool, Box<dyn Error>> {
    let url = "/lol-lobby-team-builder/champ-select/v1/has-auto-assigned-smite";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<bool>(value)?)
}

pub async fn get_lol_lobby_team_builder_champ_select_v1_implementation_active(client: RESTClient) -> Result<bool, Box<dyn Error>> {
    let url = "/lol-lobby-team-builder/champ-select/v1/implementation-active";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<bool>(value)?)
}

pub async fn get_lol_lobby_team_builder_champ_select_v1_match_token(client: RESTClient) -> Result<String, Box<dyn Error>> {
    let url = "/lol-lobby-team-builder/champ-select/v1/match-token";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<String>(value)?)
}

pub async fn get_lol_lobby_team_builder_champ_select_v1_pickable_champion_ids(client: RESTClient) -> Result<Vec<i32>, Box<dyn Error>> {
    let url = "/lol-lobby-team-builder/champ-select/v1/pickable-champion-ids";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<i32>>(value)?)
}

pub async fn get_lol_lobby_team_builder_champ_select_v1_pickable_skin_ids(client: RESTClient) -> Result<Vec<i32>, Box<dyn Error>> {
    let url = "/lol-lobby-team-builder/champ-select/v1/pickable-skin-ids";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<i32>>(value)?)
}

pub async fn get_lol_lobby_team_builder_champ_select_v1_preferences(client: RESTClient) -> Result<LolLobbyTeamBuilderChampionSelectPreferences, Box<dyn Error>> {
    let url = "/lol-lobby-team-builder/champ-select/v1/preferences";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolLobbyTeamBuilderChampionSelectPreferences>(value)?)
}

pub async fn get_lol_lobby_team_builder_champ_select_v1_sending_loadouts_gcos_enabled(client: RESTClient) -> Result<bool, Box<dyn Error>> {
    let url = "/lol-lobby-team-builder/champ-select/v1/sending-loadouts-gcos-enabled";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<bool>(value)?)
}

pub async fn get_lol_lobby_team_builder_champ_select_v1_session(client: RESTClient) -> Result<LolLobbyTeamBuilderChampSelectSession, Box<dyn Error>> {
    let url = "/lol-lobby-team-builder/champ-select/v1/session";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolLobbyTeamBuilderChampSelectSession>(value)?)
}

pub async fn get_lol_lobby_team_builder_champ_select_v1_session_my_selection(client: RESTClient) -> Result<LolLobbyTeamBuilderChampSelectPlayerSelection, Box<dyn Error>> {
    let url = "/lol-lobby-team-builder/champ-select/v1/session/my-selection";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolLobbyTeamBuilderChampSelectPlayerSelection>(value)?)
}

pub async fn get_lol_lobby_team_builder_champ_select_v1_session_obfuscated_summoner_ids(client: RESTClient) -> Result<Vec<u64>, Box<dyn Error>> {
    let url = "/lol-lobby-team-builder/champ-select/v1/session/obfuscated-summoner-ids";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<u64>>(value)?)
}

pub async fn get_lol_lobby_team_builder_champ_select_v1_session_swaps(client: RESTClient) -> Result<Vec<LolLobbyTeamBuilderChampSelectSwapContract>, Box<dyn Error>> {
    let url = "/lol-lobby-team-builder/champ-select/v1/session/swaps";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolLobbyTeamBuilderChampSelectSwapContract>>(value)?)
}

pub async fn get_lol_lobby_team_builder_champ_select_v1_session_swaps_by_id(client: RESTClient, id: i64) -> Result<LolLobbyTeamBuilderChampSelectSwapContract, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-lobby-team-builder/champ-select/v1/session/swaps/{{id}}", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolLobbyTeamBuilderChampSelectSwapContract>(value)?)
}

pub async fn get_lol_lobby_team_builder_champ_select_v1_session_timer(client: RESTClient) -> Result<LolLobbyTeamBuilderChampSelectTimer, Box<dyn Error>> {
    let url = "/lol-lobby-team-builder/champ-select/v1/session/timer";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolLobbyTeamBuilderChampSelectTimer>(value)?)
}

pub async fn get_lol_lobby_team_builder_champ_select_v1_session_trades(client: RESTClient) -> Result<Vec<LolLobbyTeamBuilderChampSelectTradeContract>, Box<dyn Error>> {
    let url = "/lol-lobby-team-builder/champ-select/v1/session/trades";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolLobbyTeamBuilderChampSelectTradeContract>>(value)?)
}

pub async fn get_lol_lobby_team_builder_champ_select_v1_session_trades_by_id(client: RESTClient, id: i64) -> Result<LolLobbyTeamBuilderChampSelectTradeContract, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-lobby-team-builder/champ-select/v1/session/trades/{{id}}", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolLobbyTeamBuilderChampSelectTradeContract>(value)?)
}

pub async fn get_lol_lobby_team_builder_champ_select_v1_team_boost(client: RESTClient) -> Result<LolLobbyTeamBuilderTeamBoost, Box<dyn Error>> {
    let url = "/lol-lobby-team-builder/champ-select/v1/team-boost";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolLobbyTeamBuilderTeamBoost>(value)?)
}

pub async fn get_lol_lobby_team_builder_v1_matchmaking(client: RESTClient) -> Result<LolLobbyTeamBuilderMatchmakingSearchResource, Box<dyn Error>> {
    let url = "/lol-lobby-team-builder/v1/matchmaking";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolLobbyTeamBuilderMatchmakingSearchResource>(value)?)
}

pub async fn get_lol_lobby_v1_autofill_displayed(client: RESTClient) -> Result<bool, Box<dyn Error>> {
    let url = "/lol-lobby/v1/autofill-displayed";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<bool>(value)?)
}

pub async fn get_lol_lobby_v1_custom_games(client: RESTClient) -> Result<Vec<LolLobbyLobbyCustomGame>, Box<dyn Error>> {
    let url = "/lol-lobby/v1/custom-games";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolLobbyLobbyCustomGame>>(value)?)
}

pub async fn get_lol_lobby_v1_custom_games_by_id(client: RESTClient, id: i32) -> Result<LolLobbyLobbyCustomGame, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-lobby/v1/custom-games/{{id}}", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolLobbyLobbyCustomGame>(value)?)
}

pub async fn get_lol_lobby_v1_lobby_availability(client: RESTClient) -> Result<LolLobbyQueueAvailability, Box<dyn Error>> {
    let url = "/lol-lobby/v1/lobby/availability";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolLobbyQueueAvailability>(value)?)
}

pub async fn get_lol_lobby_v1_lobby_countdown(client: RESTClient) -> Result<i64, Box<dyn Error>> {
    let url = "/lol-lobby/v1/lobby/countdown";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<i64>(value)?)
}

pub async fn get_lol_lobby_v1_lobby_invitations(client: RESTClient) -> Result<Vec<LolLobbyLobbyInvitation>, Box<dyn Error>> {
    let url = "/lol-lobby/v1/lobby/invitations";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolLobbyLobbyInvitation>>(value)?)
}

pub async fn get_lol_lobby_v1_lobby_invitations_by_id(client: RESTClient, id: String) -> Result<LolLobbyLobbyInvitation, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-lobby/v1/lobby/invitations/{{id}}", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolLobbyLobbyInvitation>(value)?)
}

pub async fn get_lol_lobby_v1_parties_gamemode(client: RESTClient) -> Result<LolLobbyGameModeDto, Box<dyn Error>> {
    let url = "/lol-lobby/v1/parties/gamemode";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolLobbyGameModeDto>(value)?)
}

pub async fn get_lol_lobby_v1_parties_player(client: RESTClient) -> Result<LolLobbyPlayerDto, Box<dyn Error>> {
    let url = "/lol-lobby/v1/parties/player";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolLobbyPlayerDto>(value)?)
}

pub async fn get_lol_lobby_v1_party_rewards(client: RESTClient) -> Result<LolLobbyLobbyPartyRewards, Box<dyn Error>> {
    let url = "/lol-lobby/v1/party-rewards";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolLobbyLobbyPartyRewards>(value)?)
}

pub async fn get_lol_lobby_v2_comms_members(client: RESTClient) -> Result<LolLobbyPremadePartyDto, Box<dyn Error>> {
    let url = "/lol-lobby/v2/comms/members";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolLobbyPremadePartyDto>(value)?)
}

pub async fn get_lol_lobby_v2_comms_token(client: RESTClient) -> Result<String, Box<dyn Error>> {
    let url = "/lol-lobby/v2/comms/token";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<String>(value)?)
}

pub async fn get_lol_lobby_v2_eligibility_game_select_eligibility_hash(client: RESTClient) -> Result<i64, Box<dyn Error>> {
    let url = "/lol-lobby/v2/eligibility/game-select-eligibility-hash";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<i64>(value)?)
}

pub async fn get_lol_lobby_v2_eligibility_initial_configuration_complete(client: RESTClient) -> Result<bool, Box<dyn Error>> {
    let url = "/lol-lobby/v2/eligibility/initial-configuration-complete";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<bool>(value)?)
}

pub async fn get_lol_lobby_v2_lobby(client: RESTClient) -> Result<LolLobbyLobbyDto, Box<dyn Error>> {
    let url = "/lol-lobby/v2/lobby";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolLobbyLobbyDto>(value)?)
}

pub async fn get_lol_lobby_v2_lobby_custom_available_bots(client: RESTClient) -> Result<Vec<LolLobbyLobbyBotChampion>, Box<dyn Error>> {
    let url = "/lol-lobby/v2/lobby/custom/available-bots";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolLobbyLobbyBotChampion>>(value)?)
}

pub async fn get_lol_lobby_v2_lobby_custom_bots_enabled(client: RESTClient) -> Result<bool, Box<dyn Error>> {
    let url = "/lol-lobby/v2/lobby/custom/bots-enabled";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<bool>(value)?)
}

pub async fn get_lol_lobby_v2_lobby_invitations(client: RESTClient) -> Result<Vec<LolLobbyLobbyInvitationDto>, Box<dyn Error>> {
    let url = "/lol-lobby/v2/lobby/invitations";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolLobbyLobbyInvitationDto>>(value)?)
}

pub async fn get_lol_lobby_v2_lobby_matchmaking_search_state(client: RESTClient) -> Result<LolLobbyLobbyMatchmakingSearchResource, Box<dyn Error>> {
    let url = "/lol-lobby/v2/lobby/matchmaking/search-state";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolLobbyLobbyMatchmakingSearchResource>(value)?)
}

pub async fn get_lol_lobby_v2_lobby_members(client: RESTClient) -> Result<Vec<LolLobbyLobbyParticipantDto>, Box<dyn Error>> {
    let url = "/lol-lobby/v2/lobby/members";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolLobbyLobbyParticipantDto>>(value)?)
}

pub async fn get_lol_lobby_v2_notifications(client: RESTClient) -> Result<Vec<LolLobbyLobbyNotification>, Box<dyn Error>> {
    let url = "/lol-lobby/v2/notifications";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolLobbyLobbyNotification>>(value)?)
}

pub async fn get_lol_lobby_v2_party_active(client: RESTClient) -> Result<bool, Box<dyn Error>> {
    let url = "/lol-lobby/v2/party-active";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<bool>(value)?)
}

pub async fn get_lol_lobby_v2_party_eog_status(client: RESTClient) -> Result<LolLobbyPartyStatusDto, Box<dyn Error>> {
    let url = "/lol-lobby/v2/party/eog-status";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolLobbyPartyStatusDto>(value)?)
}

pub async fn get_lol_lobby_v2_received_invitations(client: RESTClient) -> Result<Vec<LolLobbyReceivedInvitationDto>, Box<dyn Error>> {
    let url = "/lol-lobby/v2/received-invitations";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolLobbyReceivedInvitationDto>>(value)?)
}

pub async fn get_lol_lobby_v2_registration_status(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/lol-lobby/v2/registration-status";
    let value = client.get(url.to_owned()).await?;
    Ok(value)
}

pub async fn get_lol_login_v1_account_state(client: RESTClient) -> Result<LolLoginAccountStateResource, Box<dyn Error>> {
    let url = "/lol-login/v1/account-state";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolLoginAccountStateResource>(value)?)
}

pub async fn get_lol_login_v1_login_connection_state(client: RESTClient) -> Result<LolLoginLoginConnectionState, Box<dyn Error>> {
    let url = "/lol-login/v1/login-connection-state";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolLoginLoginConnectionState>(value)?)
}

pub async fn get_lol_login_v1_login_data_packet(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/lol-login/v1/login-data-packet";
    let value = client.get(url.to_owned()).await?;
    Ok(value)
}

pub async fn get_lol_login_v1_login_in_game_creds(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/lol-login/v1/login-in-game-creds";
    let value = client.get(url.to_owned()).await?;
    Ok(value)
}

pub async fn get_lol_login_v1_login_platform_credentials(client: RESTClient) -> Result<LolLoginPlatformGeneratedCredentials, Box<dyn Error>> {
    let url = "/lol-login/v1/login-platform-credentials";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolLoginPlatformGeneratedCredentials>(value)?)
}

pub async fn get_lol_login_v1_login_queue_state(client: RESTClient) -> Result<LolLoginLoginQueue, Box<dyn Error>> {
    let url = "/lol-login/v1/login-queue-state";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolLoginLoginQueue>(value)?)
}

pub async fn get_lol_login_v1_session(client: RESTClient) -> Result<LolLoginLoginSession, Box<dyn Error>> {
    let url = "/lol-login/v1/session";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolLoginLoginSession>(value)?)
}

pub async fn get_lol_login_v1_wallet(client: RESTClient) -> Result<LolLoginLoginSessionWallet, Box<dyn Error>> {
    let url = "/lol-login/v1/wallet";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolLoginLoginSessionWallet>(value)?)
}

pub async fn get_lol_login_v2_league_session_init_token(client: RESTClient) -> Result<LolLoginLeagueSessionTokenEnvelope, Box<dyn Error>> {
    let url = "/lol-login/v2/league-session-init-token";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolLoginLeagueSessionTokenEnvelope>(value)?)
}

pub async fn get_lol_loot_v1_currency_configuration(client: RESTClient) -> Result<(), Box<dyn Error>> {
    let url = "/lol-loot/v1/currency-configuration";
    client.get(url.to_owned()).await?;
    Ok(())
}

pub async fn get_lol_loot_v1_enabled(client: RESTClient) -> Result<bool, Box<dyn Error>> {
    let url = "/lol-loot/v1/enabled";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<bool>(value)?)
}

pub async fn get_lol_loot_v1_loot_grants(client: RESTClient) -> Result<Vec<LolLootLootGrantNotification>, Box<dyn Error>> {
    let url = "/lol-loot/v1/loot-grants";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolLootLootGrantNotification>>(value)?)
}

pub async fn get_lol_loot_v1_loot_items(client: RESTClient) -> Result<Vec<LolLootLootItem>, Box<dyn Error>> {
    let url = "/lol-loot/v1/loot-items";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolLootLootItem>>(value)?)
}

pub async fn get_lol_loot_v1_loot_odds_by_recipe_name(client: RESTClient, recipe_name: String) -> Result<LolLootVerboseLootOddsResponse, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-loot/v1/loot-odds/{{recipe_name}}", &json!({"recipe_name": recipe_name}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolLootVerboseLootOddsResponse>(value)?)
}

pub async fn get_lol_loot_v1_milestones(client: RESTClient, minimize_response: bool) -> Result<Vec<LolLootLootMilestones>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-loot/v1/milestones?minimizeResponse={{minimize_response}}", &json!({"minimize_response": minimize_response}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolLootLootMilestones>>(value)?)
}

pub async fn get_lol_loot_v1_milestones_by_loot_milestones_id(client: RESTClient, loot_milestones_id: String) -> Result<LolLootLootMilestones, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-loot/v1/milestones/{{loot_milestones_id}}", &json!({"loot_milestones_id": loot_milestones_id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolLootLootMilestones>(value)?)
}

pub async fn get_lol_loot_v1_milestones_by_loot_milestones_id_claim_progress(client: RESTClient, loot_milestones_id: String) -> Result<LolLootLootMilestonesClaimResponse, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-loot/v1/milestones/{{loot_milestones_id}}/claimProgress", &json!({"loot_milestones_id": loot_milestones_id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolLootLootMilestonesClaimResponse>(value)?)
}

pub async fn get_lol_loot_v1_milestones_by_loot_milestones_id_counter(client: RESTClient, loot_milestones_id: String) -> Result<LolLootLootMilestonesCounter, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-loot/v1/milestones/{{loot_milestones_id}}/counter", &json!({"loot_milestones_id": loot_milestones_id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolLootLootMilestonesCounter>(value)?)
}

pub async fn get_lol_loot_v1_milestones_counters(client: RESTClient) -> Result<Vec<LolLootLootMilestonesCounter>, Box<dyn Error>> {
    let url = "/lol-loot/v1/milestones/counters";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolLootLootMilestonesCounter>>(value)?)
}

pub async fn get_lol_loot_v1_milestones_items(client: RESTClient) -> Result<Vec<String>, Box<dyn Error>> {
    let url = "/lol-loot/v1/milestones/items";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<String>>(value)?)
}

pub async fn get_lol_loot_v1_new_player_check_done(client: RESTClient) -> Result<bool, Box<dyn Error>> {
    let url = "/lol-loot/v1/new-player-check-done";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<bool>(value)?)
}

pub async fn get_lol_loot_v1_player_display_categories(client: RESTClient) -> Result<Vec<String>, Box<dyn Error>> {
    let url = "/lol-loot/v1/player-display-categories";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<String>>(value)?)
}

pub async fn get_lol_loot_v1_player_loot(client: RESTClient) -> Result<Vec<LolLootPlayerLoot>, Box<dyn Error>> {
    let url = "/lol-loot/v1/player-loot";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolLootPlayerLoot>>(value)?)
}

pub async fn get_lol_loot_v1_player_loot_by_loot_id(client: RESTClient, loot_id: String) -> Result<LolLootPlayerLoot, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-loot/v1/player-loot/{{loot_id}}", &json!({"loot_id": loot_id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolLootPlayerLoot>(value)?)
}

pub async fn get_lol_loot_v1_player_loot_by_loot_id_context_menu(client: RESTClient, loot_id: String) -> Result<Vec<LolLootContextMenu>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-loot/v1/player-loot/{{loot_id}}/context-menu", &json!({"loot_id": loot_id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolLootContextMenu>>(value)?)
}

pub async fn get_lol_loot_v1_player_loot_map(client: RESTClient) -> Result<HashMap<String, LolLootPlayerLoot>, Box<dyn Error>> {
    let url = "/lol-loot/v1/player-loot-map";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<HashMap<String, LolLootPlayerLoot>>(value)?)
}

pub async fn get_lol_loot_v1_player_loot_notifications(client: RESTClient) -> Result<Vec<LolLootPlayerLootNotification>, Box<dyn Error>> {
    let url = "/lol-loot/v1/player-loot-notifications";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolLootPlayerLootNotification>>(value)?)
}

pub async fn get_lol_loot_v1_ready(client: RESTClient) -> Result<bool, Box<dyn Error>> {
    let url = "/lol-loot/v1/ready";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<bool>(value)?)
}

pub async fn get_lol_loot_v1_recipes_configuration(client: RESTClient) -> Result<(), Box<dyn Error>> {
    let url = "/lol-loot/v1/recipes/configuration";
    client.get(url.to_owned()).await?;
    Ok(())
}

pub async fn get_lol_loot_v1_recipes_initial_item_by_loot_id(client: RESTClient, loot_id: String) -> Result<Vec<LolLootRecipeWithMilestones>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-loot/v1/recipes/initial-item/{{loot_id}}", &json!({"loot_id": loot_id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolLootRecipeWithMilestones>>(value)?)
}

pub async fn get_lol_loot_v2_player_loot_map(client: RESTClient) -> Result<LolLootPlayerLootMap, Box<dyn Error>> {
    let url = "/lol-loot/v2/player-loot-map";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolLootPlayerLootMap>(value)?)
}

pub async fn get_lol_loyalty_v1_status_notification(client: RESTClient) -> Result<LolLoyaltyLoyaltyStatusNotification, Box<dyn Error>> {
    let url = "/lol-loyalty/v1/status-notification";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolLoyaltyLoyaltyStatusNotification>(value)?)
}

pub async fn get_lol_maps_v1_map_by_id(client: RESTClient, id: i64) -> Result<LolMapsMaps, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-maps/v1/map/{{id}}", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolMapsMaps>(value)?)
}

pub async fn get_lol_maps_v1_maps(client: RESTClient) -> Result<Vec<LolMapsMaps>, Box<dyn Error>> {
    let url = "/lol-maps/v1/maps";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolMapsMaps>>(value)?)
}

pub async fn get_lol_maps_v2_map_by_id_by_game_mode(client: RESTClient, id: i64, game_mode: String) -> Result<LolMapsMaps, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-maps/v2/map/{id}/{{game_mode}}", &json!({"id": id, "game_mode": game_mode}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolMapsMaps>(value)?)
}

pub async fn get_lol_maps_v2_map_by_id_by_game_mode_by_game_mutator(client: RESTClient, id: i64, game_mode: String, game_mutator: String) -> Result<LolMapsMaps, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-maps/v2/map/{id}/{gameMode}/{{game_mutator}}", &json!({"id": id, "game_mode": game_mode, "game_mutator": game_mutator}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolMapsMaps>(value)?)
}

pub async fn get_lol_maps_v2_maps(client: RESTClient) -> Result<Vec<LolMapsMaps>, Box<dyn Error>> {
    let url = "/lol-maps/v2/maps";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolMapsMaps>>(value)?)
}

pub async fn get_lol_marketing_preferences_v1_partition_by_partition_key(client: RESTClient, partition_key: String) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-marketing-preferences/v1/partition/{{partition_key}}", &json!({"partition_key": partition_key}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<HashMap<String, String>>(value)?)
}

pub async fn get_lol_marketing_preferences_v1_ready(client: RESTClient) -> Result<bool, Box<dyn Error>> {
    let url = "/lol-marketing-preferences/v1/ready";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<bool>(value)?)
}

pub async fn get_lol_match_history_v1_delta(client: RESTClient) -> Result<LolMatchHistoryMatchHistoryPlayerDelta, Box<dyn Error>> {
    let url = "/lol-match-history/v1/delta";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolMatchHistoryMatchHistoryPlayerDelta>(value)?)
}

pub async fn get_lol_match_history_v1_game_timelines_by_game_id(client: RESTClient, game_id: u64) -> Result<LolMatchHistoryMatchHistoryTimelineFrames, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-match-history/v1/game-timelines/{{game_id}}", &json!({"game_id": game_id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolMatchHistoryMatchHistoryTimelineFrames>(value)?)
}

pub async fn get_lol_match_history_v1_games_by_game_id(client: RESTClient, game_id: u64) -> Result<LolMatchHistoryMatchHistoryGame, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-match-history/v1/games/{{game_id}}", &json!({"game_id": game_id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolMatchHistoryMatchHistoryGame>(value)?)
}

pub async fn get_lol_match_history_v1_products_lol_by_puuid_matches(client: RESTClient, puuid: String, beg_index: Option<u32>, end_index: Option<u32>) -> Result<LolMatchHistoryMatchHistoryList, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-match-history/v1/products/lol/{{puuid}}/matches?begIndex={{beg_index}}&endIndex={{end_index}}", &json!({"puuid": puuid, "beg_index": beg_index, "end_index": end_index}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolMatchHistoryMatchHistoryList>(value)?)
}

pub async fn get_lol_match_history_v1_products_lol_current_summoner_matches(client: RESTClient, beg_index: Option<u32>, end_index: Option<u32>) -> Result<LolMatchHistoryMatchHistoryList, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-match-history/v1/products/lol/current-summoner/matches?begIndex={{beg_index}}&endIndex={{end_index}}", &json!({"beg_index": beg_index, "end_index": end_index}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolMatchHistoryMatchHistoryList>(value)?)
}

pub async fn get_lol_match_history_v1_products_tft_by_puuid_matches(client: RESTClient, puuid: String, begin: Option<u32>, count: Option<u32>, tag: Option<String>) -> Result<LolMatchHistoryGAMHSMatchHistoryList, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-match-history/v1/products/tft/{{puuid}}/matches?begin={{begin}}&count={{count}}&tag={{tag}}", &json!({"puuid": puuid, "begin": begin, "count": count, "tag": tag}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolMatchHistoryGAMHSMatchHistoryList>(value)?)
}

pub async fn get_lol_match_history_v1_recently_played_summoners(client: RESTClient) -> Result<Vec<LolMatchHistoryRecentlyPlayedSummoner>, Box<dyn Error>> {
    let url = "/lol-match-history/v1/recently-played-summoners";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolMatchHistoryRecentlyPlayedSummoner>>(value)?)
}

pub async fn get_lol_match_history_v1_web_url(client: RESTClient) -> Result<String, Box<dyn Error>> {
    let url = "/lol-match-history/v1/web-url";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<String>(value)?)
}

pub async fn get_lol_match_history_v3_matchlist_account_by_account_id(client: RESTClient, account_id: u64, beg_index: Option<u32>, end_index: Option<u32>) -> Result<LolMatchHistoryMatchHistoryList, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-match-history/v3/matchlist/account/{{account_id}}?begIndex={{beg_index}}&endIndex={{end_index}}", &json!({"account_id": account_id, "beg_index": beg_index, "end_index": end_index}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolMatchHistoryMatchHistoryList>(value)?)
}

pub async fn get_lol_matchmaking_v1_ready_check(client: RESTClient) -> Result<LolMatchmakingMatchmakingReadyCheckResource, Box<dyn Error>> {
    let url = "/lol-matchmaking/v1/ready-check";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolMatchmakingMatchmakingReadyCheckResource>(value)?)
}

pub async fn get_lol_matchmaking_v1_search(client: RESTClient) -> Result<LolMatchmakingMatchmakingSearchResource, Box<dyn Error>> {
    let url = "/lol-matchmaking/v1/search";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolMatchmakingMatchmakingSearchResource>(value)?)
}

pub async fn get_lol_matchmaking_v1_search_errors(client: RESTClient) -> Result<Vec<LolMatchmakingMatchmakingSearchErrorResource>, Box<dyn Error>> {
    let url = "/lol-matchmaking/v1/search/errors";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolMatchmakingMatchmakingSearchErrorResource>>(value)?)
}

pub async fn get_lol_matchmaking_v1_search_errors_by_id(client: RESTClient, id: i32) -> Result<LolMatchmakingMatchmakingSearchErrorResource, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-matchmaking/v1/search/errors/{{id}}", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolMatchmakingMatchmakingSearchErrorResource>(value)?)
}

pub async fn get_lol_missions_v1_data(client: RESTClient) -> Result<PlayerMissionEligibilityData, Box<dyn Error>> {
    let url = "/lol-missions/v1/data";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<PlayerMissionEligibilityData>(value)?)
}

pub async fn get_lol_missions_v1_missions(client: RESTClient) -> Result<Vec<PlayerMissionDTO>, Box<dyn Error>> {
    let url = "/lol-missions/v1/missions";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<PlayerMissionDTO>>(value)?)
}

pub async fn get_lol_missions_v1_series(client: RESTClient) -> Result<Vec<SeriesDTO>, Box<dyn Error>> {
    let url = "/lol-missions/v1/series";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<SeriesDTO>>(value)?)
}

pub async fn get_lol_npe_rewards_v1_challenges_progress(client: RESTClient) -> Result<LolNpeRewardsChallengesProgress, Box<dyn Error>> {
    let url = "/lol-npe-rewards/v1/challenges/progress";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolNpeRewardsChallengesProgress>(value)?)
}

pub async fn get_lol_npe_rewards_v1_level_rewards(client: RESTClient) -> Result<LolNpeRewardsRewardSeries, Box<dyn Error>> {
    let url = "/lol-npe-rewards/v1/level-rewards";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolNpeRewardsRewardSeries>(value)?)
}

pub async fn get_lol_npe_rewards_v1_level_rewards_state(client: RESTClient) -> Result<LolNpeRewardsRewardSeriesState, Box<dyn Error>> {
    let url = "/lol-npe-rewards/v1/level-rewards/state";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolNpeRewardsRewardSeriesState>(value)?)
}

pub async fn get_lol_npe_rewards_v1_login_rewards(client: RESTClient) -> Result<LolNpeRewardsRewardSeries, Box<dyn Error>> {
    let url = "/lol-npe-rewards/v1/login-rewards";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolNpeRewardsRewardSeries>(value)?)
}

pub async fn get_lol_npe_rewards_v1_login_rewards_state(client: RESTClient) -> Result<LolNpeRewardsRewardSeriesState, Box<dyn Error>> {
    let url = "/lol-npe-rewards/v1/login-rewards/state";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolNpeRewardsRewardSeriesState>(value)?)
}

pub async fn get_lol_npe_tutorial_path_v1_rewards_champ(client: RESTClient) -> Result<LolNpeTutorialPathCollectionsChampion, Box<dyn Error>> {
    let url = "/lol-npe-tutorial-path/v1/rewards/champ";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolNpeTutorialPathCollectionsChampion>(value)?)
}

pub async fn get_lol_npe_tutorial_path_v1_settings(client: RESTClient) -> Result<LolNpeTutorialPathAccountSettingsTutorial, Box<dyn Error>> {
    let url = "/lol-npe-tutorial-path/v1/settings";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolNpeTutorialPathAccountSettingsTutorial>(value)?)
}

pub async fn get_lol_npe_tutorial_path_v1_tutorials(client: RESTClient) -> Result<Vec<LolNpeTutorialPathTutorial>, Box<dyn Error>> {
    let url = "/lol-npe-tutorial-path/v1/tutorials";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolNpeTutorialPathTutorial>>(value)?)
}

pub async fn get_lol_patch_v1_checking_enabled(client: RESTClient) -> Result<bool, Box<dyn Error>> {
    let url = "/lol-patch/v1/checking-enabled";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<bool>(value)?)
}

pub async fn get_lol_patch_v1_environment(client: RESTClient) -> Result<LolPatchChunkingPatcherEnvironment, Box<dyn Error>> {
    let url = "/lol-patch/v1/environment";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolPatchChunkingPatcherEnvironment>(value)?)
}

pub async fn get_lol_patch_v1_game_version(client: RESTClient) -> Result<String, Box<dyn Error>> {
    let url = "/lol-patch/v1/game-version";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<String>(value)?)
}

pub async fn get_lol_patch_v1_notifications(client: RESTClient) -> Result<Vec<LolPatchNotification>, Box<dyn Error>> {
    let url = "/lol-patch/v1/notifications";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolPatchNotification>>(value)?)
}

pub async fn get_lol_patch_v1_products_league_of_legends_install_location(client: RESTClient) -> Result<LolPatchInstallPaths, Box<dyn Error>> {
    let url = "/lol-patch/v1/products/league_of_legends/install-location";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolPatchInstallPaths>(value)?)
}

pub async fn get_lol_patch_v1_products_league_of_legends_state(client: RESTClient) -> Result<LolPatchProductState, Box<dyn Error>> {
    let url = "/lol-patch/v1/products/league_of_legends/state";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolPatchProductState>(value)?)
}

pub async fn get_lol_patch_v1_products_league_of_legends_supported_game_releases(client: RESTClient) -> Result<LolPatchSupportedGameReleases, Box<dyn Error>> {
    let url = "/lol-patch/v1/products/league_of_legends/supported-game-releases";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolPatchSupportedGameReleases>(value)?)
}

pub async fn get_lol_patch_v1_status(client: RESTClient) -> Result<LolPatchStatus, Box<dyn Error>> {
    let url = "/lol-patch/v1/status";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolPatchStatus>(value)?)
}

pub async fn get_lol_perks_v1_currentpage(client: RESTClient) -> Result<LolPerksPerkPageResource, Box<dyn Error>> {
    let url = "/lol-perks/v1/currentpage";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolPerksPerkPageResource>(value)?)
}

pub async fn get_lol_perks_v1_inventory(client: RESTClient) -> Result<LolPerksPlayerInventory, Box<dyn Error>> {
    let url = "/lol-perks/v1/inventory";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolPerksPlayerInventory>(value)?)
}

pub async fn get_lol_perks_v1_pages(client: RESTClient) -> Result<Vec<LolPerksPerkPageResource>, Box<dyn Error>> {
    let url = "/lol-perks/v1/pages";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolPerksPerkPageResource>>(value)?)
}

pub async fn get_lol_perks_v1_pages_by_id(client: RESTClient, id: i32) -> Result<LolPerksPerkPageResource, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-perks/v1/pages/{{id}}", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolPerksPerkPageResource>(value)?)
}

pub async fn get_lol_perks_v1_perks(client: RESTClient) -> Result<Vec<LolPerksPerkUIPerk>, Box<dyn Error>> {
    let url = "/lol-perks/v1/perks";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolPerksPerkUIPerk>>(value)?)
}

pub async fn get_lol_perks_v1_perks_disabled(client: RESTClient) -> Result<Vec<i32>, Box<dyn Error>> {
    let url = "/lol-perks/v1/perks/disabled";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<i32>>(value)?)
}

pub async fn get_lol_perks_v1_perks_gameplay_updated(client: RESTClient) -> Result<Vec<i32>, Box<dyn Error>> {
    let url = "/lol-perks/v1/perks/gameplay-updated";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<i32>>(value)?)
}

pub async fn get_lol_perks_v1_recommended_pages_champion_by_champion_id_position_by_position_map_by_map_id(client: RESTClient, champion_id: i32, position: String, map_id: i32) -> Result<Vec<LolPerksPerkUIRecommendedPage>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-perks/v1/recommended-pages/champion/{championId}/position/{position}/map/{{map_id}}", &json!({"champion_id": champion_id, "position": position, "map_id": map_id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolPerksPerkUIRecommendedPage>>(value)?)
}

pub async fn get_lol_perks_v1_settings(client: RESTClient) -> Result<LolPerksUISettings, Box<dyn Error>> {
    let url = "/lol-perks/v1/settings";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolPerksUISettings>(value)?)
}

pub async fn get_lol_perks_v1_show_auto_modified_pages_notification(client: RESTClient) -> Result<bool, Box<dyn Error>> {
    let url = "/lol-perks/v1/show-auto-modified-pages-notification";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<bool>(value)?)
}

pub async fn get_lol_perks_v1_styles(client: RESTClient) -> Result<Vec<LolPerksPerkUIStyle>, Box<dyn Error>> {
    let url = "/lol-perks/v1/styles";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolPerksPerkUIStyle>>(value)?)
}

pub async fn get_lol_pft_v2_survey(client: RESTClient) -> Result<LolPftPFTSurvey, Box<dyn Error>> {
    let url = "/lol-pft/v2/survey";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolPftPFTSurvey>(value)?)
}

pub async fn get_lol_platform_config_v1_initial_configuration_complete(client: RESTClient) -> Result<bool, Box<dyn Error>> {
    let url = "/lol-platform-config/v1/initial-configuration-complete";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<bool>(value)?)
}

pub async fn get_lol_platform_config_v1_namespaces(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/lol-platform-config/v1/namespaces";
    let value = client.get(url.to_owned()).await?;
    Ok(value)
}

pub async fn get_lol_platform_config_v1_namespaces_by_ns(client: RESTClient, ns: String) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-platform-config/v1/namespaces/{{ns}}", &json!({"ns": ns}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(value)
}

pub async fn get_lol_platform_config_v1_namespaces_by_ns_by_key(client: RESTClient, ns: String, key: String) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-platform-config/v1/namespaces/{ns}/{{key}}", &json!({"ns": ns, "key": key}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(value)
}

pub async fn get_lol_player_behavior_v1_ban(client: RESTClient) -> Result<LolPlayerBehaviorBanNotification, Box<dyn Error>> {
    let url = "/lol-player-behavior/v1/ban";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolPlayerBehaviorBanNotification>(value)?)
}

pub async fn get_lol_player_behavior_v1_chat_restriction(client: RESTClient) -> Result<LolPlayerBehaviorRestrictionNotification, Box<dyn Error>> {
    let url = "/lol-player-behavior/v1/chat-restriction";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolPlayerBehaviorRestrictionNotification>(value)?)
}

pub async fn get_lol_player_behavior_v1_code_of_conduct_notification(client: RESTClient) -> Result<LolPlayerBehaviorCodeOfConductNotification, Box<dyn Error>> {
    let url = "/lol-player-behavior/v1/code-of-conduct-notification";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolPlayerBehaviorCodeOfConductNotification>(value)?)
}

pub async fn get_lol_player_behavior_v1_config(client: RESTClient) -> Result<LolPlayerBehaviorPlayerBehaviorConfig, Box<dyn Error>> {
    let url = "/lol-player-behavior/v1/config";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolPlayerBehaviorPlayerBehaviorConfig>(value)?)
}

pub async fn get_lol_player_behavior_v1_ranked_restriction(client: RESTClient) -> Result<LolPlayerBehaviorRestrictionNotification, Box<dyn Error>> {
    let url = "/lol-player-behavior/v1/ranked-restriction";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolPlayerBehaviorRestrictionNotification>(value)?)
}

pub async fn get_lol_player_behavior_v1_reform_card(client: RESTClient) -> Result<LolPlayerBehaviorReformCard, Box<dyn Error>> {
    let url = "/lol-player-behavior/v1/reform-card";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolPlayerBehaviorReformCard>(value)?)
}

pub async fn get_lol_player_behavior_v1_reporter_feedback(client: RESTClient) -> Result<Vec<LolPlayerBehaviorReporterFeedback>, Box<dyn Error>> {
    let url = "/lol-player-behavior/v1/reporter-feedback";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolPlayerBehaviorReporterFeedback>>(value)?)
}

pub async fn get_lol_player_behavior_v1_reporter_feedback_by_id(client: RESTClient, id: String) -> Result<LolPlayerBehaviorReporterFeedback, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-player-behavior/v1/reporter-feedback/{{id}}", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolPlayerBehaviorReporterFeedback>(value)?)
}

pub async fn get_lol_player_behavior_v2_reform_card(client: RESTClient) -> Result<LolPlayerBehaviorReformCardV2, Box<dyn Error>> {
    let url = "/lol-player-behavior/v2/reform-card";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolPlayerBehaviorReformCardV2>(value)?)
}

pub async fn get_lol_player_behavior_v2_reporter_feedback(client: RESTClient) -> Result<Vec<LolPlayerBehaviorReporterFeedbackMessage>, Box<dyn Error>> {
    let url = "/lol-player-behavior/v2/reporter-feedback";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolPlayerBehaviorReporterFeedbackMessage>>(value)?)
}

pub async fn get_lol_player_behavior_v3_reform_cards(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/lol-player-behavior/v3/reform-cards";
    let value = client.get(url.to_owned()).await?;
    Ok(value)
}

pub async fn get_lol_player_level_up_v1_level_up(client: RESTClient) -> Result<LolPlayerLevelUpPlayerLevelUpEvent, Box<dyn Error>> {
    let url = "/lol-player-level-up/v1/level-up";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolPlayerLevelUpPlayerLevelUpEvent>(value)?)
}

pub async fn get_lol_player_level_up_v1_level_up_notifications_by_plugin_name(client: RESTClient, plugin_name: String) -> Result<LolPlayerLevelUpPlayerLevelUpEventAck, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-player-level-up/v1/level-up-notifications/{{plugin_name}}", &json!({"plugin_name": plugin_name}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolPlayerLevelUpPlayerLevelUpEventAck>(value)?)
}

pub async fn get_lol_player_messaging_v1_celebration_notification(client: RESTClient) -> Result<LolPlayerMessagingDynamicCelebrationMessagingNotificationResource, Box<dyn Error>> {
    let url = "/lol-player-messaging/v1/celebration/notification";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolPlayerMessagingDynamicCelebrationMessagingNotificationResource>(value)?)
}

pub async fn get_lol_player_messaging_v1_notification(client: RESTClient) -> Result<LolPlayerMessagingPlayerMessagingNotificationResource, Box<dyn Error>> {
    let url = "/lol-player-messaging/v1/notification";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolPlayerMessagingPlayerMessagingNotificationResource>(value)?)
}

pub async fn get_lol_player_preferences_v1_player_preferences_ready(client: RESTClient) -> Result<bool, Box<dyn Error>> {
    let url = "/lol-player-preferences/v1/player-preferences-ready";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<bool>(value)?)
}

pub async fn get_lol_player_preferences_v1_preference_by_type(client: RESTClient, type_: String, hash: Option<String>) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-player-preferences/v1/preference/{{type_}}?hash={{hash}}", &json!({"type_": type_, "hash": hash}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(value)
}

pub async fn get_lol_pre_end_of_game_v1_current_sequence_event(client: RESTClient) -> Result<LolPreEndOfGameSequenceEvent, Box<dyn Error>> {
    let url = "/lol-pre-end-of-game/v1/currentSequenceEvent";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolPreEndOfGameSequenceEvent>(value)?)
}

pub async fn get_lol_premade_voice_v1_availability(client: RESTClient) -> Result<LolPremadeVoiceVoiceAvailability, Box<dyn Error>> {
    let url = "/lol-premade-voice/v1/availability";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolPremadeVoiceVoiceAvailability>(value)?)
}

pub async fn get_lol_premade_voice_v1_capturedevices(client: RESTClient) -> Result<Vec<LolPremadeVoiceDeviceResource>, Box<dyn Error>> {
    let url = "/lol-premade-voice/v1/capturedevices";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolPremadeVoiceDeviceResource>>(value)?)
}

pub async fn get_lol_premade_voice_v1_first_experience(client: RESTClient) -> Result<LolPremadeVoiceFirstExperience, Box<dyn Error>> {
    let url = "/lol-premade-voice/v1/first-experience";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolPremadeVoiceFirstExperience>(value)?)
}

pub async fn get_lol_premade_voice_v1_mic_test(client: RESTClient) -> Result<LolPremadeVoiceAudioPropertiesResource, Box<dyn Error>> {
    let url = "/lol-premade-voice/v1/mic-test";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolPremadeVoiceAudioPropertiesResource>(value)?)
}

pub async fn get_lol_premade_voice_v1_participant_records(client: RESTClient) -> Result<Vec<LolPremadeVoicePremadeVoiceParticipantDto>, Box<dyn Error>> {
    let url = "/lol-premade-voice/v1/participant-records";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolPremadeVoicePremadeVoiceParticipantDto>>(value)?)
}

pub async fn get_lol_premade_voice_v1_participants(client: RESTClient) -> Result<Vec<LolPremadeVoicePremadeVoiceParticipantDto>, Box<dyn Error>> {
    let url = "/lol-premade-voice/v1/participants";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolPremadeVoicePremadeVoiceParticipantDto>>(value)?)
}

pub async fn get_lol_premade_voice_v1_settings(client: RESTClient) -> Result<LolPremadeVoiceSettingsResource, Box<dyn Error>> {
    let url = "/lol-premade-voice/v1/settings";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolPremadeVoiceSettingsResource>(value)?)
}

pub async fn get_lol_progression_v1_groups_by_group_id_configuration(client: RESTClient, group_id: String) -> Result<LolProgressionGroup, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-progression/v1/groups/{{group_id}}/configuration", &json!({"group_id": group_id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolProgressionGroup>(value)?)
}

pub async fn get_lol_progression_v1_groups_by_group_id_instance_data(client: RESTClient, group_id: String) -> Result<LolProgressionEntityInstance, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-progression/v1/groups/{{group_id}}/instanceData", &json!({"group_id": group_id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolProgressionEntityInstance>(value)?)
}

pub async fn get_lol_progression_v1_groups_configuration(client: RESTClient) -> Result<Vec<LolProgressionGroup>, Box<dyn Error>> {
    let url = "/lol-progression/v1/groups/configuration";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolProgressionGroup>>(value)?)
}

pub async fn get_lol_progression_v1_ready(client: RESTClient) -> Result<bool, Box<dyn Error>> {
    let url = "/lol-progression/v1/ready";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<bool>(value)?)
}

pub async fn get_lol_publishing_content_v1_listeners_allow_list_by_region(client: RESTClient, region: String) -> Result<Vec<String>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-publishing-content/v1/listeners/allow-list/{{region}}", &json!({"region": region}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<String>>(value)?)
}

pub async fn get_lol_publishing_content_v1_listeners_client_data(client: RESTClient) -> Result<LolPublishingContentClientData, Box<dyn Error>> {
    let url = "/lol-publishing-content/v1/listeners/client-data";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolPublishingContentClientData>(value)?)
}

pub async fn get_lol_publishing_content_v1_listeners_pubhub_config(client: RESTClient) -> Result<LolPublishingContentPubHubConfig, Box<dyn Error>> {
    let url = "/lol-publishing-content/v1/listeners/pubhub-config";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolPublishingContentPubHubConfig>(value)?)
}

pub async fn get_lol_publishing_content_v1_ready(client: RESTClient) -> Result<bool, Box<dyn Error>> {
    let url = "/lol-publishing-content/v1/ready";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<bool>(value)?)
}

pub async fn get_lol_publishing_content_v1_settings(client: RESTClient) -> Result<LolPublishingContentPublishingSettings, Box<dyn Error>> {
    let url = "/lol-publishing-content/v1/settings";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolPublishingContentPublishingSettings>(value)?)
}

pub async fn get_lol_publishing_content_v1_tft_hub_cards(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/lol-publishing-content/v1/tft-hub-cards";
    let value = client.get(url.to_owned()).await?;
    Ok(value)
}

pub async fn get_lol_purchase_widget_v1_configuration(client: RESTClient) -> Result<LolPurchaseWidgetPurchaseWidgetConfig, Box<dyn Error>> {
    let url = "/lol-purchase-widget/v1/configuration";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolPurchaseWidgetPurchaseWidgetConfig>(value)?)
}

pub async fn get_lol_purchase_widget_v1_order_notifications(client: RESTClient) -> Result<Vec<LolPurchaseWidgetOrderNotificationResource>, Box<dyn Error>> {
    let url = "/lol-purchase-widget/v1/order-notifications";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolPurchaseWidgetOrderNotificationResource>>(value)?)
}

pub async fn get_lol_purchase_widget_v1_purchasable_item(client: RESTClient, inventory_type: String, item_id: i64) -> Result<LolPurchaseWidgetPurchasableItem, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-purchase-widget/v1/purchasable-item?inventoryType={{inventory_type}}&itemId={{item_id}}", &json!({"inventory_type": inventory_type, "item_id": item_id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolPurchaseWidgetPurchasableItem>(value)?)
}

pub async fn get_lol_purchase_widget_v3_base_skin_line_data_by_offer_id(client: RESTClient, offer_id: String) -> Result<LolPurchaseWidgetBaseSkinLineDto, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-purchase-widget/v3/base-skin-line-data/{{offer_id}}", &json!({"offer_id": offer_id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolPurchaseWidgetBaseSkinLineDto>(value)?)
}

pub async fn get_lol_purchase_widget_v3_purchase_offer_order_statuses(client: RESTClient) -> Result<LolPurchaseWidgetPurchaseOfferOrderStatuses, Box<dyn Error>> {
    let url = "/lol-purchase-widget/v3/purchase-offer-order-statuses";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolPurchaseWidgetPurchaseOfferOrderStatuses>(value)?)
}

pub async fn get_lol_ranked_v1_apex_leagues_by_queue_type_by_tier(client: RESTClient, queue_type: LolRankedLeagueQueueType, tier: LolRankedLeagueTier) -> Result<LolRankedLeagueLadderInfo, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-ranked/v1/apex-leagues/{queueType}/{{tier}}", &json!({"queue_type": queue_type, "tier": tier}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolRankedLeagueLadderInfo>(value)?)
}

pub async fn get_lol_ranked_v1_challenger_ladders_enabled(client: RESTClient) -> Result<Vec<String>, Box<dyn Error>> {
    let url = "/lol-ranked/v1/challenger-ladders-enabled";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<String>>(value)?)
}

pub async fn get_lol_ranked_v1_current_lp_change_notification(client: RESTClient) -> Result<LolRankedLcuLeagueNotification, Box<dyn Error>> {
    let url = "/lol-ranked/v1/current-lp-change-notification";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolRankedLcuLeagueNotification>(value)?)
}

pub async fn get_lol_ranked_v1_current_ranked_stats(client: RESTClient) -> Result<LolRankedRankedStats, Box<dyn Error>> {
    let url = "/lol-ranked/v1/current-ranked-stats";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolRankedRankedStats>(value)?)
}

pub async fn get_lol_ranked_v1_eos_notifications(client: RESTClient) -> Result<Vec<LolRankedEosNotificationResource>, Box<dyn Error>> {
    let url = "/lol-ranked/v1/eos-notifications";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolRankedEosNotificationResource>>(value)?)
}

pub async fn get_lol_ranked_v1_eos_rewards(client: RESTClient) -> Result<LolRankedEosRewardsConfig, Box<dyn Error>> {
    let url = "/lol-ranked/v1/eos-rewards";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolRankedEosRewardsConfig>(value)?)
}

pub async fn get_lol_ranked_v1_league_ladders_by_puuid(client: RESTClient, puuid: String) -> Result<Vec<LolRankedLeagueLadderInfo>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-ranked/v1/league-ladders/{{puuid}}", &json!({"puuid": puuid}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolRankedLeagueLadderInfo>>(value)?)
}

pub async fn get_lol_ranked_v1_notifications(client: RESTClient) -> Result<Vec<LolRankedLcuLeagueNotification>, Box<dyn Error>> {
    let url = "/lol-ranked/v1/notifications";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolRankedLcuLeagueNotification>>(value)?)
}

pub async fn get_lol_ranked_v1_ranked_stats(client: RESTClient, puuids: Vec<String>) -> Result<HashMap<String, LolRankedRankedStats>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-ranked/v1/ranked-stats?puuids={{puuids}}", &json!({"puuids": puuids}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<HashMap<String, LolRankedRankedStats>>(value)?)
}

pub async fn get_lol_ranked_v1_ranked_stats_by_puuid(client: RESTClient, puuid: String) -> Result<LolRankedRankedStats, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-ranked/v1/ranked-stats/{{puuid}}", &json!({"puuid": puuid}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolRankedRankedStats>(value)?)
}

pub async fn get_lol_ranked_v1_rated_ladder_by_queue_type(client: RESTClient, queue_type: LolRankedLeagueQueueType) -> Result<LolRankedRatedLadderInfo, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-ranked/v1/rated-ladder/{{queue_type}}", &json!({"queue_type": queue_type}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolRankedRatedLadderInfo>(value)?)
}

pub async fn get_lol_ranked_v1_signed_ranked_stats(client: RESTClient) -> Result<LolRankedSignedRankedStatsDTO, Box<dyn Error>> {
    let url = "/lol-ranked/v1/signed-ranked-stats";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolRankedSignedRankedStatsDTO>(value)?)
}

pub async fn get_lol_ranked_v1_social_leaderboard_ranked_queue_stats_for_puuids(client: RESTClient, queue_type: LolRankedLeagueQueueType, puuids: Vec<String>) -> Result<HashMap<String, LolRankedSocialLeaderboardRankedQueueStats>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-ranked/v1/social-leaderboard-ranked-queue-stats-for-puuids?queueType={{queue_type}}&puuids={{puuids}}", &json!({"queue_type": queue_type, "puuids": puuids}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<HashMap<String, LolRankedSocialLeaderboardRankedQueueStats>>(value)?)
}

pub async fn get_lol_ranked_v1_splits_config(client: RESTClient) -> Result<LolRankedRewardsInfo, Box<dyn Error>> {
    let url = "/lol-ranked/v1/splits-config";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolRankedRewardsInfo>(value)?)
}

pub async fn get_lol_ranked_v1_top_rated_ladders_enabled(client: RESTClient) -> Result<Vec<String>, Box<dyn Error>> {
    let url = "/lol-ranked/v1/top-rated-ladders-enabled";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<String>>(value)?)
}

pub async fn get_lol_ranked_v2_tiers(client: RESTClient, summoner_ids: Vec<u64>, queue_types: Vec<LolRankedLeagueQueueType>) -> Result<Vec<LolRankedParticipantTiers>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-ranked/v2/tiers?summonerIds={{summoner_ids}}&queueTypes={{queue_types}}", &json!({"summoner_ids": summoner_ids, "queue_types": queue_types}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolRankedParticipantTiers>>(value)?)
}

pub async fn get_lol_regalia_v2_config(client: RESTClient) -> Result<LolRegaliaRegaliaFrontendConfig, Box<dyn Error>> {
    let url = "/lol-regalia/v2/config";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolRegaliaRegaliaFrontendConfig>(value)?)
}

pub async fn get_lol_regalia_v2_current_summoner_regalia(client: RESTClient) -> Result<LolRegaliaRegaliaWithPreferences, Box<dyn Error>> {
    let url = "/lol-regalia/v2/current-summoner/regalia";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolRegaliaRegaliaWithPreferences>(value)?)
}

pub async fn get_lol_regalia_v2_summoners_by_summoner_id_queues_by_queue_positions_by_position_regalia(client: RESTClient, summoner_id: u64, queue: String, position: String) -> Result<LolRegaliaRegalia, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-regalia/v2/summoners/{summonerId}/queues/{queue}/positions/{{position}}/regalia", &json!({"summoner_id": summoner_id, "queue": queue, "position": position}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolRegaliaRegalia>(value)?)
}

pub async fn get_lol_regalia_v2_summoners_by_summoner_id_queues_by_queue_regalia(client: RESTClient, summoner_id: u64, queue: String) -> Result<LolRegaliaRegalia, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-regalia/v2/summoners/{summonerId}/queues/{{queue}}/regalia", &json!({"summoner_id": summoner_id, "queue": queue}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolRegaliaRegalia>(value)?)
}

pub async fn get_lol_regalia_v2_summoners_by_summoner_id_regalia(client: RESTClient, summoner_id: u64, hovercard: bool) -> Result<LolRegaliaRegalia, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-regalia/v2/summoners/{{summoner_id}}/regalia?hovercard={{hovercard}}", &json!({"summoner_id": summoner_id, "hovercard": hovercard}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolRegaliaRegalia>(value)?)
}

pub async fn get_lol_regalia_v2_summoners_by_summoner_id_regalia_async(client: RESTClient, summoner_id: u64) -> Result<LolRegaliaRegaliaAsync, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-regalia/v2/summoners/{{summoner_id}}/regalia/async", &json!({"summoner_id": summoner_id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolRegaliaRegaliaAsync>(value)?)
}

pub async fn get_lol_replays_v1_configuration(client: RESTClient) -> Result<LolReplaysReplaysConfiguration, Box<dyn Error>> {
    let url = "/lol-replays/v1/configuration";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolReplaysReplaysConfiguration>(value)?)
}

pub async fn get_lol_replays_v1_metadata_by_game_id(client: RESTClient, game_id: u64) -> Result<LolReplaysReplayMetadata, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-replays/v1/metadata/{{game_id}}", &json!({"game_id": game_id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolReplaysReplayMetadata>(value)?)
}

pub async fn get_lol_replays_v1_rofls_path(client: RESTClient) -> Result<String, Box<dyn Error>> {
    let url = "/lol-replays/v1/rofls/path";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<String>(value)?)
}

pub async fn get_lol_replays_v1_rofls_path_default(client: RESTClient) -> Result<String, Box<dyn Error>> {
    let url = "/lol-replays/v1/rofls/path/default";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<String>(value)?)
}

pub async fn get_lol_rewards_v1_grants(client: RESTClient, status: LolRewardsGrantStatus) -> Result<Vec<LolRewardsRewardGrant>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-rewards/v1/grants?status={{status}}", &json!({"status": status}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolRewardsRewardGrant>>(value)?)
}

pub async fn get_lol_rewards_v1_groups(client: RESTClient, types: Option<Vec<String>>) -> Result<Vec<LolRewardsSvcRewardGroup>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-rewards/v1/groups?types={{types}}", &json!({"types": types}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolRewardsSvcRewardGroup>>(value)?)
}

pub async fn get_lol_rms_v1_champion_mastery_leaveup_update(client: RESTClient) -> Result<Vec<LolRiotMessagingServiceChampionMasteryLevelUp>, Box<dyn Error>> {
    let url = "/lol-rms/v1/champion-mastery-leaveup-update";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolRiotMessagingServiceChampionMasteryLevelUp>>(value)?)
}

pub async fn get_lol_rso_auth_configuration_v3_ready_state(client: RESTClient) -> Result<LolRsoAuthRSOConfigReadyState, Box<dyn Error>> {
    let url = "/lol-rso-auth/configuration/v3/ready-state";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolRsoAuthRSOConfigReadyState>(value)?)
}

pub async fn get_lol_rso_auth_v1_auth_hints_hint(client: RESTClient) -> Result<LolRsoAuthAuthHint, Box<dyn Error>> {
    let url = "/lol-rso-auth/v1/auth-hints/hint";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolRsoAuthAuthHint>(value)?)
}

pub async fn get_lol_rso_auth_v1_authorization(client: RESTClient) -> Result<LolRsoAuthAuthorization, Box<dyn Error>> {
    let url = "/lol-rso-auth/v1/authorization";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolRsoAuthAuthorization>(value)?)
}

pub async fn get_lol_rso_auth_v1_authorization_access_token(client: RESTClient) -> Result<LolRsoAuthAccessToken, Box<dyn Error>> {
    let url = "/lol-rso-auth/v1/authorization/access-token";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolRsoAuthAccessToken>(value)?)
}

pub async fn get_lol_rso_auth_v1_authorization_error(client: RESTClient) -> Result<LolRsoAuthAuthError, Box<dyn Error>> {
    let url = "/lol-rso-auth/v1/authorization/error";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolRsoAuthAuthError>(value)?)
}

pub async fn get_lol_rso_auth_v1_authorization_id_token(client: RESTClient) -> Result<LolRsoAuthIdToken, Box<dyn Error>> {
    let url = "/lol-rso-auth/v1/authorization/id-token";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolRsoAuthIdToken>(value)?)
}

pub async fn get_lol_rso_auth_v1_authorization_userinfo(client: RESTClient) -> Result<LolRsoAuthUserInfo, Box<dyn Error>> {
    let url = "/lol-rso-auth/v1/authorization/userinfo";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolRsoAuthUserInfo>(value)?)
}

pub async fn get_lol_rso_auth_v1_status_by_platform_id(client: RESTClient, platform_id: String) -> Result<LolRsoAuthRegionStatus, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-rso-auth/v1/status/{{platform_id}}", &json!({"platform_id": platform_id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolRsoAuthRegionStatus>(value)?)
}

pub async fn get_lol_service_status_v1_lcu_status(client: RESTClient) -> Result<LolServiceStatusServiceStatusResource, Box<dyn Error>> {
    let url = "/lol-service-status/v1/lcu-status";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolServiceStatusServiceStatusResource>(value)?)
}

pub async fn get_lol_service_status_v1_ticker_messages(client: RESTClient) -> Result<Vec<LolServiceStatusTickerMessage>, Box<dyn Error>> {
    let url = "/lol-service-status/v1/ticker-messages";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolServiceStatusTickerMessage>>(value)?)
}

pub async fn get_lol_settings_v1_account_by_category(client: RESTClient, category: String) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-settings/v1/account/{{category}}", &json!({"category": category}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(value)
}

pub async fn get_lol_settings_v1_account_didreset(client: RESTClient) -> Result<bool, Box<dyn Error>> {
    let url = "/lol-settings/v1/account/didreset";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<bool>(value)?)
}

pub async fn get_lol_settings_v1_local_by_category(client: RESTClient, category: String) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-settings/v1/local/{{category}}", &json!({"category": category}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(value)
}

pub async fn get_lol_settings_v2_account_by_pp_type_by_category(client: RESTClient, pp_type: String, category: String) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-settings/v2/account/{ppType}/{{category}}", &json!({"pp_type": pp_type, "category": category}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(value)
}

pub async fn get_lol_settings_v2_config(client: RESTClient) -> Result<LolSettingsSettingsConfig, Box<dyn Error>> {
    let url = "/lol-settings/v2/config";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolSettingsSettingsConfig>(value)?)
}

pub async fn get_lol_settings_v2_didreset_by_pp_type(client: RESTClient, pp_type: String) -> Result<bool, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-settings/v2/didreset/{{pp_type}}", &json!({"pp_type": pp_type}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<bool>(value)?)
}

pub async fn get_lol_settings_v2_local_by_category(client: RESTClient, category: String) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-settings/v2/local/{{category}}", &json!({"category": category}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(value)
}

pub async fn get_lol_settings_v2_ready(client: RESTClient) -> Result<bool, Box<dyn Error>> {
    let url = "/lol-settings/v2/ready";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<bool>(value)?)
}

pub async fn get_lol_shutdown_v1_notification(client: RESTClient) -> Result<LolShutdownShutdownNotification, Box<dyn Error>> {
    let url = "/lol-shutdown/v1/notification";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolShutdownShutdownNotification>(value)?)
}

pub async fn get_lol_simple_dialog_messages_v1_messages(client: RESTClient) -> Result<Vec<LolSimpleDialogMessagesMessage>, Box<dyn Error>> {
    let url = "/lol-simple-dialog-messages/v1/messages";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolSimpleDialogMessagesMessage>>(value)?)
}

pub async fn get_lol_social_leaderboard_v1_leaderboard_next_update_time(client: RESTClient, queue_type: LolSocialLeaderboardLeagueQueueType) -> Result<i64, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-social-leaderboard/v1/leaderboard-next-update-time?queueType={{queue_type}}", &json!({"queue_type": queue_type}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<i64>(value)?)
}

pub async fn get_lol_social_leaderboard_v1_social_leaderboard_data(client: RESTClient, queue_type: LolSocialLeaderboardLeagueQueueType) -> Result<LolSocialLeaderboardSocialLeaderboardData, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-social-leaderboard/v1/social-leaderboard-data?queueType={{queue_type}}", &json!({"queue_type": queue_type}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolSocialLeaderboardSocialLeaderboardData>(value)?)
}

pub async fn get_lol_spectator_v1_spectate(client: RESTClient) -> Result<LolSpectatorSpectateGameInfo, Box<dyn Error>> {
    let url = "/lol-spectator/v1/spectate";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolSpectatorSpectateGameInfo>(value)?)
}

pub async fn get_lol_statstones_v1_eog_notifications_by_game_id(client: RESTClient, game_id: u64) -> Result<LolStatstonesEogNotificationEnvelope, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-statstones/v1/eog-notifications/{{game_id}}", &json!({"game_id": game_id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolStatstonesEogNotificationEnvelope>(value)?)
}

pub async fn get_lol_statstones_v1_featured_champion_statstones_by_champion_item_id(client: RESTClient, champion_item_id: i32) -> Result<Vec<LolStatstonesStatstone>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-statstones/v1/featured-champion-statstones/{{champion_item_id}}", &json!({"champion_item_id": champion_item_id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolStatstonesStatstone>>(value)?)
}

pub async fn get_lol_statstones_v1_profile_summary_by_puuid(client: RESTClient, puuid: String) -> Result<Vec<LolStatstonesProfileStatstoneSummary>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-statstones/v1/profile-summary/{{puuid}}", &json!({"puuid": puuid}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolStatstonesProfileStatstoneSummary>>(value)?)
}

pub async fn get_lol_statstones_v1_statstone_by_content_id_owned(client: RESTClient, content_id: String) -> Result<bool, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-statstones/v1/statstone/{{content_id}}/owned", &json!({"content_id": content_id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<bool>(value)?)
}

pub async fn get_lol_statstones_v1_statstones_enabled_queue_ids(client: RESTClient) -> Result<Vec<i32>, Box<dyn Error>> {
    let url = "/lol-statstones/v1/statstones-enabled-queue-ids";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<i32>>(value)?)
}

pub async fn get_lol_statstones_v1_vignette_notifications(client: RESTClient) -> Result<Vec<Value>, Box<dyn Error>> {
    let url = "/lol-statstones/v1/vignette-notifications";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<Value>>(value)?)
}

pub async fn get_lol_statstones_v2_player_statstones_self_by_champion_item_id(client: RESTClient, champion_item_id: i32) -> Result<Vec<LolStatstonesStatstoneSet>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-statstones/v2/player-statstones-self/{{champion_item_id}}", &json!({"champion_item_id": champion_item_id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolStatstonesStatstoneSet>>(value)?)
}

pub async fn get_lol_statstones_v2_player_summary_self(client: RESTClient) -> Result<Vec<LolStatstonesChampionStatstoneSummary>, Box<dyn Error>> {
    let url = "/lol-statstones/v2/player-summary-self";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolStatstonesChampionStatstoneSummary>>(value)?)
}

pub async fn get_lol_store_v1_by_page_type(client: RESTClient, page_type: String) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-store/v1/{{page_type}}", &json!({"page_type": page_type}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(value)
}

pub async fn get_lol_store_v1_catalog(client: RESTClient, inventory_type: Option<Vec<String>>, item_id: Option<Vec<i32>>) -> Result<Vec<LolStoreCatalogItem>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-store/v1/catalog?inventoryType={{inventory_type}}&itemId={{item_id}}", &json!({"inventory_type": inventory_type, "item_id": item_id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolStoreCatalogItem>>(value)?)
}

pub async fn get_lol_store_v1_catalog_by_instance_ids(client: RESTClient, instance_ids: Vec<String>) -> Result<Vec<LolStoreCatalogItem>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-store/v1/catalogByInstanceIds?instanceIds={{instance_ids}}", &json!({"instance_ids": instance_ids}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolStoreCatalogItem>>(value)?)
}

pub async fn get_lol_store_v1_catalog_by_inventory_type(client: RESTClient, inventory_type: String, item_ids: Vec<i32>) -> Result<Vec<LolStoreCatalogItem>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-store/v1/catalog/{{inventory_type}}?itemIds={{item_ids}}", &json!({"inventory_type": inventory_type, "item_ids": item_ids}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolStoreCatalogItem>>(value)?)
}

pub async fn get_lol_store_v1_catalog_sales(client: RESTClient) -> Result<Vec<LolStoreItemSale>, Box<dyn Error>> {
    let url = "/lol-store/v1/catalog/sales";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolStoreItemSale>>(value)?)
}

pub async fn get_lol_store_v1_get_store_url(client: RESTClient) -> Result<String, Box<dyn Error>> {
    let url = "/lol-store/v1/getStoreUrl";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<String>(value)?)
}

pub async fn get_lol_store_v1_giftablefriends(client: RESTClient) -> Result<Vec<LolStoreGiftingFriend>, Box<dyn Error>> {
    let url = "/lol-store/v1/giftablefriends";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolStoreGiftingFriend>>(value)?)
}

pub async fn get_lol_store_v1_item_keys_from_instance_ids(client: RESTClient, instance_ids: Vec<String>) -> Result<HashMap<String, LolStoreItemKey>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-store/v1/itemKeysFromInstanceIds?instanceIds={{instance_ids}}", &json!({"instance_ids": instance_ids}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<HashMap<String, LolStoreItemKey>>(value)?)
}

pub async fn get_lol_store_v1_item_keys_from_offer_ids(client: RESTClient, offer_ids: Vec<String>) -> Result<HashMap<String, LolStoreItemKey>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-store/v1/itemKeysFromOfferIds?offerIds={{offer_ids}}", &json!({"offer_ids": offer_ids}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<HashMap<String, LolStoreItemKey>>(value)?)
}

pub async fn get_lol_store_v1_last_page(client: RESTClient) -> Result<String, Box<dyn Error>> {
    let url = "/lol-store/v1/lastPage";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<String>(value)?)
}

pub async fn get_lol_store_v1_offers(client: RESTClient, inventory_type_u_u_i_ds: Option<Vec<String>>) -> Result<Vec<LolStoreCapOffer>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-store/v1/offers?inventoryTypeUUIDs={{inventory_type_u_u_i_ds}}", &json!({"inventory_type_u_u_i_ds": inventory_type_u_u_i_ds}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolStoreCapOffer>>(value)?)
}

pub async fn get_lol_store_v1_offers_by_offer_id(client: RESTClient, offer_id: String) -> Result<LolStoreCapOffer, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-store/v1/offers/{{offer_id}}", &json!({"offer_id": offer_id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolStoreCapOffer>(value)?)
}

pub async fn get_lol_store_v1_order_notifications(client: RESTClient) -> Result<Vec<LolStoreOrderNotificationResource>, Box<dyn Error>> {
    let url = "/lol-store/v1/order-notifications";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolStoreOrderNotificationResource>>(value)?)
}

pub async fn get_lol_store_v1_order_notifications_by_id(client: RESTClient, id: u64) -> Result<LolStoreOrderNotificationResource, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-store/v1/order-notifications/{{id}}", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolStoreOrderNotificationResource>(value)?)
}

pub async fn get_lol_store_v1_payment_details(client: RESTClient, action: String, gift_recipient_account_id: Option<u64>, gift_message: Option<String>) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-store/v1/paymentDetails?action={{action}}&giftRecipientAccountId={{gift_recipient_account_id}}&giftMessage={{gift_message}}", &json!({"action": action, "gift_recipient_account_id": gift_recipient_account_id, "gift_message": gift_message}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(value)
}

pub async fn get_lol_store_v1_skins_by_skin_id(client: RESTClient, skin_id: i32) -> Result<LolStoreCatalogItem, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-store/v1/skins/{{skin_id}}", &json!({"skin_id": skin_id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolStoreCatalogItem>(value)?)
}

pub async fn get_lol_store_v1_status(client: RESTClient) -> Result<LolStoreStoreStatus, Box<dyn Error>> {
    let url = "/lol-store/v1/status";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolStoreStoreStatus>(value)?)
}

pub async fn get_lol_store_v1_store_ready(client: RESTClient) -> Result<bool, Box<dyn Error>> {
    let url = "/lol-store/v1/store-ready";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<bool>(value)?)
}

pub async fn get_lol_store_v1_wallet(client: RESTClient) -> Result<LolStoreWallet, Box<dyn Error>> {
    let url = "/lol-store/v1/wallet";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolStoreWallet>(value)?)
}

pub async fn get_lol_store_v2_offers(client: RESTClient, type_id: Option<String>) -> Result<Vec<LolStoreCapOffer>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-store/v2/offers?typeId={{type_id}}", &json!({"type_id": type_id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolStoreCapOffer>>(value)?)
}

pub async fn get_lol_suggested_players_v1_suggested_players(client: RESTClient) -> Result<Vec<LolSuggestedPlayersSuggestedPlayersSuggestedPlayer>, Box<dyn Error>> {
    let url = "/lol-suggested-players/v1/suggested-players";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolSuggestedPlayersSuggestedPlayersSuggestedPlayer>>(value)?)
}

pub async fn get_lol_summoner_v1_check_name_availability_by_name(client: RESTClient, name: String) -> Result<bool, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-summoner/v1/check-name-availability/{{name}}", &json!({"name": name}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<bool>(value)?)
}

pub async fn get_lol_summoner_v1_check_name_availability_new_summoners_by_name(client: RESTClient, name: String) -> Result<bool, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-summoner/v1/check-name-availability-new-summoners/{{name}}", &json!({"name": name}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<bool>(value)?)
}

pub async fn get_lol_summoner_v1_current_summoner(client: RESTClient) -> Result<LolSummonerSummoner, Box<dyn Error>> {
    let url = "/lol-summoner/v1/current-summoner";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolSummonerSummoner>(value)?)
}

pub async fn get_lol_summoner_v1_current_summoner_account_and_summoner_ids(client: RESTClient) -> Result<LolSummonerAccountIdAndSummonerId, Box<dyn Error>> {
    let url = "/lol-summoner/v1/current-summoner/account-and-summoner-ids";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolSummonerAccountIdAndSummonerId>(value)?)
}

pub async fn get_lol_summoner_v1_current_summoner_autofill(client: RESTClient) -> Result<Vec<LolSummonerAutoFillQueueDto>, Box<dyn Error>> {
    let url = "/lol-summoner/v1/current-summoner/autofill";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolSummonerAutoFillQueueDto>>(value)?)
}

pub async fn get_lol_summoner_v1_current_summoner_jwt(client: RESTClient) -> Result<String, Box<dyn Error>> {
    let url = "/lol-summoner/v1/current-summoner/jwt";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<String>(value)?)
}

pub async fn get_lol_summoner_v1_current_summoner_profile_privacy(client: RESTClient) -> Result<LolSummonerProfilePrivacy, Box<dyn Error>> {
    let url = "/lol-summoner/v1/current-summoner/profile-privacy";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolSummonerProfilePrivacy>(value)?)
}

pub async fn get_lol_summoner_v1_current_summoner_reroll_points(client: RESTClient) -> Result<LolSummonerSummonerRerollPoints, Box<dyn Error>> {
    let url = "/lol-summoner/v1/current-summoner/rerollPoints";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolSummonerSummonerRerollPoints>(value)?)
}

pub async fn get_lol_summoner_v1_current_summoner_summoner_profile(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/lol-summoner/v1/current-summoner/summoner-profile";
    let value = client.get(url.to_owned()).await?;
    Ok(value)
}

pub async fn get_lol_summoner_v1_profile_privacy_enabled(client: RESTClient) -> Result<LolSummonerProfilePrivacyEnabledState, Box<dyn Error>> {
    let url = "/lol-summoner/v1/profile-privacy-enabled";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolSummonerProfilePrivacyEnabledState>(value)?)
}

pub async fn get_lol_summoner_v1_status(client: RESTClient) -> Result<LolSummonerStatus, Box<dyn Error>> {
    let url = "/lol-summoner/v1/status";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolSummonerStatus>(value)?)
}

pub async fn get_lol_summoner_v1_summoner_profile(client: RESTClient, puuid: String) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-summoner/v1/summoner-profile?puuid={{puuid}}", &json!({"puuid": puuid}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(value)
}

pub async fn get_lol_summoner_v1_summoner_requests_ready(client: RESTClient) -> Result<bool, Box<dyn Error>> {
    let url = "/lol-summoner/v1/summoner-requests-ready";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<bool>(value)?)
}

pub async fn get_lol_summoner_v1_summoners(client: RESTClient, name: String) -> Result<LolSummonerSummoner, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-summoner/v1/summoners?name={{name}}", &json!({"name": name}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolSummonerSummoner>(value)?)
}

pub async fn get_lol_summoner_v1_summoners_by_id(client: RESTClient, id: u64) -> Result<LolSummonerSummoner, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-summoner/v1/summoners/{{id}}", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolSummonerSummoner>(value)?)
}

pub async fn get_lol_summoner_v1_summoners_by_puuid_cached_by_puuid(client: RESTClient, puuid: String) -> Result<LolSummonerSummoner, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-summoner/v1/summoners-by-puuid-cached/{{puuid}}", &json!({"puuid": puuid}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolSummonerSummoner>(value)?)
}

pub async fn get_lol_summoner_v2_summoner_icons(client: RESTClient, ids: Vec<u64>) -> Result<Vec<LolSummonerSummonerIdAndIcon>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-summoner/v2/summoner-icons?ids={{ids}}", &json!({"ids": ids}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolSummonerSummonerIdAndIcon>>(value)?)
}

pub async fn get_lol_summoner_v2_summoner_names(client: RESTClient, ids: Vec<u64>) -> Result<Vec<LolSummonerSummonerIdAndName>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-summoner/v2/summoner-names?ids={{ids}}", &json!({"ids": ids}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolSummonerSummonerIdAndName>>(value)?)
}

pub async fn get_lol_summoner_v2_summoners(client: RESTClient, ids: Option<Vec<u64>>) -> Result<Vec<LolSummonerSummoner>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-summoner/v2/summoners?ids={{ids}}", &json!({"ids": ids}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolSummonerSummoner>>(value)?)
}

pub async fn get_lol_summoner_v2_summoners_puuid_by_puuid(client: RESTClient, puuid: String) -> Result<LolSummonerSummoner, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-summoner/v2/summoners/puuid/{{puuid}}", &json!({"puuid": puuid}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolSummonerSummoner>(value)?)
}

pub async fn get_lol_tastes_v1_ready(client: RESTClient) -> Result<bool, Box<dyn Error>> {
    let url = "/lol-tastes/v1/ready";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<bool>(value)?)
}

pub async fn get_lol_tastes_v1_skins_model(client: RESTClient) -> Result<LolTastesDataModelResponse, Box<dyn Error>> {
    let url = "/lol-tastes/v1/skins-model";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolTastesDataModelResponse>(value)?)
}

pub async fn get_lol_tastes_v1_tft_overview_model(client: RESTClient) -> Result<LolTastesDataModelResponse, Box<dyn Error>> {
    let url = "/lol-tastes/v1/tft-overview-model";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolTastesDataModelResponse>(value)?)
}

pub async fn get_lol_tft_v1_tft_direct_to_hub(client: RESTClient) -> Result<bool, Box<dyn Error>> {
    let url = "/lol-tft/v1/tft/directToHub";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<bool>(value)?)
}

pub async fn get_lol_tft_v1_tft_home_hub(client: RESTClient) -> Result<LolTftLolTftHomeHub, Box<dyn Error>> {
    let url = "/lol-tft/v1/tft/homeHub";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolTftLolTftHomeHub>(value)?)
}

pub async fn get_lol_tft_v1_tft_news_hub(client: RESTClient) -> Result<LolTftLolTftNewsHub, Box<dyn Error>> {
    let url = "/lol-tft/v1/tft/newsHub";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolTftLolTftNewsHub>(value)?)
}

pub async fn get_lol_tft_v2_tft_battlepass(client: RESTClient) -> Result<LolMissionsTftPaidBattlepass, Box<dyn Error>> {
    let url = "/lol-tft/v2/tft/battlepass";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolMissionsTftPaidBattlepass>(value)?)
}

pub async fn get_lol_token_upsell_v1_all(client: RESTClient) -> Result<Vec<LolWorldsTokenCardTokenUpsell>, Box<dyn Error>> {
    let url = "/lol-token-upsell/v1/all";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolWorldsTokenCardTokenUpsell>>(value)?)
}

pub async fn get_lol_trophies_v1_current_summoner_trophies_profile(client: RESTClient) -> Result<LolTrophiesTrophyProfileData, Box<dyn Error>> {
    let url = "/lol-trophies/v1/current-summoner/trophies/profile";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolTrophiesTrophyProfileData>(value)?)
}

pub async fn get_lol_trophies_v1_players_by_puuid_trophies_profile(client: RESTClient, puuid: String) -> Result<LolTrophiesTrophyProfileData, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-trophies/v1/players/{{puuid}}/trophies/profile", &json!({"puuid": puuid}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolTrophiesTrophyProfileData>(value)?)
}

pub async fn get_lol_yourshop_v1_has_permissions(client: RESTClient) -> Result<bool, Box<dyn Error>> {
    let url = "/lol-yourshop/v1/has-permissions";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<bool>(value)?)
}

pub async fn get_lol_yourshop_v1_modal(client: RESTClient) -> Result<bool, Box<dyn Error>> {
    let url = "/lol-yourshop/v1/modal";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<bool>(value)?)
}

pub async fn get_lol_yourshop_v1_offers(client: RESTClient) -> Result<Vec<LolYourshopUIOffer>, Box<dyn Error>> {
    let url = "/lol-yourshop/v1/offers";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<LolYourshopUIOffer>>(value)?)
}

pub async fn get_lol_yourshop_v1_ready(client: RESTClient) -> Result<bool, Box<dyn Error>> {
    let url = "/lol-yourshop/v1/ready";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<bool>(value)?)
}

pub async fn get_lol_yourshop_v1_status(client: RESTClient) -> Result<LolYourshopUIStatus, Box<dyn Error>> {
    let url = "/lol-yourshop/v1/status";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<LolYourshopUIStatus>(value)?)
}

pub async fn get_lol_yourshop_v1_themed(client: RESTClient) -> Result<bool, Box<dyn Error>> {
    let url = "/lol-yourshop/v1/themed";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<bool>(value)?)
}

pub async fn get_memory_v1_fe_processes_ready(client: RESTClient) -> Result<bool, Box<dyn Error>> {
    let url = "/memory/v1/fe-processes-ready";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<bool>(value)?)
}

pub async fn get_patcher_v1_notifications(client: RESTClient) -> Result<Vec<PatcherNotification>, Box<dyn Error>> {
    let url = "/patcher/v1/notifications";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<PatcherNotification>>(value)?)
}

pub async fn get_patcher_v1_p2p_status(client: RESTClient) -> Result<PatcherP2PStatus, Box<dyn Error>> {
    let url = "/patcher/v1/p2p/status";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<PatcherP2PStatus>(value)?)
}

pub async fn get_patcher_v1_products(client: RESTClient) -> Result<Vec<String>, Box<dyn Error>> {
    let url = "/patcher/v1/products";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<String>>(value)?)
}

pub async fn get_patcher_v1_products_by_product_id_paths(client: RESTClient, product_id: String) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/patcher/v1/products/{{product_id}}/paths", &json!({"product_id": product_id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<HashMap<String, String>>(value)?)
}

pub async fn get_patcher_v1_products_by_product_id_state(client: RESTClient, product_id: String) -> Result<PatcherProductState, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/patcher/v1/products/{{product_id}}/state", &json!({"product_id": product_id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<PatcherProductState>(value)?)
}

pub async fn get_patcher_v1_products_by_product_id_tags(client: RESTClient, product_id: String) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/patcher/v1/products/{{product_id}}/tags", &json!({"product_id": product_id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<HashMap<String, String>>(value)?)
}

pub async fn get_patcher_v1_status(client: RESTClient) -> Result<PatcherStatus, Box<dyn Error>> {
    let url = "/patcher/v1/status";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<PatcherStatus>(value)?)
}

pub async fn get_performance_v1_memory(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/performance/v1/memory";
    let value = client.get(url.to_owned()).await?;
    Ok(value)
}

pub async fn get_performance_v1_report(client: RESTClient) -> Result<Vec<Value>, Box<dyn Error>> {
    let url = "/performance/v1/report";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<Value>>(value)?)
}

pub async fn get_performance_v1_system_info(client: RESTClient, full: Option<i32>) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/performance/v1/system-info?full={{full}}", &json!({"full": full}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(value)
}

pub async fn get_player_notifications_v1_config(client: RESTClient) -> Result<PlayerNotificationsPlayerNotificationConfigResource, Box<dyn Error>> {
    let url = "/player-notifications/v1/config";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<PlayerNotificationsPlayerNotificationConfigResource>(value)?)
}

pub async fn get_player_notifications_v1_notifications(client: RESTClient) -> Result<Vec<PlayerNotificationsPlayerNotificationResource>, Box<dyn Error>> {
    let url = "/player-notifications/v1/notifications";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<PlayerNotificationsPlayerNotificationResource>>(value)?)
}

pub async fn get_player_notifications_v1_notifications_by_id(client: RESTClient, id: u64) -> Result<PlayerNotificationsPlayerNotificationResource, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/player-notifications/v1/notifications/{{id}}", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<PlayerNotificationsPlayerNotificationResource>(value)?)
}

pub async fn get_plugin_manager_v1_external_plugins_availability(client: RESTClient) -> Result<ExternalPluginsResource, Box<dyn Error>> {
    let url = "/plugin-manager/v1/external-plugins/availability";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<ExternalPluginsResource>(value)?)
}

pub async fn get_plugin_manager_v1_status(client: RESTClient) -> Result<PluginManagerResource, Box<dyn Error>> {
    let url = "/plugin-manager/v1/status";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<PluginManagerResource>(value)?)
}

pub async fn get_plugin_manager_v2_descriptions(client: RESTClient) -> Result<Vec<PluginDescriptionResource>, Box<dyn Error>> {
    let url = "/plugin-manager/v2/descriptions";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<PluginDescriptionResource>>(value)?)
}

pub async fn get_plugin_manager_v2_descriptions_by_plugin(client: RESTClient, plugin: String) -> Result<PluginDescriptionResource, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/plugin-manager/v2/descriptions/{{plugin}}", &json!({"plugin": plugin}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<PluginDescriptionResource>(value)?)
}

pub async fn get_plugin_manager_v2_plugins(client: RESTClient) -> Result<Vec<PluginResource>, Box<dyn Error>> {
    let url = "/plugin-manager/v2/plugins";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<PluginResource>>(value)?)
}

pub async fn get_plugin_manager_v2_plugins_by_plugin(client: RESTClient, plugin: String) -> Result<PluginResource, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/plugin-manager/v2/plugins/{{plugin}}", &json!({"plugin": plugin}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<PluginResource>(value)?)
}

pub async fn get_plugin_manager_v3_plugins_manifest(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/plugin-manager/v3/plugins-manifest";
    let value = client.get(url.to_owned()).await?;
    Ok(value)
}

pub async fn get_process_control_v1_process(client: RESTClient) -> Result<ProcessControlProcess, Box<dyn Error>> {
    let url = "/process-control/v1/process";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<ProcessControlProcess>(value)?)
}

pub async fn get_riot_messaging_service_v1_message_by_a(client: RESTClient, a: String) -> Result<RmsMessage, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/riot-messaging-service/v1/message/{{a}}", &json!({"a": a}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<RmsMessage>(value)?)
}

pub async fn get_riot_messaging_service_v1_message_by_a_by_b(client: RESTClient, a: String, b: String) -> Result<RmsMessage, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/riot-messaging-service/v1/message/{a}/{{b}}", &json!({"a": a, "b": b}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<RmsMessage>(value)?)
}

pub async fn get_riot_messaging_service_v1_message_by_a_by_b_by_c(client: RESTClient, a: String, b: String, c: String) -> Result<RmsMessage, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/riot-messaging-service/v1/message/{a}/{b}/{{c}}", &json!({"a": a, "b": b, "c": c}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<RmsMessage>(value)?)
}

pub async fn get_riot_messaging_service_v1_message_by_a_by_b_by_c_by_d(client: RESTClient, a: String, b: String, c: String, d: String) -> Result<RmsMessage, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/riot-messaging-service/v1/message/{a}/{b}/{c}/{{d}}", &json!({"a": a, "b": b, "c": c, "d": d}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<RmsMessage>(value)?)
}

pub async fn get_riot_messaging_service_v1_message_by_a_by_b_by_c_by_d_by_e(client: RESTClient, a: String, b: String, c: String, d: String, e: String) -> Result<RmsMessage, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/riot-messaging-service/v1/message/{a}/{b}/{c}/{d}/{{e}}", &json!({"a": a, "b": b, "c": c, "d": d, "e": e}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<RmsMessage>(value)?)
}

pub async fn get_riot_messaging_service_v1_message_by_a_by_b_by_c_by_d_by_e_by_f(client: RESTClient, a: String, b: String, c: String, d: String, e: String, f: String) -> Result<RmsMessage, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/riot-messaging-service/v1/message/{a}/{b}/{c}/{d}/{e}/{{f}}", &json!({"a": a, "b": b, "c": c, "d": d, "e": e, "f": f}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<RmsMessage>(value)?)
}

pub async fn get_riot_messaging_service_v1_session(client: RESTClient) -> Result<RiotMessagingServiceSession, Box<dyn Error>> {
    let url = "/riot-messaging-service/v1/session";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<RiotMessagingServiceSession>(value)?)
}

pub async fn get_riot_messaging_service_v1_state(client: RESTClient) -> Result<RiotMessagingServiceState, Box<dyn Error>> {
    let url = "/riot-messaging-service/v1/state";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<RiotMessagingServiceState>(value)?)
}

pub async fn get_riotclient_affinity(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/riotclient/affinity";
    let value = client.get(url.to_owned()).await?;
    Ok(value)
}

pub async fn get_riotclient_app_name(client: RESTClient) -> Result<String, Box<dyn Error>> {
    let url = "/riotclient/app-name";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<String>(value)?)
}

pub async fn get_riotclient_app_port(client: RESTClient) -> Result<u16, Box<dyn Error>> {
    let url = "/riotclient/app-port";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<u16>(value)?)
}

pub async fn get_riotclient_auth_token(client: RESTClient) -> Result<String, Box<dyn Error>> {
    let url = "/riotclient/auth-token";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<String>(value)?)
}

pub async fn get_riotclient_command_line_args(client: RESTClient) -> Result<Vec<String>, Box<dyn Error>> {
    let url = "/riotclient/command-line-args";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<Vec<String>>(value)?)
}

pub async fn get_riotclient_get_region_locale(client: RESTClient) -> Result<RegionLocale, Box<dyn Error>> {
    let url = "/riotclient/get_region_locale";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<RegionLocale>(value)?)
}

pub async fn get_riotclient_machine_id(client: RESTClient) -> Result<String, Box<dyn Error>> {
    let url = "/riotclient/machine-id";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<String>(value)?)
}

pub async fn get_riotclient_region_locale(client: RESTClient) -> Result<RegionLocale, Box<dyn Error>> {
    let url = "/riotclient/region-locale";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<RegionLocale>(value)?)
}

pub async fn get_riotclient_system_info_v1_basic_info(client: RESTClient) -> Result<BasicSystemInfo, Box<dyn Error>> {
    let url = "/riotclient/system-info/v1/basic-info";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<BasicSystemInfo>(value)?)
}

pub async fn get_riotclient_trace(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/riotclient/trace";
    let value = client.get(url.to_owned()).await?;
    Ok(value)
}

pub async fn get_riotclient_ux_crash_count(client: RESTClient) -> Result<u32, Box<dyn Error>> {
    let url = "/riotclient/ux-crash-count";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<u32>(value)?)
}

pub async fn get_riotclient_ux_state(client: RESTClient) -> Result<String, Box<dyn Error>> {
    let url = "/riotclient/ux-state";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<String>(value)?)
}

pub async fn get_riotclient_v1_crash_reporting_environment(client: RESTClient) -> Result<CrashReportingEnvironment, Box<dyn Error>> {
    let url = "/riotclient/v1/crash-reporting/environment";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<CrashReportingEnvironment>(value)?)
}

pub async fn get_riotclient_zoom_scale(client: RESTClient) -> Result<f64, Box<dyn Error>> {
    let url = "/riotclient/zoom-scale";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<f64>(value)?)
}

pub async fn get_sanitizer_v1_status(client: RESTClient) -> Result<SanitizerSanitizerStatus, Box<dyn Error>> {
    let url = "/sanitizer/v1/status";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<SanitizerSanitizerStatus>(value)?)
}

pub async fn get_system_v1_builds(client: RESTClient) -> Result<BuildInfo, Box<dyn Error>> {
    let url = "/system/v1/builds";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<BuildInfo>(value)?)
}

pub async fn get_telemetry_v1_application_start_time(client: RESTClient) -> Result<u64, Box<dyn Error>> {
    let url = "/telemetry/v1/application-start-time";
    let value = client.get(url.to_owned()).await?;
    Ok(serde_json::from_value::<u64>(value)?)
}

pub async fn head_by_plugin_assets_by_path(client: RESTClient, plugin: String, path: String, if_none_match: Option<String>) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/{{plugin}}/assets/{+path}?path={{path}}&if-none-match={{if_none_match}}", &json!({"plugin": plugin, "path": path, "if_none_match": if_none_match}))?;
    let url = format!("{}", template_url);
    let value = client.head(url.to_owned()).await?;
    Ok(value)
}

pub async fn http2_async_delete(client: RESTClient, async_token: u32) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/async/v1/status/{{async_token}}", &json!({"async_token": async_token}))?;
    let url = format!("{}", template_url);
    let value = client.delete(url.to_owned()).await?;
    Ok(value)
}

pub async fn http2_async_result(client: RESTClient, async_token: u32) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/async/v1/result/{{async_token}}", &json!({"async_token": async_token}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(value)
}

pub async fn http2_async_status(client: RESTClient, async_token: u32) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/async/v1/status/{{async_token}}", &json!({"async_token": async_token}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(value)
}

pub async fn http_async_delete(client: RESTClient, async_token: u32) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/async/v1/status/{{async_token}}", &json!({"async_token": async_token}))?;
    let url = format!("{}", template_url);
    let value = client.delete(url.to_owned()).await?;
    Ok(value)
}

pub async fn http_async_result(client: RESTClient, async_token: u32) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/async/v1/result/{{async_token}}", &json!({"async_token": async_token}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(value)
}

pub async fn http_async_status(client: RESTClient, async_token: u32) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/async/v1/status/{{async_token}}", &json!({"async_token": async_token}))?;
    let url = format!("{}", template_url);
    let value = client.get(url.to_owned()).await?;
    Ok(value)
}

pub async fn patch_lol_champ_select_legacy_v1_session_actions_by_id(client: RESTClient, id: u64, body: LolChampSelectLegacyChampSelectAction) -> Result<Value, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-champ-select-legacy/v1/session/actions/{{id}}", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.patch(url.to_owned(), Some(body)).await?;
    Ok(value)
}

pub async fn patch_lol_champ_select_legacy_v1_session_my_selection(client: RESTClient, body: LolChampSelectLegacyChampSelectMySelection) -> Result<Value, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-champ-select-legacy/v1/session/my-selection";
    let value = client.patch(url.to_owned(), Some(body)).await?;
    Ok(value)
}

pub async fn patch_lol_champ_select_v1_session_actions_by_id(client: RESTClient, id: i64, body: LolChampSelectChampSelectAction) -> Result<Value, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-champ-select/v1/session/actions/{{id}}", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.patch(url.to_owned(), Some(body)).await?;
    Ok(value)
}

pub async fn patch_lol_champ_select_v1_session_my_selection(client: RESTClient, body: LolChampSelectChampSelectMySelection) -> Result<Value, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-champ-select/v1/session/my-selection";
    let value = client.patch(url.to_owned(), Some(body)).await?;
    Ok(value)
}

pub async fn patch_lol_cosmetics_v1_recent_by_type(client: RESTClient, type_: String, content_ids: Vec<String>) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-cosmetics/v1/recent/{{type_}}?contentIds={{content_ids}}", &json!({"type_": type_, "content_ids": content_ids}))?;
    let url = format!("{}", template_url);
    let value = client.patch(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn patch_lol_game_settings_v1_game_settings(client: RESTClient, settings_resource: Value) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-game-settings/v1/game-settings?settingsResource={{settings_resource}}", &json!({"settings_resource": settings_resource}))?;
    let url = format!("{}", template_url);
    let value = client.patch(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn patch_lol_game_settings_v1_input_settings(client: RESTClient, settings_resource: Value) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-game-settings/v1/input-settings?settingsResource={{settings_resource}}", &json!({"settings_resource": settings_resource}))?;
    let url = format!("{}", template_url);
    let value = client.patch(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn patch_lol_loadouts_v4_loadouts_by_id(client: RESTClient, id: String, body: LolLoadoutsUpdateLoadoutDTO) -> Result<LolLoadoutsScopedLoadout, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-loadouts/v4/loadouts/{{id}}", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.patch(url.to_owned(), Some(body)).await?;
    Ok(serde_json::from_value::<LolLoadoutsScopedLoadout>(value)?)
}

pub async fn patch_lol_lobby_team_builder_champ_select_v1_session_actions_by_id(client: RESTClient, id: i32, body: LolLobbyTeamBuilderChampSelectAction) -> Result<Value, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-lobby-team-builder/champ-select/v1/session/actions/{{id}}", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.patch(url.to_owned(), Some(body)).await?;
    Ok(value)
}

pub async fn patch_lol_lobby_team_builder_champ_select_v1_session_my_selection(client: RESTClient, body: LolLobbyTeamBuilderChampSelectMySelection) -> Result<Value, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-lobby-team-builder/champ-select/v1/session/my-selection";
    let value = client.patch(url.to_owned(), Some(body)).await?;
    Ok(value)
}

pub async fn patch_lol_npe_tutorial_path_v1_tutorials_init(client: RESTClient) -> Result<(), Box<dyn Error>> {
    let url = "/lol-npe-tutorial-path/v1/tutorials/init";
    client.patch(url.to_owned(), None).await?;
    Ok(())
}

pub async fn patch_lol_rewards_v1_grants_by_grant_id_view(client: RESTClient, grant_id: String) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-rewards/v1/grants/{{grant_id}}/view", &json!({"grant_id": grant_id}))?;
    let url = format!("{}", template_url);
    client.patch(url.to_owned(), None).await?;
    Ok(())
}

pub async fn patch_lol_settings_v1_account_by_category(client: RESTClient, category: String, body: LolSettingsSettingCategory) -> Result<Value, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-settings/v1/account/{{category}}", &json!({"category": category}))?;
    let url = format!("{}", template_url);
    let value = client.patch(url.to_owned(), Some(body)).await?;
    Ok(value)
}

pub async fn patch_lol_settings_v1_local_by_category(client: RESTClient, category: String, body: LolSettingsSettingCategory) -> Result<Value, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-settings/v1/local/{{category}}", &json!({"category": category}))?;
    let url = format!("{}", template_url);
    let value = client.patch(url.to_owned(), Some(body)).await?;
    Ok(value)
}

pub async fn patch_lol_settings_v2_account_by_pp_type_by_category(client: RESTClient, pp_type: String, category: String, body: LolSettingsSettingCategory) -> Result<Value, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-settings/v2/account/{ppType}/{{category}}", &json!({"pp_type": pp_type, "category": category}))?;
    let url = format!("{}", template_url);
    let value = client.patch(url.to_owned(), Some(body)).await?;
    Ok(value)
}

pub async fn patch_lol_settings_v2_local_by_category(client: RESTClient, category: String, body: LolSettingsSettingCategory) -> Result<Value, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-settings/v2/local/{{category}}", &json!({"category": category}))?;
    let url = format!("{}", template_url);
    let value = client.patch(url.to_owned(), Some(body)).await?;
    Ok(value)
}

pub async fn patch_patcher_v1_p2p_status(client: RESTClient, body: PatcherP2PStatusUpdate) -> Result<Value, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/patcher/v1/p2p/status";
    let value = client.patch(url.to_owned(), Some(body)).await?;
    Ok(value)
}

pub async fn patch_telemetry_v3_slis_add_bool_diagnostic(client: RESTClient, body: SLIBoolDiagnostic) -> Result<(), Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/telemetry/v3/slis/add-bool-diagnostic";
    client.patch(url.to_owned(), Some(body)).await?;
    Ok(())
}

pub async fn patch_telemetry_v3_slis_add_double_diagnostic(client: RESTClient, body: SLIDoubleDiagnostic) -> Result<(), Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/telemetry/v3/slis/add-double-diagnostic";
    client.patch(url.to_owned(), Some(body)).await?;
    Ok(())
}

pub async fn patch_telemetry_v3_slis_add_int_diagnostic(client: RESTClient, body: SLIIntDiagnostic) -> Result<(), Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/telemetry/v3/slis/add-int-diagnostic";
    client.patch(url.to_owned(), Some(body)).await?;
    Ok(())
}

pub async fn patch_telemetry_v3_slis_add_label(client: RESTClient, body: SLILabel) -> Result<(), Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/telemetry/v3/slis/add-label";
    client.patch(url.to_owned(), Some(body)).await?;
    Ok(())
}

pub async fn patch_telemetry_v3_slis_add_string_diagnostic(client: RESTClient, body: SLIStringDiagnostic) -> Result<(), Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/telemetry/v3/slis/add-string-diagnostic";
    client.patch(url.to_owned(), Some(body)).await?;
    Ok(())
}

pub async fn post_cookie_jar_v1_cookies(client: RESTClient, cookie: Vec<Cookie>) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/cookie-jar/v1/cookies?cookie={{cookie}}", &json!({"cookie": cookie}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_data_store_v1_install_settings_by_path(client: RESTClient, path: String, data: Value) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/data-store/v1/install-settings/{+path}?path={{path}}&data={{data}}", &json!({"path": path, "data": data}))?;
    let url = format!("{}", template_url);
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_lol_account_verification_v1_authenticate(client: RESTClient, body: LolAccountVerificationAuthenticateRequest) -> Result<LolAccountVerificationAuthenticateResponse, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-account-verification/v1/authenticate";
    let value = client.post(url.to_owned(), Some(body)).await?;
    Ok(serde_json::from_value::<LolAccountVerificationAuthenticateResponse>(value)?)
}

pub async fn post_lol_account_verification_v1_confirm_activation_pin(client: RESTClient, body: LolAccountVerificationConfirmActivationPinRequest) -> Result<(), Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-account-verification/v1/confirmActivationPin";
    client.post(url.to_owned(), Some(body)).await?;
    Ok(())
}

pub async fn post_lol_account_verification_v1_confirm_deactivation_pin(client: RESTClient, body: LolAccountVerificationConfirmDeactivationPinRequest) -> Result<(), Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-account-verification/v1/confirmDeactivationPin";
    client.post(url.to_owned(), Some(body)).await?;
    Ok(())
}

pub async fn post_lol_account_verification_v1_invalidate(client: RESTClient) -> Result<LolAccountVerificationInvalidateResponse, Box<dyn Error>> {
    let url = "/lol-account-verification/v1/invalidate";
    let value = client.post(url.to_owned(), None).await?;
    Ok(serde_json::from_value::<LolAccountVerificationInvalidateResponse>(value)?)
}

pub async fn post_lol_account_verification_v1_send_activation_pin(client: RESTClient, body: LolAccountVerificationSendActivationPinRequest) -> Result<(), Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-account-verification/v1/sendActivationPin";
    client.post(url.to_owned(), Some(body)).await?;
    Ok(())
}

pub async fn post_lol_account_verification_v1_send_deactivation_pin(client: RESTClient, send_deactivation_pin_request: String) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-account-verification/v1/sendDeactivationPin?SendDeactivationPinRequest={{send_deactivation_pin_request}}", &json!({"send_deactivation_pin_request": send_deactivation_pin_request}))?;
    let url = format!("{}", template_url);
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_lol_account_verification_v1_send_token(client: RESTClient, body: LolAccountVerificationSendTokenRequest) -> Result<LolAccountVerificationSendTokenResponse, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-account-verification/v1/send-token";
    let value = client.post(url.to_owned(), Some(body)).await?;
    Ok(serde_json::from_value::<LolAccountVerificationSendTokenResponse>(value)?)
}

pub async fn post_lol_account_verification_v1_verify(client: RESTClient, body: LolAccountVerificationVerifyRequest) -> Result<LolAccountVerificationVerifyResponse, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-account-verification/v1/verify";
    let value = client.post(url.to_owned(), Some(body)).await?;
    Ok(serde_json::from_value::<LolAccountVerificationVerifyResponse>(value)?)
}

pub async fn post_lol_career_stats_v1_champion_stats_percentiles(client: RESTClient, body: Vec<LolCareerStatsStatsQueryRequest>) -> Result<Vec<LolCareerStatsStatisticsPercentilesResponse>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-career-stats/v1/champion-stats-percentiles?body={{body}}", &json!({"body": body}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(serde_json::from_value::<Vec<LolCareerStatsStatisticsPercentilesResponse>>(value)?)
}

pub async fn post_lol_career_stats_v1_position_stats_percentiles(client: RESTClient, body: Vec<LolCareerStatsPositionStatsQueryRequest>) -> Result<Vec<LolCareerStatsStatisticsPercentilesResponse>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-career-stats/v1/position-stats-percentiles?body={{body}}", &json!({"body": body}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(serde_json::from_value::<Vec<LolCareerStatsStatisticsPercentilesResponse>>(value)?)
}

pub async fn post_lol_challenges_v1_ack_challenge_update_by_id(client: RESTClient, id: u64) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-challenges/v1/ack-challenge-update/{{id}}", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_lol_challenges_v1_update_player_preferences(client: RESTClient, body: LolChallengesChallengesPlayerPreferences) -> Result<(), Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-challenges/v1/update-player-preferences";
    client.post(url.to_owned(), Some(body)).await?;
    Ok(())
}

pub async fn post_lol_champ_select_legacy_v1_battle_training_launch(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/lol-champ-select-legacy/v1/battle-training/launch";
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_champ_select_legacy_v1_session_actions_by_id_complete(client: RESTClient, id: i64) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-champ-select-legacy/v1/session/actions/{{id}}/complete", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_champ_select_legacy_v1_session_my_selection_reroll(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/lol-champ-select-legacy/v1/session/my-selection/reroll";
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_champ_select_legacy_v1_session_trades_by_id_accept(client: RESTClient, id: i64) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-champ-select-legacy/v1/session/trades/{{id}}/accept", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_champ_select_legacy_v1_session_trades_by_id_cancel(client: RESTClient, id: i64) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-champ-select-legacy/v1/session/trades/{{id}}/cancel", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_champ_select_legacy_v1_session_trades_by_id_decline(client: RESTClient, id: i64) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-champ-select-legacy/v1/session/trades/{{id}}/decline", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_champ_select_legacy_v1_session_trades_by_id_request(client: RESTClient, id: i64) -> Result<LolChampSelectLegacyChampSelectTradeContract, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-champ-select-legacy/v1/session/trades/{{id}}/request", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(serde_json::from_value::<LolChampSelectLegacyChampSelectTradeContract>(value)?)
}

pub async fn post_lol_champ_select_v1_battle_training_launch(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/lol-champ-select/v1/battle-training/launch";
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_champ_select_v1_ongoing_swap_by_id_clear(client: RESTClient, id: i64) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-champ-select/v1/ongoing-swap/{{id}}/clear", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_champ_select_v1_ongoing_trade_by_id_clear(client: RESTClient, id: i64) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-champ-select/v1/ongoing-trade/{{id}}/clear", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_champ_select_v1_retrieve_latest_game_dto(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/lol-champ-select/v1/retrieve-latest-game-dto";
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_champ_select_v1_session_actions_by_id_complete(client: RESTClient, id: i64) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-champ-select/v1/session/actions/{{id}}/complete", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_champ_select_v1_session_bench_swap_by_champion_id(client: RESTClient, champion_id: i32) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-champ-select/v1/session/bench/swap/{{champion_id}}", &json!({"champion_id": champion_id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_champ_select_v1_session_my_selection_reroll(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/lol-champ-select/v1/session/my-selection/reroll";
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_champ_select_v1_session_simple_inventory(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/lol-champ-select/v1/session/simple-inventory";
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_champ_select_v1_session_swaps_by_id_accept(client: RESTClient, id: i64) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-champ-select/v1/session/swaps/{{id}}/accept", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_champ_select_v1_session_swaps_by_id_cancel(client: RESTClient, id: i64) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-champ-select/v1/session/swaps/{{id}}/cancel", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_champ_select_v1_session_swaps_by_id_decline(client: RESTClient, id: i64) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-champ-select/v1/session/swaps/{{id}}/decline", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_champ_select_v1_session_swaps_by_id_request(client: RESTClient, id: i64) -> Result<LolChampSelectChampSelectSwapContract, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-champ-select/v1/session/swaps/{{id}}/request", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(serde_json::from_value::<LolChampSelectChampSelectSwapContract>(value)?)
}

pub async fn post_lol_champ_select_v1_session_trades_by_id_accept(client: RESTClient, id: i64) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-champ-select/v1/session/trades/{{id}}/accept", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_champ_select_v1_session_trades_by_id_cancel(client: RESTClient, id: i64) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-champ-select/v1/session/trades/{{id}}/cancel", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_champ_select_v1_session_trades_by_id_decline(client: RESTClient, id: i64) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-champ-select/v1/session/trades/{{id}}/decline", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_champ_select_v1_session_trades_by_id_request(client: RESTClient, id: i64) -> Result<LolChampSelectChampSelectTradeContract, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-champ-select/v1/session/trades/{{id}}/request", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(serde_json::from_value::<LolChampSelectChampSelectTradeContract>(value)?)
}

pub async fn post_lol_champ_select_v1_team_boost_purchase(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/lol-champ-select/v1/team-boost/purchase";
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_champ_select_v1_toggle_favorite_by_champion_id_by_position(client: RESTClient, champion_id: i64, position: String) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-champ-select/v1/toggle-favorite/{championId}/{{position}}", &json!({"champion_id": champion_id, "position": position}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_champ_select_v1_toggle_player_muted(client: RESTClient, body: LolChampSelectMutedPlayerInfo) -> Result<Value, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-champ-select/v1/toggle-player-muted";
    let value = client.post(url.to_owned(), Some(body)).await?;
    Ok(value)
}

pub async fn post_lol_chat_v1_blocked_players(client: RESTClient, body: LolChatBlockedPlayerResource) -> Result<Value, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-chat/v1/blocked-players";
    let value = client.post(url.to_owned(), Some(body)).await?;
    Ok(value)
}

pub async fn post_lol_chat_v1_conversations(client: RESTClient, body: LolChatConversationResource) -> Result<LolChatConversationResource, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-chat/v1/conversations";
    let value = client.post(url.to_owned(), Some(body)).await?;
    Ok(serde_json::from_value::<LolChatConversationResource>(value)?)
}

pub async fn post_lol_chat_v1_conversations_by_id_closed(client: RESTClient, id: String) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-chat/v1/conversations/{{id}}/closed", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_chat_v1_conversations_by_id_messages(client: RESTClient, id: String, body: LolChatConversationMessageResource) -> Result<LolChatConversationMessageResource, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-chat/v1/conversations/{{id}}/messages", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), Some(body)).await?;
    Ok(serde_json::from_value::<LolChatConversationMessageResource>(value)?)
}

pub async fn post_lol_chat_v1_conversations_by_id_participants(client: RESTClient, id: String, body: LolChatUserResource) -> Result<Value, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-chat/v1/conversations/{{id}}/participants", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), Some(body)).await?;
    Ok(value)
}

pub async fn post_lol_chat_v1_conversations_eog_chat_toggle(client: RESTClient, is_toggled_on: bool) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-chat/v1/conversations/eog-chat-toggle?isToggledOn={{is_toggled_on}}", &json!({"is_toggled_on": is_toggled_on}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_chat_v1_friend_groups(client: RESTClient, body: LolChatGroupResource) -> Result<Value, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-chat/v1/friend-groups";
    let value = client.post(url.to_owned(), Some(body)).await?;
    Ok(value)
}

pub async fn post_lol_chat_v1_friend_requests(client: RESTClient, body: LolChatFriendRequestResource) -> Result<Value, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-chat/v1/friend-requests";
    let value = client.post(url.to_owned(), Some(body)).await?;
    Ok(value)
}

pub async fn post_lol_chat_v1_player_mutes(client: RESTClient, body: LolChatPlayerMuteUpdate) -> Result<Value, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-chat/v1/player-mutes";
    let value = client.post(url.to_owned(), Some(body)).await?;
    Ok(value)
}

pub async fn post_lol_chat_v1_session_rso(client: RESTClient, body: LolChatAuthResourceRsoAccessToken) -> Result<LolChatSessionResource, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-chat/v1/session/rso";
    let value = client.post(url.to_owned(), Some(body)).await?;
    Ok(serde_json::from_value::<LolChatSessionResource>(value)?)
}

pub async fn post_lol_chat_v1_system_mutes(client: RESTClient, body: LolChatPlayerMuteUpdate) -> Result<Value, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-chat/v1/system-mutes";
    let value = client.post(url.to_owned(), Some(body)).await?;
    Ok(value)
}

pub async fn post_lol_clash_v1_eog_player_update_acknowledge(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/lol-clash/v1/eog-player-update/acknowledge";
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_clash_v1_events(client: RESTClient, uuids: Vec<String>) -> Result<HashMap<String, ClashEventData>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-clash/v1/events?uuids={{uuids}}", &json!({"uuids": uuids}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(serde_json::from_value::<HashMap<String, ClashEventData>>(value)?)
}

pub async fn post_lol_clash_v1_game_end_acknowledge(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/lol-clash/v1/game-end/acknowledge";
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_clash_v1_lft_player(client: RESTClient, body: LolClashLftState) -> Result<Value, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-clash/v1/lft/player";
    let value = client.post(url.to_owned(), Some(body)).await?;
    Ok(value)
}

pub async fn post_lol_clash_v1_lft_player_find(client: RESTClient, body: LolClashFindPlayers) -> Result<Vec<PlayerFinderDTO>, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-clash/v1/lft/player/find";
    let value = client.post(url.to_owned(), Some(body)).await?;
    Ok(serde_json::from_value::<Vec<PlayerFinderDTO>>(value)?)
}

pub async fn post_lol_clash_v1_lft_team(client: RESTClient, body: LolClashTeamOpenState) -> Result<Value, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-clash/v1/lft/team";
    let value = client.post(url.to_owned(), Some(body)).await?;
    Ok(value)
}

pub async fn post_lol_clash_v1_lft_team_by_roster_id_request(client: RESTClient, roster_id: String) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-clash/v1/lft/team/{{roster_id}}/request", &json!({"roster_id": roster_id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_clash_v1_lft_team_fetch_requests(client: RESTClient, tournament_id: i64) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-clash/v1/lft/team/fetch-requests?tournamentId={{tournament_id}}", &json!({"tournament_id": tournament_id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_clash_v1_lft_team_find(client: RESTClient, body: LolClashFindTeams) -> Result<Vec<OpenedTeamDTO>, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-clash/v1/lft/team/find";
    let value = client.post(url.to_owned(), Some(body)).await?;
    Ok(serde_json::from_value::<Vec<OpenedTeamDTO>>(value)?)
}

pub async fn post_lol_clash_v1_notifications_acknowledge(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/lol-clash/v1/notifications/acknowledge";
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_clash_v1_refresh(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/lol-clash/v1/refresh";
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_clash_v1_roster_by_roster_id_accept(client: RESTClient, roster_id: String) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-clash/v1/roster/{{roster_id}}/accept", &json!({"roster_id": roster_id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_clash_v1_roster_by_roster_id_cancel_withdraw(client: RESTClient, roster_id: String) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-clash/v1/roster/{{roster_id}}/cancel-withdraw", &json!({"roster_id": roster_id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_clash_v1_roster_by_roster_id_change_all_details(client: RESTClient, roster_id: String, body: LolClashRosterDetails) -> Result<Value, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-clash/v1/roster/{{roster_id}}/change-all-details", &json!({"roster_id": roster_id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), Some(body)).await?;
    Ok(value)
}

pub async fn post_lol_clash_v1_roster_by_roster_id_change_icon(client: RESTClient, roster_id: String, body: LolClashChangeIconRequest) -> Result<Value, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-clash/v1/roster/{{roster_id}}/change-icon", &json!({"roster_id": roster_id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), Some(body)).await?;
    Ok(value)
}

pub async fn post_lol_clash_v1_roster_by_roster_id_change_name(client: RESTClient, roster_id: String, body: LolClashChangeNameRequest) -> Result<Value, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-clash/v1/roster/{{roster_id}}/change-name", &json!({"roster_id": roster_id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), Some(body)).await?;
    Ok(value)
}

pub async fn post_lol_clash_v1_roster_by_roster_id_change_short_name(client: RESTClient, roster_id: String, body: LolClashChangeNameRequest) -> Result<Value, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-clash/v1/roster/{{roster_id}}/change-short-name", &json!({"roster_id": roster_id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), Some(body)).await?;
    Ok(value)
}

pub async fn post_lol_clash_v1_roster_by_roster_id_decline(client: RESTClient, roster_id: String) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-clash/v1/roster/{{roster_id}}/decline", &json!({"roster_id": roster_id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_clash_v1_roster_by_roster_id_disband(client: RESTClient, roster_id: String) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-clash/v1/roster/{{roster_id}}/disband", &json!({"roster_id": roster_id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_clash_v1_roster_by_roster_id_invite(client: RESTClient, roster_id: String, summoner_ids: Vec<u64>) -> Result<Vec<LolClashClientFailedInvite>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-clash/v1/roster/{{roster_id}}/invite?summonerIds={{summoner_ids}}", &json!({"roster_id": roster_id, "summoner_ids": summoner_ids}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(serde_json::from_value::<Vec<LolClashClientFailedInvite>>(value)?)
}

pub async fn post_lol_clash_v1_roster_by_roster_id_kick(client: RESTClient, roster_id: String, body: LolClashKickRequest) -> Result<Value, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-clash/v1/roster/{{roster_id}}/kick", &json!({"roster_id": roster_id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), Some(body)).await?;
    Ok(value)
}

pub async fn post_lol_clash_v1_roster_by_roster_id_leave(client: RESTClient, roster_id: String) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-clash/v1/roster/{{roster_id}}/leave", &json!({"roster_id": roster_id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_clash_v1_roster_by_roster_id_lockin(client: RESTClient, roster_id: String) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-clash/v1/roster/{{roster_id}}/lockin", &json!({"roster_id": roster_id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_clash_v1_roster_by_roster_id_set_position(client: RESTClient, roster_id: String, body: LolClashSetPositionRequest) -> Result<Value, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-clash/v1/roster/{{roster_id}}/set-position", &json!({"roster_id": roster_id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), Some(body)).await?;
    Ok(value)
}

pub async fn post_lol_clash_v1_roster_by_roster_id_set_ticket(client: RESTClient, roster_id: String, body: LolClashSetTicketRequest) -> Result<Value, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-clash/v1/roster/{{roster_id}}/set-ticket", &json!({"roster_id": roster_id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), Some(body)).await?;
    Ok(value)
}

pub async fn post_lol_clash_v1_roster_by_roster_id_suggest(client: RESTClient, roster_id: String, summoner_ids: Vec<u64>) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-clash/v1/roster/{{roster_id}}/suggest?summonerIds={{summoner_ids}}", &json!({"roster_id": roster_id, "summoner_ids": summoner_ids}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_clash_v1_roster_by_roster_id_suggest_by_summoner_id_accept(client: RESTClient, roster_id: String, summoner_id: u64) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-clash/v1/roster/{rosterId}/suggest/{{summoner_id}}/accept", &json!({"roster_id": roster_id, "summoner_id": summoner_id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_clash_v1_roster_by_roster_id_suggest_by_summoner_id_decline(client: RESTClient, roster_id: String, summoner_id: u64) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-clash/v1/roster/{rosterId}/suggest/{{summoner_id}}/decline", &json!({"roster_id": roster_id, "summoner_id": summoner_id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_clash_v1_roster_by_roster_id_suggest_by_summoner_id_revoke(client: RESTClient, roster_id: String, summoner_id: u64) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-clash/v1/roster/{rosterId}/suggest/{{summoner_id}}/revoke", &json!({"roster_id": roster_id, "summoner_id": summoner_id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_clash_v1_roster_by_roster_id_ticket_offer_by_summoner_id_accept(client: RESTClient, roster_id: String, summoner_id: u64) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-clash/v1/roster/{rosterId}/ticket-offer/{{summoner_id}}/accept", &json!({"roster_id": roster_id, "summoner_id": summoner_id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_clash_v1_roster_by_roster_id_ticket_offer_by_summoner_id_decline(client: RESTClient, roster_id: String, summoner_id: u64) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-clash/v1/roster/{rosterId}/ticket-offer/{{summoner_id}}/decline", &json!({"roster_id": roster_id, "summoner_id": summoner_id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_clash_v1_roster_by_roster_id_ticket_offer_by_summoner_id_offer(client: RESTClient, roster_id: String, summoner_id: u64, body: LolClashOfferTicketRequest) -> Result<Value, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-clash/v1/roster/{rosterId}/ticket-offer/{{summoner_id}}/offer", &json!({"roster_id": roster_id, "summoner_id": summoner_id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), Some(body)).await?;
    Ok(value)
}

pub async fn post_lol_clash_v1_roster_by_roster_id_ticket_offer_by_summoner_id_revoke(client: RESTClient, roster_id: String, summoner_id: u64) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-clash/v1/roster/{rosterId}/ticket-offer/{{summoner_id}}/revoke", &json!({"roster_id": roster_id, "summoner_id": summoner_id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_clash_v1_roster_by_roster_id_transfer_captain(client: RESTClient, roster_id: String, new_captain_summoner_id: u64) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-clash/v1/roster/{{roster_id}}/transfer-captain?newCaptainSummonerId={{new_captain_summoner_id}}", &json!({"roster_id": roster_id, "new_captain_summoner_id": new_captain_summoner_id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_clash_v1_roster_by_roster_id_unlockin(client: RESTClient, roster_id: String) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-clash/v1/roster/{{roster_id}}/unlockin", &json!({"roster_id": roster_id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_clash_v1_roster_by_roster_id_unwithdraw(client: RESTClient, roster_id: String) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-clash/v1/roster/{{roster_id}}/unwithdraw", &json!({"roster_id": roster_id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_clash_v1_roster_by_roster_id_update_logos(client: RESTClient, roster_id: String) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-clash/v1/roster/{{roster_id}}/update-logos", &json!({"roster_id": roster_id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_clash_v1_roster_by_roster_id_withdraw(client: RESTClient, roster_id: String) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-clash/v1/roster/{{roster_id}}/withdraw", &json!({"roster_id": roster_id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_clash_v1_simple_state_flags_by_id_acknowledge(client: RESTClient, id: String) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-clash/v1/simple-state-flags/{{id}}/acknowledge", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_clash_v1_tournament_by_tournament_id_create_roster(client: RESTClient, tournament_id: i64, body: LolClashRosterDetails) -> Result<Value, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-clash/v1/tournament/{{tournament_id}}/create-roster", &json!({"tournament_id": tournament_id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), Some(body)).await?;
    Ok(value)
}

pub async fn post_lol_clash_v1_update_logos(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/lol-clash/v1/update-logos";
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_clash_v1_voice(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/lol-clash/v1/voice";
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_clash_v1_voice_delay_by_delay_seconds(client: RESTClient, delay_seconds: f64) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-clash/v1/voice-delay/{{delay_seconds}}", &json!({"delay_seconds": delay_seconds}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_email_verification_v1_confirm_email(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/lol-email-verification/v1/confirm-email";
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_end_of_game_v1_gameclient_eog_stats_block(client: RESTClient, body: LolEndOfGameGameClientEndOfGameStats) -> Result<Value, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-end-of-game/v1/gameclient-eog-stats-block";
    let value = client.post(url.to_owned(), Some(body)).await?;
    Ok(value)
}

pub async fn post_lol_end_of_game_v1_state_dismiss_stats(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/lol-end-of-game/v1/state/dismiss-stats";
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_end_of_game_v2_player_complaints(client: RESTClient, body: LolEndOfGameEndOfGamePlayerComplaintV2) -> Result<LolEndOfGameEndOfGamePlayerComplaintV2, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-end-of-game/v2/player-complaints";
    let value = client.post(url.to_owned(), Some(body)).await?;
    Ok(serde_json::from_value::<LolEndOfGameEndOfGamePlayerComplaintV2>(value)?)
}

pub async fn post_lol_esport_stream_notifications_v1_send_stats(client: RESTClient, event_type: String, match_id: String) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-esport-stream-notifications/v1/send-stats?eventType={{event_type}}&matchId={{match_id}}", &json!({"event_type": event_type, "match_id": match_id}))?;
    let url = format!("{}", template_url);
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_lol_event_shop_v1_claim_select_all(client: RESTClient) -> Result<(), Box<dyn Error>> {
    let url = "/lol-event-shop/v1/claim-select-all";
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_lol_event_shop_v1_claim_select_bonus_iteration(client: RESTClient) -> Result<(), Box<dyn Error>> {
    let url = "/lol-event-shop/v1/claim-select-bonus-iteration";
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_lol_event_shop_v1_lazy_load_data(client: RESTClient) -> Result<(), Box<dyn Error>> {
    let url = "/lol-event-shop/v1/lazy-load-data";
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_lol_event_shop_v1_purchase_offer(client: RESTClient, body: LolEventShopPurchaseOfferRequest) -> Result<LolEventShopPurchaseOfferResponseV3, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-event-shop/v1/purchase-offer";
    let value = client.post(url.to_owned(), Some(body)).await?;
    Ok(serde_json::from_value::<LolEventShopPurchaseOfferResponseV3>(value)?)
}

pub async fn post_lol_game_client_chat_v1_instant_messages(client: RESTClient, summoner_name: String, message: String) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-game-client-chat/v1/instant-messages?summonerName={{summoner_name}}&message={{message}}", &json!({"summoner_name": summoner_name, "message": message}))?;
    let url = format!("{}", template_url);
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_lol_game_client_chat_v1_party_messages(client: RESTClient, message: String) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-game-client-chat/v1/party-messages?message={{message}}", &json!({"message": message}))?;
    let url = format!("{}", template_url);
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_lol_game_settings_v1_reload_post_game(client: RESTClient) -> Result<(), Box<dyn Error>> {
    let url = "/lol-game-settings/v1/reload-post-game";
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_lol_game_settings_v1_save(client: RESTClient) -> Result<bool, Box<dyn Error>> {
    let url = "/lol-game-settings/v1/save";
    let value = client.post(url.to_owned(), None).await?;
    Ok(serde_json::from_value::<bool>(value)?)
}

pub async fn post_lol_gameflow_v1_ack_failed_to_launch(client: RESTClient) -> Result<(), Box<dyn Error>> {
    let url = "/lol-gameflow/v1/ack-failed-to-launch";
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_lol_gameflow_v1_basic_tutorial_start(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/lol-gameflow/v1/basic-tutorial/start";
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_gameflow_v1_battle_training_start(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/lol-gameflow/v1/battle-training/start";
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_gameflow_v1_battle_training_stop(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/lol-gameflow/v1/battle-training/stop";
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_gameflow_v1_client_received_message(client: RESTClient, messsage: String) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-gameflow/v1/client-received-message?messsage={{messsage}}", &json!({"messsage": messsage}))?;
    let url = format!("{}", template_url);
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_lol_gameflow_v1_early_exit(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/lol-gameflow/v1/early-exit";
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_gameflow_v1_extra_game_client_args(client: RESTClient, extra_game_client_args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-gameflow/v1/extra-game-client-args?extraGameClientArgs={{extra_game_client_args}}", &json!({"extra_game_client_args": extra_game_client_args}))?;
    let url = format!("{}", template_url);
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_lol_gameflow_v1_gameflow_metadata_player_status(client: RESTClient, body: LolGameflowPlayerStatus) -> Result<(), Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-gameflow/v1/gameflow-metadata/player-status";
    client.post(url.to_owned(), Some(body)).await?;
    Ok(())
}

pub async fn post_lol_gameflow_v1_gameflow_metadata_registration_status(client: RESTClient, body: LolGameflowRegistrationStatus) -> Result<(), Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-gameflow/v1/gameflow-metadata/registration-status";
    client.post(url.to_owned(), Some(body)).await?;
    Ok(())
}

pub async fn post_lol_gameflow_v1_pre_end_game_transition(client: RESTClient, enabled: bool) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-gameflow/v1/pre-end-game-transition?enabled={{enabled}}", &json!({"enabled": enabled}))?;
    let url = format!("{}", template_url);
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_lol_gameflow_v1_reconnect(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/lol-gameflow/v1/reconnect";
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_gameflow_v1_session_dodge(client: RESTClient, body: LolGameflowGameflowGameDodge) -> Result<(), Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-gameflow/v1/session/dodge";
    client.post(url.to_owned(), Some(body)).await?;
    Ok(())
}

pub async fn post_lol_gameflow_v1_session_event(client: RESTClient, session: String) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-gameflow/v1/session/event?session={{session}}", &json!({"session": session}))?;
    let url = format!("{}", template_url);
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_lol_gameflow_v1_session_game_configuration(client: RESTClient, body: LolGameflowQueue) -> Result<(), Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-gameflow/v1/session/game-configuration";
    client.post(url.to_owned(), Some(body)).await?;
    Ok(())
}

pub async fn post_lol_gameflow_v1_session_request_enter_gameflow(client: RESTClient, event_type: String) -> Result<bool, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-gameflow/v1/session/request-enter-gameflow?eventType={{event_type}}", &json!({"event_type": event_type}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(serde_json::from_value::<bool>(value)?)
}

pub async fn post_lol_gameflow_v1_session_request_lobby(client: RESTClient) -> Result<bool, Box<dyn Error>> {
    let url = "/lol-gameflow/v1/session/request-lobby";
    let value = client.post(url.to_owned(), None).await?;
    Ok(serde_json::from_value::<bool>(value)?)
}

pub async fn post_lol_gameflow_v1_session_request_tournament_checkin(client: RESTClient) -> Result<bool, Box<dyn Error>> {
    let url = "/lol-gameflow/v1/session/request-tournament-checkin";
    let value = client.post(url.to_owned(), None).await?;
    Ok(serde_json::from_value::<bool>(value)?)
}

pub async fn post_lol_gameflow_v1_session_tournament_ended(client: RESTClient) -> Result<(), Box<dyn Error>> {
    let url = "/lol-gameflow/v1/session/tournament-ended";
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_lol_gameflow_v1_spectate_launch(client: RESTClient, target_summoner_name: Option<String>) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-gameflow/v1/spectate/launch?targetSummonerName={{target_summoner_name}}", &json!({"target_summoner_name": target_summoner_name}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_gameflow_v1_spectate_quit(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/lol-gameflow/v1/spectate/quit";
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_gameflow_v1_tick(client: RESTClient) -> Result<(), Box<dyn Error>> {
    let url = "/lol-gameflow/v1/tick";
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_lol_gameflow_v1_watch_launch(client: RESTClient, args: Vec<String>) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-gameflow/v1/watch/launch?args={{args}}", &json!({"args": args}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_gameflow_v2_spectate_launch(client: RESTClient, body: LolGameflowSpectateGameInfoResource) -> Result<Value, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-gameflow/v2/spectate/launch";
    let value = client.post(url.to_owned(), Some(body)).await?;
    Ok(value)
}

pub async fn post_lol_highlights_v1_file_browser_by_highlight_id(client: RESTClient, highlight_id: u64) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-highlights/v1/file-browser/{{highlight_id}}", &json!({"highlight_id": highlight_id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_highlights_v1_highlights(client: RESTClient) -> Result<Vec<LolHighlightsHighlight>, Box<dyn Error>> {
    let url = "/lol-highlights/v1/highlights";
    let value = client.post(url.to_owned(), None).await?;
    Ok(serde_json::from_value::<Vec<LolHighlightsHighlight>>(value)?)
}

pub async fn post_lol_honeyfruit_v1_vng_publisher_settings(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/lol-honeyfruit/v1/vng-publisher-settings";
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_honor_v2_v1_honor_player(client: RESTClient, body: LolHonorV2ApiHonorPlayerServerRequest) -> Result<String, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-honor-v2/v1/honor-player";
    let value = client.post(url.to_owned(), Some(body)).await?;
    Ok(serde_json::from_value::<String>(value)?)
}

pub async fn post_lol_honor_v2_v1_late_recognition_ack(client: RESTClient) -> Result<(), Box<dyn Error>> {
    let url = "/lol-honor-v2/v1/late-recognition/ack";
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_lol_honor_v2_v1_level_change_ack(client: RESTClient) -> Result<(), Box<dyn Error>> {
    let url = "/lol-honor-v2/v1/level-change/ack";
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_lol_honor_v2_v1_mutual_honor_ack(client: RESTClient) -> Result<(), Box<dyn Error>> {
    let url = "/lol-honor-v2/v1/mutual-honor/ack";
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_lol_honor_v2_v1_reward_granted_ack(client: RESTClient) -> Result<(), Box<dyn Error>> {
    let url = "/lol-honor-v2/v1/reward-granted/ack";
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_lol_inventory_v1_notification_acknowledge(client: RESTClient, id: i64) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-inventory/v1/notification/acknowledge?id={{id}}", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_lol_item_sets_v1_item_sets_by_summoner_id_sets(client: RESTClient, summoner_id: u64, body: LolItemSetsItemSet) -> Result<(), Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-item-sets/v1/item-sets/{{summoner_id}}/sets", &json!({"summoner_id": summoner_id}))?;
    let url = format!("{}", template_url);
    client.post(url.to_owned(), Some(body)).await?;
    Ok(())
}

pub async fn post_lol_item_sets_v1_item_sets_validate(client: RESTClient, body: LolItemSetsValidateItemSetNameInput) -> Result<LolItemSetsValidateItemSetNameResponse, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-item-sets/v1/item-sets/validate";
    let value = client.post(url.to_owned(), Some(body)).await?;
    Ok(serde_json::from_value::<LolItemSetsValidateItemSetNameResponse>(value)?)
}

pub async fn post_lol_kr_shutdown_law_v1_rating_screen_acknowledge(client: RESTClient) -> Result<(), Box<dyn Error>> {
    let url = "/lol-kr-shutdown-law/v1/rating-screen/acknowledge";
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_lol_license_agreement_v1_agreements_by_id_accept(client: RESTClient, id: String) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-license-agreement/v1/agreements/{{id}}/accept", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_license_agreement_v1_agreements_by_id_decline(client: RESTClient, id: String) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-license-agreement/v1/agreements/{{id}}/decline", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_loadouts_v4_loadouts(client: RESTClient, body: LolLoadoutsCreateLoadoutDTO) -> Result<LolLoadoutsScopedLoadout, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-loadouts/v4/loadouts";
    let value = client.post(url.to_owned(), Some(body)).await?;
    Ok(serde_json::from_value::<LolLoadoutsScopedLoadout>(value)?)
}

pub async fn post_lol_lobby_team_builder_champ_select_v1_retrieve_latest_game_dto(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/lol-lobby-team-builder/champ-select/v1/retrieve-latest-game-dto";
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_lobby_team_builder_champ_select_v1_session_actions_by_id_complete(client: RESTClient, id: i32) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-lobby-team-builder/champ-select/v1/session/actions/{{id}}/complete", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_lobby_team_builder_champ_select_v1_session_bench_swap_by_champion_id(client: RESTClient, champion_id: i32) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-lobby-team-builder/champ-select/v1/session/bench/swap/{{champion_id}}", &json!({"champion_id": champion_id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_lobby_team_builder_champ_select_v1_session_my_selection_reroll(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/lol-lobby-team-builder/champ-select/v1/session/my-selection/reroll";
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_lobby_team_builder_champ_select_v1_session_swaps_by_id_accept(client: RESTClient, id: i32) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-lobby-team-builder/champ-select/v1/session/swaps/{{id}}/accept", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_lobby_team_builder_champ_select_v1_session_swaps_by_id_cancel(client: RESTClient, id: i32) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-lobby-team-builder/champ-select/v1/session/swaps/{{id}}/cancel", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_lobby_team_builder_champ_select_v1_session_swaps_by_id_decline(client: RESTClient, id: i32) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-lobby-team-builder/champ-select/v1/session/swaps/{{id}}/decline", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_lobby_team_builder_champ_select_v1_session_swaps_by_id_request(client: RESTClient, id: i32) -> Result<LolLobbyTeamBuilderChampSelectSwapContract, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-lobby-team-builder/champ-select/v1/session/swaps/{{id}}/request", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(serde_json::from_value::<LolLobbyTeamBuilderChampSelectSwapContract>(value)?)
}

pub async fn post_lol_lobby_team_builder_champ_select_v1_session_trades_by_id_accept(client: RESTClient, id: i32) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-lobby-team-builder/champ-select/v1/session/trades/{{id}}/accept", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_lobby_team_builder_champ_select_v1_session_trades_by_id_cancel(client: RESTClient, id: i32) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-lobby-team-builder/champ-select/v1/session/trades/{{id}}/cancel", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_lobby_team_builder_champ_select_v1_session_trades_by_id_decline(client: RESTClient, id: i32) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-lobby-team-builder/champ-select/v1/session/trades/{{id}}/decline", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_lobby_team_builder_champ_select_v1_session_trades_by_id_request(client: RESTClient, id: i32) -> Result<LolLobbyTeamBuilderChampSelectTradeContract, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-lobby-team-builder/champ-select/v1/session/trades/{{id}}/request", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(serde_json::from_value::<LolLobbyTeamBuilderChampSelectTradeContract>(value)?)
}

pub async fn post_lol_lobby_team_builder_champ_select_v1_simple_inventory(client: RESTClient) -> Result<(), Box<dyn Error>> {
    let url = "/lol-lobby-team-builder/champ-select/v1/simple-inventory";
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_lol_lobby_team_builder_champ_select_v1_team_boost_purchase(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/lol-lobby-team-builder/champ-select/v1/team-boost/purchase";
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_lobby_team_builder_v1_ready_check_accept(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/lol-lobby-team-builder/v1/ready-check/accept";
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_lobby_team_builder_v1_ready_check_decline(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/lol-lobby-team-builder/v1/ready-check/decline";
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_lobby_v1_clash(client: RESTClient, token: String) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-lobby/v1/clash?token={{token}}", &json!({"token": token}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_lobby_v1_custom_games_by_id_join(client: RESTClient, id: u64, body: LolLobbyLobbyCustomJoinParameters) -> Result<Value, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-lobby/v1/custom-games/{{id}}/join", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), Some(body)).await?;
    Ok(value)
}

pub async fn post_lol_lobby_v1_custom_games_refresh(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/lol-lobby/v1/custom-games/refresh";
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_lobby_v1_lobby_custom_bots(client: RESTClient, body: LolLobbyLobbyBotParams) -> Result<Value, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-lobby/v1/lobby/custom/bots";
    let value = client.post(url.to_owned(), Some(body)).await?;
    Ok(value)
}

pub async fn post_lol_lobby_v1_lobby_custom_bots_by_summoner_internal_name(client: RESTClient, summoner_internal_name: String, body: LolLobbyLobbyBotParams) -> Result<Value, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-lobby/v1/lobby/custom/bots/{{summoner_internal_name}}", &json!({"summoner_internal_name": summoner_internal_name}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), Some(body)).await?;
    Ok(value)
}

pub async fn post_lol_lobby_v1_lobby_custom_cancel_champ_select(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/lol-lobby/v1/lobby/custom/cancel-champ-select";
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_lobby_v1_lobby_custom_start_champ_select(client: RESTClient) -> Result<LolLobbyLobbyCustomChampSelectStartResponse, Box<dyn Error>> {
    let url = "/lol-lobby/v1/lobby/custom/start-champ-select";
    let value = client.post(url.to_owned(), None).await?;
    Ok(serde_json::from_value::<LolLobbyLobbyCustomChampSelectStartResponse>(value)?)
}

pub async fn post_lol_lobby_v1_lobby_custom_switch_teams(client: RESTClient, team: Option<String>) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-lobby/v1/lobby/custom/switch-teams?team={{team}}", &json!({"team": team}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_lobby_v1_lobby_invitations(client: RESTClient, body: LolLobbyLobbyInvitation) -> Result<LolLobbyLobbyInvitation, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-lobby/v1/lobby/invitations";
    let value = client.post(url.to_owned(), Some(body)).await?;
    Ok(serde_json::from_value::<LolLobbyLobbyInvitation>(value)?)
}

pub async fn post_lol_lobby_v1_tournaments_by_id_join(client: RESTClient, id: String) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-lobby/v1/tournaments/{{id}}/join", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_lobby_v2_eligibility_party(client: RESTClient) -> Result<Vec<LolLobbyEligibility>, Box<dyn Error>> {
    let url = "/lol-lobby/v2/eligibility/party";
    let value = client.post(url.to_owned(), None).await?;
    Ok(serde_json::from_value::<Vec<LolLobbyEligibility>>(value)?)
}

pub async fn post_lol_lobby_v2_eligibility_self(client: RESTClient) -> Result<Vec<LolLobbyEligibility>, Box<dyn Error>> {
    let url = "/lol-lobby/v2/eligibility/self";
    let value = client.post(url.to_owned(), None).await?;
    Ok(serde_json::from_value::<Vec<LolLobbyEligibility>>(value)?)
}

pub async fn post_lol_lobby_v2_eog_invitations(client: RESTClient, invitations: Vec<LolLobbyLobbyInvitationDto>) -> Result<Vec<LolLobbyLobbyInvitationDto>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-lobby/v2/eog-invitations?invitations={{invitations}}", &json!({"invitations": invitations}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(serde_json::from_value::<Vec<LolLobbyLobbyInvitationDto>>(value)?)
}

pub async fn post_lol_lobby_v2_lobby(client: RESTClient, body: LolLobbyLobbyChangeGameDto) -> Result<LolLobbyLobbyDto, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-lobby/v2/lobby";
    let value = client.post(url.to_owned(), Some(body)).await?;
    Ok(serde_json::from_value::<LolLobbyLobbyDto>(value)?)
}

pub async fn post_lol_lobby_v2_lobby_invitations(client: RESTClient, invitations: Vec<LolLobbyLobbyInvitationDto>) -> Result<Vec<LolLobbyLobbyInvitationDto>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-lobby/v2/lobby/invitations?invitations={{invitations}}", &json!({"invitations": invitations}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(serde_json::from_value::<Vec<LolLobbyLobbyInvitationDto>>(value)?)
}

pub async fn post_lol_lobby_v2_lobby_matchmaking_search(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/lol-lobby/v2/lobby/matchmaking/search";
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_lobby_v2_lobby_members_by_summoner_id_grant_invite(client: RESTClient, summoner_id: u64) -> Result<u64, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-lobby/v2/lobby/members/{{summoner_id}}/grant-invite", &json!({"summoner_id": summoner_id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(serde_json::from_value::<u64>(value)?)
}

pub async fn post_lol_lobby_v2_lobby_members_by_summoner_id_kick(client: RESTClient, summoner_id: u64) -> Result<u64, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-lobby/v2/lobby/members/{{summoner_id}}/kick", &json!({"summoner_id": summoner_id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(serde_json::from_value::<u64>(value)?)
}

pub async fn post_lol_lobby_v2_lobby_members_by_summoner_id_promote(client: RESTClient, summoner_id: u64) -> Result<u64, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-lobby/v2/lobby/members/{{summoner_id}}/promote", &json!({"summoner_id": summoner_id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(serde_json::from_value::<u64>(value)?)
}

pub async fn post_lol_lobby_v2_lobby_members_by_summoner_id_revoke_invite(client: RESTClient, summoner_id: u64) -> Result<u64, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-lobby/v2/lobby/members/{{summoner_id}}/revoke-invite", &json!({"summoner_id": summoner_id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(serde_json::from_value::<u64>(value)?)
}

pub async fn post_lol_lobby_v2_lobby_team_by_team(client: RESTClient, team: String) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-lobby/v2/lobby/team/{{team}}", &json!({"team": team}))?;
    let url = format!("{}", template_url);
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_lol_lobby_v2_matchmaking_quick_search(client: RESTClient, body: LolLobbyLobbyChangeGameDto) -> Result<Value, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-lobby/v2/matchmaking/quick-search";
    let value = client.post(url.to_owned(), Some(body)).await?;
    Ok(value)
}

pub async fn post_lol_lobby_v2_notifications(client: RESTClient, body: LolLobbyLobbyNotification) -> Result<(), Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-lobby/v2/notifications";
    client.post(url.to_owned(), Some(body)).await?;
    Ok(())
}

pub async fn post_lol_lobby_v2_parties_overrides_enabled_for_team_builder_queues(client: RESTClient, enabled_for_teambuilder_queues: bool) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-lobby/v2/parties/overrides/EnabledForTeamBuilderQueues?enabledForTeambuilderQueues={{enabled_for_teambuilder_queues}}", &json!({"enabled_for_teambuilder_queues": enabled_for_teambuilder_queues}))?;
    let url = format!("{}", template_url);
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_lol_lobby_v2_party_by_party_id_join(client: RESTClient, party_id: String, body: LolLobbyCustomJoinOptionsDto) -> Result<Value, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-lobby/v2/party/{{party_id}}/join", &json!({"party_id": party_id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), Some(body)).await?;
    Ok(value)
}

pub async fn post_lol_lobby_v2_play_again(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/lol-lobby/v2/play-again";
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_lobby_v2_play_again_decline(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/lol-lobby/v2/play-again-decline";
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_lobby_v2_received_invitations_by_invitation_id_accept(client: RESTClient, invitation_id: String) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-lobby/v2/received-invitations/{{invitation_id}}/accept", &json!({"invitation_id": invitation_id}))?;
    let url = format!("{}", template_url);
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_lol_lobby_v2_received_invitations_by_invitation_id_decline(client: RESTClient, invitation_id: String) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-lobby/v2/received-invitations/{{invitation_id}}/decline", &json!({"invitation_id": invitation_id}))?;
    let url = format!("{}", template_url);
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_lol_login_v1_account_state(client: RESTClient) -> Result<(), Box<dyn Error>> {
    let url = "/lol-login/v1/account-state";
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_lol_login_v1_change_summoner_name(client: RESTClient, name: String) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-login/v1/change-summoner-name?name={{name}}", &json!({"name": name}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_login_v1_delete_rso_on_close(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/lol-login/v1/delete-rso-on-close";
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_login_v1_league_session_status(client: RESTClient, league_session: LolLoginLeagueSessionStatus) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-login/v1/leagueSessionStatus?LeagueSession={{league_session}}", &json!({"league_session": league_session}))?;
    let url = format!("{}", template_url);
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_lol_login_v1_service_proxy_async_requests_by_service_name_by_method_name(client: RESTClient, service_name: String, method_name: String, plugin_id: u32) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-login/v1/service-proxy-async-requests/{serviceName}/{{method_name}}?pluginId={{plugin_id}}", &json!({"service_name": service_name, "method_name": method_name, "plugin_id": plugin_id}))?;
    let url = format!("{}", template_url);
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_lol_login_v1_service_proxy_uuid_requests(client: RESTClient, service_name: String, method_name: String, plugin_id: u32, timeout_millis: u64, payload: String) -> Result<String, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-login/v1/service-proxy-uuid-requests?serviceName={{service_name}}&methodName={{method_name}}&pluginId={{plugin_id}}&timeoutMillis={{timeout_millis}}&payload={{payload}}", &json!({"service_name": service_name, "method_name": method_name, "plugin_id": plugin_id, "timeout_millis": timeout_millis, "payload": payload}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(serde_json::from_value::<String>(value)?)
}

pub async fn post_lol_login_v1_session(client: RESTClient, body: LolLoginUsernameAndPassword) -> Result<LolLoginLoginSession, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-login/v1/session";
    let value = client.post(url.to_owned(), Some(body)).await?;
    Ok(serde_json::from_value::<LolLoginLoginSession>(value)?)
}

pub async fn post_lol_login_v1_session_invoke(client: RESTClient, destination: String, method: String, args: Vec<Value>) -> Result<LolLoginLcdsResponse, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-login/v1/session/invoke?destination={{destination}}&method={{method}}&args={{args}}", &json!({"destination": destination, "method": method, "args": args}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(serde_json::from_value::<LolLoginLcdsResponse>(value)?)
}

pub async fn post_lol_login_v1_summoner_session(client: RESTClient, body: LolLoginSummonerSessionResource) -> Result<Value, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-login/v1/summoner-session";
    let value = client.post(url.to_owned(), Some(body)).await?;
    Ok(value)
}

pub async fn post_lol_login_v1_summoner_session_failed(client: RESTClient, response_code: i32) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-login/v1/summoner-session-failed?responseCode={{response_code}}", &json!({"response_code": response_code}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_loot_v1_milestones_by_loot_milestones_id_claim(client: RESTClient, loot_milestones_id: String) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-loot/v1/milestones/{{loot_milestones_id}}/claim", &json!({"loot_milestones_id": loot_milestones_id}))?;
    let url = format!("{}", template_url);
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_lol_loot_v1_new_player_check_done_by_new_value(client: RESTClient, new_value: bool) -> Result<String, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-loot/v1/new-player-check-done/{{new_value}}", &json!({"new_value": new_value}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(serde_json::from_value::<String>(value)?)
}

pub async fn post_lol_loot_v1_player_loot_by_loot_id_context_menu(client: RESTClient, loot_id: String) -> Result<Vec<LolLootContextMenu>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-loot/v1/player-loot/{{loot_id}}/context-menu", &json!({"loot_id": loot_id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(serde_json::from_value::<Vec<LolLootContextMenu>>(value)?)
}

pub async fn post_lol_loot_v1_player_loot_by_loot_name_redeem(client: RESTClient, loot_name: String) -> Result<LolLootPlayerLootUpdate, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-loot/v1/player-loot/{{loot_name}}/redeem", &json!({"loot_name": loot_name}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(serde_json::from_value::<LolLootPlayerLootUpdate>(value)?)
}

pub async fn post_lol_loot_v1_player_loot_notifications_by_id_acknowledge(client: RESTClient, id: String) -> Result<String, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-loot/v1/player-loot-notifications/{{id}}/acknowledge", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(serde_json::from_value::<String>(value)?)
}

pub async fn post_lol_loot_v1_recipes_by_recipe_name_craft(client: RESTClient, recipe_name: String, player_loot_list: Vec<String>, repeat: Option<i32>) -> Result<LolLootPlayerLootUpdate, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-loot/v1/recipes/{{recipe_name}}/craft?playerLootList={{player_loot_list}}&repeat={{repeat}}", &json!({"recipe_name": recipe_name, "player_loot_list": player_loot_list, "repeat": repeat}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(serde_json::from_value::<LolLootPlayerLootUpdate>(value)?)
}

pub async fn post_lol_loot_v1_recipes_initial_item_by_loot_id(client: RESTClient, loot_id: String) -> Result<Vec<LolLootRecipeWithMilestones>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-loot/v1/recipes/initial-item/{{loot_id}}", &json!({"loot_id": loot_id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(serde_json::from_value::<Vec<LolLootRecipeWithMilestones>>(value)?)
}

pub async fn post_lol_loot_v1_refresh(client: RESTClient, force: bool) -> Result<String, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-loot/v1/refresh?force={{force}}", &json!({"force": force}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(serde_json::from_value::<String>(value)?)
}

pub async fn post_lol_loyalty_v1_update_loyalty_inventory(client: RESTClient, body: LolLoyaltyLoyaltyRewards) -> Result<(), Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-loyalty/v1/updateLoyaltyInventory";
    client.post(url.to_owned(), Some(body)).await?;
    Ok(())
}

pub async fn post_lol_maps_v1_map(client: RESTClient, body: LolMapsMaps) -> Result<(), Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-maps/v1/map";
    client.post(url.to_owned(), Some(body)).await?;
    Ok(())
}

pub async fn post_lol_marketing_preferences_v1_partition_by_partition_key(client: RESTClient, partition_key: String, body: HashMap<String, String>) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-marketing-preferences/v1/partition/{{partition_key}}?body={{body}}", &json!({"partition_key": partition_key, "body": body}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(serde_json::from_value::<HashMap<String, String>>(value)?)
}

pub async fn post_lol_match_history_v1_acs_endpoint_override(client: RESTClient, body: LolMatchHistoryAcsEndPoint) -> Result<Value, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-match-history/v1/acs-endpoint-override";
    let value = client.post(url.to_owned(), Some(body)).await?;
    Ok(value)
}

pub async fn post_lol_matchmaking_v1_ready_check_accept(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/lol-matchmaking/v1/ready-check/accept";
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_matchmaking_v1_ready_check_decline(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/lol-matchmaking/v1/ready-check/decline";
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_matchmaking_v1_search(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/lol-matchmaking/v1/search";
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_missions_v1_force(client: RESTClient) -> Result<(), Box<dyn Error>> {
    let url = "/lol-missions/v1/force";
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_lol_npe_rewards_v1_challenges_opt(client: RESTClient) -> Result<(), Box<dyn Error>> {
    let url = "/lol-npe-rewards/v1/challenges/opt";
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_lol_patch_v1_products_league_of_legends_detect_corruption_request(client: RESTClient) -> Result<(), Box<dyn Error>> {
    let url = "/lol-patch/v1/products/league_of_legends/detect-corruption-request";
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_lol_patch_v1_products_league_of_legends_partial_repair_request(client: RESTClient) -> Result<(), Box<dyn Error>> {
    let url = "/lol-patch/v1/products/league_of_legends/partial-repair-request";
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_lol_patch_v1_products_league_of_legends_start_checking_request(client: RESTClient) -> Result<(), Box<dyn Error>> {
    let url = "/lol-patch/v1/products/league_of_legends/start-checking-request";
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_lol_patch_v1_products_league_of_legends_start_patching_request(client: RESTClient) -> Result<(), Box<dyn Error>> {
    let url = "/lol-patch/v1/products/league_of_legends/start-patching-request";
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_lol_patch_v1_products_league_of_legends_stop_checking_request(client: RESTClient) -> Result<(), Box<dyn Error>> {
    let url = "/lol-patch/v1/products/league_of_legends/stop-checking-request";
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_lol_patch_v1_products_league_of_legends_stop_patching_request(client: RESTClient, restart: bool) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-patch/v1/products/league_of_legends/stop-patching-request?restart={{restart}}", &json!({"restart": restart}))?;
    let url = format!("{}", template_url);
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_lol_perks_v1_pages(client: RESTClient, body: LolPerksPerkPageResource) -> Result<LolPerksPerkPageResource, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-perks/v1/pages";
    let value = client.post(url.to_owned(), Some(body)).await?;
    Ok(serde_json::from_value::<LolPerksPerkPageResource>(value)?)
}

pub async fn post_lol_perks_v1_show_auto_modified_pages_notification(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/lol-perks/v1/show-auto-modified-pages-notification";
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_perks_v1_update_page_order(client: RESTClient, body: LolPerksUpdatePageOrderRequest) -> Result<Value, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-perks/v1/update-page-order";
    let value = client.post(url.to_owned(), Some(body)).await?;
    Ok(value)
}

pub async fn post_lol_pft_v2_events(client: RESTClient, body: LolPftPFTEvent) -> Result<Value, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-pft/v2/events";
    let value = client.post(url.to_owned(), Some(body)).await?;
    Ok(value)
}

pub async fn post_lol_pft_v2_survey(client: RESTClient, body: LolPftPFTSurvey) -> Result<(), Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-pft/v2/survey";
    client.post(url.to_owned(), Some(body)).await?;
    Ok(())
}

pub async fn post_lol_player_behavior_v2_reporter_feedback_by_key(client: RESTClient, key: String) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-player-behavior/v2/reporter-feedback/{{key}}", &json!({"key": key}))?;
    let url = format!("{}", template_url);
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_lol_player_level_up_v1_level_up_notifications_by_plugin_name(client: RESTClient, plugin_name: String, body: LolPlayerLevelUpPlayerLevelUpEventAck) -> Result<(), Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-player-level-up/v1/level-up-notifications/{{plugin_name}}", &json!({"plugin_name": plugin_name}))?;
    let url = format!("{}", template_url);
    client.post(url.to_owned(), Some(body)).await?;
    Ok(())
}

pub async fn post_lol_player_preferences_v1_hash(client: RESTClient, preferences: String) -> Result<String, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-player-preferences/v1/hash?preferences={{preferences}}", &json!({"preferences": preferences}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(serde_json::from_value::<String>(value)?)
}

pub async fn post_lol_player_preferences_v1_player_preferences_endpoint_override(client: RESTClient, body: LolPlayerPreferencesPlayerPreferencesEndpoint) -> Result<Value, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-player-preferences/v1/player-preferences-endpoint-override";
    let value = client.post(url.to_owned(), Some(body)).await?;
    Ok(value)
}

pub async fn post_lol_player_report_sender_v1_champ_select_reports_category_by_category(client: RESTClient, category: String, body: LolPlayerReportSenderChampSelectSummonerInfo) -> Result<Value, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-player-report-sender/v1/champ-select-reports/category/{{category}}", &json!({"category": category}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), Some(body)).await?;
    Ok(value)
}

pub async fn post_lol_player_report_sender_v1_end_of_game_reports(client: RESTClient, body: LolPlayerReportSenderEndOfGamePlayerReport) -> Result<Value, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-player-report-sender/v1/end-of-game-reports";
    let value = client.post(url.to_owned(), Some(body)).await?;
    Ok(value)
}

pub async fn post_lol_pre_end_of_game_v1_complete_by_sequence_event_name(client: RESTClient, sequence_event_name: String) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-pre-end-of-game/v1/complete/{{sequence_event_name}}", &json!({"sequence_event_name": sequence_event_name}))?;
    let url = format!("{}", template_url);
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_lol_pre_end_of_game_v1_registration_by_sequence_event_name_by_priority(client: RESTClient, sequence_event_name: String, priority: i32) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-pre-end-of-game/v1/registration/{sequenceEventName}/{{priority}}", &json!({"sequence_event_name": sequence_event_name, "priority": priority}))?;
    let url = format!("{}", template_url);
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_lol_premade_voice_v1_first_experience_game(client: RESTClient) -> Result<(), Box<dyn Error>> {
    let url = "/lol-premade-voice/v1/first-experience/game";
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_lol_premade_voice_v1_first_experience_lcu(client: RESTClient) -> Result<(), Box<dyn Error>> {
    let url = "/lol-premade-voice/v1/first-experience/lcu";
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_lol_premade_voice_v1_first_experience_reset(client: RESTClient) -> Result<(), Box<dyn Error>> {
    let url = "/lol-premade-voice/v1/first-experience/reset";
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_lol_premade_voice_v1_game_client_updated_p_t_t_key(client: RESTClient, new_key: String) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-premade-voice/v1/gameClientUpdatedPTTKey?newKey={{new_key}}", &json!({"new_key": new_key}))?;
    let url = format!("{}", template_url);
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_lol_premade_voice_v1_mic_test(client: RESTClient) -> Result<Value, Box<dyn Error>> {
    let url = "/lol-premade-voice/v1/mic-test";
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_premade_voice_v1_push_to_talk_check_available(client: RESTClient, prompt: i32) -> Result<bool, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-premade-voice/v1/push-to-talk/check-available?prompt={{prompt}}", &json!({"prompt": prompt}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(serde_json::from_value::<bool>(value)?)
}

pub async fn post_lol_premade_voice_v1_session(client: RESTClient) -> Result<(), Box<dyn Error>> {
    let url = "/lol-premade-voice/v1/session";
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_lol_premade_voice_v1_settings_reset(client: RESTClient) -> Result<(), Box<dyn Error>> {
    let url = "/lol-premade-voice/v1/settings/reset";
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_lol_purchase_widget_v1_purchasable_items_by_inventory_type(client: RESTClient, inventory_type: String, item_ids: Vec<i64>) -> Result<LolPurchaseWidgetItemChoices, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-purchase-widget/v1/purchasable-items/{{inventory_type}}?itemIds={{item_ids}}", &json!({"inventory_type": inventory_type, "item_ids": item_ids}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(serde_json::from_value::<LolPurchaseWidgetItemChoices>(value)?)
}

pub async fn post_lol_purchase_widget_v2_purchase_items(client: RESTClient, body: LolPurchaseWidgetPurchaseRequest) -> Result<Value, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-purchase-widget/v2/purchaseItems";
    let value = client.post(url.to_owned(), Some(body)).await?;
    Ok(value)
}

pub async fn post_lol_purchase_widget_v3_purchase_offer(client: RESTClient, body: LolPurchaseWidgetPurchaseOfferRequestV3) -> Result<LolPurchaseWidgetPurchaseOfferResponseV3, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-purchase-widget/v3/purchaseOffer";
    let value = client.post(url.to_owned(), Some(body)).await?;
    Ok(serde_json::from_value::<LolPurchaseWidgetPurchaseOfferResponseV3>(value)?)
}

pub async fn post_lol_purchase_widget_v3_validate_offer(client: RESTClient, body: LolPurchaseWidgetValidateOfferRequestV3) -> Result<LolPurchaseWidgetValidateOfferResponseV3, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-purchase-widget/v3/validateOffer";
    let value = client.post(url.to_owned(), Some(body)).await?;
    Ok(serde_json::from_value::<LolPurchaseWidgetValidateOfferResponseV3>(value)?)
}

pub async fn post_lol_ranked_v1_eos_notifications_by_id_acknowledge(client: RESTClient, id: String) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-ranked/v1/eos-notifications/{{id}}/acknowledge", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_ranked_v1_notifications_by_id_acknowledge(client: RESTClient, id: u64) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-ranked/v1/notifications/{{id}}/acknowledge", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_replays_v1_metadata_by_game_id_create_game_version_by_game_version_game_type_by_game_type_queue_id_by_queue_id(client: RESTClient, game_id: u64, game_version: String, game_type: String, queue_id: i32) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-replays/v1/metadata/{gameId}/create/gameVersion/{gameVersion}/gameType/{gameType}/queueId/{{queue_id}}", &json!({"game_id": game_id, "game_version": game_version, "game_type": game_type, "queue_id": queue_id}))?;
    let url = format!("{}", template_url);
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_lol_replays_v1_rofls_by_game_id_download(client: RESTClient, game_id: u64, body: LolReplaysReplayContextData) -> Result<(), Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-replays/v1/rofls/{{game_id}}/download", &json!({"game_id": game_id}))?;
    let url = format!("{}", template_url);
    client.post(url.to_owned(), Some(body)).await?;
    Ok(())
}

pub async fn post_lol_replays_v1_rofls_by_game_id_download_graceful(client: RESTClient, game_id: u64, body: LolReplaysReplayContextData) -> Result<(), Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-replays/v1/rofls/{{game_id}}/download/graceful", &json!({"game_id": game_id}))?;
    let url = format!("{}", template_url);
    client.post(url.to_owned(), Some(body)).await?;
    Ok(())
}

pub async fn post_lol_replays_v1_rofls_by_game_id_watch(client: RESTClient, game_id: u64, body: LolReplaysReplayContextData) -> Result<(), Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-replays/v1/rofls/{{game_id}}/watch", &json!({"game_id": game_id}))?;
    let url = format!("{}", template_url);
    client.post(url.to_owned(), Some(body)).await?;
    Ok(())
}

pub async fn post_lol_replays_v1_rofls_scan(client: RESTClient) -> Result<(), Box<dyn Error>> {
    let url = "/lol-replays/v1/rofls/scan";
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_lol_replays_v2_metadata_by_game_id_create(client: RESTClient, game_id: u64, body: LolReplaysReplayCreateMetadata) -> Result<(), Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-replays/v2/metadata/{{game_id}}/create", &json!({"game_id": game_id}))?;
    let url = format!("{}", template_url);
    client.post(url.to_owned(), Some(body)).await?;
    Ok(())
}

pub async fn post_lol_rewards_v1_grants_by_grant_id_select(client: RESTClient, grant_id: String, body: LolRewardsSelectionRequestDTO) -> Result<LolRewardsRewardGrant, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-rewards/v1/grants/{{grant_id}}/select", &json!({"grant_id": grant_id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), Some(body)).await?;
    Ok(serde_json::from_value::<LolRewardsRewardGrant>(value)?)
}

pub async fn post_lol_rewards_v1_select_bulk(client: RESTClient, selection: Vec<LolRewardsSelectionRequestDTO>) -> Result<HashMap<String, LolRewardsSelectGrantStatusResponse>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-rewards/v1/select-bulk?selection={{selection}}", &json!({"selection": selection}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(serde_json::from_value::<HashMap<String, LolRewardsSelectGrantStatusResponse>>(value)?)
}

pub async fn post_lol_rso_auth_v1_authorization_gas(client: RESTClient, body: LolRsoAuthRSOPlayerCredentials) -> Result<LolRsoAuthAuthorization, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-rso-auth/v1/authorization/gas";
    let value = client.post(url.to_owned(), Some(body)).await?;
    Ok(serde_json::from_value::<LolRsoAuthAuthorization>(value)?)
}

pub async fn post_lol_rso_auth_v1_authorization_refresh(client: RESTClient) -> Result<LolRsoAuthAuthorization, Box<dyn Error>> {
    let url = "/lol-rso-auth/v1/authorization/refresh";
    let value = client.post(url.to_owned(), None).await?;
    Ok(serde_json::from_value::<LolRsoAuthAuthorization>(value)?)
}

pub async fn post_lol_rso_auth_v1_authorization_userinfo(client: RESTClient) -> Result<LolRsoAuthUserInfo, Box<dyn Error>> {
    let url = "/lol-rso-auth/v1/authorization/userinfo";
    let value = client.post(url.to_owned(), None).await?;
    Ok(serde_json::from_value::<LolRsoAuthUserInfo>(value)?)
}

pub async fn post_lol_rso_auth_v1_device_id(client: RESTClient) -> Result<LolRsoAuthDeviceId, Box<dyn Error>> {
    let url = "/lol-rso-auth/v1/device-id";
    let value = client.post(url.to_owned(), None).await?;
    Ok(serde_json::from_value::<LolRsoAuthDeviceId>(value)?)
}

pub async fn post_lol_rso_auth_v2_config(client: RESTClient, body: LolRsoAuthPublicClientConfig) -> Result<(), Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-rso-auth/v2/config";
    client.post(url.to_owned(), Some(body)).await?;
    Ok(())
}

pub async fn post_lol_settings_v1_account_save(client: RESTClient) -> Result<(), Box<dyn Error>> {
    let url = "/lol-settings/v1/account/save";
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_lol_simple_dialog_messages_v1_messages(client: RESTClient, body: LolSimpleDialogMessagesLocalMessageRequest) -> Result<Value, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-simple-dialog-messages/v1/messages";
    let value = client.post(url.to_owned(), Some(body)).await?;
    Ok(value)
}

pub async fn post_lol_spectator_v1_buddy_spectate(client: RESTClient, summoner_or_team_names: Vec<String>) -> Result<LolSpectatorSummonerOrTeamAvailabilty, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-spectator/v1/buddy/spectate?summonerOrTeamNames={{summoner_or_team_names}}", &json!({"summoner_or_team_names": summoner_or_team_names}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(serde_json::from_value::<LolSpectatorSummonerOrTeamAvailabilty>(value)?)
}

pub async fn post_lol_spectator_v1_spectate_launch(client: RESTClient, body: LolSpectatorSpectateGameInfo) -> Result<Value, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-spectator/v1/spectate/launch";
    let value = client.post(url.to_owned(), Some(body)).await?;
    Ok(value)
}

pub async fn post_lol_statstones_v1_featured_champion_statstones_by_champion_item_id_by_statstone_id(client: RESTClient, champion_item_id: i32, statstone_id: String, body: LolStatstonesStatstoneFeaturedRequest) -> Result<Value, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-statstones/v1/featured-champion-statstones/{championItemId}/{{statstone_id}}", &json!({"champion_item_id": champion_item_id, "statstone_id": statstone_id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), Some(body)).await?;
    Ok(value)
}

pub async fn post_lol_store_v1_last_page(client: RESTClient, page_type: String) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-store/v1/lastPage?pageType={{page_type}}", &json!({"page_type": page_type}))?;
    let url = format!("{}", template_url);
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_lol_store_v1_notifications_acknowledge(client: RESTClient, id: String) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-store/v1/notifications/acknowledge?id={{id}}", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_lol_suggested_players_v1_reported_player(client: RESTClient, body: LolSuggestedPlayersSuggestedPlayersReportedPlayer) -> Result<(), Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-suggested-players/v1/reported-player";
    client.post(url.to_owned(), Some(body)).await?;
    Ok(())
}

pub async fn post_lol_suggested_players_v1_victorious_comrade(client: RESTClient, body: LolSuggestedPlayersSuggestedPlayersVictoriousComrade) -> Result<(), Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-suggested-players/v1/victorious-comrade";
    client.post(url.to_owned(), Some(body)).await?;
    Ok(())
}

pub async fn post_lol_summoner_v1_current_summoner_name(client: RESTClient, name: String) -> Result<LolSummonerSummoner, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-summoner/v1/current-summoner/name?name={{name}}", &json!({"name": name}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(serde_json::from_value::<LolSummonerSummoner>(value)?)
}

pub async fn post_lol_summoner_v1_current_summoner_summoner_profile(client: RESTClient, body: LolSummonerSummonerProfileUpdate) -> Result<Value, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-summoner/v1/current-summoner/summoner-profile";
    let value = client.post(url.to_owned(), Some(body)).await?;
    Ok(value)
}

pub async fn post_lol_summoner_v1_summoners(client: RESTClient, body: LolSummonerSummonerRequestedName) -> Result<LolSummonerSummoner, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-summoner/v1/summoners";
    let value = client.post(url.to_owned(), Some(body)).await?;
    Ok(serde_json::from_value::<LolSummonerSummoner>(value)?)
}

pub async fn post_lol_summoner_v2_summoners_names(client: RESTClient, summoner_names: Vec<String>) -> Result<Vec<LolSummonerSummoner>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-summoner/v2/summoners/names?summonerNames={{summoner_names}}", &json!({"summoner_names": summoner_names}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(serde_json::from_value::<Vec<LolSummonerSummoner>>(value)?)
}

pub async fn post_lol_summoner_v2_summoners_puuid(client: RESTClient, puuids: Vec<String>) -> Result<Vec<LolSummonerSummoner>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-summoner/v2/summoners/puuid?puuids={{puuids}}", &json!({"puuids": puuids}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(serde_json::from_value::<Vec<LolSummonerSummoner>>(value)?)
}

pub async fn post_lol_yourshop_v1_offers_by_id_purchase(client: RESTClient, id: String) -> Result<LolYourshopPurchaseResponse, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-yourshop/v1/offers/{{id}}/purchase", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(serde_json::from_value::<LolYourshopPurchaseResponse>(value)?)
}

pub async fn post_lol_yourshop_v1_offers_by_id_reveal(client: RESTClient, id: String) -> Result<Vec<LolYourshopUIOffer>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-yourshop/v1/offers/{{id}}/reveal", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(serde_json::from_value::<Vec<LolYourshopUIOffer>>(value)?)
}

pub async fn post_lol_yourshop_v1_permissions(client: RESTClient, body: LolYourshopPlayerPermissions) -> Result<LolYourshopPlayerPermissions, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-yourshop/v1/permissions";
    let value = client.post(url.to_owned(), Some(body)).await?;
    Ok(serde_json::from_value::<LolYourshopPlayerPermissions>(value)?)
}

pub async fn post_memory_v1_notify_fe_processes_ready(client: RESTClient) -> Result<(), Box<dyn Error>> {
    let url = "/memory/v1/notify-fe-processes-ready";
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_memory_v1_snapshot(client: RESTClient, process_ids: Vec<u32>, context: String) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/memory/v1/snapshot?processIds={{process_ids}}&context={{context}}", &json!({"process_ids": process_ids, "context": context}))?;
    let url = format!("{}", template_url);
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_patcher_v1_notifications(client: RESTClient, notification_id: PatcherNotificationId) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/patcher/v1/notifications?notificationId={{notification_id}}", &json!({"notification_id": notification_id}))?;
    let url = format!("{}", template_url);
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_patcher_v1_products_by_product_id_detect_corruption_request(client: RESTClient, product_id: String) -> Result<PatcherProductState, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/patcher/v1/products/{{product_id}}/detect-corruption-request", &json!({"product_id": product_id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(serde_json::from_value::<PatcherProductState>(value)?)
}

pub async fn post_patcher_v1_products_by_product_id_partial_repair_request(client: RESTClient, product_id: String) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/patcher/v1/products/{{product_id}}/partial-repair-request", &json!({"product_id": product_id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_patcher_v1_products_by_product_id_signal_start_patching_delayed(client: RESTClient, product_id: String) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/patcher/v1/products/{{product_id}}/signal-start-patching-delayed", &json!({"product_id": product_id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_patcher_v1_products_by_product_id_start_checking_request(client: RESTClient, product_id: String) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/patcher/v1/products/{{product_id}}/start-checking-request", &json!({"product_id": product_id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_patcher_v1_products_by_product_id_start_patching_request(client: RESTClient, product_id: String) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/patcher/v1/products/{{product_id}}/start-patching-request", &json!({"product_id": product_id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_patcher_v1_products_by_product_id_stop_checking_request(client: RESTClient, product_id: String) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/patcher/v1/products/{{product_id}}/stop-checking-request", &json!({"product_id": product_id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_patcher_v1_products_by_product_id_stop_patching_request(client: RESTClient, product_id: String) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/patcher/v1/products/{{product_id}}/stop-patching-request", &json!({"product_id": product_id}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn post_payments_v1_pmc_start_url(client: RESTClient, body: PaymentsFrontEndRequest) -> Result<PaymentsFrontEndResult, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/payments/v1/pmc-start-url";
    let value = client.post(url.to_owned(), Some(body)).await?;
    Ok(serde_json::from_value::<PaymentsFrontEndResult>(value)?)
}

pub async fn post_performance_v1_process_by_process_id(client: RESTClient, process_id: u32) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/performance/v1/process/{{process_id}}", &json!({"process_id": process_id}))?;
    let url = format!("{}", template_url);
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_performance_v1_report_restart(client: RESTClient, sample_length: Option<i32>, sample_count: Option<i32>) -> Result<Vec<Value>, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/performance/v1/report/restart?sampleLength={{sample_length}}&sampleCount={{sample_count}}", &json!({"sample_length": sample_length, "sample_count": sample_count}))?;
    let url = format!("{}", template_url);
    let value = client.post(url.to_owned(), None).await?;
    Ok(serde_json::from_value::<Vec<Value>>(value)?)
}

pub async fn post_player_notifications_v1_notifications(client: RESTClient, body: PlayerNotificationsPlayerNotificationResource) -> Result<PlayerNotificationsPlayerNotificationResource, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/player-notifications/v1/notifications";
    let value = client.post(url.to_owned(), Some(body)).await?;
    Ok(serde_json::from_value::<PlayerNotificationsPlayerNotificationResource>(value)?)
}

pub async fn post_process_control_v1_process_quit(client: RESTClient) -> Result<(), Box<dyn Error>> {
    let url = "/process-control/v1/process/quit";
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_riot_messaging_service_v1_connect(client: RESTClient, id_token: String) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/riot-messaging-service/v1/connect?idToken={{id_token}}", &json!({"id_token": id_token}))?;
    let url = format!("{}", template_url);
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_riot_messaging_service_v1_entitlements(client: RESTClient, body: RiotMessagingServiceEntitlementsToken) -> Result<(), Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/riot-messaging-service/v1/entitlements";
    client.post(url.to_owned(), Some(body)).await?;
    Ok(())
}

pub async fn post_riotclient_addorupdatemetric(client: RESTClient, group: String, object: String, name: String, value: u64) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/riotclient/addorupdatemetric?group={{group}}&object={{object}}&name={{name}}&value={{value}}", &json!({"group": group, "object": object, "name": name, "value": value}))?;
    let url = format!("{}", template_url);
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_riotclient_affinity(client: RESTClient, new_affinity: String) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/riotclient/affinity?newAffinity={{new_affinity}}", &json!({"new_affinity": new_affinity}))?;
    let url = format!("{}", template_url);
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_riotclient_kill_and_restart_ux(client: RESTClient) -> Result<(), Box<dyn Error>> {
    let url = "/riotclient/kill-and-restart-ux";
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_riotclient_kill_ux(client: RESTClient) -> Result<(), Box<dyn Error>> {
    let url = "/riotclient/kill-ux";
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_riotclient_launch_ux(client: RESTClient) -> Result<(), Box<dyn Error>> {
    let url = "/riotclient/launch-ux";
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_riotclient_new_args(client: RESTClient, args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/riotclient/new-args?args={{args}}", &json!({"args": args}))?;
    let url = format!("{}", template_url);
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_riotclient_open_url_in_browser(client: RESTClient, url: String) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/riotclient/open-url-in-browser?url={{url}}", &json!({"url": url}))?;
    let url = format!("{}", template_url);
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_riotclient_show_swagger(client: RESTClient) -> Result<(), Box<dyn Error>> {
    let url = "/riotclient/show-swagger";
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_riotclient_unload(client: RESTClient) -> Result<(), Box<dyn Error>> {
    let url = "/riotclient/unload";
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_riotclient_ux_allow_foreground(client: RESTClient) -> Result<(), Box<dyn Error>> {
    let url = "/riotclient/ux-allow-foreground";
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_riotclient_ux_flash(client: RESTClient) -> Result<(), Box<dyn Error>> {
    let url = "/riotclient/ux-flash";
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_riotclient_ux_minimize(client: RESTClient) -> Result<(), Box<dyn Error>> {
    let url = "/riotclient/ux-minimize";
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_riotclient_ux_show(client: RESTClient) -> Result<(), Box<dyn Error>> {
    let url = "/riotclient/ux-show";
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_riotclient_v1_crash_reporting_logs(client: RESTClient, log_file_path: String) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/riotclient/v1/crash-reporting/logs?logFilePath={{log_file_path}}", &json!({"log_file_path": log_file_path}))?;
    let url = format!("{}", template_url);
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_riotclient_v1_elevation_requests(client: RESTClient, body: ElevationRequest) -> Result<(), Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/riotclient/v1/elevation-requests";
    client.post(url.to_owned(), Some(body)).await?;
    Ok(())
}

pub async fn post_riotclient_zoom_scale(client: RESTClient, new_zoom_scale: f64) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/riotclient/zoom-scale?newZoomScale={{new_zoom_scale}}", &json!({"new_zoom_scale": new_zoom_scale}))?;
    let url = format!("{}", template_url);
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_sanitizer_v1_contains_sanitized(client: RESTClient, body: SanitizerContainsSanitizedRequest) -> Result<SanitizerContainsSanitizedResponse, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/sanitizer/v1/containsSanitized";
    let value = client.post(url.to_owned(), Some(body)).await?;
    Ok(serde_json::from_value::<SanitizerContainsSanitizedResponse>(value)?)
}

pub async fn post_sanitizer_v1_sanitize(client: RESTClient, body: SanitizerSanitizeRequest) -> Result<SanitizerSanitizeResponse, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/sanitizer/v1/sanitize";
    let value = client.post(url.to_owned(), Some(body)).await?;
    Ok(serde_json::from_value::<SanitizerSanitizeResponse>(value)?)
}

pub async fn post_telemetry_v1_common_data_by_key(client: RESTClient, key: String, value: String) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/telemetry/v1/common-data/{{key}}?value={{value}}", &json!({"key": key, "value": value}))?;
    let url = format!("{}", template_url);
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_telemetry_v1_events_by_event_type(client: RESTClient, event_type: String, event_data: HashMap<String, Value>) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/telemetry/v1/events/{{event_type}}?eventData={{event_data}}", &json!({"event_type": event_type, "event_data": event_data}))?;
    let url = format!("{}", template_url);
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_telemetry_v1_events_once_by_event_type(client: RESTClient, event_type: String, once_tag: String, event_data: HashMap<String, Value>) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/telemetry/v1/events-once/{{event_type}}?onceTag={{once_tag}}&eventData={{event_data}}", &json!({"event_type": event_type, "once_tag": once_tag, "event_data": event_data}))?;
    let url = format!("{}", template_url);
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_telemetry_v1_events_with_perf_info_by_event_type(client: RESTClient, event_type: String, event_data: HashMap<String, Value>) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/telemetry/v1/events-with-perf-info/{{event_type}}?eventData={{event_data}}", &json!({"event_type": event_type, "event_data": event_data}))?;
    let url = format!("{}", template_url);
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_telemetry_v3_events_by_event_type(client: RESTClient, event_type: String, event_data: HashMap<String, Value>) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/telemetry/v3/events/{{event_type}}?eventData={{event_data}}", &json!({"event_type": event_type, "event_data": event_data}))?;
    let url = format!("{}", template_url);
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_telemetry_v3_events_once_by_event_type(client: RESTClient, event_type: String, once_tag: String, event_data: HashMap<String, Value>) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/telemetry/v3/events-once/{{event_type}}?onceTag={{once_tag}}&eventData={{event_data}}", &json!({"event_type": event_type, "once_tag": once_tag, "event_data": event_data}))?;
    let url = format!("{}", template_url);
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_telemetry_v3_slis_counts(client: RESTClient, body: SLICount) -> Result<(), Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/telemetry/v3/slis/counts";
    client.post(url.to_owned(), Some(body)).await?;
    Ok(())
}

pub async fn post_telemetry_v3_uptime_tracking_notify_failure(client: RESTClient, body: NotifyFailureRequest) -> Result<(), Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/telemetry/v3/uptime-tracking/notify-failure";
    client.post(url.to_owned(), Some(body)).await?;
    Ok(())
}

pub async fn post_telemetry_v3_uptime_tracking_notify_success(client: RESTClient, body: NotifySuccessRequest) -> Result<(), Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/telemetry/v3/uptime-tracking/notify-success";
    client.post(url.to_owned(), Some(body)).await?;
    Ok(())
}

pub async fn post_tracing_v1_performance_by_name(client: RESTClient, name: String) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/tracing/v1/performance/{{name}}", &json!({"name": name}))?;
    let url = format!("{}", template_url);
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_tracing_v1_trace_critical_flow(client: RESTClient, body: TracingCriticalFlowEventV1) -> Result<(), Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/tracing/v1/trace/critical-flow";
    client.post(url.to_owned(), Some(body)).await?;
    Ok(())
}

pub async fn post_tracing_v1_trace_event(client: RESTClient, body: TracingEventV1) -> Result<(), Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/tracing/v1/trace/event";
    client.post(url.to_owned(), Some(body)).await?;
    Ok(())
}

pub async fn post_tracing_v1_trace_module(client: RESTClient, body: TracingModuleV1) -> Result<(), Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/tracing/v1/trace/module";
    client.post(url.to_owned(), Some(body)).await?;
    Ok(())
}

pub async fn post_tracing_v1_trace_non_timing_event_by_event_name(client: RESTClient, event_name: String, when: u64, value: String, unit: String) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/tracing/v1/trace/non-timing-event/{{event_name}}?when={{when}}&value={{value}}&unit={{unit}}", &json!({"event_name": event_name, "when": when, "value": value, "unit": unit}))?;
    let url = format!("{}", template_url);
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_tracing_v1_trace_phase_begin(client: RESTClient, body: TracingPhaseBeginV1) -> Result<(), Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/tracing/v1/trace/phase/begin";
    client.post(url.to_owned(), Some(body)).await?;
    Ok(())
}

pub async fn post_tracing_v1_trace_phase_end(client: RESTClient, body: TracingPhaseEndV1) -> Result<(), Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/tracing/v1/trace/phase/end";
    client.post(url.to_owned(), Some(body)).await?;
    Ok(())
}

pub async fn post_tracing_v1_trace_step_event(client: RESTClient, event_name: String) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/tracing/v1/trace/step-event?eventName={{event_name}}", &json!({"event_name": event_name}))?;
    let url = format!("{}", template_url);
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_tracing_v1_trace_time_series_event_by_event_name(client: RESTClient, event_name: String, when: u64) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/tracing/v1/trace/time-series-event/{{event_name}}?when={{when}}", &json!({"event_name": event_name, "when": when}))?;
    let url = format!("{}", template_url);
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn post_tracing_v1_trace_time_series_event_by_event_name_marker_by_marker_name(client: RESTClient, event_name: String, marker_name: String, when: u64) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/tracing/v1/trace/time-series-event/{eventName}/marker/{{marker_name}}?when={{when}}", &json!({"event_name": event_name, "marker_name": marker_name, "when": when}))?;
    let url = format!("{}", template_url);
    client.post(url.to_owned(), None).await?;
    Ok(())
}

pub async fn put_client_config_v1_entitlements_token(client: RESTClient, body: ClientConfigEntitlementsUpdate) -> Result<(), Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/client-config/v1/entitlements-token";
    client.put(url.to_owned(), Some(body)).await?;
    Ok(())
}

pub async fn put_client_config_v1_refresh_config_status(client: RESTClient) -> Result<(), Box<dyn Error>> {
    let url = "/client-config/v1/refresh-config-status";
    client.put(url.to_owned(), None).await?;
    Ok(())
}

pub async fn put_client_config_v2_namespace_changes(client: RESTClient, body: ClientConfigConfigNamespaceUpdate) -> Result<(), Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/client-config/v2/namespace-changes";
    client.put(url.to_owned(), Some(body)).await?;
    Ok(())
}

pub async fn put_lol_banners_v1_current_summoner_flags_equipped(client: RESTClient, body: LolBannersBannerFlag) -> Result<LolBannersBannerFlag, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-banners/v1/current-summoner/flags/equipped";
    let value = client.put(url.to_owned(), Some(body)).await?;
    Ok(serde_json::from_value::<LolBannersBannerFlag>(value)?)
}

pub async fn put_lol_chat_v1_conversations_active(client: RESTClient, body: LolChatActiveConversationResource) -> Result<Value, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-chat/v1/conversations/active";
    let value = client.put(url.to_owned(), Some(body)).await?;
    Ok(value)
}

pub async fn put_lol_chat_v1_conversations_by_id(client: RESTClient, id: String, body: LolChatConversationResource) -> Result<LolChatConversationResource, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-chat/v1/conversations/{{id}}", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.put(url.to_owned(), Some(body)).await?;
    Ok(serde_json::from_value::<LolChatConversationResource>(value)?)
}

pub async fn put_lol_chat_v1_conversations_by_id_closed(client: RESTClient, id: String) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-chat/v1/conversations/{{id}}/closed", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.put(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn put_lol_chat_v1_friend_groups_by_id(client: RESTClient, id: u32, body: LolChatGroupResource) -> Result<Value, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-chat/v1/friend-groups/{{id}}", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.put(url.to_owned(), Some(body)).await?;
    Ok(value)
}

pub async fn put_lol_chat_v1_friend_groups_order(client: RESTClient, body: LolChatFriendGroupOrder) -> Result<Value, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-chat/v1/friend-groups/order";
    let value = client.put(url.to_owned(), Some(body)).await?;
    Ok(value)
}

pub async fn put_lol_chat_v1_friend_requests_by_id(client: RESTClient, id: String, body: LolChatFriendRequestResource) -> Result<Value, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-chat/v1/friend-requests/{{id}}", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.put(url.to_owned(), Some(body)).await?;
    Ok(value)
}

pub async fn put_lol_chat_v1_friends_by_id(client: RESTClient, id: String, body: LolChatFriendResource) -> Result<Value, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-chat/v1/friends/{{id}}", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.put(url.to_owned(), Some(body)).await?;
    Ok(value)
}

pub async fn put_lol_chat_v1_me(client: RESTClient, body: LolChatUserResource) -> Result<LolChatUserResource, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-chat/v1/me";
    let value = client.put(url.to_owned(), Some(body)).await?;
    Ok(serde_json::from_value::<LolChatUserResource>(value)?)
}

pub async fn put_lol_chat_v1_settings(client: RESTClient, data: Value, do_async: Option<bool>) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-chat/v1/settings?data={{data}}&doAsync={{do_async}}", &json!({"data": data, "do_async": do_async}))?;
    let url = format!("{}", template_url);
    let value = client.put(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn put_lol_chat_v1_settings_by_key(client: RESTClient, key: String, value: Value, do_async: Option<bool>) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-chat/v1/settings/{{key}}?value={{value}}&doAsync={{do_async}}", &json!({"key": key, "value": value, "do_async": do_async}))?;
    let url = format!("{}", template_url);
    let value = client.put(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn put_lol_cosmetics_v1_selection_companion(client: RESTClient, item_id: i32) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-cosmetics/v1/selection/companion?itemId={{item_id}}", &json!({"item_id": item_id}))?;
    let url = format!("{}", template_url);
    let value = client.put(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn put_lol_cosmetics_v1_selection_tft_damage_skin(client: RESTClient, item_id: i32) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-cosmetics/v1/selection/tft-damage-skin?itemId={{item_id}}", &json!({"item_id": item_id}))?;
    let url = format!("{}", template_url);
    let value = client.put(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn put_lol_cosmetics_v1_selection_tft_map_skin(client: RESTClient, item_id: i32) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-cosmetics/v1/selection/tft-map-skin?itemId={{item_id}}", &json!({"item_id": item_id}))?;
    let url = format!("{}", template_url);
    let value = client.put(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn put_lol_email_verification_v1_email(client: RESTClient, body: LolEmailVerificationEmailUpdate) -> Result<Value, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-email-verification/v1/email";
    let value = client.put(url.to_owned(), Some(body)).await?;
    Ok(value)
}

pub async fn put_lol_highlights_v1_highlights_by_id(client: RESTClient, id: u64, body: LolHighlightsHighlight) -> Result<LolHighlightsHighlight, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-highlights/v1/highlights/{{id}}", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.put(url.to_owned(), Some(body)).await?;
    Ok(serde_json::from_value::<LolHighlightsHighlight>(value)?)
}

pub async fn put_lol_honeyfruit_v1_debug_linking_status(client: RESTClient, body: LolHoneyfruitHoneyfruitLinkingStatus) -> Result<Value, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-honeyfruit/v1/debug-linking-status";
    let value = client.put(url.to_owned(), Some(body)).await?;
    Ok(value)
}

pub async fn put_lol_honeyfruit_v1_debug_vng_publisher_settings(client: RESTClient, body: LolHoneyfruitHoneyfruitVNGPublisherSettings) -> Result<Value, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-honeyfruit/v1/debug-vng-publisher-settings";
    let value = client.put(url.to_owned(), Some(body)).await?;
    Ok(value)
}

pub async fn put_lol_honeyfruit_v1_linking_status(client: RESTClient, body: LolHoneyfruitHoneyfruitLinkingAction) -> Result<Value, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-honeyfruit/v1/linking-status";
    let value = client.put(url.to_owned(), Some(body)).await?;
    Ok(value)
}

pub async fn put_lol_item_sets_v1_item_sets_by_summoner_id_sets(client: RESTClient, summoner_id: u64, body: LolItemSetsItemSets) -> Result<(), Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-item-sets/v1/item-sets/{{summoner_id}}/sets", &json!({"summoner_id": summoner_id}))?;
    let url = format!("{}", template_url);
    client.put(url.to_owned(), Some(body)).await?;
    Ok(())
}

pub async fn put_lol_loadouts_v4_loadouts_by_id(client: RESTClient, id: String, body: LolLoadoutsUpdateLoadoutDTO) -> Result<LolLoadoutsScopedLoadout, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-loadouts/v4/loadouts/{{id}}", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.put(url.to_owned(), Some(body)).await?;
    Ok(serde_json::from_value::<LolLoadoutsScopedLoadout>(value)?)
}

pub async fn put_lol_lobby_v1_autofill_displayed(client: RESTClient) -> Result<bool, Box<dyn Error>> {
    let url = "/lol-lobby/v1/autofill-displayed";
    let value = client.put(url.to_owned(), None).await?;
    Ok(serde_json::from_value::<bool>(value)?)
}

pub async fn put_lol_lobby_v1_lobby_members_local_member_position_preferences(client: RESTClient, body: LolLobbyLobbyPositionPreferences) -> Result<Value, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-lobby/v1/lobby/members/localMember/position-preferences";
    let value = client.put(url.to_owned(), Some(body)).await?;
    Ok(value)
}

pub async fn put_lol_lobby_v1_parties_active(client: RESTClient, active: i32) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-lobby/v1/parties/active?active={{active}}", &json!({"active": active}))?;
    let url = format!("{}", template_url);
    client.put(url.to_owned(), None).await?;
    Ok(())
}

pub async fn put_lol_lobby_v1_parties_by_party_id_members_by_puuid_role(client: RESTClient, party_id: String, puuid: String, role: String) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-lobby/v1/parties/{partyId}/members/{{puuid}}/role?role={{role}}", &json!({"party_id": party_id, "puuid": puuid, "role": role}))?;
    let url = format!("{}", template_url);
    client.put(url.to_owned(), None).await?;
    Ok(())
}

pub async fn put_lol_lobby_v1_parties_metadata(client: RESTClient, body: LolLobbyPartyMemberMetadataDto) -> Result<(), Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-lobby/v1/parties/metadata";
    client.put(url.to_owned(), Some(body)).await?;
    Ok(())
}

pub async fn put_lol_lobby_v1_parties_queue(client: RESTClient, queue_id: i32) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-lobby/v1/parties/queue?queueId={{queue_id}}", &json!({"queue_id": queue_id}))?;
    let url = format!("{}", template_url);
    client.put(url.to_owned(), None).await?;
    Ok(())
}

pub async fn put_lol_lobby_v1_parties_ready(client: RESTClient, ready: i32) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-lobby/v1/parties/ready?ready={{ready}}", &json!({"ready": ready}))?;
    let url = format!("{}", template_url);
    client.put(url.to_owned(), None).await?;
    Ok(())
}

pub async fn put_lol_lobby_v2_lobby_members_local_member_position_preferences(client: RESTClient, body: LolLobbyLobbyPositionPreferences) -> Result<Value, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-lobby/v2/lobby/members/localMember/position-preferences";
    let value = client.put(url.to_owned(), Some(body)).await?;
    Ok(value)
}

pub async fn put_lol_lobby_v2_lobby_party_type(client: RESTClient, party_type: String) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-lobby/v2/lobby/partyType?partyType={{party_type}}", &json!({"party_type": party_type}))?;
    let url = format!("{}", template_url);
    let value = client.put(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn put_lol_login_v1_shutdown_locks_by_lock_name(client: RESTClient, lock_name: String) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-login/v1/shutdown-locks/{{lock_name}}", &json!({"lock_name": lock_name}))?;
    let url = format!("{}", template_url);
    client.put(url.to_owned(), None).await?;
    Ok(())
}

pub async fn put_lol_loot_v1_loot_odds_evaluate_query(client: RESTClient, body: QueryEvaluationRequestDTO) -> Result<Vec<LolLootQueryEvaluatedLootItem>, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-loot/v1/loot-odds/evaluateQuery";
    let value = client.put(url.to_owned(), Some(body)).await?;
    Ok(serde_json::from_value::<Vec<LolLootQueryEvaluatedLootItem>>(value)?)
}

pub async fn put_lol_matchmaking_v1_search(client: RESTClient, body: LolMatchmakingMatchmakingSearchResource) -> Result<Value, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-matchmaking/v1/search";
    let value = client.put(url.to_owned(), Some(body)).await?;
    Ok(value)
}

pub async fn put_lol_missions_v1_player(client: RESTClient, body: IdsDTO) -> Result<(), Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-missions/v1/player";
    client.put(url.to_owned(), Some(body)).await?;
    Ok(())
}

pub async fn put_lol_missions_v1_player_by_mission_id(client: RESTClient, mission_id: String, body: LolMissionsRewardGroupsSelection) -> Result<(), Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-missions/v1/player/{{mission_id}}", &json!({"mission_id": mission_id}))?;
    let url = format!("{}", template_url);
    client.put(url.to_owned(), Some(body)).await?;
    Ok(())
}

pub async fn put_lol_missions_v2_player_opt(client: RESTClient, body: LolMissionsSeriesOpt) -> Result<(), Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-missions/v2/player/opt";
    client.put(url.to_owned(), Some(body)).await?;
    Ok(())
}

pub async fn put_lol_npe_tutorial_path_v1_settings(client: RESTClient, body: LolNpeTutorialPathAccountSettingsTutorial) -> Result<(), Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-npe-tutorial-path/v1/settings";
    client.put(url.to_owned(), Some(body)).await?;
    Ok(())
}

pub async fn put_lol_npe_tutorial_path_v1_tutorials_by_tutorial_id_view(client: RESTClient, tutorial_id: String) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-npe-tutorial-path/v1/tutorials/{{tutorial_id}}/view", &json!({"tutorial_id": tutorial_id}))?;
    let url = format!("{}", template_url);
    client.put(url.to_owned(), None).await?;
    Ok(())
}

pub async fn put_lol_patch_v1_game_patch_url(client: RESTClient, url: String) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-patch/v1/game-patch-url?url={{url}}", &json!({"url": url}))?;
    let url = format!("{}", template_url);
    client.put(url.to_owned(), None).await?;
    Ok(())
}

pub async fn put_lol_patch_v1_ux(client: RESTClient, body: LolPatchUxResource) -> Result<(), Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-patch/v1/ux";
    client.put(url.to_owned(), Some(body)).await?;
    Ok(())
}

pub async fn put_lol_perks_v1_currentpage(client: RESTClient, id: i32) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-perks/v1/currentpage?id={{id}}", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.put(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn put_lol_perks_v1_pages_by_id(client: RESTClient, id: i32, body: LolPerksPerkPageResource) -> Result<LolPerksPerkPageResource, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-perks/v1/pages/{{id}}", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    let value = client.put(url.to_owned(), Some(body)).await?;
    Ok(serde_json::from_value::<LolPerksPerkPageResource>(value)?)
}

pub async fn put_lol_perks_v1_pages_validate(client: RESTClient, body: LolPerksValidatePageNameData) -> Result<LolPerksValidateItemSetNameResponse, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-perks/v1/pages/validate";
    let value = client.put(url.to_owned(), Some(body)).await?;
    Ok(serde_json::from_value::<LolPerksValidateItemSetNameResponse>(value)?)
}

pub async fn put_lol_perks_v1_perks_ack_gameplay_updated(client: RESTClient, ids: Vec<i32>) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-perks/v1/perks/ack-gameplay-updated?ids={{ids}}", &json!({"ids": ids}))?;
    let url = format!("{}", template_url);
    let value = client.put(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn put_lol_perks_v1_settings(client: RESTClient, body: LolPerksUISettings) -> Result<Value, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-perks/v1/settings";
    let value = client.put(url.to_owned(), Some(body)).await?;
    Ok(value)
}

pub async fn put_lol_player_behavior_v3_reform_card_by_id(client: RESTClient, id: String) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-player-behavior/v3/reform-card/{{id}}", &json!({"id": id}))?;
    let url = format!("{}", template_url);
    client.put(url.to_owned(), None).await?;
    Ok(())
}

pub async fn put_lol_player_preferences_v1_preference(client: RESTClient, body: LolPlayerPreferencesPlayerPreferences) -> Result<Value, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-player-preferences/v1/preference";
    let value = client.put(url.to_owned(), Some(body)).await?;
    Ok(value)
}

pub async fn put_lol_premade_voice_v1_capturedevices(client: RESTClient, handle: String) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-premade-voice/v1/capturedevices?handle={{handle}}", &json!({"handle": handle}))?;
    let url = format!("{}", template_url);
    client.put(url.to_owned(), None).await?;
    Ok(())
}

pub async fn put_lol_premade_voice_v1_participants_by_puuid_mute(client: RESTClient, puuid: String, muted: i32) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-premade-voice/v1/participants/{{puuid}}/mute?muted={{muted}}", &json!({"puuid": puuid, "muted": muted}))?;
    let url = format!("{}", template_url);
    client.put(url.to_owned(), None).await?;
    Ok(())
}

pub async fn put_lol_premade_voice_v1_participants_by_puuid_volume(client: RESTClient, puuid: String, volume: i32) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-premade-voice/v1/participants/{{puuid}}/volume?volume={{volume}}", &json!({"puuid": puuid, "volume": volume}))?;
    let url = format!("{}", template_url);
    client.put(url.to_owned(), None).await?;
    Ok(())
}

pub async fn put_lol_premade_voice_v1_self_activation_sensitivity(client: RESTClient, sensitivity: i32) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-premade-voice/v1/self/activationSensitivity?sensitivity={{sensitivity}}", &json!({"sensitivity": sensitivity}))?;
    let url = format!("{}", template_url);
    client.put(url.to_owned(), None).await?;
    Ok(())
}

pub async fn put_lol_premade_voice_v1_self_input_mode(client: RESTClient, input_mode: LolPremadeVoiceInputMode) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-premade-voice/v1/self/inputMode?inputMode={{input_mode}}", &json!({"input_mode": input_mode}))?;
    let url = format!("{}", template_url);
    client.put(url.to_owned(), None).await?;
    Ok(())
}

pub async fn put_lol_premade_voice_v1_self_mic_level(client: RESTClient, mic_level: i32) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-premade-voice/v1/self/micLevel?micLevel={{mic_level}}", &json!({"mic_level": mic_level}))?;
    let url = format!("{}", template_url);
    client.put(url.to_owned(), None).await?;
    Ok(())
}

pub async fn put_lol_premade_voice_v1_self_mute(client: RESTClient, muted: i32) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-premade-voice/v1/self/mute?muted={{muted}}", &json!({"muted": muted}))?;
    let url = format!("{}", template_url);
    client.put(url.to_owned(), None).await?;
    Ok(())
}

pub async fn put_lol_regalia_v2_current_summoner_regalia(client: RESTClient, body: LolRegaliaRegaliaPreferences) -> Result<LolRegaliaRegaliaWithPreferences, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-regalia/v2/current-summoner/regalia";
    let value = client.put(url.to_owned(), Some(body)).await?;
    Ok(serde_json::from_value::<LolRegaliaRegaliaWithPreferences>(value)?)
}

pub async fn put_lol_settings_v1_account_by_category(client: RESTClient, category: String, body: LolSettingsSettingCategory) -> Result<Value, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-settings/v1/account/{{category}}", &json!({"category": category}))?;
    let url = format!("{}", template_url);
    let value = client.put(url.to_owned(), Some(body)).await?;
    Ok(value)
}

pub async fn put_lol_settings_v2_account_by_pp_type_by_category(client: RESTClient, pp_type: String, category: String, body: LolSettingsSettingCategory) -> Result<Value, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-settings/v2/account/{ppType}/{{category}}", &json!({"pp_type": pp_type, "category": category}))?;
    let url = format!("{}", template_url);
    let value = client.put(url.to_owned(), Some(body)).await?;
    Ok(value)
}

pub async fn put_lol_summoner_v1_current_summoner_icon(client: RESTClient, body: LolSummonerSummonerIcon) -> Result<LolSummonerSummoner, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/lol-summoner/v1/current-summoner/icon";
    let value = client.put(url.to_owned(), Some(body)).await?;
    Ok(serde_json::from_value::<LolSummonerSummoner>(value)?)
}

pub async fn put_lol_summoner_v1_current_summoner_profile_privacy(client: RESTClient, body: LolSummonerProfilePrivacySetting) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-summoner/v1/current-summoner/profile-privacy?body={{body}}", &json!({"body": body}))?;
    let url = format!("{}", template_url);
    let value = client.put(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn put_lol_tft_v1_tft_experiment_bucket(client: RESTClient, value: u8) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/lol-tft/v1/tft_experiment_bucket?value={{value}}", &json!({"value": value}))?;
    let url = format!("{}", template_url);
    let value = client.put(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn put_patcher_v1_ux(client: RESTClient, body: PatcherUxResource) -> Result<(), Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/patcher/v1/ux";
    client.put(url.to_owned(), Some(body)).await?;
    Ok(())
}

pub async fn put_player_notifications_v1_config(client: RESTClient, body: PlayerNotificationsPlayerNotificationConfigResource) -> Result<PlayerNotificationsPlayerNotificationConfigResource, Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/player-notifications/v1/config";
    let value = client.put(url.to_owned(), Some(body)).await?;
    Ok(serde_json::from_value::<PlayerNotificationsPlayerNotificationConfigResource>(value)?)
}

pub async fn put_player_notifications_v1_notifications_by_id(client: RESTClient, id: u64, notification: Value) -> Result<PlayerNotificationsPlayerNotificationResource, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/player-notifications/v1/notifications/{{id}}?notification={{notification}}", &json!({"id": id, "notification": notification}))?;
    let url = format!("{}", template_url);
    let value = client.put(url.to_owned(), None).await?;
    Ok(serde_json::from_value::<PlayerNotificationsPlayerNotificationResource>(value)?)
}

pub async fn put_riotclient_splash(client: RESTClient, splash: String) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/riotclient/splash?splash={{splash}}", &json!({"splash": splash}))?;
    let url = format!("{}", template_url);
    client.put(url.to_owned(), None).await?;
    Ok(())
}

pub async fn put_riotclient_ux_load_complete(client: RESTClient) -> Result<(), Box<dyn Error>> {
    let url = "/riotclient/ux-load-complete";
    client.put(url.to_owned(), None).await?;
    Ok(())
}

pub async fn put_riotclient_ux_state_ack(client: RESTClient, request_id: u32) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/riotclient/ux-state/ack?requestId={{request_id}}", &json!({"request_id": request_id}))?;
    let url = format!("{}", template_url);
    client.put(url.to_owned(), None).await?;
    Ok(())
}

pub async fn put_riotclient_v1_auth_tokens_by_auth_token(client: RESTClient, auth_token: String) -> Result<Value, Box<dyn Error>> {
    let reg = Handlebars::new();
    let template_url = reg.render_template("/riotclient/v1/auth-tokens/{{auth_token}}", &json!({"auth_token": auth_token}))?;
    let url = format!("{}", template_url);
    let value = client.put(url.to_owned(), None).await?;
    Ok(value)
}

pub async fn put_riotclient_v1_crash_reporting_environment(client: RESTClient, body: CrashReportingEnvironment) -> Result<(), Box<dyn Error>> {
    let body = serde_json::to_value(&body)?;
    let url = "/riotclient/v1/crash-reporting/environment";
    client.put(url.to_owned(), Some(body)).await?;
    Ok(())
}

