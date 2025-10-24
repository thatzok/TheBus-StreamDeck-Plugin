use std::collections::HashMap;

// Access items from the parent module (main.rs)
use super::{get_value_or_empty, ActionInstance};
use rusty_patio::streamdeck::generic::StreamDeckTarget;
use rusty_patio::streamdeck::{client::StreamDeckClient, events::event_received::EventReceived};
use the_bus_telemetry::api::{send_telemetry_bus_cmd, RequestConfig};

pub fn get_indicator_image_on(indicatorselector: &str) -> String {
    let mut d = "";

    d = match indicatorselector {
        "IndicatorRight" => "actions/assets/indicator_right_on.png",
        "IndicatorLeft" => "actions/assets/indicator_left_on.png",
        "WarningLights" => "actions/assets/warninglights_on.png",
        _ => "actions/assets/warninglights_on.png",
    };

    d.to_string()
}
pub fn get_indicator_image_off(indicatorselector: &str) -> String {
    let mut d = "";

    d = match indicatorselector {
        "IndicatorRight" => "actions/assets/indicator_right_off.png",
        "IndicatorLeft" => "actions/assets/indicator_left_off.png",
        "WarningLights" => "actions/assets/warninglights_off.png",
        _ => "actions/assets/warninglights_on.png",
    };

    d.to_string()
}

pub async fn handle_event_indicators(
    event: EventReceived,
    config: &RequestConfig,
    buttons: &mut HashMap<String, ActionInstance>,
    client: &mut StreamDeckClient,
) {
    match event {
        EventReceived::WillAppear(event) => {
            if !buttons.contains_key(&event.context) {
                let btn = ActionInstance {
                    title: "".to_string(),
                    uuid: event.action.clone(),
                    state: 0, // state: event.payload.state.unwrap_or(0),
                    value: 0,
                    settings: event.payload.settings,
                };
                buttons.insert(event.context.clone(), btn);
                client.transmitter.set_state(event.context.clone(), 0).await;

                if let Some(button) = buttons.get_mut(&event.context) {
                    let mut indicatorselector = get_value_or_empty(&button.settings, "IndicatorSelector");
                    if indicatorselector.is_empty() {
                        indicatorselector = "IndicatorLeft".to_string();
                    }

                    let mut image = get_indicator_image_off(indicatorselector.as_str());


                    let _ = client
                        .transmitter
                        .set_image(
                            event.context.clone(),
                            image.to_string(),
                            StreamDeckTarget::HARDWARE_AND_SOFTWARE,
                            None,
                        )
                        .await;
                }
            }
        }

        EventReceived::WillDisappear(event) => {
            if buttons.contains_key(&event.context) {
                buttons.remove(&event.context);
            }
        }

        EventReceived::TitleParametersDidChange(event) => {
            if let Some(button) = buttons.get_mut(&event.context) {
                if let Some(t) = event.payload.settings.get("title").and_then(|v| v.as_str()) {
                    button.title = t.to_string();
                }
            }
        }

        EventReceived::KeyDown(event) => {
            if let Some(button) = buttons.get_mut(&event.context) {
                let mut indicatorselector = get_value_or_empty(&button.settings, "IndicatorSelector");
                if indicatorselector.is_empty() {
                    indicatorselector = "IndicatorLeft".to_string();
                }


                let cmd = match indicatorselector.as_str() {
                    "IndicatorRight" => "sendevent?event=IndicatorUp",
                    "IndicatorLeft" => "sendevent?event=IndicatorDown",
                    "WarningLights" => "sendevent?event=ToggleWarningLights",
                    _ => "sendevent?event=ToggleWarningLights",
                };

                let _ = send_telemetry_bus_cmd(&config, cmd).await;

            }
        }

        EventReceived::KeyUp(event) => {
            if let Some(button) = buttons.get_mut(&event.context) {
/*
                let mut doorselector = get_value_or_empty(&button.settings, "DoorSelector");
                if doorselector.is_empty() {
                    doorselector = "Door 1".to_string();
                }

                let d = get_door_action(doorselector.as_str(), &config.vehicle_model);

                let cmd = format!("sendeventrelease?event={}", d);

                let _ = send_telemetry_bus_cmd(&config, &cmd).await;

 */
            }
        }

        EventReceived::DidReceiveSettings(event) => {
            if let Some(button) = buttons.get_mut(&event.context) {
                button.settings = event.payload.settings;
            }
        }

        _ => {}
    }
}
