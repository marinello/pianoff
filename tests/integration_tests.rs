use midi_cc_sender::*;
use std::error::Error;

#[test]
fn test_complete_validation_flow() {
    // Test complete validation flow with various inputs
    let test_cases = vec![
        // (value_input, channel_input, expected_value, expected_channel, should_have_warnings)
        ("64", "5", 64, 5, false),
        ("0", "0", 0, 0, false),
        ("127", "15", 127, 15, false),
        ("", "", 0, 0, false), // Empty inputs use defaults
        ("128", "16", 0, 0, true), // Out of range values
        ("abc", "xyz", 0, 0, true), // Invalid inputs
    ];

    for (value_input, channel_input, expected_value, expected_channel, should_have_warnings) in test_cases {
        let (value, value_warning) = validate_midi_value(value_input);
        let (channel, channel_warning) = validate_midi_channel(channel_input);
        
        assert_eq!(value, expected_value, 
                  "Value validation failed for input: '{}'", value_input);
        assert_eq!(channel, expected_channel, 
                  "Channel validation failed for input: '{}'", channel_input);
        
        if should_have_warnings {
            assert!(value_warning.is_some() || channel_warning.is_some(),
                   "Expected warnings for inputs: '{}', '{}'", value_input, channel_input);
        }
    }
}

#[test]
fn test_midi_message_creation_flow() -> Result<(), Box<dyn Error>> {
    // Test the complete flow from validation to message creation
    let test_scenarios = vec![
        ("0", "0", [0xB0, 122, 0]),
        ("127", "15", [0xBF, 122, 127]),
        ("64", "8", [0xB8, 122, 64]),
    ];

    for (value_input, channel_input, expected_message) in test_scenarios {
        let (value, _) = validate_midi_value(value_input);
        let (channel, _) = validate_midi_channel(channel_input);
        let message = create_midi_cc_122_message(value, channel)?;
        
        assert_eq!(message, expected_message,
                  "Message creation failed for value: {}, channel: {}", value, channel);
    }

    Ok(())
}

#[test]
fn test_error_propagation() {
    // Test that errors are properly propagated through the system
    
    // Test invalid MIDI message creation
    let result = create_midi_cc_122_message(128, 0);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Invalid MIDI value"));

    let result = create_midi_cc_122_message(0, 16);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Invalid MIDI channel"));
}

#[test]
fn test_local_control_interpretation_flow() {
    // Test the complete flow including value interpretation
    let test_cases = vec![
        (0, "Local Control Off"),
        (127, "Local Control On"),
        (64, "Custom Local Control Value"),
        (1, "Custom Local Control Value"),
        (126, "Custom Local Control Value"),
    ];

    for (value, expected_interpretation) in test_cases {
        let interpretation = interpret_local_control_value(value);
        assert_eq!(interpretation, expected_interpretation,
                  "Interpretation failed for value: {}", value);
    }
}

#[test]
fn test_validation_with_whitespace() {
    // Test that validation handles whitespace correctly
    let whitespace_cases = vec![
        (" 64 ", 64),
        ("\t127\t", 127),
        ("\n0\n", 0),
        ("  100  ", 100),
    ];

    for (input, expected) in whitespace_cases {
        let (value, warning) = validate_midi_value(input);
        assert_eq!(value, expected, "Whitespace handling failed for: '{}'", input);
        assert!(warning.is_none(), "Unexpected warning for valid input with whitespace");
    }
}

#[test]
fn test_comprehensive_error_scenarios() {
    // Test various error scenarios that could occur in real usage
    
    // Test extreme values
    let extreme_values = vec![
        "999999",
        "-999999", 
        "18446744073709551615", // u64::MAX as string
    ];

    for extreme_value in extreme_values {
        let (value, warning) = validate_midi_value(extreme_value);
        assert_eq!(value, 0, "Extreme value should default to 0: {}", extreme_value);
        assert!(warning.is_some(), "Should have warning for extreme value: {}", extreme_value);
    }

    // Test special characters
    let special_chars = vec![
        "!@#$%",
        "64.0",
        "0x40", // Hex notation
        "64e0", // Scientific notation
    ];

    for special_input in special_chars {
        let (value, warning) = validate_midi_value(special_input);
        assert_eq!(value, 0, "Special character input should default to 0: {}", special_input);
        assert!(warning.is_some(), "Should have warning for special input: {}", special_input);
    }
}

#[test]
fn test_midi_protocol_compliance() {
    // Test that generated MIDI messages comply with MIDI protocol
    
    for channel in 0..=15 {
        for value in [0, 1, 64, 126, 127] {
            let message = create_midi_cc_122_message(value, channel).unwrap();
            
            // Check message structure
            assert_eq!(message.len(), 3, "MIDI message should be 3 bytes");
            
            // Check status byte (Control Change + channel)
            assert_eq!(message[0] & 0xF0, 0xB0, "Should be Control Change message");
            assert_eq!(message[0] & 0x0F, channel, "Channel should be encoded correctly");
            
            // Check controller number
            assert_eq!(message[1], 122, "Controller should be 122");
            
            // Check value
            assert_eq!(message[2], value, "Value should match input");
            
            // Ensure all bytes are valid MIDI data
            assert!(message[0] >= 0x80, "Status byte should have MSB set");
            assert!(message[1] < 0x80, "Data byte 1 should not have MSB set");
            assert!(message[2] < 0x80, "Data byte 2 should not have MSB set");
        }
    }
}