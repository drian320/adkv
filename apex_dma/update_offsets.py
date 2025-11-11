#!/usr/bin/env python3
"""
offsets.iniからoffsets.hを自動生成するスクリプト

使い方:
    python3 update_offsets.py
"""

import configparser
import re
from datetime import datetime
from typing import Dict, Optional


class OffsetsUpdater:
    def __init__(self, ini_path: str = "offsets.ini", header_path: str = "offsets.h"):
        self.ini_path = ini_path
        self.header_path = header_path
        # strict=Falseで重複キーを許可
        self.config = configparser.ConfigParser(strict=False)
        self.config.read(ini_path)
        self.today = datetime.now().strftime("%Y/%m/%d")

    def get_value(self, section: str, key: str) -> Optional[str]:
        """INIファイルから値を取得"""
        try:
            return self.config.get(section, key)
        except (configparser.NoSectionError, configparser.NoOptionError):
            return None

    def format_offset(self, value: str) -> str:
        """オフセット値をフォーマット（0xプレフィックスを確保）"""
        if not value.startswith("0x"):
            return f"0x{value}"
        return value

    def generate_header(self) -> str:
        """offsets.hファイルの内容を生成"""
        lines = []

        # ゲームバージョン
        lines.append(f"#define GameVersion v3.0.1.29 //[Miscellaneous].GameVersion updated {self.today}")

        # Miscellaneousセクションから主要なオフセット
        entitylist = self.get_value("Miscellaneous", "cl_entitylist")
        if entitylist:
            lines.append(f"#define OFFSET_ENTITYLIST {self.format_offset(entitylist)} //[Miscellaneous].cl_entitylist updated {self.today}")

        local_player = self.get_value("Miscellaneous", "LocalPlayer")
        if local_player:
            lines.append(f"#define OFFSET_LOCAL_ENT {self.format_offset(local_player)} //[Miscellaneous].LocalPlayer updated {self.today}")

        local_entity_handle = self.get_value("Miscellaneous", "LocalEntityHandle")
        if local_entity_handle:
            lines.append(f"#define OFFSET_LOCAL_ENTITY_HANDLE {self.format_offset(local_entity_handle)} //[Miscellaneous].LocalEntityHandle updated {self.today}")

        name_list = self.get_value("Miscellaneous", "NameList")
        if name_list:
            lines.append(f"#define OFFSET_NAME_LIST {self.format_offset(name_list)} //[Miscellaneous].NameList updated {self.today}")

        global_vars = self.get_value("Miscellaneous", "GlobalVars")
        if global_vars:
            lines.append(f"#define OFFSET_GLOBAL_VARS {self.format_offset(global_vars)} //[Miscellaneous].GlobalVars updated {self.today}")

        lines.append(" ")

        # レベル名とクライアントステート
        level_name = self.get_value("Miscellaneous", "LevelName")
        if level_name:
            lines.append(f"#define OFFSET_LEVELNAME {self.format_offset(level_name)} //[Miscellaneous].LevelName updated {self.today}")

        client_state = self.get_value("Miscellaneous", "ClientState")
        if client_state:
            lines.append(f"#define OFFSET_CLIENTSTATE {self.format_offset(client_state)} //[Miscellaneous].ClientState updated {self.today}")
            lines.append("#define OFFSET_SIGNONSTATE OFFSET_CLIENTSTATE + 0xAC //SignonState (ClientState + 0xAC)")

        host_map = self.get_value("ConVars", "host_map")
        if host_map:
            lines.append(f"#define OFFSET_HOST_MAP {self.format_offset(host_map)} + 0x58 //[ConVars].host_map + 0x58 updated {self.today}")

        lines.append(" ")

        # RecvTable.DT_BaseEntityとDT_Playerから
        team = self.get_value("RecvTable.DT_BaseEntity", "m_iTeamNum")
        if team:
            lines.append(f"#define OFFSET_TEAM {self.format_offset(team)} //[RecvTable.DT_BaseEntity].m_iTeamNum updated {self.today}")

        health = self.get_value("RecvTable.DT_Player", "m_iHealth")
        if health:
            lines.append(f"#define OFFSET_HEALTH {self.format_offset(health)} //[RecvTable.DT_Player].m_iHealth updated {self.today}")

        max_health = self.get_value("RecvTable.DT_Player", "m_iMaxHealth")
        if max_health:
            lines.append(f"#define OFFSET_MAXHEALTH {self.format_offset(max_health)} //[RecvTable.DT_Player].m_iMaxHealth updated {self.today}")

        lines.append("")

        # シールド関連
        shield = self.get_value("RecvTable.DT_BaseEntity", "m_shieldHealth")
        if shield:
            lines.append(f"#define OFFSET_SHIELD {self.format_offset(shield)} //[RecvTable.DT_BaseEntity].m_shieldHealth updated {self.today}")

        max_shield = self.get_value("RecvTable.DT_BaseEntity", "m_shieldHealthMax")
        if max_shield:
            lines.append(f"#define OFFSET_MAXSHIELD {self.format_offset(max_shield)} //[RecvTable.DT_BaseEntity].m_shieldHealthMax updated {self.today}")

        armor_type = self.get_value("RecvTable.DT_Player", "m_armorType")
        if armor_type:
            lines.append(f"#define OFFSET_ARMORTYPE {self.format_offset(armor_type)} //[RecvTable.DT_Player].m_armorType updated {self.today}")

        # 名前関連
        name = self.get_value("RecvTable.DT_BaseEntity", "m_iName")
        if name:
            lines.append(f"#define OFFSET_NAME {self.format_offset(name)} //[RecvTable.DT_BaseEntity].m_iName updated {self.today}")

        sign_name = self.get_value("RecvTable.DT_BaseEntity", "m_iSignifierName")
        if sign_name:
            lines.append(f"#define OFFSET_SIGN_NAME {self.format_offset(sign_name)} //[RecvTable.DT_BaseEntity].m_iSignifierName updated {self.today}")

        # 速度と可視性
        abs_velocity = self.get_value("DataMap.DT_BaseEntity", "m_vecAbsVelocity")
        if abs_velocity:
            lines.append(f"#define OFFSET_ABS_VELOCITY {self.format_offset(abs_velocity)} //[DataMap.DT_BaseEntity].m_vecAbsVelocity updated {self.today}")

        visible_time = self.get_value("Miscellaneous", "CPlayer!lastVisibleTime")
        if visible_time:
            lines.append(f"#define OFFSET_VISIBLE_TIME {self.format_offset(visible_time)} //[Miscellaneous].CPlayer!lastVisibleTime updated {self.today}")
            lines.append(f"#define OFFSET_LAST_AIMEDAT_TIME {self.format_offset(visible_time)}   //[Miscellaneous].CPlayer!lastVisibleTime + 0x8")

        zooming = self.get_value("RecvTable.DT_Player", "m_bZooming")
        if zooming:
            lines.append(f"#define OFFSET_ZOOMING {self.format_offset(zooming)} //[RecvTable.DT_Player].m_bZooming updated {self.today}")

        lines.append("")

        # ビューオフセットとアクティブ武器
        view_offset = self.get_value("DataMap.DT_BaseEntity", "m_currentFrame.viewOffset")
        if view_offset:
            lines.append(f"#define OFFSET_VIEW_OFFSET {self.format_offset(view_offset)} //[DataMap.DT_BaseEntity].m_currentFrame.viewOffset updated {self.today}")

        inventory = self.get_value("RecvTable.DT_Player", "m_inventory")
        if inventory:
            lines.append(f"#define OFFSET_ACTIVE_WEAPON {self.format_offset(inventory)} + 0x0058 //[RecvTable.DT_Player].m_inventory + WeaponInventory_Client>activeWeapons updated {self.today}")

        lines.append("")

        # オブザーバー関連
        observer_list = self.get_value("Miscellaneous", "ObserverList")
        if observer_list:
            lines.append("#define OFFSET_SPECTATOR_LIST OFFSET_OBSERVER_LIST")
            lines.append(f"#define OFFSET_OBSERVER_LIST  {self.format_offset(observer_list)} //[Miscellaneous].ObserverList updated {self.today}")

        lines.append("#define OFFSET_OBSERVER_ARRAY 0x0954 //[DataMapTypes.DT_GlobalNonRewinding].m_playerObserver - [DataMap.C_ObserverMode].m_observerTarget")

        lines.append("")

        # しゃがみ状態
        duck_state = self.get_value("RecvTable.DT_Player", "m_duckState")
        if duck_state:
            lines.append(f"#define OFFSET_IN_DUCKSTATE {self.format_offset(duck_state)} //[RecvTable.DT_Player].m_duckState updated {self.today}")

        lines.append("")

        # ボタン関連
        in_duck = self.get_value("Buttons", "in_duck")
        if in_duck:
            lines.append(f"#define OFFSET_IN_DUCK {self.format_offset(in_duck)} //[Buttons].in_duck updated {self.today}")

        traversal_progress = self.get_value("DataMap.DT_Player", "m_traversalProgress")
        if traversal_progress:
            lines.append(f"#define OFFSET_TRAVERSAL_PROGRESS {self.format_offset(traversal_progress)} //[DataMap.DT_Player].m_traversalProgress updated {self.today}")

        traversal_start = self.get_value("DataMap.DT_Player", "m_traversalStartTime")
        if traversal_start:
            lines.append(f"#define OFFSET_TRAVERSAL_STARTTIME {self.format_offset(traversal_start)} //[DataMap.DT_Player].m_traversalStartTime updated {self.today}")

        traversal_release = self.get_value("RecvTable.DT_LocalPlayerExclusive", "m_traversalReleaseTime")
        if traversal_release:
            lines.append(f"#define OFFSET_TRAVERSAL_RELEASE_TIME {self.format_offset(traversal_release)} //[RecvTable.DT_LocalPlayerExclusive].m_traversalReleaseTime updated {self.today}")

        lines.append("")

        in_jump = self.get_value("Buttons", "in_jump")
        if in_jump:
            lines.append(f"#define OFFSET_IN_JUMP {self.format_offset(in_jump)} //[Buttons].in_jump updated {self.today}")

        in_toggle_duck = self.get_value("Buttons", "in_toggle_duck")
        if in_toggle_duck:
            lines.append(f"#define OFFSET_IN_TOGGLE_DUCK {self.format_offset(in_toggle_duck)} //[Buttons].in_toggle_duck updated {self.today}")

        lines.append(" ")

        # 武器関連
        weapon_name = self.get_value("RecvTable.DT_WeaponX", "m_weaponNameIndex")
        if weapon_name:
            lines.append(f"#define OFFSET_WEAPON_NAME {self.format_offset(weapon_name)} //[RecvTable.DT_WeaponX].m_weaponNameIndex updated {self.today}")

        off_weapon = self.get_value("DataMap.DT_BaseCombatCharacter", "m_latestNonOffhandWeapons")
        if off_weapon:
            lines.append(f"#define OFFSET_OFF_WEAPON {self.format_offset(off_weapon)} //[DataMap.DT_BaseCombatCharacter].m_latestNonOffhandWeapons updated {self.today}")

        wall_run_start = self.get_value("RecvTable.DT_LocalPlayerExclusive", "m_wallRunStartTime")
        if wall_run_start:
            lines.append(f"#define OFFSET_WALL_RUN_START_TIME {self.format_offset(wall_run_start)} //[RecvTable.DT_LocalPlayerExclusive]->m_wallRunStartTime updated {self.today}")

        wall_run_clear = self.get_value("RecvTable.DT_LocalPlayerExclusive", "m_wallRunClearTime")
        if wall_run_clear:
            lines.append(f"#define OFFSET_WALL_RUN_CLEAR_TIME {self.format_offset(wall_run_clear)} //[RecvTable.DT_LocalPlayerExclusive]->m_wallRunClearTime updated {self.today}")

        lines.append("")

        # フラグと攻撃ボタン
        flags = self.get_value("DataMap.DT_Player", "m_fFlags")
        if flags:
            lines.append(f"#define OFFSET_FLAGS {self.format_offset(flags)} //[DataMap.DT_Player].m_fFlags updated {self.today}")

        in_attack = self.get_value("Buttons", "in_attack")
        if in_attack:
            lines.append(f"#define OFFSET_IN_ATTACK {self.format_offset(in_attack)} //[Buttons].in_attack updated {self.today}")

        in_zoom = self.get_value("Buttons", "in_zoom")
        if in_zoom:
            lines.append(f"#define OFFSET_IN_ZOOM {self.format_offset(in_zoom)} //[Buttons].in_zoom updated {self.today}")

        in_forward = self.get_value("Buttons", "in_forward")
        if in_forward:
            lines.append(f"#define OFFSET_IN_FORWARD {self.format_offset(in_forward)} //[Buttons].in_forward updated {self.today}")

        in_backward = self.get_value("Buttons", "in_backward")
        if in_backward:
            lines.append(f"#define OFFSET_IN_BACKWARD {self.format_offset(in_backward)} //[Buttons].in_backward updated {self.today}")

        lines.append(" ")

        # ライフステートとブリードアウト
        life_state = self.get_value("RecvTable.DT_Player", "m_lifeState")
        if life_state:
            lines.append(f"#define OFFSET_LIFE_STATE {self.format_offset(life_state)} //[RecvTable.DT_Player].m_lifeState updated {self.today}")

        bleed_out = self.get_value("RecvTable.DT_Player", "m_bleedoutState")
        if bleed_out:
            lines.append(f"#define OFFSET_BLEED_OUT_STATE {self.format_offset(bleed_out)} //[RecvTable.DT_Player].m_bleedoutState updated {self.today}")

        lines.append(" ")

        # 位置とボーン
        origin = self.get_value("DataMap.DT_BaseEntity", "m_vecAbsOrigin")
        if origin:
            lines.append(f"#define OFFSET_ORIGIN {self.format_offset(origin)} //[DataMap.DT_BaseEntity].m_vecAbsOrigin updated {self.today}")

        force_bone = self.get_value("RecvTable.DT_BaseAnimating", "m_nForceBone")
        if force_bone:
            lines.append(f"#define OFFSET_BONES {self.format_offset(force_bone)} + 0x48 //[RecvTable.DT_BaseAnimating].m_nForceBone + 0x48 updated {self.today}")

        studio_hdr = self.get_value("Miscellaneous", "CBaseAnimating!m_pStudioHdr")
        if studio_hdr:
            lines.append(f"#define OFFSET_STUDIOHDR {self.format_offset(studio_hdr)} //[Miscellaneous].CBaseAnimating!m_pStudioHdr updated {self.today}")

        aim_punch = self.get_value("DataMap.DT_Player", "m_currentFrameLocalPlayer.m_vecPunchWeapon_Angle")
        if aim_punch:
            lines.append(f"#define OFFSET_AIMPUNCH {self.format_offset(aim_punch)} //[DataMap.DT_Player].m_currentFrameLocalPlayer.m_vecPunchWeapon_Angle updated {self.today}")

        camera_pos = self.get_value("Miscellaneous", "CPlayer!camera_origin")
        if camera_pos:
            lines.append(f"#define OFFSET_CAMERAPOS {self.format_offset(camera_pos)} //[Miscellaneous].CPlayer!camera_origin updated {self.today}")

        ammo_pool = self.get_value("RecvTable.DT_Player", "m_ammoPoolCapacity")
        if ammo_pool:
            lines.append(f"#define OFFSET_VIEWANGLES {self.format_offset(ammo_pool)} - 0x14 //[RecvTable.DT_Player].m_ammoPoolCapacity - 0x14 updated {self.today}")
            lines.append("#define OFFSET_BREATH_ANGLES OFFSET_VIEWANGLES - 0x10")

        observer_mode = self.get_value("RecvTable.DT_LocalPlayerExclusive", "m_iObserverMode")
        if observer_mode:
            lines.append(f"#define OFFSET_OBSERVER_MODE {self.format_offset(observer_mode)} //[RecvTable.DT_LocalPlayerExclusive].m_iObserverMode updated {self.today}")

        observer_target = self.get_value("RecvTable.DT_LocalPlayerExclusive", "m_hObserverTarget")
        if observer_target:
            lines.append(f"#define OFFSET_OBSERVING_TARGET {self.format_offset(observer_target)} //[RecvTable.DT_LocalPlayerExclusive].m_hObserverTarget updated {self.today}")

        in_use = self.get_value("Buttons", "in_use")
        if in_use:
            lines.append(f"#define OFFSET_IN_USE {self.format_offset(in_use)} //[Buttons].in_use updated {self.today}")

        lines.append(" ")

        # マトリックスとレンダー
        view_matrix = self.get_value("Miscellaneous", "ViewMatrix")
        if view_matrix:
            lines.append(f"#define OFFSET_MATRIX {self.format_offset(view_matrix)} //[Miscellaneous].ViewMatrix updated {self.today}")

        view_render = self.get_value("Miscellaneous", "ViewRender")
        if view_render:
            lines.append(f"#define OFFSET_RENDER {self.format_offset(view_render)} //[Miscellaneous].ViewRender updated {self.today}")

        lines.append(" ")

        # 武器詳細
        primary_weapons = self.get_value("RecvTable.DT_BaseCombatCharacter", "m_latestPrimaryWeapons")
        if primary_weapons:
            lines.append(f"#define OFFSET_WEAPON {self.format_offset(primary_weapons)} //[RecvTable.DT_BaseCombatCharacter].m_latestPrimaryWeapons updated {self.today}")

        projectile_speed = self.get_value("Miscellaneous", "CWeaponX!m_flProjectileSpeed")
        if projectile_speed:
            lines.append(f"#define OFFSET_BULLET_SPEED {self.format_offset(projectile_speed)} //[Miscellaneous].CWeaponX!m_flProjectileSpeed updated {self.today}")

        projectile_scale = self.get_value("Miscellaneous", "CWeaponX!m_flProjectileScale")
        if projectile_scale:
            lines.append(f"#define OFFSET_BULLET_SCALE {self.format_offset(projectile_scale)} //[Miscellaneous].CWeaponX!m_flProjectileScale updated {self.today}")

        zoom_fov = self.get_value("RecvTable.DT_WeaponX", "m_playerData")
        if zoom_fov:
            lines.append(f"#define OFFSET_ZOOM_FOV {self.format_offset(zoom_fov)} //[RecvTable.DT_WeaponX].m_playerData + m_curZoomFOV updated {self.today}")

        ammo = self.get_value("RecvTable.DT_PropSurvival", "m_ammoInClip")
        if ammo:
            lines.append(f"#define OFFSET_AMMO {self.format_offset(ammo)} //[RecvTable.DT_PropSurvival].m_ammoInClip updated {self.today}")

        lines.append(" ")

        # アイテム関連
        item_id = self.get_value("RecvTable.DT_PropSurvival", "m_customScriptInt")
        if item_id:
            lines.append(f"#define OFFSET_ITEM_ID {self.format_offset(item_id)} //[RecvTable.DT_PropSurvival].m_customScriptInt updated {self.today}")

        model_name = self.get_value("DataMap.DT_BaseEntity", "m_ModelName")
        if model_name:
            lines.append(f"#define OFFSET_MODELNAME {self.format_offset(model_name)} //[DataMap.DT_BaseEntity].m_ModelName updated {self.today}")

        if item_id:
            lines.append(f"#define OFFSET_M_CUSTOMSCRIPTINT {self.format_offset(item_id)} //[RecvTable.DT_PropSurvival].m_customScriptInt updated {self.today}")

        ammo_pool_count = self.get_value("DataMap.DT_Player", "m_currentFramePlayer.m_ammoPoolCount")
        if ammo_pool_count:
            lines.append(f"#define OFFSET_YAW {self.format_offset(ammo_pool_count)} //[DataMap.DT_Player].m_currentFramePlayer.m_ammoPoolCount - 0x8 updated {self.today}")

        lines.append(" ")

        # グロー関連
        highlight_contexts = self.get_value("RecvTable.DT_HighlightSettings", "m_highlightGenericContexts")
        if highlight_contexts:
            lines.append(f"#define OFFSET_GLOW_ENABLE {self.format_offset(highlight_contexts)} //[RecvTable.DT_HighlightSettings].m_highlightGenericContexts updated {self.today}")
            lines.append(f"#define OFFSET_GLOW_THROUGH_WALLS {self.format_offset(highlight_contexts)} //[RecvTable.DT_HighlightSettings].m_highlightGenericContexts updated {self.today}")

        lines.append(" ")

        # タイムベース
        time_base = self.get_value("DataMap.DT_Player", "m_currentFramePlayer.timeBase")
        if time_base:
            lines.append(f"#define OFFSET_TIME_BASE {self.format_offset(time_base)} //[DataMap.DT_Player].m_currentFramePlayer.timeBase updated {self.today}")

        float_look_start = self.get_value("DataMap.C_PlayerLocalData", "m_playerFloatLookStartTime")
        if float_look_start:
            lines.append(f"#define GLOW_START_TIME {self.format_offset(float_look_start)} + 0x30 //m_playerFloatLookStartTime updated {self.today}")

        highlight_server = self.get_value("RecvTable.DT_HighlightSettings", "m_highlightServerActiveStates")
        if highlight_server:
            lines.append(f"#define OFFSET_HIGHLIGHTSERVERACTIVESTATES {self.format_offset(highlight_server)} //[RecvTable.DT_HighlightSettings].m_highlightServerActiveStates updated {self.today}")

        lines.append("")

        # グロー関連の追加定義
        lines.append("#define OFFSET_GLOW_FIX 0x0278")
        lines.append("#define OFFSET_GLOW_ENABLE_GLOW_CONTEXT OFFSET_GLOW_ENABLE")
        lines.append("#define OFFSET_GLOW_CONTEXT_ID 0x029c")
        lines.append("#define OFFSET_GLOW_THROUGH_WALLS_GLOW_VISIBLE_TYPE OFFSET_GLOW_THROUGH_WALLS")
        lines.append("#define GLOW_LIFE_TIME 0x03A4 + 0x30")
        lines.append("#define GLOW_DISTANCE 0x0264")
        lines.append("#define GLOW_TYPE 0x029c")
        lines.append("#define GLOW_COLOR 0x01D0 + 0x30")
        lines.append("#define GLOW_FADE 0x0388 + 0x30")

        highlight_settings = self.get_value("Miscellaneous", "HighlightSettings")
        if highlight_settings:
            lines.append(f"#define HIGHLIGHT_SETTINGS {self.format_offset(highlight_settings)} //[Miscellaneous].HighlightSettings updated {self.today}")

        lines.append("#define HIGHLIGHT_TYPE_SIZE 0x34")

        crosshair_target = self.get_value("Miscellaneous", "CWeaponX!lastCrosshairTargetTime")
        if crosshair_target:
            lines.append(f"#define OFFSET_CROSSHAIR_LAST {self.format_offset(crosshair_target)} //[Miscellaneous].CWeaponX!lastCrosshairTargetTime updated {self.today}")

        input_system = self.get_value("Miscellaneous", "InputSystem")
        if input_system:
            lines.append(f"#define OFFSET_INPUT_SYSTEM {self.format_offset(input_system)} //[Miscellaneous].InputSystem updated {self.today}")

        lines.append("")

        # スカイダイブとグラップル
        skydive_state = self.get_value("RecvTable.DT_Player", "m_skydiveState")
        if skydive_state:
            lines.append(f"#define OFFSET_SKYDIVE_STATE {self.format_offset(skydive_state)} //[RecvTable.DT_Player].m_skydiveState updated {self.today}")

        grapple_active = self.get_value("RecvTable.DT_Player", "m_grappleActive")
        if grapple_active:
            lines.append(f"#define OFFSET_GRAPPLEACTIVED       {self.format_offset(grapple_active)} //[RecvTable.DT_Player].m_grappleActive updated {self.today}")

        grapple = self.get_value("RecvTable.DT_Player", "m_grapple")
        if grapple:
            lines.append(f"#define OFFSET_GRAPPLE              {self.format_offset(grapple)} //[RecvTable.DT_Player].m_grapple updated {self.today}")

        grapple_attached = self.get_value("RecvTable.DT_GrappleData", "m_grappleAttached")
        if grapple_attached:
            lines.append(f"#define OFFSET_GRAPPLEATTACHED      {self.format_offset(grapple_attached)} //[RecvTable.DT_GrappleData].m_grappleAttached updated {self.today}")

        xp = self.get_value("RecvTable.DT_Player", "m_xp")
        if xp:
            lines.append(f"#define OFFSET_m_xp\t\t    {self.format_offset(xp)} //[RecvTable.DT_Player].m_xp updated {self.today}")

        grade = self.get_value("RecvTable.DT_BaseEntity", "m_grade")
        if grade:
            lines.append(f"#define OFFSET_GRADE {self.format_offset(grade)} //[RecvTable.DT_BaseEntity].m_grade updated {self.today}")

        lines.append("")

        return "\n".join(lines)

    def update_header(self):
        """offsets.hファイルを更新"""
        print(f"Reading {self.ini_path}...")
        header_content = self.generate_header()

        print(f"Writing to {self.header_path}...")
        with open(self.header_path, "w") as f:
            f.write(header_content)

        print(f"✓ {self.header_path} が正常に更新されました")
        print(f"  更新日時: {self.today}")


def main():
    updater = OffsetsUpdater()
    updater.update_header()


if __name__ == "__main__":
    main()
