# Atelier Reel app icon

**The hidden sun behind a strip of film**: an ivory disc with an ochre film strip crossing its
lower half, three frames and its sprocket holes, on a volcanic black field. Three colours, from the
Nation's palette: volcanic black `#141210`, ivory `#efe6d2`, ochre `#d9a441`.

**Tile:** `viewBox="0 0 512 512"`, a rounded square with `rx=112`, as all the Atelier icons. The
macOS files add Apple's 824/1024 margin (`filmcraft-macos-512.png`, `.icns`). `filmcraft.svg` is
the master; the file names keep their upstream `filmcraft` names so packaging stays untouched.

**Provenance:** drawn for Atelier Reel by hand as SVG (2026). It replaces FilmCraft's owl icon,
which stays in FilmCraft's own repository. Every file here has a `.attribution` sidecar and a row in
`ATTRIBUTION.md`. Licence: `LICENSE.txt` (MIT OR Apache-2.0).

**Regenerating:** `packaging/icons.sh` (needs resvg; iconutil on macOS for the .icns). The
committed renders were made from the same SVG with cairosvg and Pillow.
