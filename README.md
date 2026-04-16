JVSGS (JVS Ground Station) is a GCS primarily made for controlling UAVs.

The objective of the project is to make a safe, resiliant, relatively cheap platform to control any RC device with advanced features, integrating a IRX4+ transmitter by @pascallanger as its recommended transmitter.

The project features a two battery system with hot swap, making the operation time virtually infinite.
How it works:
When the main (12v) power source is connected, the connection of both batteries are disabled, and then the system starts charging and balancing them. The main battery, when not interrupted by the main power source, cuts the backup battery, effectively making the operation without an external power source infinite as long as you have enough 2S (7.4v) batteries with a balance lead available. Includes BMS protection and voltage monitoring.

Tools used:
The firmware is written in rust using the RTIC framework.
The electronics design is made using KiCAD.
The 3D printable model is made using FreeCAD.


Credits:
Developed by [@jvsmitt](gitlab.com/jvmitt)
Tools and resources used: [DIY-Multiprotocol-TX-Module](https://github.com/pascallanger/DIY-Multiprotocol-TX-Module), [RTIC](https://github.com/rtic-rs/rtic), [KiCAD](https://github.com/KiCad), [FreeCAD](https://github.com/FreeCAD/FreeCAD).
