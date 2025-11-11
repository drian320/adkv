#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include "shared_memory.h"

int main() {
    printf("C++ Structure Sizes:\n");
    printf("  sizeof(Settings): %zu bytes\n", sizeof(Settings));
    printf("  sizeof(Player): %zu bytes\n", sizeof(Player));
    printf("  sizeof(Spectator): %zu bytes\n", sizeof(Spectator));
    printf("  sizeof(SharedData): %zu bytes\n", sizeof(SharedData));
    printf("\n");

    printf("Settings field offsets:\n");
    printf("  aim_enabled: %zu\n", offsetof(Settings, aim_enabled));
    printf("  esp_enabled: %zu\n", offsetof(Settings, esp_enabled));
    printf("  player_glow_enabled: %zu\n", offsetof(Settings, player_glow_enabled));
    printf("  aim_no_recoil: %zu\n", offsetof(Settings, aim_no_recoil));
    printf("  aiming: %zu\n", offsetof(Settings, aiming));
    printf("  shooting: %zu\n", offsetof(Settings, shooting));
    printf("  firing_range: %zu\n", offsetof(Settings, firing_range));
    printf("  onevone: %zu\n", offsetof(Settings, onevone));
    printf("  max_dist: %zu\n", offsetof(Settings, max_dist));
    printf("  smooth: %zu\n", offsetof(Settings, smooth));
    printf("  max_fov: %zu\n", offsetof(Settings, max_fov));
    printf("  bone: %zu\n", offsetof(Settings, bone));
    printf("\n");

    printf("SharedData field offsets:\n");
    printf("  settings: %zu\n", offsetof(SharedData, settings));
    printf("  players: %zu\n", offsetof(SharedData, players));
    printf("  spectators_list: %zu\n", offsetof(SharedData, spectators_list));

    return 0;
}
