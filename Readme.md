*** Preconditions ***

Checkout the archive:
```
git clone git@github.com:tnl-rwa/lpc55-rust-sandbox.git
cd lpc55-rust-sandbox
git submodule update --init --recursive
```

Windows with WSL2
Tested with Ubuntu20.04 WSL2 system, but should work with any ditribution
Should also work with a native Linux system, any OS

Download the Segger J-Link GDB server from:
https://www.segger.com/downloads/jlink/

Start the JLink GDB Server
C:\SEGGER\JLink\JLinkGDBServer.exe -strict -device LPC55S69 -if SWD -vd

Install VsCode
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


