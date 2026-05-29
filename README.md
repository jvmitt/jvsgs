JVSGS (JVS Ground Station) is a GCS primarily made for controlling UAVs.<br>

The objective of this project is to make a safe, resistant and relatively cheap platform to control any RC device, integrating advanced control features. Allows connection of any external USB controller. As of now, using it isnt recommended since it hasnt been tested thoroughly and might cause accidents.<br>

The project firmware is written in Rust, features a STM32F411CEU6 as its chip, a redundant two battery system with hot-swap and charging, and a case for ease of use.<br>

How the power system works:<br>
When the main (12v) power source is connected, the connection of both batteries are disabled and the charging modules activates. Includes an ORING circuit that enables hot-swap, making the operation without an external power source infinite as long as you have enough 2S (7.4v) batteries with a balance lead available or maybe even car batteries for the main power source. Includes BMS protection and voltage monitoring.<br>

Credits:<br>
Developed by [@jvmitt](gitlab.com/jvmitt).<br>
Tools and resources used:<br> 
Recommended transmitter: [DIY-Multiprotocol-TX-Module](https://github.com/pascallanger/DIY-Multiprotocol-TX-Module);<br>
Embedded framework: [RTIC](https://github.com/rtic-rs/rtic);<br> 
Electronics design: [KiCAD](https://github.com/KiCad);<br> 
Case 3D model design: [FreeCAD](https://github.com/FreeCAD/FreeCAD).<br>
