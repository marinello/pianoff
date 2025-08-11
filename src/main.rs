use midi_cc_sender::{
    create_midi_cc_122_message, interpret_local_control_value, validate_midi_channel,
    validate_midi_value,
};
use midir::{MidiOutput, MidiOutputConnection};
use std::error::Error;
use std::io::{self, Write};

/// Lists available MIDI output ports and prompts user for selection
/// Returns an established MIDI connection or error
fn list_and_select_port() -> Result<MidiOutputConnection, Box<dyn Error>> {
    let midi_out = MidiOutput::new("MIDI CC Sender")?;

    // Get available output ports
    let out_ports = midi_out.ports();

    // Handle case when no MIDI ports are available
    if out_ports.is_empty() {
        return Err("No MIDI output ports available.".into());
    }

    // Display available ports with numbered list
    println!("Available MIDI ports:");
    for (i, port) in out_ports.iter().enumerate() {
        let port_name = midi_out
            .port_name(port)
            .unwrap_or_else(|_| format!("Unknown Port {}", i));
        println!("{}: {}", i, port_name);
    }

    // Prompt user for port selection
    print!("Select a port by number: ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    // Parse and validate port selection
    let port_index: usize = input
        .trim()
        .parse()
        .map_err(|_| "Invalid input: Please enter a valid number")?;

    if port_index >= out_ports.len() {
        return Err(format!(
            "Invalid port selection: Port {} does not exist. Available ports: 0-{}",
            port_index,
            out_ports.len() - 1
        )
        .into());
    }

    // Establish connection to selected port
    let selected_port = &out_ports[port_index];
    let port_name = midi_out
        .port_name(selected_port)
        .unwrap_or_else(|_| format!("Port {}", port_index));

    let connection = midi_out
        .connect(selected_port, &format!("midi-cc-sender-{}", port_index))
        .map_err(|e| format!("Failed to connect to MIDI port '{}': {}", port_name, e))?;

    println!("Connected to MIDI port: {}", port_name);

    Ok(connection)
}

/// Prompts user for MIDI value and channel with validation and default handling
/// Returns tuple of (value, channel) or error
fn get_user_input() -> Result<(u8, u8), Box<dyn Error>> {
    // Get MIDI value (0-127)
    print!("Enter MIDI value (0-127, default 0): ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let (value, warning) = validate_midi_value(&input);
    if let Some(warning_msg) = warning {
        println!("{}", warning_msg);
    }

    // Get MIDI channel (0-15)
    print!("Enter MIDI channel (0-15, default 0): ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let (channel, warning) = validate_midi_channel(&input);
    if let Some(warning_msg) = warning {
        println!("{}", warning_msg);
    }

    // Display interpretation of value
    let control_state = interpret_local_control_value(value);
    println!(
        "Using MIDI value: {} ({}) on channel: {}",
        value, control_state, channel
    );

    Ok((value, channel))
}

/// Creates and sends MIDI Control Change message #122 (Local Control)
/// Displays confirmation message and handles transmission errors
fn send_midi_cc_122(
    connection: &mut MidiOutputConnection,
    value: u8,
    channel: u8,
) -> Result<(), Box<dyn Error>> {
    // Create MIDI Control Change message using helper function
    let midi_message = create_midi_cc_122_message(value, channel)?;

    // Send the message through the MIDI connection
    connection
        .send(&midi_message)
        .map_err(|e| format!("Failed to send MIDI message: {}", e))?;

    // Display confirmation message
    let control_state = interpret_local_control_value(value);
    let control_display = if value == 0 || value == 127 {
        control_state.to_string()
    } else {
        format!("Local Control Value {}", value)
    };

    println!(
        "✓ Successfully sent MIDI CC #122: {} (value: {}) on channel {}",
        control_display, value, channel
    );

    Ok(())
}

/// Main application function that orchestrates the complete workflow
fn main() -> Result<(), Box<dyn Error>> {
    // Display welcome message and instructions
    println!("MIDI Control Change #122 (Local Control) Sender");
    println!("===============================================");
    println!();
    println!("This utility sends MIDI Control Change message #122 to toggle Local Control");
    println!("on MIDI devices. Local Control determines whether a MIDI keyboard's keys");
    println!("trigger its internal sounds (On) or only send MIDI data (Off).");
    println!();
    println!("Value 0   = Local Control Off (keys send MIDI only)");
    println!("Value 127 = Local Control On (keys trigger internal sounds)");
    println!();

    // Step 1: Discover and select MIDI port
    println!("Step 1: Select MIDI Output Port");
    println!("-------------------------------");
    let mut connection = list_and_select_port().map_err(|e| {
        eprintln!("Failed to establish MIDI connection: {}", e);
        e
    })?;

    println!();

    // Step 2: Get user input for value and channel
    println!("Step 2: Configure MIDI Parameters");
    println!("---------------------------------");
    let (value, channel) = get_user_input().map_err(|e| {
        eprintln!("Failed to get user input: {}", e);
        e
    })?;

    println!();

    // Step 3: Send MIDI message
    println!("Step 3: Send MIDI Message");
    println!("-------------------------");
    send_midi_cc_122(&mut connection, value, channel).map_err(|e| {
        eprintln!("Failed to send MIDI message: {}", e);
        e
    })?;

    println!();
    println!("Operation completed successfully!");
    println!("The MIDI device should now have updated Local Control settings.");

    // Connection is automatically closed when it goes out of scope
    Ok(())
}
