use midi;
use portmidi;

use midi::{RawMessage, ToRawMessages};

use clap::{crate_authors, crate_version, Clap};

#[derive(Clap)]
#[clap(version = crate_version!(), author = crate_authors!())]
pub struct Opts {
    #[clap(long, short, about = "Show all available devices")]
    list_devices: bool,

    #[clap(long, short, about = "Select output device (either id or name)")]
    output_device: Option<String>,

    #[clap(
        long,
        short,
        default_value = "100",
        about = "How hard notes are pressed (0 to 255)"
    )]
    velocity: u8,

    #[clap(
        long,
        short,
        default_value = "3",
        about = "The initial mode before another one gets selected"
    )]
    initial_mode: u8,

    #[clap(
        long,
        short,
        default_value = "3",
        about = "This value will be added on the selected mode for octave selection"
    )]
    mode_offset: u8,
}

struct State {
    mode_index: u8,
}

fn main() {
    let ref opts = Opts::parse();

    let pm = portmidi::PortMidi::new().unwrap();

    if opts.list_devices {
        cmd_list_devices(&pm);
        return;
    }

    let selected_device = if let Some(output_device_name) = &opts.output_device {
        if let Some(device) = find_output_device(&pm, &output_device_name) {
            device
        } else {
            eprintln!(
                "Error: Failed to find given device. Use -l to list all available output devices."
            );
            std::process::exit(1);
        }
    } else {
        if let Some(device) = default_output_device(&pm) {
            device
        } else {
            eprintln!("Error: Failed to find a default output device. Use -l to see what output device are available (if any).");
            std::process::exit(1);
        }
    };

    println!(
        "Using midi output device: {} - {}",
        selected_device.id(),
        selected_device.name()
    );

    let mut out_port = pm.output_port(selected_device, 512).unwrap();

    out_port
        .write_event(to_portmidi_message(midi::Start))
        .unwrap();

    let mut state = State {
        mode_index: opts.initial_mode - 1,
    };

    println!("Mode: {}", state.mode_index + 1);

    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let line = line.trim_end();
        /*println!("EVENT");
        out_port
            .write_message(to_portmidi_message(midi::NoteOn(midi::Ch1, 69, 50)))
            .unwrap();*/

        handle_input(&mut out_port, &mut state, opts, line);
    }
}

fn default_midi_message() -> portmidi::MidiMessage {
    portmidi::MidiMessage {
        status: 0,
        data1: 0,
        data2: 0,
        data3: 0,
    }
}

fn to_portmidi_message(message: midi::Message) -> portmidi::MidiMessage {
    let raw_messages = message.to_raw_messages();
    if raw_messages.len() != 1 {
        panic!("Expected only 1 message for raw message: {:?}", message);
    }

    match raw_messages[0] {
        RawMessage::Raw(raw_status) => portmidi::MidiMessage {
            status: raw_status,
            ..default_midi_message()
        },
        RawMessage::Status(status) => portmidi::MidiMessage {
            status,
            ..default_midi_message()
        },
        RawMessage::StatusData(status, data1) => portmidi::MidiMessage {
            status,
            data1,
            ..default_midi_message()
        },
        RawMessage::StatusDataData(status, data1, data2) => portmidi::MidiMessage {
            status,
            data1,
            data2,
            ..default_midi_message()
        },
    }
}

fn cmd_list_devices(pm: &portmidi::PortMidi) {
    let available_devices = pm.devices().unwrap();

    println!("Input devices: (not supported)");
    for device in &available_devices {
        if device.direction() == portmidi::Direction::Input {
            println!(" - {}: {}", device.id(), device.name())
        }
    }
    println!("");

    println!("Output devices:");
    for device in &available_devices {
        if device.direction() == portmidi::Direction::Output {
            println!(" - {}: {}", device.id(), device.name())
        }
    }
}

fn find_output_device(pm: &portmidi::PortMidi, device_name: &str) -> Option<portmidi::DeviceInfo> {
    let device_name = device_name.to_lowercase();

    for device in &pm.devices().unwrap() {
        if device.direction() == portmidi::Direction::Output {
            if device.id().to_string() == device_name {
                return Some(device.clone());
            }
            if device.name().to_lowercase() == device_name {
                return Some(device.clone());
            }
        }
    }
    None
}

fn default_output_device(pm: &portmidi::PortMidi) -> Option<portmidi::DeviceInfo> {
    for device in &pm.devices().unwrap() {
        if device.direction() == portmidi::Direction::Output {
            return Some(device.clone());
        }
    }
    None
}

fn handle_input(
    out_port: &mut portmidi::OutputPort,
    state: &mut State,
    opts: &Opts,
    input_line: &str,
) {
    let is_pressed = if input_line.starts_with("PRESS ") {
        true
    } else if input_line.starts_with("RELEASE ") {
        false
    } else {
        eprintln!("Invalid input: {}", input_line);
        return;
    };

    let words: Vec<_> = input_line.split(" ").into_iter().collect();
    let key_name = words[1];

    if is_pressed && key_name.starts_with('m') {
        let mode_str = key_name
            .replace("m", "")
            .replace("\n", "")
            .replace("\r", "")
            .trim()
            .to_owned();
        let mode_index: i32 = match mode_str.parse() {
            Ok(num) => num,
            Err(e) => {
                eprintln!("Failed to parse mode id \"{}\". Error: {}", mode_str, e);
                return;
            }
        };

        if mode_index > 8 || mode_index < 1 {
            // we use only 5. But up to 8 should be allowed, too
            eprintln!("Mode not supported: {}", mode_index);
            return;
        }

        state.mode_index = mode_index as u8 - 1;
        println!("Mode: {}", state.mode_index + 1);
        return;
    } else if key_name.starts_with('m') {
        return;
    }

    // Play note
    match to_note_key(state.mode_index, opts, key_name) {
        Ok(note) => {
            if is_pressed {
                out_port
                    .write_message(to_portmidi_message(midi::NoteOn(
                        midi::Ch1,
                        note,
                        opts.velocity,
                    )))
                    .unwrap();
            } else {
                out_port
                    .write_message(to_portmidi_message(midi::NoteOff(
                        midi::Ch1,
                        note,
                        opts.velocity,
                    )))
                    .unwrap();
            }
        }
        Err(e) => {
            eprintln!("Failed to find key \"{}\": {}", key_name, e);
        }
    }
}

fn to_note_key(mode_index: u8, opts: &Opts, key_name: &str) -> Result<u8, String> {
    let keys = vec![
        "w1", "b1", "w2", "b2", "w3", "w4", "b3", "w5", "b4", "w6", "b5", "w7",
        "w8", "b6", "w9", "b7", "w10", "w11", "b8", "w12", "b9", "w13", "b10", "w14", "w15",
    ];
    if !keys.contains(&key_name) {
        return Err(format!("Unknown key_name: {}", key_name));
    }

    Ok(((mode_index + opts.mode_offset) * 12)
        + (keys.iter().position(|key| &key_name == key).unwrap() as u8))
}
