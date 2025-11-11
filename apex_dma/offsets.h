#define GameVersion v3.0.1.29 //[Miscellaneous].GameVersion updated 2025/11/12
#define OFFSET_ENTITYLIST 0x65fe758 //[Miscellaneous].cl_entitylist updated 2025/11/12
#define OFFSET_LOCAL_ENT 0x2a7beb8 //[Miscellaneous].LocalPlayer updated 2025/11/12
#define OFFSET_LOCAL_ENTITY_HANDLE 0x2001fa8 //[Miscellaneous].LocalEntityHandle updated 2025/11/12
#define OFFSET_NAME_LIST 0x9039140 //[Miscellaneous].NameList updated 2025/11/12
#define OFFSET_GLOBAL_VARS 0x1f59cc0 //[Miscellaneous].GlobalVars updated 2025/11/12
 
#define OFFSET_LEVELNAME 0x1f5a144 //[Miscellaneous].LevelName updated 2025/11/12
#define OFFSET_CLIENTSTATE 0x1f59f70 //[Miscellaneous].ClientState updated 2025/11/12
#define OFFSET_SIGNONSTATE OFFSET_CLIENTSTATE + 0xAC //SignonState (ClientState + 0xAC)
#define OFFSET_HOST_MAP 0x01e772b0 + 0x58 //[ConVars].host_map + 0x58 updated 2025/11/12
 
#define OFFSET_HEALTH 0x0324 //[RecvTable.DT_Player].m_iHealth updated 2025/11/12
#define OFFSET_MAXHEALTH 0x0468 //[RecvTable.DT_Player].m_iMaxHealth updated 2025/11/12
#define OFFSET_SHIELD 0x0168 //[RecvTable.DT_Player].m_shieldHealth updated 2025/11/12
#define OFFSET_MAXSHIELD 0x016c //[RecvTable.DT_Player].m_shieldHealthMax updated 2025/11/12

#define OFFSET_TEAM 0x0334 //[RecvTable.DT_Player].m_iTeamNum updated 2025/11/12
#define OFFSET_NAME 0x0470 //[RecvTable.DT_Player].m_iSignifierName updated 2025/11/12
#define OFFSET_ARMORTYPE 0x4974 //[RecvTable.DT_Player].m_armorType updated 2025/11/12
#define OFFSET_ABS_VELOCITY 0x0170 //[RecvTable.DT_Player].m_vecAbsVelocity updated 2025/11/12
#define OFFSET_ORIGIN 0x017c //[RecvTable.DT_Player].m_vecAbsOrigin updated 2025/11/12
#define OFFSET_VISIBLE_TIME 0x1a54 //[Miscellaneous].CPlayer!lastVisibleTime updated 2025/11/12
#define OFFSET_LAST_AIMEDAT_TIME 0x1a54   //[Miscellaneous].CPlayer!lastVisibleTime + 0x8
#define OFFSET_ZOOMING 0x1ca1 //[RecvTable.DT_Player].m_bZooming updated 2025/11/12

#define OFFSET_ACTIVE_WEAPON 0x1948 + 0x0058 //[RecvTable.DT_Player].m_inventory + WeaponInventory_Client>activeWeapons updated 2025/11/12
#define OFFSET_WEAPON 0x1948 //[RecvTable.DT_Player].m_inventory updated 2025/11/12

#define OFFSET_ZOOM_FOV 0x00c4 //[RecvTable.DT_WeaponX].m_targetZoomFOV updated 2025/11/12
#define OFFSET_AMMO 0x1600 //[RecvTable.DT_WeaponX].m_ammoInClip updated 2025/11/12

#define OFFSET_SPECTATOR_LIST OFFSET_OBSERVER_LIST
#define OFFSET_OBSERVER_LIST  0x6600778 //[Miscellaneous].ObserverList updated 2025/11/12
#define OFFSET_OBSERVER_ARRAY 0x0954 //[DataMapTypes.DT_GlobalNonRewinding].m_playerObserver - [DataMap.C_ObserverMode].m_observerTarget

#define OFFSET_IN_DUCKSTATE 0x2c24 //[RecvTable.DT_Player].m_duckState updated 2025/11/12

#define OFFSET_IN_DUCK 0x040d9658 //[Buttons].in_duck updated 2025/11/12

#define OFFSET_IN_TOGGLE_DUCK 0x040d9478 //[Buttons].in_toggle_duck updated 2025/11/12
 

#define OFFSET_IN_ATTACK 0x040d9468 //[Buttons].in_attack updated 2025/11/12
#define OFFSET_IN_ZOOM 0x040d95e0 //[Buttons].in_zoom updated 2025/11/12
#define OFFSET_IN_FORWARD 0x040d8ae0 //[Buttons].in_forward updated 2025/11/12
#define OFFSET_IN_BACKWARD 0x040d8b08 //[Buttons].in_backward updated 2025/11/12
 
#define OFFSET_LIFE_STATE 0x0690 //[RecvTable.DT_Player].m_lifeState updated 2025/11/12
#define OFFSET_BLEED_OUT_STATE 0x2920 //[RecvTable.DT_Player].m_bleedoutState updated 2025/11/12
 
#define OFFSET_BONES 0x0da8 + 0x48 //[RecvTable.DT_BaseAnimating].m_nForceBone + 0x48 updated 2025/11/12
#define OFFSET_STUDIOHDR 0xff0 //[Miscellaneous].CBaseAnimating!m_pStudioHdr updated 2025/11/12
#define OFFSET_CAMERAPOS 0x1fb4 //[Miscellaneous].CPlayer!camera_origin updated 2025/11/12
#define OFFSET_VIEWANGLES 0x26f4 - 0x14 //[RecvTable.DT_Player].m_ammoPoolCapacity - 0x14 updated 2025/11/12
#define OFFSET_BREATH_ANGLES OFFSET_VIEWANGLES - 0x10
#define OFFSET_YAW 0x0038 //[RecvTable.DT_BaseEntity].m_angRotation.y updated 2025/11/12
#define OFFSET_AIMPUNCH 0x2508 //[DataMap.DT_Player].m_currentFrameLocalPlayer.m_vecPunchWeapon_Angle updated 2025/11/12
#define OFFSET_IN_USE 0x040d95d0 //[Buttons].in_use updated 2025/11/12
 
#define OFFSET_MATRIX 0x11a350 //[Miscellaneous].ViewMatrix updated 2025/11/12
#define OFFSET_RENDER 0x40d8ab8 //[Miscellaneous].ViewRender updated 2025/11/12
 
#define OFFSET_BULLET_SPEED 0x2818 //[Miscellaneous].CWeaponX!m_flProjectileSpeed updated 2025/11/12
#define OFFSET_BULLET_SCALE 0x2820 //[Miscellaneous].CWeaponX!m_flProjectileScale updated 2025/11/12
 
 
#define OFFSET_GLOW_ENABLE 0x0299 //[RecvTable.DT_HighlightSettings].m_highlightGenericContexts updated 2025/11/12
#define OFFSET_GLOW_THROUGH_WALLS 0x0299 //[RecvTable.DT_HighlightSettings].m_highlightGenericContexts updated 2025/11/12
 
#define GLOW_START_TIME 0x02c4 + 0x30 //m_playerFloatLookStartTime updated 2025/11/12

#define OFFSET_GLOW_FIX 0x0278
#define OFFSET_GLOW_ENABLE_GLOW_CONTEXT OFFSET_GLOW_ENABLE
#define OFFSET_GLOW_CONTEXT_ID 0x029c
#define OFFSET_GLOW_THROUGH_WALLS_GLOW_VISIBLE_TYPE OFFSET_GLOW_THROUGH_WALLS
#define GLOW_LIFE_TIME 0x03A4 + 0x30
#define GLOW_DISTANCE 0x0264
#define GLOW_TYPE 0x029c
#define GLOW_COLOR 0x01D0 + 0x30
#define GLOW_FADE 0x0388 + 0x30
#define HIGHLIGHT_SETTINGS 0x6d48f30 //[Miscellaneous].HighlightSettings updated 2025/11/12
#define HIGHLIGHT_TYPE_SIZE 0x34
#define OFFSET_CROSSHAIR_LAST 0x1a5c //[Miscellaneous].CWeaponX!lastCrosshairTargetTime updated 2025/11/12
#define OFFSET_INPUT_SYSTEM 0x200b800 //[Miscellaneous].InputSystem updated 2025/11/12

#define OFFSET_SKYDIVE_STATE 0x49d4 //[RecvTable.DT_Player].m_skydiveState updated 2025/11/12
#define OFFSET_GRAPPLEACTIVED       0x2f10 //[RecvTable.DT_Player].m_grappleActive updated 2025/11/12
#define OFFSET_GRAPPLE              0x2e88 //[RecvTable.DT_Player].m_grapple updated 2025/11/12
#define OFFSET_GRAPPLEATTACHED      0x0048 //[RecvTable.DT_GrappleData].m_grappleAttached updated 2025/11/12
#define OFFSET_m_xp		    0x3984 //[RecvTable.DT_Player].m_xp updated 2025/11/12

#define OFFSET_TRAVERSAL_PROGRESS   0x2d34 //[DataMap.DT_Player].m_traversalProgress updated 2025/11/12
#define OFFSET_TRAVERSAL_STARTTIME  0x2d3c //[DataMap.DT_Player].m_traversalStartTime updated 2025/11/12
#define OFFSET_WALL_RUN_START_TIME  0x386c //[RecvTable.DT_LocalPlayerExclusive].m_wallRunStartTime updated 2025/11/12
#define OFFSET_WALL_RUN_CLEAR_TIME  0x3870 //[RecvTable.DT_LocalPlayerExclusive].m_wallRunClearTime updated 2025/11/12
#define OFFSET_FLAGS                0x00c8 //[DataMap.DT_Player].m_fFlags updated 2025/11/12
#define OFFSET_TIME_BASE            0x2158 //[DataMap.DT_Player].m_currentFramePlayer.timeBase updated 2025/11/12

#define OFFSET_IN_JUMP              0x040d9550 //[Buttons].in_jump (estimated from adjacent buttons) updated 2025/11/12
