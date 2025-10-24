use std::collections::HashMap;

// Access items from the parent module (main.rs)
use super::{get_value_or_empty, ActionInstance};
use rusty_patio::streamdeck::generic::StreamDeckTarget;
use rusty_patio::streamdeck::{client::StreamDeckClient, events::event_received::EventReceived};
use the_bus_telemetry::api::{send_telemetry_bus_cmd, RequestConfig};

pub fn get_coin_title(coin: &str) -> String {
    let mut d = "";
    d = match coin {
        "Coins5" => "0.05 €",
        "Coins10" => "0.10 €",
        "Coins15" => "0.15 €",
        "Coins20" => "0.20 €",
        "Coins30" => "0.30 €",
        "Coins50" => "0.50 €",
        "Coins60" => "0.60 €",
        "Coins100" => "1.00 €",
        "Coins200" => "2.00 €",
        "Coins400" => "4.00 €",
        "Coins600" => "6.00 €",
        "Coins800" => "8.00 €",
        "Take Cash Money" => "Grab",
        _ => "Grab",
    };
    d.to_string()
}

pub async fn handle_event_change(
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
                    let autolabel = get_value_or_empty(&button.settings, "AutoLabel");
                    if autolabel == "True" {
                        let cashchangeselect =
                            get_value_or_empty(&button.settings, "CashChangeSelect");
                        let title = get_coin_title(&cashchangeselect);

                        let _ = client
                            .transmitter
                            .set_title(
                                event.context.clone(),
                                title.to_string(),
                                StreamDeckTarget::HARDWARE_AND_SOFTWARE,
                                None,
                            )
                            .await;
                    }
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

        EventReceived::KeyDown(event) => if let Some(button) = buttons.get_mut(&event.context) {},

        EventReceived::KeyUp(event) => {
            if let Some(button) = buttons.get_mut(&event.context) {
                let cashchangeselect = get_value_or_empty(&button.settings, "CashChangeSelect");

                let cmd = format!("sendevent?event={}", cashchangeselect);
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
