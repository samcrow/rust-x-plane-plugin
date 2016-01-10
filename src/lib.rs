// Copyright (c) 2015 rust-x-plane-plugin developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

//!
//! The `xplane_plugin` crate provides an easy way to create X-Plane plugins in Rust.
//!
//! To import the macro, the crate must be imported like this:
//!
//! ```no_main
//! #[macro_use]
//! extern crate xplane_plugin;
//! ```
//!
//! Creating a plugin involves three steps:
//!
//! 1. Create a struct for your plugin
//! 2. Implement Plugin for your plugin struct
//! 3. Place `xplane_plugin!(YourPluginStruct)` in a file, not in any function
//!
//! # Examples
//!
//! ```no_main
//! #[macro_use]
//! extern crate xplane_plugin;
//! use xplane_plugin::*;
//! struct TestPlugin;
//! impl Plugin for TestPlugin {
//!     fn start() -> Option<Self> {
//!         Some(TestPlugin)
//!     }
//!     fn enable(&mut self) {
//!
//!     }
//!     fn disable(&mut self) {
//!
//!     }
//!
//!     fn stop(&mut self) {
//!
//!     }
//!     fn info<'a, 'b, 'c>(&self) -> PluginInfo<'a, 'b, 'c> {
//!         PluginInfo {
//!             name: "Test Plugin",
//!             signature: "org.samcrow.rustplugin.test",
//!             description: "A plugin written in Rust",
//!         }
//!     }
//! }
//!
//! xplane_plugin!(TestPlugin);
//! ```
//!

/// Stores information about a plugin that is provided to X-Plane
pub struct PluginInfo<'a, 'b, 'c> {
    /// The plugin name
    pub name: &'a str,
    /// The plugin's signature, in reverse DNS format
    pub signature: &'b str,
    /// A description of the plugin
    pub description: &'c str,
}

/// The trait that all plugins should implement
pub trait Plugin : Sized {
    /// Called when X-Plane loads this plugin
    /// On success, returns a plugin object
    fn start() -> Option<Self>;
    /// Called when the plugin is enabled
    fn enable(&mut self);
    /// Called when the plugin is disabled
    fn disable(&mut self);

    /// Returns information on this plugin
    fn info<'a, 'b, 'c>(&self) -> PluginInfo<'a, 'b, 'c>;

    // Called when the plugin is stopped
    fn stop(&mut self);
}

/// Creates an X-Plane plugin
///
/// Provide the name of your plugin struct. The callbacks that X-Plane uses will be created.
///
#[macro_export]
macro_rules! xplane_plugin {
    ($plugin_type: ty) => (
        extern crate libc;
        use std::ptr;
        use std::ffi;
        type PluginType = $plugin_type;
        type PluginPtr = *mut PluginType;
        // The plugin
        static mut PLUGIN: PluginPtr = 0 as PluginPtr;

        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn XPluginStart(outName: *mut libc::c_char, outSig: *mut libc::c_char,
            outDescription: *mut libc::c_char) -> libc::c_int
        {
            // Create the plugin, temporarily, on the stack
            let plugin_option = PluginType::start();

            match plugin_option {
                Some(plugin) => {
                    // Allocate storage
                    PLUGIN = Box::into_raw(Box::new(plugin));

                    let info = (*PLUGIN).info();

                    match ffi::CString::new(info.name).ok() {
                        Some(name) => libc::strcpy(outName, name.as_ptr()),
                        None => libc::strcpy(outName, b"<invalid>".as_ptr() as *const libc::c_char),
                    };
                    match ffi::CString::new(info.signature).ok() {
                        Some(signature) => libc::strcpy(outSig, signature.as_ptr()),
                        None => libc::strcpy(outSig, b"<invalid>".as_ptr() as *const libc::c_char),
                    };
                    match ffi::CString::new(info.description).ok() {
                        Some(description) => libc::strcpy(outDescription, description.as_ptr()),
                        None => libc::strcpy(outDescription, b"<invalid>".as_ptr() as *const libc::c_char),
                    };

                    // Success
                    1
                },
                None => {
                    // Return failure
                    0
                },
            }
        }

        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn XPluginStop() {
            (*PLUGIN).stop();
            // Free plugin
            let plugin_box = Box::from_raw(PLUGIN);
            drop(plugin_box);
            PLUGIN = ptr::null_mut();
        }

        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn XPluginEnable() {
            (*PLUGIN).enable();
        }

        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn XPluginDisable() {
            (*PLUGIN).disable();
        }

        #[allow(non_snake_case)]
        #[allow(unused_variables)]
        #[no_mangle]
        pub unsafe extern "C" fn XPluginReceiveMessage(inFrom: libc::c_int, inMessage: libc::c_int,
            inParam: *mut libc::c_void)
        {
            // Nothing
        }
    )
}

/// This module tests basic use of the plugin macro
#[cfg(test)]
pub mod test {
    use super::*;
    struct TestPlugin;
    impl Plugin for TestPlugin {
        fn start() -> Option<Self> {
            Some(TestPlugin)
        }
        fn enable(&mut self) {

        }
        fn disable(&mut self) {

        }

        fn stop(&mut self) {

        }
        fn info<'a, 'b, 'c>(&self) -> PluginInfo<'a, 'b, 'c> {
            PluginInfo {
                name: "Test Plugin",
                signature: "org.samcrow.rustplugin.test",
                description: "A plugin written in Rust",
            }
        }
    }

    xplane_plugin!(TestPlugin);
}
