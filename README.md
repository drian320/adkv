# apex_dma_kvm_pub 3.0
 Apex Legends QEMU/KVM hack memflow

UnknownCheats thread: https://www.unknowncheats.me/forum/apex-legends/406426-kvm-vmread-apex-esp-aimbot.html

Bone IDs reference: https://www.unknowncheats.me/wiki/Apex_Legends_Bones_and_Hitboxes

Game version (Steam & origin): v3.0.2.31 

Updated to work with Apex DX 12

if you still stuck at : 
- Init with qemu connector...
- Can't create qemu connector
- Init with kvm connector...
- Can't create kvm connector

I added a script install_memflow.sh to install memflow-kvm & memflow-qemu at https://github.com/albatror/memflow so
- Download sources from there *.zip or git
- Extract it
- Install Cargo & Rust with the famous curl https://sh.rustup.rs -sSf | sh
- Install memflow with the famous curl --proto '=https' --tlsv1.2 -sSf https://sh.memflow.io | sh
- Compile with the build.sh to see if any errors, if errors, fix it/them then redo build.
- When build is ok without errors, Download from https://github.com/albatror/memflow/blob/main/install_memflow.sh
- Make the script executable, Edit it, Read and Save.
- Run it. You will see if its OK in the console :) then you will see in /apex_dma/memflow_lib/ 2 new folders :
    * memflow-kvm
    * memflow-qemu
- Go back to rebuild the complete sources.
    
(DONT FORGET TO UPDATE WITH YOUR PATH IF NEEDED IN THE SCRIPT AT # Export PATH & # Define variables)

- Working on Windows 10 20H1 (only)

- 1920/1080 (need to be changed manually)
- 2560/1440 (default)

VISUALS :
 - ESP Box, XP Level, Line, Name, Distance
 - ESP Seer Health and Shield
 - Circle FOV
 - Players Glow (GREEN when visible, RED when not visible and YELLOW when knocked - Can be changed in the menu of the overlay)
 - Items Glow (complete disable)
 - Spectators count + Name List (Name List temporary disable but you can active it)

FEATURES :
 - Added a visual DOT to know if you're connected on the server (GREEN DOT) or disconnected (RED DOT)
 - Added the BruteForce CR3/DTB fix by MisterY
 - New function to read localPlayer with OFFSET_LOCAL_ENTITY_HANDLE //[Miscellaneous].LocalEntityHandle (seem more easy for new updates)
 - Press F1 (To Activate GloW, ESP Seer Health/Shield and Aimbot based on default conf)
 - Dynamic FOV/AIM/SMOOTH [70m by default] Process Updated & Optimized 04/12/2024 - Settings added in overlay
 - AutoSuperGlide (nothing to do, it do for you :) ) !Someone tell me to check that, i will.
 - AutoWallJump updated (Slide then jump on a wall, dont touch anything else and it'll auto-walljump)
 - 1V1 (F9)
 - Training with dummies (F10)

!!! WARNING !!!

 - If using Client.exe and Overlay.exe - obfuscate both files to avoid detection.
 - If using Client.exe and Overlay from NVIDIA - obfuscate both files to avoid detection.

 - You need to manually change the path on the client side at apex_guest/Client/Client/main.cpp - Change USERS to yours.
 -> #include "C:\Users\YOUR_USER\WHERE_YOU_KNOW\apex_guest\Client\Client\imgui\imgui.h"

<img src="https://github.com/albatror/adkv/blob/master/demo/settingsS.png" style="display: block; margin: auto;" />
<img src="https://github.com/albatror/adkv/blob/master/demo/settingsS2.png" style="display: block; margin: auto;" />
<img src="https://github.com/albatror/adkv/blob/master/demo/Demo2.png" style="display: block; margin: auto;" />
<img src="https://github.com/albatror/adkv/blob/master/demo/settingsN2.png" style="display: block; margin: auto;" />
<img src="https://github.com/albatror/adkv/blob/master/demo/Demo4.png" style="display: block; margin: auto;" />
<img src="https://github.com/albatror/adkv/blob/master/demo/ingame1.png" style="display: block; margin: auto;" />
<img src="https://github.com/albatror/adkv/blob/master/demo/ingame2.png" style="display: block; margin: auto;" />
<img src="https://github.com/albatror/adkv/blob/master/demo/connected.png" style="display: block; margin: auto;" />
