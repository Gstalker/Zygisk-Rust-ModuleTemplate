use std::os::raw::*;

use jni::{objects::JString, sys::*};

#[allow(non_camel_case_types)]
type c_bool = bool;
type Module = crate::module::RawModule;

pub const API_VERSION: c_long = 2;

#[repr(C)]
pub(crate) struct ModuleAbi {
    pub api_version: c_long,
    pub this: &'static mut Module,
    pub pre_app_specialize: extern "C" fn(&mut Module, &mut AppSpecializeArgs),
    pub post_app_specialize: extern "C" fn(&mut Module, &AppSpecializeArgs),
    pub pre_server_specialize: extern "C" fn(&mut Module, &mut ServerSpecializeArgs),
    pub post_server_specialize: extern "C" fn(&mut Module, &ServerSpecializeArgs),
}

#[repr(C)]
pub(crate) struct RawApiTable {
    // These first 2 entries are permanent, shall never change across API versions
    pub this: *const (),
    pub register_module: Option<extern "C" fn(*const RawApiTable, *mut ModuleAbi) -> c_bool>,

    // Utility functions
    pub hook_jni_native_methods:
        Option<extern "C" fn(*mut JNIEnv, *const c_char, *mut JNINativeMethod, c_int)>,
    pub plt_hook_register:
        Option<extern "C" fn(*const c_char, *const c_char, *mut (), *mut *mut ())>,
    pub plt_hook_exclude: Option<extern "C" fn(*const c_char, *const c_char)>,
    pub plt_hook_commit: Option<extern "C" fn() -> c_bool>,

    // Zygisk functions
    pub connect_companion: Option<extern "C" fn(*const ()) -> c_int>,
    pub set_option: Option<extern "C" fn(*const (), ZygiskOption)>,
    pub get_module_dir: Option<extern "C" fn(*const ()) -> c_int>,
    pub get_flags: Option<extern "C" fn(*const ()) -> u32>,
}

#[repr(C)]
pub struct AppSpecializeArgs<'a> {
    // Required arguments. These arguments are guaranteed to exist on all Android versions.
    pub uid: &'a mut jint,
    pub gid: &'a mut jint,
    pub gids: &'a mut jintArray,
    pub runtime_flags: &'a mut jint,
    pub mount_external: &'a mut jint,
    pub se_info: &'a mut JString<'a>,
    pub nice_name: &'a mut JString<'a>,
    pub instruction_set: &'a mut JString<'a>,
    pub app_data_dir: &'a mut JString<'a>,

    // Optional arguments. Please check whether the pointer is null before de-referencing
    pub is_child_zygote: Option<&'a jboolean>,
    pub is_top_app: Option<&'a jboolean>,
    pub pkg_data_info_list: Option<&'a jobjectArray>,
    pub whitelisted_data_info_list: Option<&'a jobjectArray>,
    pub mount_data_dirs: Option<&'a jboolean>,
    pub mount_storage_dirs: Option<&'a jboolean>,
}

#[repr(C)]
pub struct ServerSpecializeArgs<'a> {
    pub uid: &'a mut jint,
    pub gid: &'a mut jint,
    pub gids: &'a mut jintArray,
    pub runtime_flags: &'a mut jint,
    pub permitted_capabilities: &'a mut jlong,
    pub effective_capabilities: &'a mut jlong,
}

/// Zygisk module options, used in [ZygiskApi::set_option()](crate::ZygiskApi::set_option).

// Note: the original definition is `enum Option : int`. This is a best-effort approach.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ZygiskOption {
    /// Force Magisk's denylist unmount routines to run on this process.
    ///
    /// Setting this option only makes sense in `preAppSpecialize`.
    /// The actual unmounting happens during app process specialization.
    ///
    /// Set this option to force all Magisk and modules' files to be unmounted from the
    /// mount namespace of the process, regardless of the denylist enforcement status.
    ForceDenylistUnmount = 0,

    /// When this option is set, your module's library will be `dlclose`-ed after `post[XXX]Specialize`.
    /// Be aware that after `dlclose`-ing your module, all of your code will be unmapped from memory.
    ///
    /// YOU MUST NOT ENABLE THIS OPTION AFTER HOOKING ANY FUNCTIONS IN THE PROCESS.
    DlcloseModuleLibrary = 1,
}

bitflags::bitflags! {
    /// Bit masks of the return value of [ZygiskApi::get_flags()](crate::ZygiskApi::get_flags).
    pub struct StateFlags: u32 {
        /// The user has granted root access to the current process.
        const PROCESS_GRANTED_ROOT = (1 << 0);

        /// The current process was added on the denylist.
        const PROCESS_ON_DENYLIST = (1 << 1);
    }
}
