use std::collections::HashMap;
use std::fs::OpenOptions;

use std::io::Write;
use std::time::Duration;

use rusty_patio::{
    streamdeck::{
        arguments::StreamDeckArgs, client::StreamDeckClient, events::event_received::EventReceived,
        generic::StreamDeckTarget,
    },
    websocket::connect_streamdeck,
};

use crate::action_fixing_brake::handle_event_fixing_brake;
use crate::action_fixing_gearselect::handle_event_fixing_gearselect;
use crate::action_ignition::handle_event_ignition;
use crate::action_inbus::handle_event_inbus;
use crate::action_indicators::{
    get_indicator_image_off, get_indicator_image_on, handle_event_indicators,
};
use crate::action_passenger_doors::handle_event_passenger_doors;
use crate::action_stop_brake::handle_event_stop_brake;
use the_bus_telemetry::api::{get_current_vehicle_name, get_vehicle, RequestConfig};
use the_bus_telemetry::api2vehicle::get_vehicle_state_from_api;
use the_bus_telemetry::vehicle::{init_vehicle_state, print_vehicle_state};
use the_bus_telemetry::vehicle_diff::compare_vehicle_states;

mod action_fixing_brake;
mod action_fixing_gearselect;
mod action_ignition;
mod action_inbus;
mod action_indicators;
mod action_passenger_doors;
mod action_stop_brake;

const UUID_FIXING_BRAKE: &str = "de.thatzok.thebus.fixingbrake";
const UUID_INBUS: &str = "de.thatzok.thebus.inbus";
const UUID_GEARSELECT: &str = "de.thatzok.thebus.gearselect";
const UUID_IGNITION: &str = "de.thatzok.thebus.ignition";
const UUID_PASSENGER_DOORS: &str = "de.thatzok.thebus.dooraction";
const UUID_INDICATORS: &str = "de.thatzok.thebus.indicatorcontrol";
const UUID_STOP_BRAKE: &str = "de.thatzok.thebus.stopbrake";

struct ActionInstance {
    title: String,
    uuid: String,
    state: u8,
    value: i64,
    settings: HashMap<String, serde_json::Value>,
}

fn describe_event(event: &EventReceived) -> String {
    match event {
        EventReceived::ApplicationDidLaunch(_e) => "ApplicationDidLaunch".to_string(),
        EventReceived::ApplicationDidTerminate(_e) => "ApplicationDidTerminate".to_string(),
        EventReceived::DeviceDidConnect(_e) => "DeviceDidConnect".to_string(),
        EventReceived::DeviceDidDisconnect(_e) => "DeviceDidDisconnect".to_string(),
        EventReceived::DialPress(e) => format!(
            "DialPress {} {} payload={:?}",
            e.action, e.context, e.payload
        ),
        EventReceived::DialRotate(e) => format!(
            "DialRotate {} {} payload={:?}",
            e.action, e.context, e.payload
        ),
        EventReceived::DidReceiveSettings(e) => format!(
            "DidReceiveSettings {} {} payload={:?}",
            e.action, e.context, e.payload
        ),
        EventReceived::DidReceiveGlobalSettings(_e) => "DidReceiveGlobalSettings".to_string(),
        EventReceived::KeyDown(e) => {
            format!("KeyDown {} {} payload={:?}", e.action, e.context, e.payload)
        }
        EventReceived::KeyUp(e) => {
            format!("KeyUp {} {} payload={:?}", e.action, e.context, e.payload)
        }
        EventReceived::PropertyInspectorDidAppear(e) => {
            format!("PropertyInspectorDidAppear {} {}", e.action, e.context)
        }
        EventReceived::SendToPlugin(e) => format!(
            "SendToPlugin {} {} payload={:?}",
            e.action, e.context, e.payload
        ),
        EventReceived::SystemDidWakeUp(_e) => "SystemDidWakeUp".to_string(),
        EventReceived::TitleParametersDidChange(e) => format!(
            "TitleParametersDidChange {} {} payload={:?}",
            e.action, e.context, e.payload
        ),
        EventReceived::TouchTap(e) => format!(
            "TouchTap {} {} payload={:?}",
            e.action, e.context, e.payload
        ),
        EventReceived::UnknownEvent(e) => format!("UnknownEvent {}", e),
        EventReceived::EventDeserializationError(e) => format!("EventDeserializationError {}", e),
        EventReceived::WillAppear(e) => format!(
            "WillAppear {} {} payload={:?}",
            e.action, e.context, e.payload
        ),
        EventReceived::WillDisappear(e) => format!(
            "WillDisappear {} {} payload={:?}",
            e.action, e.context, e.payload
        ),
        _ => "Other event".to_string(),
    }
}

fn write_all_buttons_to_log<W: Write>(writer: &mut W, buttons: &HashMap<String, ActionInstance>) {
    let _ = writeln!(writer, "-- Buttons snapshot ({} entries) --", buttons.len());
    for (context, btn) in buttons.iter() {
        let settings_str = serde_json::to_string(&btn.settings)
            .unwrap_or_else(|_| "<invalid settings>".to_string());
        let _ = writeln!(
            writer,
            "context={} title='{}' uuid={} state={} value={} settings={}",
            context, btn.title, btn.uuid, btn.state, btn.value, settings_str
        );
    }
    let _ = writeln!(writer, "-- End buttons snapshot --");
}

fn logger(msg: &str) {
    let mut log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("streamdeck.log")
        .expect("Failed to open or create streamdeck.log");

    writeln!(log_file, "{}", msg);
}
fn get_value_or_empty(map: &HashMap<String, serde_json::Value>, key: &str) -> String {
    let bla = map.get(key).map(|s| s.clone());
    if bla.is_none() {
        return "".to_string();
    }
    let fasel: serde_json::Value = bla.unwrap();
    let ret = fasel.as_str().unwrap_or("");
    ret.to_string()
}

async fn set_value_for_uuid(
    buttons: &mut HashMap<String, ActionInstance>,
    uuid: &str,
    value: i64,
    client: &mut StreamDeckClient,
) {
    for (context, btn) in buttons.iter_mut() {
        if btn.uuid == uuid {
            if btn.value != value {
                btn.value = value;
                client
                    .transmitter
                    .set_title(
                        context.clone(),
                        btn.value.to_string() + "%",
                        StreamDeckTarget::HARDWARE_AND_SOFTWARE,
                        None,
                    )
                    .await
                    .unwrap();
            }
        }
    }
}

async fn set_state_for_uuid(
    buttons: &mut HashMap<String, ActionInstance>,
    uuid: &str,
    state: u8,
    client: &mut StreamDeckClient,
) {
    for (context, btn) in buttons.iter_mut() {
        if btn.uuid == uuid {
            if btn.state != state {
                btn.state = state;
                client
                    .transmitter
                    .set_state(context.clone(), btn.state)
                    .await
            }
        }
    }
}

async fn set_title_for_uuid(
    buttons: &mut HashMap<String, ActionInstance>,
    uuid: &str,
    value: String,
    client: &mut StreamDeckClient,
) {
    for (context, btn) in buttons.iter_mut() {
        if btn.uuid == uuid {
            client
                .transmitter
                .set_title(
                    context.clone(),
                    value.clone(),
                    StreamDeckTarget::HARDWARE_AND_SOFTWARE,
                    None,
                )
                .await
                .unwrap();
        }
    }
}
async fn set_gearselect_for_uuid(
    buttons: &mut HashMap<String, ActionInstance>,
    uuid: &str,
    state: u8,
    client: &mut StreamDeckClient,
) {
    for (context, btn) in buttons.iter_mut() {
        if btn.uuid == uuid {
            if btn.state != state {
                btn.state = state;
                let mut gear = get_value_or_empty(&btn.settings, "GearSelection");
                if gear.is_empty() {
                    gear = "2".to_string();
                }

                let mut active = "off";
                if btn.state.to_string() == gear {
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

                let image = format!("actions/assets/gear_{}_{}.png", g, active);

                let _ = client
                    .transmitter
                    .set_image(
                        context.clone(),
                        image,
                        StreamDeckTarget::HARDWARE_AND_SOFTWARE,
                        None,
                    )
                    .await;
            }
        }
    }
}
async fn set_door_lamps_for_uuid(
    buttons: &mut HashMap<String, ActionInstance>,
    uuid: &str,
    lamps: [u8; 5],
    client: &mut StreamDeckClient,
) {
    for (context, btn) in buttons.iter_mut() {
        if btn.uuid == uuid {
            // first we have to determine which index is our state
            let mut doorselector = get_value_or_empty(&btn.settings, "DoorSelector");
            if doorselector.is_empty() {
                doorselector = "Door 1".to_string();
            }

            let doorindex = match doorselector.as_str() {
                "Door 1" => 1,
                "Door 2" => 2,
                "Door 3" => 3,
                "Door 4" => 4,
                _ => 0,
            };

            let state = lamps[doorindex];

            if btn.state != state {
                btn.state = state;
                let mut gear = get_value_or_empty(&btn.settings, "GearSelection");
                if gear.is_empty() {
                    gear = "2".to_string();
                }

                let mut active = "off";
                if btn.state == 1 {
                    active = "on";
                } else {
                    active = "off";
                }

                let mut image = format!("actions/assets/doorbutton_{}.png", active);

                if doorselector == "Clearance" {
                    image = format!("actions/assets/doorclearance_{}.png", active);
                }

                let _ = client
                    .transmitter
                    .set_image(
                        context.clone(),
                        image,
                        StreamDeckTarget::HARDWARE_AND_SOFTWARE,
                        None,
                    )
                    .await;
            }
        }
    }
}

async fn set_indicator_lamps_for_uuid(
    buttons: &mut HashMap<String, ActionInstance>,
    uuid: &str,
    lamps: [u8; 3],
    client: &mut StreamDeckClient,
) {
    for (context, btn) in buttons.iter_mut() {
        if btn.uuid == uuid {
            // first we have to determine which index is our state
            let mut indicatorselector = get_value_or_empty(&btn.settings, "IndicatorSelector");
            if indicatorselector.is_empty() {
                indicatorselector = "IndicatorLeft".to_string();
            }

            let indicatorindex = match indicatorselector.as_str() {
                "WarningLights" => 0,
                "IndicatorLeft" => 1,
                "IndicatorRight" => 2,
                _ => 0,
            };

            let state = lamps[indicatorindex];

            if btn.state != state {
                btn.state = state;
                let mut image = "".to_string();
                if (state == 0) {
                    image = get_indicator_image_off(indicatorselector.as_str());
                } else {
                    image = get_indicator_image_on(indicatorselector.as_str());
                }

                let _ = client
                    .transmitter
                    .set_image(
                        context.clone(),
                        image,
                        StreamDeckTarget::HARDWARE_AND_SOFTWARE,
                        None,
                    )
                    .await;
            }
        }
    }
}

#[tokio::main(worker_threads = 1)]
async fn main() {
    let args = StreamDeckArgs::new();
    let client = connect_streamdeck(&args).await;
    let mut buttons: HashMap<String, ActionInstance> = HashMap::new();

    let mut log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("streamdeck.log")
        .expect("Failed to open or create streamdeck.log");

    let mut vehicle_name = "".to_string();

    let mut config = RequestConfig::new();
    // config.debugging = true;

    let mut vehicle_state = init_vehicle_state();

    let mut zaehler = 0;

    let mut door_lamps = [0; 5];

    let mut indicator_lamps = [0; 3];
    let mut blinken_lights = 0;

    if let Ok(mut client) = client {
        let mut ticker = tokio::time::interval(Duration::from_millis(300));

        loop {
            tokio::select! {
                                maybe_event = client.received_events.recv() => {
                                    match maybe_event {
                                                        Some(event) => {

                                                                if config.debugging {
                                                                    let line = describe_event(&event);
                                                                    if let Err(e) = writeln!(log_file, "{}", line) {
                                                                        eprintln!("Failed to write to log file: {}", e);
                                                                    }
                                                                    write_all_buttons_to_log(&mut log_file, &buttons);
                                                                }

                                                            let action = match &event {
                                                                EventReceived::WillAppear(e) => e.action.clone(),
                                                                EventReceived::WillDisappear(e) => e.action.clone(),
                                                                EventReceived::TitleParametersDidChange(e) => e.action.clone(),
                                                                EventReceived::KeyDown(e) => e.action.clone() ,
                                                                EventReceived::KeyUp(e) => e.action.clone(),
                                                                EventReceived::PropertyInspectorDidAppear(e) => e.action.clone() ,
                                                                EventReceived::SendToPlugin(e) => e.action.clone() ,
                                                                EventReceived::TouchTap(e) => e.action.clone(),
                                                                EventReceived::DidReceiveSettings(e) => e.action.clone(),
                                                                EventReceived::DialPress(e) => e.action.clone() ,
                                                                EventReceived::DialRotate(e) => e.action.clone(),
                                                                _ => "".to_string(),
                                                            };

                                                            // we only care about events that have an action entry (are about a button/instance )
                                                            if action == UUID_INBUS { handle_event_inbus(event,&config, &mut buttons, &mut client).await; }
                                                            else if action == UUID_FIXING_BRAKE { handle_event_fixing_brake(event,&config, &mut buttons, &mut client).await; }
                                                            else if action == UUID_GEARSELECT { handle_event_fixing_gearselect(event,&config, &mut buttons, &mut client).await; }
                                                            else if action == UUID_IGNITION { handle_event_ignition(event,&config, &mut buttons, &mut client).await; }
                                                            else if action == UUID_PASSENGER_DOORS { handle_event_passenger_doors(event,&config, &mut buttons, &mut client).await; }
                                                            else if action == UUID_INDICATORS { handle_event_indicators(event,&config, &mut buttons, &mut client).await; }
                                                            else if action == UUID_STOP_BRAKE { handle_event_stop_brake(event,&config, &mut buttons, &mut client).await; }

                                                        }
                                                        None => break,
                                    }
                                },

                                _ = ticker.tick() => {

                                                    if (vehicle_name.is_empty()) || (zaehler>10){
                                                        config.vehicle_name = "Current".to_string();
                                                        vehicle_name = get_current_vehicle_name(&config).await;
                                                        zaehler = 0;
                                                    }

                                                    if vehicle_name.is_empty() {
                                                        vehicle_state = init_vehicle_state();
                                                        set_state_for_uuid(&mut buttons, UUID_INBUS, 0, &mut client).await;

                                                    } else {

                                                    config.vehicle_name = vehicle_name.clone();

                                                    let vehicle_response = get_vehicle(&config).await;
                                                    if vehicle_response.is_err() {
                                                        // println!("Error getting vehicle data in JSON.");
                                                        vehicle_name = "".to_string();
                                                        zaehler = 12;

                                                    } else {

                                                    zaehler = zaehler + 1;

                                                    let vehicle = vehicle_response.unwrap();
                                                    // println!("{:?}", vehicle);
                                                    if config.vehicle_model != vehicle.vehicle_model {
                                                        config.vehicle_model = vehicle.vehicle_model.clone();
                                                    }

                                                    let new_vehicle_state = get_vehicle_state_from_api(vehicle);
                                                    if config.debugging {
                                                        print_vehicle_state(&new_vehicle_state);
                                                    }

                                                    if config.debugging {
                                                        compare_vehicle_states(&vehicle_state, &new_vehicle_state, false);
                                                    }

                                                    vehicle_state = new_vehicle_state;

                                                    if blinken_lights == 0 {
                                                        blinken_lights = 1;
                                                    } else {
                                                        blinken_lights = 0;
                                                    }
                                                    // indicator: left:1, right: 2, none/off: 0
                                                    // lights_warning: 1:on 0:off
                                                    // indicator_lamps[0]=blinken_lights;

                                                    indicator_lamps[0]=vehicle_state.lights_warning;
                                                    indicator_lamps[1]=0;
                                                    indicator_lamps[2]=0;

                                                    if (vehicle_state.indicator == 1) || (vehicle_state.lights_warning == 1) {
                                                        indicator_lamps[1]=blinken_lights;
                                                    }
                                                    if (vehicle_state.indicator == 2) || (vehicle_state.lights_warning == 1) {
                                                        indicator_lamps[2]=blinken_lights;
                                                    }
                                                    
                                                    set_indicator_lamps_for_uuid(&mut buttons, UUID_INDICATORS, indicator_lamps, &mut client).await;


                                                    set_state_for_uuid(&mut buttons, UUID_INBUS, 1, &mut client).await;
                                                    set_state_for_uuid(&mut buttons, UUID_FIXING_BRAKE, vehicle_state.fixing_brake, &mut client).await;

                                                    set_gearselect_for_uuid(&mut buttons, UUID_GEARSELECT, vehicle_state.gear_selector, &mut client).await;

                                                    // lamps
                                                    door_lamps[0]=vehicle_state.doors;
                                                    door_lamps[1]=vehicle_state.lights_front_door;
                                                    door_lamps[2]=vehicle_state.lights_second_door;
                                                    door_lamps[3]=vehicle_state.lights_third_door;
                                                    door_lamps[4]=vehicle_state.lights_fourth_door;
                                                    set_door_lamps_for_uuid(&mut buttons, UUID_PASSENGER_DOORS, door_lamps, &mut client).await;

                                                    set_state_for_uuid(&mut buttons, UUID_STOP_BRAKE, vehicle_state.lights_stop_brake, &mut client).await;

                                                    }
                                        }

                                }
            }
        }
    }
}
