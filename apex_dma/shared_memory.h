#ifndef SHARED_MEMORY_H
#define SHARED_MEMORY_H

#include <sys/mman.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <unistd.h>
#include <cstring>
#include <stdexcept>

#define SHARED_MEMORY_NAME "/apex_dma_shared"
#define SHARED_MEMORY_SIZE (1024 * 1024)  // 1MB

// Rustのデータ構造と一致させる必要がある
#pragma pack(push, 1)

struct Player {
    float head_x;
    float head_y;
    float origin_x;
    float origin_y;
    int32_t health;
    int32_t shield;
    int32_t team_num;
    float distance;
    uint8_t is_visible;
    uint8_t is_knocked;
};

struct Spectator {
    char name[64];
};

struct Settings {
    uint8_t aim_enabled;
    uint8_t esp_enabled;
    uint8_t player_glow_enabled;
    uint8_t aim_no_recoil;
    uint8_t aiming;
    uint8_t shooting;
    uint8_t firing_range;
    uint8_t onevone;

    float max_dist;
    float smooth;
    float max_fov;
    int32_t bone;

    float glow_r;
    float glow_g;
    float glow_b;

    float glow_r_visible;
    float glow_g_visible;
    float glow_b_visible;

    float glow_r_knocked;
    float glow_g_knocked;
    float glow_b_knocked;
};

struct SharedData {
    uint32_t magic;              // 0xABCD
    uint64_t g_base;
    int32_t spectators;
    int32_t allied_spectators;
    size_t player_count;
    size_t spectator_count;

    Settings settings;
    Player players[100];
    Spectator spectators_list[100];
};

#pragma pack(pop)

class SharedMemoryWriter {
private:
    int shm_fd;
    void* shm_ptr;
    SharedData* data;

public:
    SharedMemoryWriter() : shm_fd(-1), shm_ptr(nullptr), data(nullptr) {}

    bool init() {
        // 既存の共有メモリを削除（クリーンスタート）
        shm_unlink(SHARED_MEMORY_NAME);

        // 共有メモリを作成（全ユーザーがアクセス可能）
        shm_fd = shm_open(SHARED_MEMORY_NAME, O_CREAT | O_RDWR, 0666);
        if (shm_fd == -1) {
            return false;
        }

        // 権限を明示的に設定
        fchmod(shm_fd, 0666);

        // サイズ設定
        if (ftruncate(shm_fd, SHARED_MEMORY_SIZE) == -1) {
            close(shm_fd);
            return false;
        }

        // メモリマップ
        shm_ptr = mmap(0, SHARED_MEMORY_SIZE, PROT_READ | PROT_WRITE, MAP_SHARED, shm_fd, 0);
        if (shm_ptr == MAP_FAILED) {
            close(shm_fd);
            return false;
        }

        data = static_cast<SharedData*>(shm_ptr);

        // 初期化
        memset(data, 0, sizeof(SharedData));
        data->magic = 0xABCD;

        return true;
    }

    void update_game_base(uint64_t g_base) {
        if (data) {
            data->g_base = g_base;
        }
    }

    void update_spectators(int32_t spectators, int32_t allied_spectators) {
        if (data) {
            data->spectators = spectators;
            data->allied_spectators = allied_spectators;
        }
    }

    void update_players(const Player* players, size_t count) {
        if (data && count <= 100) {
            memcpy(data->players, players, sizeof(Player) * count);
            data->player_count = count;
        }
    }

    void update_spectators_list(const Spectator* spectators, size_t count) {
        if (data && count <= 100) {
            memcpy(data->spectators_list, spectators, sizeof(Spectator) * count);
            data->spectator_count = count;
        }
    }

    // GUIから設定を読み取る
    void read_settings(
        int& aim,
        bool& esp,
        bool& player_glow,
        bool& aim_no_recoil,
        float& max_dist,
        float& smooth,
        float& max_fov,
        int& bone,
        float& glowr,
        float& glowg,
        float& glowb,
        float& glowrviz,
        float& glowgviz,
        float& glowbviz,
        float& glowrknocked,
        float& glowgknocked,
        float& glowbknocked,
        bool& firing_range,
        bool& shooting,
        bool& onevone
    ) {
        if (data) {
            aim = data->settings.aim_enabled;
            esp = data->settings.esp_enabled != 0;
            player_glow = data->settings.player_glow_enabled != 0;
            aim_no_recoil = data->settings.aim_no_recoil != 0;
            max_dist = data->settings.max_dist;
            smooth = data->settings.smooth;
            max_fov = data->settings.max_fov;
            bone = data->settings.bone;
            glowr = data->settings.glow_r;
            glowg = data->settings.glow_g;
            glowb = data->settings.glow_b;
            glowrviz = data->settings.glow_r_visible;
            glowgviz = data->settings.glow_g_visible;
            glowbviz = data->settings.glow_b_visible;
            glowrknocked = data->settings.glow_r_knocked;
            glowgknocked = data->settings.glow_g_knocked;
            glowbknocked = data->settings.glow_b_knocked;
            firing_range = data->settings.firing_range != 0;
            shooting = data->settings.shooting != 0;
            onevone = data->settings.onevone != 0;
        }
    }

    ~SharedMemoryWriter() {
        if (shm_ptr != nullptr && shm_ptr != MAP_FAILED) {
            munmap(shm_ptr, SHARED_MEMORY_SIZE);
        }
        if (shm_fd != -1) {
            close(shm_fd);
        }
    }
};

#endif // SHARED_MEMORY_H
