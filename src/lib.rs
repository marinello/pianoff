use std::error::Error;

/// Validates MIDI value input (0-127)
/// Returns validated value or default (0) with warning message
pub fn validate_midi_value(input: &str) -> (u8, Option<String>) {
    if input.trim().is_empty() {
        return (0, None);
    }
    
    match input.trim().parse::<u8>() {
        Ok(val) if val <= 127 => (val, None),
        Ok(val) => (0, Some(format!("Warning: Value {} is out of range (0-127). Using default value 0.", val))),
        Err(_) => (0, Some(format!("Warning: Invalid value '{}'. Using default value 0.", input.trim()))),
    }
}

/// Validates MIDI channel input (0-15)
/// Returns validated channel or default (0) with warning message
pub fn validate_midi_channel(input: &str) -> (u8, Option<String>) {
    if input.trim().is_empty() {
        return (0, None);
    }
    
    match input.trim().parse::<u8>() {
        Ok(ch) if ch <= 15 => (ch, None),
        Ok(ch) => (0, Some(format!("Warning: Channel {} is out of range (0-15). Using default channel 0.", ch))),
        Err(_) => (0, Some(format!("Warning: Invalid channel '{}'. Using default channel 0.", input.trim()))),
    }
}

/// Creates MIDI Control Change message for controller #122
/// Returns the 3-byte MIDI message array
pub fn create_midi_cc_122_message(value: u8, channel: u8) -> Result<[u8; 3], Box<dyn Error>> {
    if value > 127 {
        return Err(format!("Invalid MIDI value: {}. Must be 0-127.", value).into());
    }
    if channel > 15 {
        return Err(format!("Invalid MIDI channel: {}. Must be 0-15.", channel).into());
    }
    
    Ok([0xB0 + channel, 122, value])
}

/// Interprets MIDI value for Local Control
pub fn interpret_local_control_value(value: u8) -> &'static str {
    match value {
        0 => "Local Control Off",
        127 => "Local Control On",
        _ => "Custom Local Control Value",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_midi_value_valid_range() {
        // Test valid values within range
        assert_eq!(validate_midi_value("0"), (0, None));
        assert_eq!(validate_midi_value("64"), (64, None));
        assert_eq!(validate_midi_value("127"), (127, None));
        assert_eq!(validate_midi_value(" 100 "), (100, None)); // Test trimming
    }

    #[test]
    fn test_validate_midi_value_empty_input() {
        // Test empty input returns default
        assert_eq!(validate_midi_value(""), (0, None));
        assert_eq!(validate_midi_value("   "), (0, None));
    }

    #[test]
    fn test_validate_midi_value_out_of_range() {
        // Test values out of range
        let (value, warning) = validate_midi_value("128");
        assert_eq!(value, 0);
        assert!(warning.is_some());
        assert!(warning.unwrap().contains("out of range"));

        let (value, warning) = validate_midi_value("255");
        assert_eq!(value, 0);
        assert!(warning.is_some());
    }

    #[test]
    fn test_validate_midi_value_invalid_input() {
        // Test invalid input
        let (value, warning) = validate_midi_value("abc");
        assert_eq!(value, 0);
        assert!(warning.is_some());
        assert!(warning.unwrap().contains("Invalid value"));

        let (value, warning) = validate_midi_value("12.5");
        assert_eq!(value, 0);
        assert!(warning.is_some());
    }

    #[test]
    fn test_validate_midi_channel_valid_range() {
        // Test valid channels within range
        assert_eq!(validate_midi_channel("0"), (0, None));
        assert_eq!(validate_midi_channel("8"), (8, None));
        assert_eq!(validate_midi_channel("15"), (15, None));
        assert_eq!(validate_midi_channel(" 10 "), (10, None)); // Test trimming
    }

    #[test]
    fn test_validate_midi_channel_empty_input() {
        // Test empty input returns default
        assert_eq!(validate_midi_channel(""), (0, None));
        assert_eq!(validate_midi_channel("   "), (0, None));
    }

    #[test]
    fn test_validate_midi_channel_out_of_range() {
        // Test channels out of range
        let (channel, warning) = validate_midi_channel("16");
        assert_eq!(channel, 0);
        assert!(warning.is_some());
        assert!(warning.unwrap().contains("out of range"));

        let (channel, warning) = validate_midi_channel("255");
        assert_eq!(channel, 0);
        assert!(warning.is_some());
    }

    #[test]
    fn test_validate_midi_channel_invalid_input() {
        // Test invalid input
        let (channel, warning) = validate_midi_channel("xyz");
        assert_eq!(channel, 0);
        assert!(warning.is_some());
        assert!(warning.unwrap().contains("Invalid channel"));

        let (channel, warning) = validate_midi_channel("5.5");
        assert_eq!(channel, 0);
        assert!(warning.is_some());
    }

    #[test]
    fn test_create_midi_cc_122_message_valid() {
        // Test valid MIDI message creation
        assert_eq!(create_midi_cc_122_message(0, 0).unwrap(), [0xB0, 122, 0]);
        assert_eq!(create_midi_cc_122_message(127, 0).unwrap(), [0xB0, 122, 127]);
        assert_eq!(create_midi_cc_122_message(64, 5).unwrap(), [0xB5, 122, 64]);
        assert_eq!(create_midi_cc_122_message(100, 15).unwrap(), [0xBF, 122, 100]);
    }

    #[test]
    fn test_create_midi_cc_122_message_invalid_value() {
        // Test invalid MIDI value
        let result = create_midi_cc_122_message(128, 0);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid MIDI value"));

        let result = create_midi_cc_122_message(255, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_create_midi_cc_122_message_invalid_channel() {
        // Test invalid MIDI channel
        let result = create_midi_cc_122_message(0, 16);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid MIDI channel"));

        let result = create_midi_cc_122_message(0, 255);
        assert!(result.is_err());
    }

    #[test]
    fn test_interpret_local_control_value() {
        // Test Local Control value interpretation
        assert_eq!(interpret_local_control_value(0), "Local Control Off");
        assert_eq!(interpret_local_control_value(127), "Local Control On");
        assert_eq!(interpret_local_control_value(64), "Custom Local Control Value");
        assert_eq!(interpret_local_control_value(1), "Custom Local Control Value");
        assert_eq!(interpret_local_control_value(126), "Custom Local Control Value");
    }

    #[test]
    fn test_midi_message_structure() {
        // Test MIDI message structure for different channels
        for channel in 0..=15 {
            let message = create_midi_cc_122_message(64, channel).unwrap();
            assert_eq!(message[0], 0xB0 + channel); // Control Change + channel
            assert_eq!(message[1], 122); // Controller number
            assert_eq!(message[2], 64); // Value
        }
    }

    #[test]
    fn test_midi_message_boundary_values() {
        // Test boundary values for MIDI messages
        assert_eq!(create_midi_cc_122_message(0, 0).unwrap(), [0xB0, 122, 0]);
        assert_eq!(create_midi_cc_122_message(127, 15).unwrap(), [0xBF, 122, 127]);
        
        // Test just outside boundaries
        assert!(create_midi_cc_122_message(128, 0).is_err());
        assert!(create_midi_cc_122_message(0, 16).is_err());
    }

    #[test]
    fn test_error_handling_edge_cases() {
        // Test various error conditions
        let test_cases = vec![
            ("", 0, None),
            ("0", 0, None),
            ("127", 127, None),
            ("128", 0, Some("out of range")),
            ("abc", 0, Some("Invalid value")),
            ("-1", 0, Some("Invalid value")),
            ("12.34", 0, Some("Invalid value")),
        ];

        for (input, expected_value, expected_warning_contains) in test_cases {
            let (value, warning) = validate_midi_value(input);
            assert_eq!(value, expected_value, "Failed for input: '{}'", input);
            
            match expected_warning_contains {
                Some(expected_text) => {
                    assert!(warning.is_some(), "Expected warning for input: '{}'", input);
                    assert!(warning.unwrap().contains(expected_text), 
                           "Warning doesn't contain expected text for input: '{}'", input);
                }
                None => {
                    assert!(warning.is_none(), "Unexpected warning for input: '{}'", input);
                }
            }
        }
    }

    #[test]
    fn test_channel_validation_edge_cases() {
        // Test various channel validation scenarios
        let test_cases = vec![
            ("", 0, None),
            ("0", 0, None),
            ("15", 15, None),
            ("16", 0, Some("out of range")),
            ("xyz", 0, Some("Invalid channel")),
            ("-1", 0, Some("Invalid channel")),
            ("3.14", 0, Some("Invalid channel")),
        ];

        for (input, expected_channel, expected_warning_contains) in test_cases {
            let (channel, warning) = validate_midi_channel(input);
            assert_eq!(channel, expected_channel, "Failed for input: '{}'", input);
            
            match expected_warning_contains {
                Some(expected_text) => {
                    assert!(warning.is_some(), "Expected warning for input: '{}'", input);
                    assert!(warning.unwrap().contains(expected_text), 
                           "Warning doesn't contain expected text for input: '{}'", input);
                }
                None => {
                    assert!(warning.is_none(), "Unexpected warning for input: '{}'", input);
                }
            }
        }
    }
}