# The brand layer

Atelier Reel is FilmCraft with a different name on the door. We keep the change small on purpose:
upstream lands many changes a day, and every file we touch is a file that can conflict when we
pull them in. So:

**Renamed (what a person sees):** the window title, menus, dialogs, About, the Home screen and
About wordmark (now Atelier's), Help-menu and header links (the Nation, this repository), error
and crash messages in the UI crate, the project file type in open and save dialogs, installer and
package names and vendor, the browser page and favicon, and where data lives
(`~/Library/Application Support/Atelier Reel`, `%APPDATA%\Atelier Reel`).

**Not renamed (machine names):** crate names (`filmcraft-*`), the binaries (`filmcraft`,
`filmcraft-cli`), command ids and automation ids, the `ai.storyteller.filmcraft` app id and icon
file names in packaging, the `.fcproj` and `.fcgt` formats, environment variables
(`FILMCRAFT_DATA_DIR`), the Linux data folder (`~/.local/share/filmcraft`), and the docs under
`docs/`.

## Where the name lives

| Place | File |
|---|---|
| The constants and the translation rewrite | `crates/ui-egui/src/identity.rs` (`PRODUCT`, `UPSTREAM`, `rewrite`, `retired`) |
| Our own strings, with translations | `crates/ui-egui/src/i18n/brand.tsv` (added to each catalog as it loads; the upstream `.tsv` catalogs stay untouched) |
| English UI strings | literal `"Atelier Reel"` in `crates/ui-egui/src` and `apps/` |
| Links | `crates/ui-egui/src/links.rs` (menu ids keep their upstream names, the URLs are ours) |
| Wordmark | `crates/ui-egui/src/brand.rs` draws `assets/brand/atelier-wordmark*.png` |
| Data folder | `crates/engine/src/autosave.rs` (`default_data_dir`) |
| Windows file properties | `apps/filmcraft/build.rs` |
| macOS bundle | `packaging/macos/Info.plist.in` (display name, copyright, microphone prompt; identifier unchanged) |
| Windows installer | `packaging/windows/filmcraft.wxs` (names, manufacturer, links; registry keys unchanged) |
| Linux desktop entry, AppStream, nfpm | `packaging/linux/ai.storyteller.filmcraft.*`, `nfpm.yaml` |
| Browser build | `apps/filmcraft-web/web/index.html`, `favicon.png` |
| README and NOTICE | README: our header above upstream's feature sections; NOTICE: ours on top of theirs, kept on merge by `.gitattributes` |
| App icon | `assets/app-icon/` (ours; every file has a `.attribution` sidecar and a row in `ATTRIBUTION.md`) |

Still upstream's name, to be done in later passes: the creator/product tags FilmCraft writes into
exported files (AAF, OMF, MXF, EBU STL in `crates/interchange`, `crates/mxf`, `crates/captions`),
messages from the engine crates, the macOS DMG window background, and the `docs/` manuals.
