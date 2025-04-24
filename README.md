# apex_dma_kvm 2.0
 Apex Legends QEMU/KVM hack

UnknownCheats thread: https://www.unknowncheats.me/forum/apex-legends/406426-kvm-vmread-apex-esp-aimbot.html

Bone IDs reference: https://www.unknowncheats.me/wiki/Apex_Legends_Bones_and_Hitboxes

Game version (Steam & Origin): v3.0.1.29

You need to obfuscate Client.exe and Overlay.exe to be not detected.

FEATURES :
 - Work in Borderless and FullScreen
 - 2560*1440 by default (you can change in code manually - search 2560 and 1440 to replace with your numbers in apex_dma.cpp)
 - Red dot in Overlay when Client is connected to Server
 - Optimization of DTB shuffle
 - Added Dynamic Fov/Smooth/Aim from low meters (70m)
 - Added Player XP/Level
 - Added Spectators viewers + List (Need to unquote some part of code harcored to get it work - will update it to let the user choose dirrectly from the menu or keybind)
 - Added updater.py to easy update offsets.h with new offsets.ini (can be found on https://www.unknowncheats.me/forum/apex-legends/319804-apex-legends-reversal-structs-offsets-888.html) with this command : py updater.py offsets.h offsets.ini
 - Everything can be change in Menu (INSERT)
 - Aim_Key (Left mouse by default) & Aim_key2 (Right mouse) activated from 70m
 - F1 for Player Glow & Aim (after starting when In-Game)
 - F5 ESP
 - F6 AIM
 - F7 Player Glow (Only)
 - F9 for 1v1
 - F10 for Training (Dummies)

INSTALL : 
- Download sources from there *.zip or git
- Extract it
- Install Cargo & Rust with the famous curl https://sh.rustup.rs/ -sSf | sh
- Install memflow with the famous curl --proto '=https' --tlsv1.2 -sSf https://sh.memflow.io/ | sh
- check https://github.com/memflow/memflow-kvm and/or https://github.com/memflow/memflowup (better) to install memflow-kvm connector.
- Compile with the build.sh to see if any errors.
- When build is ok without errors, start Overlay (obfuscated) then Client (obfuscated)
- On the Linux console (host) start the server by : sudo ./apex_dma


IN UPDATE - NOT FINISHED
