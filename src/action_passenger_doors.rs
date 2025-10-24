use std::collections::HashMap;

// Access items from the parent module (main.rs)
use super::{get_value_or_empty, ActionInstance};
use rusty_patio::streamdeck::generic::StreamDeckTarget;
use rusty_patio::streamdeck::{client::StreamDeckClient, events::event_received::EventReceived};
use the_bus_telemetry::api::{send_telemetry_bus_cmd, RequestConfig};

pub fn get_door_action(doorselector: &str, model: &str) -> String {
    let mut d = "";

    if (model == "Citea LLE") {
        d = match doorselector {
            "Door 1" => "DoorFrontOpenClose",
            "Door 2" => "MiddleDoorOpenClose",
            "Door 3" => "RearDoorOpenClose",
            "Door 4" => "FourthDoorOpenClose",
            "Clearance" => "ToggleDoorClearance",
            _ => "DoorFrontOpenClose",
        }
    } else {
        d = match doorselector {
            "Door 1" => "DoorFrontOpenClose",
            "Door 2" => "DoorMiddleOpenClose",
            "Door 3" => "DoorRearOpenClose",
            "Door 4" => "DoorFourthOpenClose",
            "Clearance" => "ToggleDoorClearance",
            _ => "DoorFrontOpenClose",
        }
    }

    d.to_string()
}

pub async fn handle_event_passenger_doors(
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
                    let mut doorselector = get_value_or_empty(&button.settings, "DoorSelector");
                    if doorselector.is_empty() {
                        doorselector = "Door 1".to_string();
                    }

                    let mut image = "actions/assets/doorbutton_off.png";

                    if doorselector == "Clearance" {
                        image = "actions/assets/doorclearance_off.png";
                    }

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
                let mut doorselector = get_value_or_empty(&button.settings, "DoorSelector");
                if doorselector.is_empty() {
                    doorselector = "Door 1".to_string();
                }

                let d = get_door_action(doorselector.as_str(), &config.vehicle_model);

                let cmd = format!("sendeventpress?event={}", d);

                let _ = send_telemetry_bus_cmd(&config, &cmd).await;
            }
        }

        EventReceived::KeyUp(event) => {
            if let Some(button) = buttons.get_mut(&event.context) {
                let mut doorselector = get_value_or_empty(&button.settings, "DoorSelector");
                if doorselector.is_empty() {
                    doorselector = "Door 1".to_string();
                }

                let d = get_door_action(doorselector.as_str(), &config.vehicle_model);

                let cmd = format!("sendeventrelease?event={}", d);

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
