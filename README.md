# atmosensor
This is a simple project for measuring and recording air quality data from an embedded device.

## Components
* [`atmosensor-fw`](atmosensor-fw/README.md) This contains the embedded firmware application which reads from
the sensors and communicates the data over USB.
* `atmosensor-host-apps` Linux-based applications for interacting with the firmware app.
  * `atmosensor-tui` Text user interface for sending and receiving commands via USB.
* `atmosensor-kicad` KiCAD schematic and PCB layout for the hardware which connects
to the sensors.
* `usb-protocol` Documentation of the protocol being used for communicating between
the host application and the embedded firmware.
