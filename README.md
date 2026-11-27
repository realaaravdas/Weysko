<p align="left">
  <img src="assets/logo-dark.svg" alt="Websir Logo" height="44" />
</p>

# Websir

**Official Website:** [https://realaaravdas.github.io/Websir/](https://realaaravdas.github.io/Websir/) · **Releases:** [GitHub Releases](https://github.com/realaaravdas/Websir/releases)

**Websir** is a 100% pure-Rust, zero-AI visual website studio and static site compiler inspired by classic Wix Studio, built with a modern, minimalist **dark-and-white slate** workspace.

Design complete multi-page websites visually using drag-and-drop positioning, pre-built section strips, global design tokens, and responsive breakpoint previews — then compile deterministically into clean HTML5, CSS3, and vanilla JavaScript ready to launch on any web server, static host, or Google Sites.

---

## Key Features

- **Zero AI, 100% Pure Rust**: Deterministic visual builder and static compiler powered by `eframe`/`egui`, `serde_json`, `image`, `zip`, and `tiny_http`.
- **Dark & White Minimalist Slate Studio**: Deep obsidian-slate canvas (`#090D16`), slate-900 panels (`#0F172A`), crisp white primary actions (`#FFFFFF`), and subtle dot-grid alignment.
- **Interactive WYSIWYG Canvas**:
  - Click to select elements or sections; double-click text elements (`Heading`, `Paragraph`, `Button`, `Badge`) for inline canvas editing.
  - Drag to reposition with optional **8px Grid Snap** and **Smart Alignment Guides**.
  - **8-point resize handles** on elements and draggable bottom handles on section heights.
  - **Responsive Breakpoint Editor**: Switch between `Desktop (1280px)`, `Tablet (768px)`, and `Mobile (390px)` with per-breakpoint geometry overrides and Auto-Stack mobile layout.
- **16 Customizable Element Types**:
  - `Heading (H1–H6)`, `Paragraph`, `Button`, `Badge`, `Image`, `Box / Container`, `Divider`, `Navbar`, `Footer`, `Contact Form`, `List`, `Feature Card`, `FAQ Accordion`, `Video Embed (YouTube/Vimeo/MP4)`, `Custom HTML`, and `Social Links`.
- **13 Pre-Built Wix-Style Section Strips & 5 Starter Templates**:
  - `Minimalist Portfolio`, `SaaS Product Landing`, `Design Studio & Agency`, `Personal Bio & Resume`, and `Blank Canvas`.
- **6 Curated Theme Presets + Deep Customization**:
  - `Obsidian Slate`, `Pure Editorial`, `Midnight Mono`, `Nordic Steel`, `Warm Alabaster`, and `Carbon Ink`.
  - Customize global color tokens, Google Fonts / system typography, border radius, max content width, hover effects, entrance animations, box shadows, and custom global or per-element CSS.
- **Embedded Media Library**: Import PNG, JPG, WebP, GIF, or SVG images directly into your `.websir.json` project — exported cleanly to `assets/` or inline `data:` URIs.
- **Built-In Live Preview Server**: One-click pure-Rust HTTP server (`tiny_http`) serving your compiled site with instant updates.

---

## Installing & Running Websir

### Windows Installer (Recommended)
Download **`websir-v1.1.0-windows-x86_64-setup.exe`** from [GitHub Releases](https://github.com/realaaravdas/Websir/releases/latest) to install Websir with Start Menu & Desktop shortcuts, `.websir.json` file association, User `PATH` CLI registration, and a Windows Apps & Features uninstaller.

### Build from Source
```bash
# Launch the visual studio GUI
cargo run --release

# Or build both Websir Studio and the Windows Setup Installer
./scripts/build-installer.ps1
```

Or open an existing project file directly:
```bash
cargo run --release -- path/to/site.websir.json
```

---

## Exporting & Launching Your Website

Click **Export / Publish** in the top-right corner of the studio to choose from four export targets:

1. **Static Folder** — Writes `index.html`, secondary `<slug>.html` pages, `404.html`, `styles.css`, `site.js`, `assets/*`, plus optional `sitemap.xml`, `robots.txt`, and `.nojekyll`. Upload to **GitHub Pages, Nginx, Apache, AWS S3, Cloudflare Pages, or Vercel**.
2. **ZIP Archive** — Packages the full static site into a standard `.zip` ready for **Netlify Drop**, **Cloudflare Pages**, or **cPanel**.
3. **Single-File HTML** — Compiles all pages, CSS, JavaScript, and base64-encoded images into one self-contained `.html` file with client-side page switching.
4. **Google Sites Embed**:
   - Select the **Google Sites** tab in the Export modal.
   - Click **Copy Embed HTML to Clipboard**.
   - In Google Sites, open **Insert → Embed → Embed code**, paste the bundle, click **Next → Insert**, and resize the embed frame full-width.

---

## Headless CLI Mode

Websir also ships with a scriptable CLI for CI/CD pipelines and rapid exports:

```bash
# Create a new project JSON from a starter template
cargo run -- --init-template saas --out my-site.websir.json

# Export a project or template to a static folder, ZIP, or single-file HTML
cargo run -- --export template:saas --out ./dist --format folder
cargo run -- --export my-site.websir.json --out ./dist/site.zip --format zip
cargo run -- --export my-site.websir.json --out ./dist/embed.html --format google-sites

# Serve a compiled site over localhost
cargo run -- --serve template:portfolio --port 8080
```

Available template keys: `portfolio`, `saas`, `agency`, `bio`, `blank`.

---

## Keyboard Shortcuts

| Shortcut | Action |
|---|---|
| `Ctrl + Z` | Undo |
| `Ctrl + Y` / `Ctrl + Shift + Z` | Redo |
| `Ctrl + S` | Save project (`.websir.json`) |
| `Ctrl + Shift + S` | Save As |
| `Ctrl + D` | Duplicate selected element |
| `Ctrl + C` / `Ctrl + V` | Copy / Paste element |
| `Arrow Keys` / `Shift + Arrow` | Nudge selected element (`1px` / `10px`) |
| `Delete` / `Backspace` | Delete selected element |
| `Escape` | Clear selection / Close modal |
| `Double-click` on text | Edit text directly on canvas |
