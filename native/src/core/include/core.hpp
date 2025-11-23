/*
 * Original Magisk Copyright:
 * Copyright 2017 - 2025, John Wu (@topjohnwu)
 *
 * RAFAELIA Framework Additions:
 * Copyright 2025, Rafael Melo Reis (rafaelmeloreisnovo)
 * Instituto Rafael - CientiEspiritual Philosophy
 *
 * This file is part of Magisk_Rafaelia, a derivative work of Magisk.
 * 
 * Magisk_Rafaelia is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 *
 * ---
 * RAFAELIA PHILOSOPHY (Aspirational Commentary - Not Part of License):
 * 
 * Sacred Cycle: VAZIO → VERBO → CHEIO → RETRO (EMPTY → ACTION → FULL → FEEDBACK)
 * Motto: "Amor, Luz e Coerência" (Love, Light and Coherence)
 * Ethica[8]: Transparency, Accountability, Fairness, Privacy, Security,
 *            Reliability, Safety, Sustainability
 * ---
 */

#pragma once

#include <sys/socket.h>
#include <string>
#include <atomic>
#include <functional>

#include <base.hpp>

#include "../core-rs.hpp"

#define AID_ROOT   0
#define AID_SHELL  2000
#define AID_USER_OFFSET 100000

#define to_app_id(uid)  (uid % AID_USER_OFFSET)
#define to_user_id(uid) (uid / AID_USER_OFFSET)

#define SDK_INT      (MagiskD::Get().sdk_int())
#define APP_DATA_DIR (SDK_INT >= 24 ? "/data/user_de" : "/data/user")

inline int connect_daemon(RequestCode req) {
    return connect_daemon(req, false);
}

// Multi-call entrypoints
int su_client_main(int argc, char *argv[]);
int zygisk_main(int argc, char *argv[]);

struct ModuleInfo;

// Utils
const char *get_magisk_tmp();
void unlock_blocks();
bool check_key_combo();
template<typename T> requires(std::is_trivially_copyable_v<T>)
T read_any(int fd) {
    T val;
    if (xxread(fd, &val, sizeof(val)) != sizeof(val))
        return -1;
    return val;
}
template<typename T> requires(std::is_trivially_copyable_v<T>)
void write_any(int fd, T val) {
    if (fd < 0) return;
    xwrite(fd, &val, sizeof(val));
}
inline int read_int(int fd) { return read_any<int>(fd); }
inline void write_int(int fd, int val) { write_any(fd, val); }
std::string read_string(int fd);
bool read_string(int fd, std::string &str);
void write_string(int fd, std::string_view str);
template<typename T> requires(std::is_trivially_copyable_v<T>)
void write_vector(int fd, const std::vector<T> &vec) {
    write_int(fd, vec.size());
    xwrite(fd, vec.data(), vec.size() * sizeof(T));
}
template<typename T> requires(std::is_trivially_copyable_v<T>)
bool read_vector(int fd, std::vector<T> &vec) {
    int size = read_int(fd);
    vec.resize(size);
    return xread(fd, vec.data(), size * sizeof(T)) == size * sizeof(T);
}

// Scripting
void install_apk(Utf8CStr apk);
void uninstall_pkg(Utf8CStr pkg);
void exec_common_scripts(Utf8CStr stage);
void exec_module_scripts(Utf8CStr stage, const rust::Vec<ModuleInfo> &module_list);
void exec_script(Utf8CStr script);
void clear_pkg(const char *pkg, int user_id);
[[noreturn]] void install_module(Utf8CStr file);

// Denylist
extern std::atomic<bool> denylist_enforced;
int denylist_cli(rust::Vec<rust::String> &args);
void denylist_handler(int client);
void initialize_denylist();
void scan_deny_apps();
bool is_deny_target(int uid, std::string_view process);
void revert_unmount(int pid = -1) noexcept;
void update_deny_flags(int uid, rust::Str process, uint32_t &flags);

// MagiskSU
void exec_root_shell(int client, int pid, SuRequest &req, MntNsMode mode);

// Rust bindings
inline Utf8CStr get_magisk_tmp_rs() { return get_magisk_tmp(); }
inline rust::String resolve_preinit_dir_rs(Utf8CStr base_dir) {
    return resolve_preinit_dir(base_dir.c_str());
}
