# FORGE visual identity kit

Recommended reference: IBM DESIGN.md from VoltAgent/awesome-design-md. Recommended logo direction: 01 Monolith.

Files:
- DESIGN.md: FORGE-specific design rules. Put this at the root of the application repository for Pi or another coding agent.
- forge-tokens.css: light and dark CSS tokens plus a small base control style.
- FORGE-IDENTITY.html: self-contained visual guide with the generated logo comparison, theme toggle and color swatches. Open locally in a browser.
- FORGE-Logo-Concepts.png: full-resolution comparison sheet with all three generated logo directions.
- logo-prompts.json: original concept prompts used with the built-in imagegen tool and a note on the final opaque comparison.
- REFERENCE-LICENSE.txt: the license of the reference collection.

The HTML embeds the final comparison image so the visual guide remains self-contained. Font binaries are not included. Install or self-host IBM Plex Sans and IBM Plex Mono to match the specification exactly; the HTML falls back gracefully to system fonts.

Logo sheets are generated raster concepts. Refining the chosen mark into editable SVG artwork is a subsequent production step. CSS/Markdown tokens are authoritative for exact colors and measurements. The interface in the guide is a style specimen with simulated values, not an implemented banking application.

Validation: the image, named contrast pairs, HTML structure and archive integrity were checked. Desktop/mobile browser rendering was not verified because a browser executable was unavailable in the build environment.

Reference inspected at commit 8147538b4226ae41e2487a9179e3bcc1f68e8554:
https://github.com/VoltAgent/awesome-design-md/blob/8147538b4226ae41e2487a9179e3bcc1f68e8554/design-md/ibm/DESIGN.md
