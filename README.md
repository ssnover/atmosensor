# atmosensor
This is a simple project for measuring and recording air quality data from an embedded device.

## Components
* `atmosensor-fw` This contains the embedded firmware application which reads from
the sensors and communicates the data over USB.
* `atmosensor-kicad` KiCAD schematic and PCB layout for the hardware which connects
to the sensors.
* `usb-protocol` Documentation of the protocol being used for communicating between
the host application and the embedded firmware.