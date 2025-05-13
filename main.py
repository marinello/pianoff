import mido

def send_midi_command_122(value=0, channel=0):
    """
    Sends MIDI command 122 (Local Control) with the specified value on the specified channel.
    
    Parameters:
    value (int): 0 = Local Control Off, 127 = Local Control On
    channel (int): MIDI channel (0-15)
    """
    # Create a MIDI output port
    try:
        # List available output ports
        ports = mido.get_output_names()
        if not ports:
            print("No MIDI output ports available.")
            return
        
        print("Available MIDI ports:")
        for i, port in enumerate(ports):
            print(f"{i}: {port}")
        
        port_index = int(input("Select a port by number: "))
        output_port = mido.open_output(ports[port_index])
        
        # Create a control change message with controller number 122
        msg = mido.Message('control_change', 
                           channel=channel, 
                           control=122, 
                           value=value)
        
        # Send the message
        output_port.send(msg)
        print(f"Sent MIDI CC #122 (Local Control) with value {value} on channel {channel}")
        
        # Close the port
        output_port.close()
        
    except Exception as e:
        print(f"Error: {e}")

if __name__ == "__main__":
    print("MIDI Command 122 (Local Control) Sender")
    print("---------------------------------------")
    
    # Get parameters from user
    try:
        value = int(input("Enter value (0=Off, 127=On, default=0): ") or "0")
        if value not in range(128):
            print("Value must be between 0-127. Using default value 0.")
            value = 0
            
        channel = int(input("Enter MIDI channel (0-15, default=0): ") or "0")
        if channel not in range(16):
            print("Channel must be between 0-15. Using default channel 0.")
            channel = 0
            
        # Send the command
        send_midi_command_122(value, channel)
        
    except ValueError:
        print("Invalid input. Please enter numeric values.")


