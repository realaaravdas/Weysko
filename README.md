<p align="left">
  <img src="assets/logo-dark.svg" alt="Weysko Logo" height="44" />
</p>

# Weysko

**Official Website:** [https://realaaravdas.github.io/Weysko/](https://realaaravdas.github.io/Weysko/) · **Guide & Tutorial:** [User Guide](https://realaaravdas.github.io/Weysko/guide.html) · **Web Demo:** [Launch Demo](https://realaaravdas.github.io/Weysko/demo.html) · **Releases:** [GitHub Releases](https://github.com/realaaravdas/Weysko/releases)

**Weysko** is a 100% pure-Rust, zero-AI visual website studio and static site compiler inspired by classic Wix Studio, built with a minimalist **dark-and-white slate** workspace.

Start from an Adobe-style Creative Studio launcher to pick a website visual style, import a `.weysko` project, reopen past projects, or start from templates and scratch — then design visually and export clean HTML5, CSS3, and vanilla JavaScript ready for any server or Google Sites.

---

## Key Features (v1.3.2)

- **Zero AI, 100% Pure Rust**: Deterministic visual builder and static compiler powered by `eframe`/`egui`, `serde_json`, `image`, `zip`, and `tiny_http`.
- **Creative Studio Start Screen**: Choose from 6 visual styles, browse wireframe starter templates (including the 3-page **Meridian Cloud** example), build custom multi-page skeletons from scratch, import `.weysko` files, or reopen recent projects.
- **Portable `.weysko` Project Format**: Self-contained project envelope preserving all pages, sections, design tokens, responsive rules, and base64 assets (backwards-compatible with `.weysko.json`).
- **Interactive WYSIWYG Canvas & Polished Status Bar**:
  - Click to select elements or sections; double-click text elements (`Heading`, `Paragraph`, `Button`, `Badge`) for inline canvas editing.
  - Drag to reposition with optional **8px Grid Snap**, **Smart Alignment Guides**, and **8-point resize handles**.
  - **Responsive Breakpoint Editor**: `Desktop (1280px)`, `Tablet (768px)`, and `Mobile (390px)` previews with Auto-Stack mobile reflow.
- **16 Native Elements & 13 Section Strips**: `Heading`, `Paragraph`, `Button`, `Badge`, `Image`, `Box`, `Divider`, `Navbar`, `Footer`, `Contact Form`, `List`, `Feature Card`, `FAQ Accordion`, `Video Embed`, `Custom HTML`, and `Social Links`.
- **6 Starter Templates**: `Meridian Cloud (Example)`, `Minimalist Portfolio`, `SaaS Product Landing`, `Design Studio & Agency`, `Personal Bio & Resume`, and `Blank Canvas`.
- **Cross-Platform Desktop Builds**: Automated GitHub Actions releases for **Windows** (`.exe` Setup & Portable ZIP), **macOS** (`.dmg`), and **Linux** (`.AppImage`, `.deb`, `.tar.gz`).

---

## Installing & Running Weysko v1.3.2

- **Windows:** Download **`weysko-v1.3.2-windows-x86_64-setup.exe`** from [GitHub Releases](https://github.com/realaaravdas/Weysko/releases/latest) for Start Menu & Desktop shortcuts, `.weysko` / `.weysko.json` file associations, User `PATH` CLI registration, and uninstaller.
- **macOS (Apple Silicon):** Download **`weysko-v1.3.2-macos-arm64.dmg`**, open, and drag **Weysko.app** into Applications.
- **Linux (`x86_64`):** Download **`weysko-v1.3.2-linux-x86_64.AppImage`** (`chmod +x` and run) or install **`weysko-v1.3.2-linux-x86_64.deb`**.

### Build from Source
```bash
# On Debian/Ubuntu Linux, install GUI & GL headers first:
sudo apt-get update && sudo apt-get install -y \
  libgtk-3-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev \
  libxkbcommon-dev libgl1-mesa-dev libwayland-dev libxrandr-dev pkg-config

# Build & launch the visual studio GUI
cargo run --release --bin weysko

# Open an existing .weysko or .weysko.json project file directly
cargo run --release --bin weysko -- examples/meridian-cloud.weysko

# Or build both Weysko Studio and the Windows Setup Installer
powershell -ExecutionPolicy Bypass -File .\scripts\build-installer.ps1
```

---

## Exporting & Launching Your Website

Click **Export / Publish** in the studio to choose from five targets:

1. **`.weysko` Project File** — Portable editable bundle preserving pages, design tokens, breakpoints, and assets.
2. **Static Folder** — Multi-page HTML5, `styles.css`, `site.js`, `assets/*`, `404.html`, `sitemap.xml`, `robots.txt`, and `.nojekyll`.
3. **ZIP Archive** — Ready for **Netlify Drop**, **Cloudflare Pages**, or **cPanel**.
4. **Single-File HTML** — All pages, CSS, JavaScript, and base64 media in one portable HTML document.
5. **Google Sites Embed** — Click **Copy Embed HTML to Clipboard** and paste into Google Sites via **Insert → Embed → Embed code**.

---

## Headless CLI Mode

```bash
# Create a portable .weysko project file from a template
weysko --init-template meridian --out meridian-cloud.weysko

# Export to static folder, ZIP, Google Sites embed, or .weysko bundle
weysko --export meridian-cloud.weysko --out ./dist --format folder
weysko --export meridian-cloud.weysko --out ./site.zip --format zip
weysko --export meridian-cloud.weysko --out ./embed.html --format google-sites
weysko --export meridian-cloud.weysko --out ./backup.weysko --format weysko

# Serve a compiled site over localhost
weysko --serve meridian-cloud.weysko --port 8080
```

Available template keys: `meridian`, `portfolio`, `saas`, `agency`, `bio`, `blank`.

---

## Keyboard Shortcuts

| Shortcut | Action |
|---|---|
| `Ctrl + Z` / `Ctrl + Y` | Undo / Redo |
| `Ctrl + S` / `Ctrl + Shift + S` | Save / Save As (`.weysko`) |
| `Ctrl + C` / `Ctrl + V` / `Ctrl + D` | Copy / Paste / Duplicate element |
| `Ctrl + =` / `Ctrl + -` / `Ctrl + 0` | Zoom in / out / auto-fit canvas |
| `Arrow Keys` / `Shift + Arrow` | Nudge selected element (`1px` / `10px`) |
| `Delete` / `Backspace` | Delete selected element |
| `Double-click` on text | Edit text directly on canvas |
| `F1` | Toggle shortcuts quick guide |
