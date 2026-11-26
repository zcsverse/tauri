---
tauri-runtime-cef: patch:bug
---

Block Chrome's close tab, close window and exit commands (Ctrl+W, Ctrl+Shift+W, Ctrl+Shift+Q) in application windows by default. They closed the hosting browser and left an empty window that could no longer handle its close request.
