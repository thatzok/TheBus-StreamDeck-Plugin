use std::collections::HashMap;

use rusty_patio::streamdeck::{client::StreamDeckClient, events::event_received::EventReceived};
use the_bus_telemetry::api::{send_telemetry_bus_cmd, RequestConfig};
// Access items from the parent module (main.rs)
use super::ActionInstance;

pub async fn handle_event_stop_brake(
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
                /*
                if button.state == 0 {
                    button.state = 1
                } else {
                    button.state = 0
                }
                button.settings = event.payload.settings;
                client
                    .transmitter
                    .set_state(event.context.clone(), button.state)
                    .await;
                */
                let _ = send_telemetry_bus_cmd(&config, "sendevent?event=StopBrakeOnOff").await;

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
