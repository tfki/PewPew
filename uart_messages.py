import serial
import time
import random

def uart_sender(port, baudrate=9600):
    try:
        ser = serial.Serial(port, baudrate, timeout=1)
        print(f"Sende kontinuierlich auf {port} bei {baudrate} Baud")
        print("-" * 50)
        
        packet_counter = 0
        
        while True:
            packet_counter += 1
            
            # beispielmessage für Brightness  (FF bzw 255 ist DELIMITER, 01 ist message_id)
            # enthält sensortag_id, timestamp, message_id, brightness value
            # format: [ID][ID][TIME][TIME][TIME][TIME][01][BRIGHT][BRIGHT][FF]
            # Hex: 34 12 A0 86 01 00 01 C8 00 FF 
            # Dec: [52][18][160][134][1][0][1][200][0][255]
            
            # Schwarzer Bildschirm (2 Lux)
            packet_brightness_black = bytes([52, 18, 160, 134, 1, 0, 1, 2, 0, 255])
            
            # Weißer Bildschirm (2000 Lux) 
            packet_brightness_white = bytes([52, 18, 160, 134, 1, 0, 1, 208, 7, 255])
            
            # beispielmessage für buttonpress (FF bzw 255 ist DELIMITER, 02 ist message_id)
            # enthält sensortag_id, timestamp, message_id
            # format: [ID][ID][TIME][TIME][TIME][TIME][02][FF]
            # Hex: 34 12 A0 86 01 00 02 FF
            # Dec: [52][18][160][134][1][0][2][255]
            packet_button_press = bytes([52, 18, 160, 134, 1, 0, 2, 255])
            
            if packet_counter % 3 == 0:
               message = packet_brightness_white
               print(f"[{packet_counter:04d}] Weiß (2000 Lux): {list(message)}")
            else:
               message = packet_brightness_black  
               print(f"[{packet_counter:04d}] Schwarz (2 Lux): {list(message)}")
            
            ser.write(message)
            time.sleep(1)
            
    except KeyboardInterrupt:
        print("\nBeende...")
    except serial.SerialException as e:
        print(f"Serial Port Fehler: {e}")
    except Exception as e:
        print(f"Fehler: {e}")
    finally:
        if 'ser' in locals():
            ser.close()
            print("Port geschlossen")


if __name__ == "__main__":
    print("UART Kontinuierlicher Sender")
    print("=" * 40)
    
    port = input("COM Port (z.B. COM10): ").strip()
    if not port:
        port = "COM10"  # Default
    
    uart_sender(port)
    