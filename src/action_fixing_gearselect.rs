use std::collections::HashMap;

use rusty_patio::streamdeck::generic::StreamDeckTarget;
use rusty_patio::streamdeck::{client::StreamDeckClient, events::event_received::EventReceived};
use the_bus_telemetry::api::{RequestConfig, send_telemetry_bus_cmd};
// Access items from the parent module (main.rs)
use super::{ActionInstance, get_value_or_empty, logger};

pub async fn handle_event_fixing_gearselect(
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
                    let mut gear = get_value_or_empty(&button.settings, "GearSelection");
                    if gear.is_empty() {
                        gear = "2".to_string();
                    }

                    let mut active = "off";
                    if button.state.to_string() == gear {
                        active = "on";
                    } else {
                        active = "off";
                    }

                    let g = match gear.as_str() {
                        "1" => "D",
                        "2" => "N",
                        "3" => "R",
                        _ => "N",
                    };

                    let image = format!("actions/assets/gear_{}_off.png", g);

                    let _ = client
                        .transmitter
                        .set_image(
                            event.context.clone(),
                            image,
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
                let mut gear = get_value_or_empty(&button.settings, "GearSelection");
                if gear.is_empty() {
                    gear = "2".to_string();
                }

                let g = match gear.as_str() {
                    "1" => "D",
                    "2" => "N",
                    "3" => "R",
                    _ => "N",
                };

                let cmd = format!("sendeventpress?event=SetGear{}", g);

                let _ = send_telemetry_bus_cmd(&config, &cmd).await;
            }
        }

        EventReceived::KeyUp(event) => {
            if let Some(button) = buttons.get_mut(&event.context) {
                let mut gear = get_value_or_empty(&button.settings, "GearSelection");
                if gear.is_empty() {
                    gear = "2".to_string();
                }

                let g = match gear.as_str() {
                    "1" => "D",
                    "2" => "N",
                    "3" => "R",
                    _ => "N",
                };

                let cmd = format!("sendeventrelease?event=SetGear{}", g);

                let _ = send_telemetry_bus_cmd(&config, &cmd).await;
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
