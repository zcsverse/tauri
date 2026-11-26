// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use cef::*;

#[cfg(target_os = "linux")]
type CefOsEvent<'a> = Option<&'a mut cef::sys::XEvent>;
#[cfg(target_os = "macos")]
type CefOsEvent<'a> = *mut u8;
#[cfg(windows)]
type CefOsEvent<'a> = Option<&'a mut cef::sys::MSG>;

wrap_keyboard_handler! {
  pub struct TauriCefKeyboardHandler {
    devtools_enabled: bool,
  }

  impl KeyboardHandler {
    fn on_pre_key_event(
      &self,
      _browser: Option<&mut Browser>,
      event: Option<&KeyEvent>,
      _os_event: CefOsEvent<'_>,
      is_keyboard_shortcut: Option<&mut ::std::os::raw::c_int>,
    ) -> ::std::os::raw::c_int {
      let Some(event) = event else {
        return 0;
      };

      // Check if this is a keydown event.
      use cef::sys::cef_key_event_type_t;
      let keydown_type: cef::KeyEventType = cef_key_event_type_t::KEYEVENT_RAWKEYDOWN.into();
      if event.type_ != keydown_type {
        return 0;
      }

      // Get modifier keys.
      use cef::sys::cef_event_flags_t;
      #[cfg(windows)]
      let modifiers = event.modifiers as i32;
      #[cfg(not(windows))]
      let modifiers = event.modifiers;

      let ctrl = (modifiers & (cef_event_flags_t::EVENTFLAG_CONTROL_DOWN.0)) != 0;
      let shift = (modifiers & (cef_event_flags_t::EVENTFLAG_SHIFT_DOWN.0)) != 0;
      let alt = (modifiers & (cef_event_flags_t::EVENTFLAG_ALT_DOWN.0)) != 0;
      #[cfg(target_os = "macos")]
      let meta = (modifiers & cef_event_flags_t::EVENTFLAG_COMMAND_DOWN.0) != 0;
      // macOS routes these through the app menu instead; see below.
      #[cfg(target_os = "macos")]
      let _ = (ctrl, shift);

      let key_code = event.windows_key_code;

      // If devtools is disabled, block devtools keyboard shortcuts.
      if !self.devtools_enabled {
        // F12 (key code 123).
        let f12 = key_code == 123;
        // Ctrl+Shift+I (key code 73 = 'I') on Linux/Windows.
        #[cfg(not(target_os = "macos"))]
        let inspect = key_code == 73 && ctrl && shift;
        // Cmd+Opt+I on macOS.
        #[cfg(target_os = "macos")]
        let inspect = key_code == 73 && meta && alt;
        if f12 || inspect {
          if let Some(is_keyboard_shortcut) = is_keyboard_shortcut {
            *is_keyboard_shortcut = 1;
          }
          return 1;
        }
      }

      // Chrome runs its reserved accelerators (close tab and window, exit, new tab and
      // window, restore tab, tab cycling) before the page sees the key, so a page can
      // never handle them, and the ones `on_chrome_command` swallows are lost entirely:
      // Ctrl+W, Ctrl+T and Ctrl+N never reach a terminal emulator in the page.
      //
      // Reporting them as shortcuts skips that early pass: the page gets the key first,
      // and only a key it leaves unhandled goes on to the accelerator, where
      // `on_chrome_command` still decides whether the command runs.
      #[cfg(not(target_os = "macos"))]
      if ctrl && !alt && is_reserved_accelerator(key_code, shift) {
        if let Some(is_keyboard_shortcut) = is_keyboard_shortcut {
          *is_keyboard_shortcut = 1;
        }
      }

      0
    }
  }
}

/// Keys whose Ctrl accelerator maps to one of Chrome's reserved commands
/// (`kReservedCommandIds` in `chrome/browser/ui/accelerator_table.cc`).
#[cfg(not(target_os = "macos"))]
fn is_reserved_accelerator(key_code: ::std::os::raw::c_int, shift: bool) -> bool {
  const VKEY_TAB: ::std::os::raw::c_int = 0x09;
  const VKEY_PRIOR: ::std::os::raw::c_int = 0x21;
  const VKEY_NEXT: ::std::os::raw::c_int = 0x22;
  const VKEY_N: ::std::os::raw::c_int = 0x4E;
  const VKEY_Q: ::std::os::raw::c_int = 0x51;
  const VKEY_T: ::std::os::raw::c_int = 0x54;
  const VKEY_W: ::std::os::raw::c_int = 0x57;
  const VKEY_F4: ::std::os::raw::c_int = 0x73;
  match key_code {
    VKEY_W | VKEY_F4 | VKEY_N | VKEY_T | VKEY_TAB | VKEY_PRIOR | VKEY_NEXT => true,
    VKEY_Q => shift,
    _ => false,
  }
}
