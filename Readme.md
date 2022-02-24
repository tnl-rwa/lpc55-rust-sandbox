**Preconditions**

Checkout the archive:
```
git clone git@github.com:tnl-rwa/lpc55-rust-sandbox.git
cd lpc55-rust-sandbox
git submodule update --init --recursive
```

Use a system with Windows and WSL2 or use native Linux system
Tested with Ubuntu20.04 WSL2 system, but should work with any ditribution
Should also work with a native Linux system, any OS

Download the Segger J-Link GDB server from:
https://www.segger.com/downloads/jlink/

Start the JLink GDB Server (Windows example)
C:\SEGGER\JLink\JLinkGDBServer.exe -strict -device LPC55S69 -if SWD -vd

Install and start VsCode

Install plugins:
    Name: Remote Development
    Id: ms-vscode-remote.vscode-remote-extensionpack

    Name: Remote - Containers
    Id: ms-vscode-remote.remote-containers

    Name: Remote - SSH
    Id: ms-vscode-remote.remote-ssh

    Windows only:
    Name: Remote - WSL
    Id: ms-vscode-remote.remote-wsl


Op the en the folder in Vscode, in WSL.
A popup will appear at the bottom right that suggests to open the project in a container. Do that.

It takes a while to build the container.

After successful load, press F5 to build and debug the project on the target.

In launch.json you can change the `executable` line to debug another example, eg `task`


**Things that could be done**
- Create tasks, inter task communication, shared resources, queueus etc.
- Get USB working: CDCACM, HID, Whatever
- Read write flash
- I/O: SPI, I2C 
- Connect and control devices:
    - nRF2L01
    - MFRC-522 RC522 RFID RF IC card
    - HC-SR501 Adjust IR Pyroelectric Infrared PIR Motion Sensor Detector
    - 433Mhz RF transmitter and receiver kit TK0460
    - 4x4 Matrix 16 Key Membrane Switch Keypad
    - AT-09 IOS BLE 4.0 Bluetooth module CC2540 CC2541 
    - Mini 1.8 Inch Spi 128X160 Dot Cmmpatible 1602 5110
    - I2C Twi Spi Seriële Interface 20X4 Karakter HD44780 Controller
    - 0.96 "I2C 128X64 Lcd Display SSD1306 12864 
    - 0.96 "SPI 128X64 Lcd Display SSD1306 12864 


