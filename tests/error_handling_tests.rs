use midi_cc_sender::*;

#[test]
fn test_midi_value_validation_comprehensive_errors() {
    // Test comprehensive error scenarios for MIDI value validation
    let error_cases = vec![
        // (input, expected_value, expected_warning_keyword)
        ("128", 0, "out of range"),
        ("129", 0, "out of range"),
        ("255", 0, "out of range"), // u8::MAX, parses but out of MIDI range
        ("256", 0, "Invalid value"), // > u8::MAX, parsing fails
        ("999", 0, "Invalid value"), // > u8::MAX, parsing fails
        ("18446744073709551615", 0, "Invalid value"), // u64::MAX
        ("-1", 0, "Invalid value"),
        ("-128", 0, "Invalid value"),
        ("abc", 0, "Invalid value"),
        ("12.5", 0, "Invalid value"),
        ("0x40", 0, "Invalid value"), // Hex
        ("64e0", 0, "Invalid value"), // Scientific notation
        ("!@#$", 0, "Invalid value"),
        ("", 0, ""), // Empty should not have warning
        ("   ", 0, ""), // Whitespace only should not have warning
    ];

    for (input, expected_value, expected_keyword) in error_cases {
        let (value, warning) = validate_midi_value(input);
        assert_eq!(value, expected_value, "Value mismatch for input: '{}'", input);
        
        if expected_keyword.is_empty() {
            assert!(warning.is_none(), "Unexpected warning for input: '{}'", input);
        } else {
            assert!(warning.is_some(), "Expected warning for input: '{}'", input);
            let warning_msg = warning.unwrap();
            assert!(warning_msg.contains(expected_keyword), 
                   "Warning '{}' doesn't contain '{}' for input: '{}'", 
                   warning_msg, expected_keyword, input);
        }
    }
}

#[test]
fn test_midi_channel_validation_comprehensive_errors() {
    // Test comprehensive error scenarios for MIDI channel validation
    let error_cases = vec![
        // (input, expected_channel, expected_warning_keyword)
        ("16", 0, "out of range"),
        ("17", 0, "out of range"),
        ("255", 0, "out of range"), // u8::MAX, parses but out of MIDI range
        ("256", 0, "Invalid channel"), // > u8::MAX, parsing fails
        ("999", 0, "Invalid channel"), // > u8::MAX, parsing fails
        ("18446744073709551615", 0, "Invalid channel"), // u64::MAX
        ("-1", 0, "Invalid channel"),
        ("-16", 0, "Invalid channel"),
        ("xyz", 0, "Invalid channel"),
        ("3.14", 0, "Invalid channel"),
        ("0xF", 0, "Invalid channel"), // Hex
        ("1e1", 0, "Invalid channel"), // Scientific notation
        ("@#$%", 0, "Invalid channel"),
        ("", 0, ""), // Empty should not have warning
        ("   ", 0, ""), // Whitespace only should not have warning
    ];

    for (input, expected_channel, expected_keyword) in error_cases {
        let (channel, warning) = validate_midi_channel(input);
        assert_eq!(channel, expected_channel, "Channel mismatch for input: '{}'", input);
        
        if expected_keyword.is_empty() {
            assert!(warning.is_none(), "Unexpected warning for input: '{}'", input);
        } else {
            assert!(warning.is_some(), "Expected warning for input: '{}'", input);
            let warning_msg = warning.unwrap();
            assert!(warning_msg.contains(expected_keyword), 
                   "Warning '{}' doesn't contain '{}' for input: '{}'", 
                   warning_msg, expected_keyword, input);
        }
    }
}

#[test]
fn test_midi_message_creation_error_boundaries() {
    // Test error boundaries for MIDI message creation
    
    // Test value boundaries
    assert!(create_midi_cc_122_message(127, 0).is_ok()); // Max valid value
    assert!(create_midi_cc_122_message(128, 0).is_err()); // Min invalid value
    assert!(create_midi_cc_122_message(255, 0).is_err()); // Max u8 value
    
    // Test channel boundaries
    assert!(create_midi_cc_122_message(0, 15).is_ok()); // Max valid channel
    assert!(create_midi_cc_122_message(0, 16).is_err()); // Min invalid channel
    assert!(create_midi_cc_122_message(0, 255).is_err()); // Max u8 channel
    
    // Test combined invalid scenarios
    assert!(create_midi_cc_122_message(128, 16).is_err()); // Both invalid
    assert!(create_midi_cc_122_message(255, 255).is_err()); // Both max invalid
}

#[test]
fn test_error_message_content() {
    // Test that error messages contain appropriate information
    
    // Test value error messages
    let value_error = create_midi_cc_122_message(128, 0).unwrap_err();
    assert!(value_error.to_string().contains("Invalid MIDI value"));
    assert!(value_error.to_string().contains("128"));
    assert!(value_error.to_string().contains("0-127"));
    
    let value_error_255 = create_midi_cc_122_message(255, 0).unwrap_err();
    assert!(value_error_255.to_string().contains("Invalid MIDI value"));
    assert!(value_error_255.to_string().contains("255"));
    
    // Test channel error messages
    let channel_error = create_midi_cc_122_message(0, 16).unwrap_err();
    assert!(channel_error.to_string().contains("Invalid MIDI channel"));
    assert!(channel_error.to_string().contains("16"));
    assert!(channel_error.to_string().contains("0-15"));
    
    let channel_error_255 = create_midi_cc_122_message(0, 255).unwrap_err();
    assert!(channel_error_255.to_string().contains("Invalid MIDI channel"));
    assert!(channel_error_255.to_string().contains("255"));
}

#[test]
fn test_validation_warning_message_content() {
    // Test that validation warning messages contain appropriate information
    
    // Test value warning messages
    let (_, warning) = validate_midi_value("128");
    let warning_msg = warning.unwrap();
    assert!(warning_msg.contains("Warning"));
    assert!(warning_msg.contains("128"));
    assert!(warning_msg.contains("out of range"));
    assert!(warning_msg.contains("0-127"));
    assert!(warning_msg.contains("default value 0"));
    
    let (_, warning) = validate_midi_value("abc");
    let warning_msg = warning.unwrap();
    assert!(warning_msg.contains("Warning"));
    assert!(warning_msg.contains("Invalid value"));
    assert!(warning_msg.contains("abc"));
    assert!(warning_msg.contains("default value 0"));
    
    // Test channel warning messages
    let (_, warning) = validate_midi_channel("16");
    let warning_msg = warning.unwrap();
    assert!(warning_msg.contains("Warning"));
    assert!(warning_msg.contains("16"));
    assert!(warning_msg.contains("out of range"));
    assert!(warning_msg.contains("0-15"));
    assert!(warning_msg.contains("default channel 0"));
    
    let (_, warning) = validate_midi_channel("xyz");
    let warning_msg = warning.unwrap();
    assert!(warning_msg.contains("Warning"));
    assert!(warning_msg.contains("Invalid channel"));
    assert!(warning_msg.contains("xyz"));
    assert!(warning_msg.contains("default channel 0"));
}

#[test]
fn test_unicode_and_special_character_handling() {
    // Test handling of unicode and special characters
    let special_inputs = vec![
        "🎵", // Musical note emoji
        "½", // Unicode fraction
        "∞", // Infinity symbol
        "π", // Pi symbol
        "中文", // Chinese characters
        "العربية", // Arabic text
        "русский", // Cyrillic text
        "\n64\n", // Newlines
        "\t127\t", // Tabs
        "\r\n0\r\n", // Windows line endings
    ];
    
    for input in special_inputs {
        let (value, warning) = validate_midi_value(input);
        // All special characters should result in default value with warning
        // except for whitespace-wrapped valid numbers
        if input.trim().chars().all(|c| c.is_ascii_digit()) {
            let expected_val: u8 = input.trim().parse().unwrap_or(0);
            if expected_val <= 127 {
                assert_eq!(value, expected_val, "Failed for input: '{}'", input);
                assert!(warning.is_none(), "Unexpected warning for valid input: '{}'", input);
            } else {
                assert_eq!(value, 0, "Should default to 0 for out of range: '{}'", input);
                assert!(warning.is_some(), "Expected warning for out of range: '{}'", input);
            }
        } else {
            assert_eq!(value, 0, "Should default to 0 for special input: '{}'", input);
            assert!(warning.is_some(), "Expected warning for special input: '{}'", input);
        }
    }
}

#[test]
fn test_concurrent_validation_safety() {
    // Test that validation functions are safe for concurrent use
    use std::thread;
    use std::sync::Arc;
    
    let test_inputs = Arc::new(vec![
        "0", "64", "127", "128", "abc", "", "   ", "255", "-1", "12.5"
    ]);
    
    let mut handles = vec![];
    
    // Spawn multiple threads to test concurrent access
    for _ in 0..10 {
        let inputs = Arc::clone(&test_inputs);
        let handle = thread::spawn(move || {
            for input in inputs.iter() {
                let (value, _) = validate_midi_value(input);
                let (channel, _) = validate_midi_channel(input);
                
                // Basic sanity checks
                assert!(value <= 127);
                assert!(channel <= 15);
                
                // Test message creation if values are valid
                if value <= 127 && channel <= 15 {
                    let _ = create_midi_cc_122_message(value, channel);
                }
            }
        });
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().expect("Thread should complete successfully");
    }
}

#[test]
fn test_memory_safety_with_large_inputs() {
    // Test memory safety with very large string inputs
    let large_string = "9".repeat(1000); // Very large number string
    let (value, warning) = validate_midi_value(&large_string);
    assert_eq!(value, 0);
    assert!(warning.is_some());
    assert!(warning.unwrap().contains("Invalid value"));
    
    let large_text = "a".repeat(1000); // Very large text string
    let (value, warning) = validate_midi_value(&large_text);
    assert_eq!(value, 0);
    assert!(warning.is_some());
    assert!(warning.unwrap().contains("Invalid value"));
}