# Markdown Reader GUI App in Rust

## Features  
- Tabbed Markdown reader 
- Cross-platform
- Designed with future editing capabilities in mind.
- Application using the **egui** ecosystem. 
- Recent files (`C:\Users\username\AppData\Roaming\md-reader\recent.json`)
- Zoom in/out
- Markdown rendering isolated to one call (`CommonMarkViewer::new().show(ui, &mut tab.cache, &tab.content)` in `app.rs`), which means swapping to a different markdown crate is easy without touching the rest of the app logic.

## Next
todo: move this to GH Issues  

Major  
- In-doc links not working
- Remember last opened files
- Remember window position/size
- Remember zoom level
- Try/pick best markdown renderer

Medium  
 - .. 

Minor 
 - .. 

## Technology Choice: egui + eframe

| Considered | Verdict | Reason |
|---|---|---|
| **egui/eframe** | ✅ **Selected** | Mature, pure Rust, immediate-mode makes tabs and future editing natural. Large ecosystem. |
| Iced | ❌ | More boilerplate (Elm arch), tab support requires more custom work |
| Tauri | ❌ | Hybrid Rust+Web — not what a "Rust GUI app" typically implies |
| Slint | ❌ | Smaller ecosystem, DSL adds complexity |

### Key Crates & Versions

| Crate | Version | Purpose |
|---|---|---|
| `eframe` | `0.36` | Windowed application framework (wraps egui) |
| `egui` | `0.36` | Core UI library |
| `egui_commonmark` | `0.25` | Renders CommonMark/GFM inside egui panels |
| `egui_extras` | `0.36` | Extra widgets (tables, etc.) — required by egui_commonmark |
| `rfd` | `0.15` | Native file-open dialogs (cross-platform) |
| `serde` + `serde_json` | latest | Persist recent-files list to disk |
| `dirs` | latest | Locate platform config directory for settings |

> [!IMPORTANT]
> `egui_commonmark 0.25` depends on `egui ^0.36`, so we pin the entire stack to the **0.36** generation.

---

## Architecture

```
md-reader/
├── Cargo.toml
└── src/
    ├── main.rs          # Entry point, eframe::run_native
    ├── app.rs           # MdReaderApp — top-level state & update()
    ├── tab.rs           # TabState struct (path, content, scroll, cache)
    ├── menu.rs          # File & View menu rendering + actions
    └── recent.rs        # RecentFiles: load/save JSON, max 20 entries
```

### Data Model

```
MdReaderApp
 ├── tabs: Vec<TabState>        // Open documents
 ├── active_tab: usize          // Index of focused tab
 ├── recent_files: RecentFiles  // Persistent recent-file list
 ├── zoom_level: f32            // Global text scale factor
 └── CommonMarkCache            // Shared cache for images/rendering
```

Each `TabState` holds:
- `file_path: PathBuf` — source file
- `title: String` — filename for the tab label
- `content: String` — raw markdown text (kept in memory for future editing)
- `cache: CommonMarkCache` — per-tab rendering cache

> [!NOTE]
> **Future editing hook:** Keeping `content: String` as a mutable field means we can later swap the rendered view for a `TextEdit` widget and add a live-preview split pane without restructuring.


## Verification Plan

### Build & Run
```bash
cargo build
cargo run
```
- Verify window opens with menu bar and empty state.
- Open a `.md` file → content renders in a scrollable panel.
- Open multiple files → tabs appear, switching works.
- Recent files persist across app restarts.
- Zoom in/out changes text size globally.

### Cross-Platform
- Primary verification on Windows (your machine).
- Architecture is cross-platform by design (egui + rfd).

## Open Questions

> [!NOTE]
> **Why not egui_dock?** `egui_dock` provides full docking (drag-to-split, undock to window), which is powerful but adds significant complexity for an MVP. A simple horizontal tab bar (like SumatraPDF) is simpler, lighter, and can be upgraded to `egui_dock` later if needed. Let me know if you'd prefer the full docking approach from the start.

