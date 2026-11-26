---
tauri-runtime-cef: patch:bug
---

Block Chrome's close tab, close window and exit commands (Ctrl+W, Ctrl+Shift+W, Ctrl+Shift+Q) in application windows by default. They closed the hosting browser and left an empty window that could no longer handle its close request.

On Linux and Windows, Chrome's reserved accelerators (Ctrl+W, Ctrl+T, Ctrl+N, Ctrl+Tab and friends) now reach the page first instead of being consumed before it, so a page such as a terminal emulator can use them; keys the page leaves unhandled still go through the command filter.
