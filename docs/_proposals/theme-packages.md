# Editable native theme packages

Status: design proposal, not an implemented package format. Raccoon Hollow is
currently a compiled appearance profile. Installing a folder with the example
manifest below does not yet create a new appearance.

## Outcome

An author should be able to build a substantially different Spotifast appearance
by editing local files, without changing Rust code or compiling the application.
A listener installs the package and selects it in Appearance. Spotifast remains
a native Spotify client, with the same playback, navigation and library actions.

Raccoon Hollow should become an example package that exercises the common
renderer, rather than a special case that future themes must copy in Rust.
An eventual upstream change should introduce that capability with examples and
documentation. This proposal and the current prototype are kept in the fork;
no upstream pull request is being opened yet.

## What the current prototype teaches us

The existing JSON palettes are managed by `fastframe_theme::Catalog` through
`src/theme.rs`. Their scan and reload results are applied in `src/app.rs`, with
cached palettes preserving the last valid appearance. Keep that working path.

The prototype adds `AppearanceProfile::RaccoonHollow`, a separate `RaccoonLayout`
in settings, and conditions spread through the shell, typography, icons, lyrics,
collection headers and player bar. This proves the native views can look very
different, but another author still needs code to create a third appearance.
Adding more named profiles would multiply these conditions.

The useful reusable parts are the global navigation header, independent panel
geometry, font fallbacks, pixel controls and segmented spectrum. Their choices
should become properties of a resolved appearance. The raccoon sprite, palette,
font and decorative label should become package resources.

## Scope for a first version

- Folder packages with a versioned `theme.json`, local assets, author information
  and resource licenses. Editing files is the first authoring tool.
- Named colour and typography roles, widget geometry, icon replacements,
  decorations, artwork sampling and spectrum styling.
- Arrangement of existing native components in a constrained shell: navigation
  in a global header or the content area, library and auxiliary panels on either
  side, and transport at the top or bottom. Content always occupies the centre.
- Explicit narrow-window rules, validated dimensions and an accessible route
  back to the classic appearance.
- Preview, reload and actionable validation messages before committing a choice.

Defer a drag-and-drop editor, a theme marketplace and archive import until the
format and installation behaviour are stable. Folder packages are easier to
inspect, edit and share; adding ZIP import later does not require a new renderer.

The package configures existing components. It cannot invent a new Spotify
feature, alter backend actions or introduce arbitrary executable widgets.
This is a native appearance description, without HTML, CSS, scripts or a browser.

## Proposed package

```text
themes/
  raccoon-hollow/
    theme.json
    README.md
    LICENSES/
      PixelifySans.txt
      artwork.txt
    fonts/
      PixelifySans.ttf
    images/
      mascot.png
    icons/
      play.png
      pause.png
```

An illustrative manifest, subject to implementation review:

```json
{
  "format_version": 1,
  "id": "josmar.raccoon-hollow",
  "name": "Raccoon Hollow",
  "author": "Josmar",
  "variants": {
    "dark": {
      "base": "dark",
      "palette": {
        "window": "#151c19",
        "panel": "#1c2721",
        "surface": "#28372d",
        "text": "#eadfc6",
        "accent": "#a7c786"
      }
    }
  },
  "typography": {
    "primary": "fonts/PixelifySans.ttf",
    "body_size": 14,
    "lyrics_size": 26
  },
  "widgets": {
    "corner_radius": 0,
    "shadow_blur": 0,
    "image_sampling": "nearest"
  },
  "icons": {
    "play": "icons/play.png",
    "pause": "icons/pause.png"
  },
  "decoration": {
    "header_image": "images/mascot.png",
    "header_label": "Raccoon Hollow"
  },
  "layout": {
    "navigation": "header",
    "library": { "side": "left", "preferred_width": 220 },
    "auxiliary": { "side": "right", "preferred_width": 320 },
    "transport": "bottom",
    "collection_cover_size": 128,
    "narrow": { "auxiliary": "overlay" }
  },
  "visualizer": {
    "placement": "above-controls",
    "preferred_mode": "spectrum",
    "height": 62,
    "spectrum_style": "segments",
    "segment_gap": 2,
    "peak_colour": "#c89b62"
  }
}
```

Unspecified colours inherit the chosen base palette. Unspecified icons use the
standard icons. Omitted typography retains the existing fonts. The manifest
lists supported variants; a dark-only package must not silently manufacture a
light version. Body, title and lyrics roles should be independent, so pixel art
does not require a pixel font for every line of text.

The package's visualizer mode is an initial preference, not a forced playback
setting. The user's current choice wins. The spectrum always consumes the
existing post-EQ, pre-volume signal, including when volume is zero. Themes
change rendering, not audio analysis or the signal path.

## Native implementation

Resolve legacy palettes and package descriptions into an `AppearanceSpec` with
typed style, resources and shell layout. UI views read that resolved description
instead of checking whether the selected theme is Raccoon Hollow. Keep the spec
independent of `App`, backend state and Spotify objects.

Use registered native components with explicit ownership. The library, content,
navigation, auxiliary panel and transport each have one instance. Queue and
lyrics remain alternatives in the auxiliary slot. A theme may move a slot, but
may not duplicate controls with independent queue or lyrics state. All existing
controls still emit the same `Action`s, applied after drawing in `src/app.rs`.

Read files and prepare resources off the drawing path. Cache decoded assets and
font definitions; a normal frame must not scan folders or rebuild fonts. Pass an
accepted candidate back to the application, which applies style and uploads
textures through egui. Navigation and playback keep running while resources load.

The existing palette catalog remains the legacy adapter. Reuse its reload and
last-valid-result behaviour where possible. If shared package loading belongs
in fastframe, develop that change upstream using the repository's dependency
policy rather than copying dependency source into Spotifast.

## Compatibility and user control

- Existing top-level JSON palettes retain their meaning, filenames and reload
  behaviour. Package manifests live in subdirectories, avoiding discovery clashes.
- Classic light, dark and system appearances, Omarchy integration and `.wsz`
  mini-player skins remain available. Main-window packages do not reinterpret
  Winamp assets or replace its renderer.
- Store a package selection in a new optional setting. Keep classic colour,
  accent, font and visualizer preferences intact while a package is active.
  Returning to Classic restores them rather than clearing them on selection.
- Store panel and visualizer overrides separately by stable package ID. Theme
  updates must not overwrite user-resized widths or settings from another theme.
- Migrate the prototype's Raccoon Hollow selection and widths to the bundled
  package. Keep old settings readable and preserve a classic fallback for older
  application versions. Writes remain atomic.
- Spotify cover art remains original. Decorative images cannot replace track
  identity, accessible labels or the actual application name.
- OS launcher icons remain an explicit installation choice. Native window or
  in-app imagery and a desktop's cached launcher icon are different integrations;
  selecting a downloaded theme should not rewrite desktop entries.

## Editing and recovery

Start with an Appearance action to open the package folder and an explicit
Reload action. A developer preview can add debounced automatic reload after
manual reload is reliable. Parse and validate the entire candidate before
replacing the active appearance, so half-written edits do not flicker on screen.

Show errors with the package, filename and field. Keep the last accepted
appearance if an edit fails. Persist the selected ID and a small resolved
description; after restart, reload its assets and fall back to standard fonts
or icons if resources have disappeared. Keep the saved selection available
for repair without preventing the app from opening.

Always retain a keyboard-accessible reset to Classic outside package control.
Themes cannot hide essential transport controls, search/account access or the
reset path. Preserve font fallbacks for scripts and emoji, focus indication,
accessible names, zoom and adequate hit targets. Validate manifest dimensions
and apply the native minimum sizes before computing panel geometry.

Resolve resource paths inside the installed package root, including symlinks.
Bound file sizes, image dimensions and decoding work. A package uses local
resources, with no executable hooks or remote asset downloads. Report unsupported
format versions and unknown required fields instead of quietly changing layout.

## Delivery and evidence

1. Introduce the typed description, resolver, schema and folder discovery, with
   compatibility and validation tests. The default renderer stays unchanged.
2. Generalize geometry, typography, icons and spectrum rendering. Convert
   Raccoon Hollow to a package and remove theme-name conditions from views.
3. Add selection, preview, reload, error recovery and migration of the prototype's
   settings. Document a minimal package and a complete annotated example.
4. Exercise a second contrasting package using the same capabilities, without
   a new enum variant or renderer branch. This is the test of generality.
5. Prepare matching demo captures at narrow and normal sizes, in supported
   variants, with queue, lyrics, menus and validation failures. Obtain visual
   scope approval before any eventual upstream merge.

Acceptance includes unchanged legacy palette and skin tests, selecting and
restoring Classic during playback, valid and invalid reloads, missing resources,
font fallback, path validation, and independently persisted user geometry.
Use the existing demo feature for functional and accessibility coverage. Run the
full repository checks and report actual Linux, macOS and Windows coverage.

## Decisions to resolve before implementation

Choose a small vocabulary of layout operations, not a general widget-tree
language. Validate opposite-side panels and the narrow auxiliary overlay before
promising them; the current prototype only proves the global header, left
library, right lyrics and bottom transport arrangement.

Decide which loading pieces belong in fastframe and which component layout
rules are specific to Spotifast. Agree on the format's extension policy and
limits before freezing version 1. Evaluate usability with an author creating
the second theme from documentation alone.

A visual editor can later write this same manifest and preview it through the
same resolver. Keeping editing separate from rendering allows the file-based
workflow and any future editor to remain equally capable.
