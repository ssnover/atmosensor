# atmosensor-fw
Firmware application for an STM32 microcontroller which reads data from SCD30 and BME680 via I2C and can be queried for data over USB (CDC).

## Architecture
The firmware is event-driven, principally from interrupts triggered by 
SCD30 sensor indicating that it has data ready to read or by incoming
USB messages. These interrupts queue information into the event queue
which can be processed by the main loop by the command handlers.

Data can be read from the device via the USB interface (see `../usb-protocol`
for available messages). The data stream is packetized with COBS with a 
sentinel byte of `0x00`. Most messages are request-response style, with
some of the commands being proactively reported by the firmware 
application (like notification of new data being available).

## Sensors
Data comes from the Sensirion SCD30 sensor and the Bosch BME680. I've 
forked each of these crates in order to add support for more messages
or to modify how they utilize their I2C driver (cannot take ownership 
since both devices are on the same bus).