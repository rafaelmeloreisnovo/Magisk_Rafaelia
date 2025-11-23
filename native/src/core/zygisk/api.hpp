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

namespace zygisk {

struct Api;
struct AppSpecializeArgs;
struct ServerSpecializeArgs;

class ModuleBase {
public:

    // This method is called as soon as the module is loaded into the target process.
    // A Zygisk API handle will be passed as an argument.
    virtual void onLoad([[maybe_unused]] Api *api, [[maybe_unused]] JNIEnv *env) {}

    // This method is called before the app process is specialized.
    // At this point, the process just got forked from zygote, but no app specific specialization
    // is applied. This means that the process does not have any sandbox restrictions and
    // still runs with the same privilege of zygote.
    //
    // All the arguments that will be sent and used for app specialization is passed as a single
    // AppSpecializeArgs object. You can read and overwrite these arguments to change how the app
    // process will be specialized.
    //
    // If you need to run some operations as superuser, you can call Api::connectCompanion() to
    // get a socket to do IPC calls with a root companion process.
    // See Api::connectCompanion() for more info.
    virtual void preAppSpecialize([[maybe_unused]] AppSpecializeArgs *args) {}

    // This method is called after the app process is specialized.
    // At this point, the process has all sandbox restrictions enabled for this application.
    // This means that this method runs with the same privilege of the app's own code.
    virtual void postAppSpecialize([[maybe_unused]] const AppSpecializeArgs *args) {}

    // This method is called before the system server process is specialized.
    // See preAppSpecialize(args) for more info.
    virtual void preServerSpecialize([[maybe_unused]] ServerSpecializeArgs *args) {}

    // This method is called after the system server process is specialized.
    // At this point, the process runs with the privilege of system_server.
    virtual void postServerSpecialize([[maybe_unused]] const ServerSpecializeArgs *args) {}
};

struct AppSpecializeArgs {
    // Required arguments. These arguments are guaranteed to exist on all Android versions.
    jint &uid;
    jint &gid;
    jintArray &gids;
    jint &runtime_flags;
    jobjectArray &rlimits;
    jint &mount_external;
    jstring &se_info;
    jstring &nice_name;
    jstring &instruction_set;
    jstring &app_data_dir;

    // Optional arguments. Please check whether the pointer is null before de-referencing
    jintArray *const fds_to_ignore;
    jboolean *const is_child_zygote;
    jboolean *const is_top_app;
    jobjectArray *const pkg_data_info_list;
    jobjectArray *const whitelisted_data_info_list;
    jboolean *const mount_data_dirs;
    jboolean *const mount_storage_dirs;
    jboolean *const mount_sysprop_overrides;

    AppSpecializeArgs() = delete;
};

struct ServerSpecializeArgs {
    jint &uid;
    jint &gid;
    jintArray &gids;
    jint &runtime_flags;
    jlong &permitted_capabilities;
    jlong &effective_capabilities;

    ServerSpecializeArgs() = delete;
};

namespace internal {
struct api_table;
template <class T> void entry_impl(api_table *, JNIEnv *);
}

// These values are used in Api::setOption(Option)
enum Option : int {
    // Force Magisk's denylist unmount routines to run on this process.
    //
    // Setting this option only makes sense in preAppSpecialize.
    // The actual unmounting happens during app process specialization.
    //
    // Set this option to force all Magisk and modules' files to be unmounted from the
    // mount namespace of the process, regardless of the denylist enforcement status.
    FORCE_DENYLIST_UNMOUNT = 0,

    // When this option is set, your module's library will be dlclose-ed after post[XXX]Specialize.
    // Be aware that after dlclose-ing your module, all of your code will be unmapped from memory.
    // YOU MUST NOT ENABLE THIS OPTION AFTER HOOKING ANY FUNCTIONS IN THE PROCESS.
    DLCLOSE_MODULE_LIBRARY = 1,
};

// Bit masks of the return value of Api::getFlags()
enum StateFlag : uint32_t {
    // The user has granted root access to the current process
    PROCESS_GRANTED_ROOT = (1u << 0),

    // The current process was added on the denylist
    PROCESS_ON_DENYLIST = (1u << 1),
};

// All API methods will stop working after post[XXX]Specialize as Zygisk will be unloaded
// from the specialized process afterwards.
struct Api {

    // Connect to a root companion process and get a Unix domain socket for IPC.
    //
    // This API only works in the pre[XXX]Specialize methods due to SELinux restrictions.
    //
    // The pre[XXX]Specialize methods run with the same privilege of zygote.
    // If you would like to do some operations with superuser permissions, register a handler
    // function that would be called in the root process with REGISTER_ZYGISK_COMPANION(func).
    // Another good use case for a companion process is that if you want to share some resources
    // across multiple processes, hold the resources in the companion process and pass it over.
    //
    // The root companion process is ABI aware; that is, when calling this method from a 32-bit
    // process, you will be connected to a 32-bit companion process, and vice versa for 64-bit.
    //
    // Returns a file descriptor to a socket that is connected to the socket passed to your
    // module's companion request handler. Returns -1 if the connection attempt failed.
    int connectCompanion();

    // Get the file descriptor of the root folder of the current module.
    //
    // This API only works in the pre[XXX]Specialize methods.
    // Accessing the directory returned is only possible in the pre[XXX]Specialize methods
    // or in the root companion process (assuming that you sent the fd over the socket).
    // Both restrictions are due to SELinux and UID.
    //
    // Returns -1 if errors occurred.
    int getModuleDir();

    // Set various options for your module.
    // Please note that this method accepts one single option at a time.
    // Check zygisk::Option for the full list of options available.
    void setOption(Option opt);

    // Get information about the current process.
    // Returns bitwise-or'd zygisk::StateFlag values.
    uint32_t getFlags();

    // Exempt the provided file descriptor from being automatically closed.
    //
    // This API only make sense in preAppSpecialize; calling this method in any other situation
    // is either a no-op (returns true) or an error (returns false).
    //
    // When false is returned, the provided file descriptor will eventually be closed by zygote.
    bool exemptFd(int fd);

    // Hook JNI native methods for a class
    //
    // Lookup all registered JNI native methods and replace it with your own methods.
    // The original function pointer will be saved in each JNINativeMethod's fnPtr.
    // If no matching class, method name, or signature is found, that specific JNINativeMethod.fnPtr
    // will be set to nullptr.
    void hookJniNativeMethods(JNIEnv *env, const char *className, JNINativeMethod *methods, int numMethods);

    // Hook functions in the PLT (Procedure Linkage Table) of ELFs loaded in memory.
    //
    // Parsing /proc/[PID]/maps will give you the memory map of a process. As an example:
    //
    //       <address>       <perms>  <offset>   <dev>  <inode>           <pathname>
    // 56b4346000-56b4347000  r-xp    00002000   fe:00    235       /system/bin/app_process64
    // (More details: https://man7.org/linux/man-pages/man5/proc.5.html)
    //
    // The `dev` and `inode` pair uniquely identifies a file being mapped into memory.
    // For matching ELFs loaded in memory, replace function `symbol` with `newFunc`.
    // If `oldFunc` is not nullptr, the original function pointer will be saved to `oldFunc`.
    void pltHookRegister(dev_t dev, ino_t inode, const char *symbol, void *newFunc, void **oldFunc);

    // Commit all the hooks that was previously registered.
    // Returns false if an error occurred.
    bool pltHookCommit();

private:
    internal::api_table *tbl;
    template <class T> friend void internal::entry_impl(internal::api_table *, JNIEnv *);
};

// Register a class as a Zygisk module

#define REGISTER_ZYGISK_MODULE(clazz) \
void zygisk_module_entry(zygisk::internal::api_table *table, JNIEnv *env) { \
    zygisk::internal::entry_impl<clazz>(table, env);                        \
}

// Register a root companion request handler function for your module
//
// The function runs in a superuser daemon process and handles a root companion request from
// your module running in a target process. The function has to accept an integer value,
// which is a Unix domain socket that is connected to the target process.
// See Api::connectCompanion() for more info.
//
// NOTE: the function can run concurrently on multiple threads.
// Be aware of race conditions if you have globally shared resources.

#define REGISTER_ZYGISK_COMPANION(func) \
void zygisk_companion_entry(int client) { func(client); }

/*********************************************************
 * The following is internal ABI implementation detail.
 * You do not have to understand what it is doing.
 *********************************************************/

namespace internal {

struct module_abi {
    long api_version;
    ModuleBase *impl;

    void (*preAppSpecialize)(ModuleBase *, AppSpecializeArgs *);
    void (*postAppSpecialize)(ModuleBase *, const AppSpecializeArgs *);
    void (*preServerSpecialize)(ModuleBase *, ServerSpecializeArgs *);
    void (*postServerSpecialize)(ModuleBase *, const ServerSpecializeArgs *);

    module_abi(ModuleBase *module) : api_version(ZYGISK_API_VERSION), impl(module) {
        preAppSpecialize = [](auto m, auto args) { m->preAppSpecialize(args); };
        postAppSpecialize = [](auto m, auto args) { m->postAppSpecialize(args); };
        preServerSpecialize = [](auto m, auto args) { m->preServerSpecialize(args); };
        postServerSpecialize = [](auto m, auto args) { m->postServerSpecialize(args); };
    }
};

struct api_table {
    // Base
    void *impl;
    bool (*registerModule)(api_table *, module_abi *);

    void (*hookJniNativeMethods)(JNIEnv *, const char *, JNINativeMethod *, int);
    void (*pltHookRegister)(dev_t, ino_t, const char *, void *, void **);
    bool (*exemptFd)(int);
    bool (*pltHookCommit)();
    int  (*connectCompanion)(void * /* impl */);
    void (*setOption)(void * /* impl */, Option);
    int  (*getModuleDir)(void * /* impl */);
    uint32_t (*getFlags)(void * /* impl */);
};

template <class T>
void entry_impl(api_table *table, JNIEnv *env) {
    static Api api;
    api.tbl = table;
    static T module;
    ModuleBase *m = &module;
    static module_abi abi(m);
    if (!table->registerModule(table, &abi)) return;
    m->onLoad(&api, env);
}

} // namespace internal

inline int Api::connectCompanion() {
    return tbl->connectCompanion ? tbl->connectCompanion(tbl->impl) : -1;
}
inline int Api::getModuleDir() {
    return tbl->getModuleDir ? tbl->getModuleDir(tbl->impl) : -1;
}
inline void Api::setOption(Option opt) {
    if (tbl->setOption) tbl->setOption(tbl->impl, opt);
}
inline uint32_t Api::getFlags() {
    return tbl->getFlags ? tbl->getFlags(tbl->impl) : 0;
}
inline bool Api::exemptFd(int fd) {
    return tbl->exemptFd != nullptr && tbl->exemptFd(fd);
}
inline void Api::hookJniNativeMethods(JNIEnv *env, const char *className, JNINativeMethod *methods, int numMethods) {
    if (tbl->hookJniNativeMethods) tbl->hookJniNativeMethods(env, className, methods, numMethods);
}
inline void Api::pltHookRegister(dev_t dev, ino_t inode, const char *symbol, void *newFunc, void **oldFunc) {
    if (tbl->pltHookRegister) tbl->pltHookRegister(dev, inode, symbol, newFunc, oldFunc);
}
inline bool Api::pltHookCommit() {
    return tbl->pltHookCommit != nullptr && tbl->pltHookCommit();
}

} // namespace zygisk

extern "C" {

[[gnu::visibility("default"), maybe_unused]]
void zygisk_module_entry(zygisk::internal::api_table *, JNIEnv *);

[[gnu::visibility("default"), maybe_unused]]
void zygisk_companion_entry(int);

} // extern "C"
