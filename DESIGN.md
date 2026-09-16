---
name: FORGE
version: "0.1"
status: proposed-visual-identity
reference: VoltAgent/awesome-design-md/design-md/ibm/DESIGN.md
reference_commit: 8147538b4226ae41e2487a9179e3bcc1f68e8554
default_theme: light
---

# FORGE visual identity

## Purpose and reference selection

FORGE is proposed infrastructure for traditional banks to create and operate financial products on Solana. Its first product is a controlled lending sandbox with vaults, approval workflows and reconciliation. The primary audience is bank engineering, operations, credit and innovation teams. Hyperliquid is a later markets expansion.

Selected reference: [IBM DESIGN.md in VoltAgent's collection](https://github.com/VoltAgent/awesome-design-md/blob/8147538b4226ae41e2487a9179e3bcc1f68e8554/design-md/ibm/DESIGN.md). This is the repository's independent analysis of IBM's marketing design, not an official FORGE or IBM specification. This document is a FORGE-specific adaptation.

| Candidate reviewed | Fit and selection rationale |
| --- | --- |
| IBM | Selected for enterprise hierarchy, precise alignment, flat surfaces, readable typography and deliberate geometry. |
| Stripe | Strong financial API reference; the extracted marketing language relies more heavily on atmospheric gradients. |
| Coinbase | Strong institutional finance reference; its extracted visual identity is more directly associated with a crypto exchange. |
| HashiCorp | Strong infrastructure reference; its dark-only marketing treatment and many product colors add unnecessary complexity for FORGE's first bank workflow. |

Carry forward IBM's discipline, 4px spacing rhythm, Plex typography and quiet surface hierarchy. FORGE introduces its own graphite/copper palette, 4px interactive corners, a complete dark theme, original logo concepts and banking workflow components. The source explicitly leaves product-level banking patterns and full dark mode outside its extracted coverage; those sections here are new proposals.

## Brand character

Institutional clarity. Engineered control.

The identity should feel precise, stable, legible and deliberate. Warm paper and copper suggest a considered engineering craft; graphite provides the visual weight. Large type carries the marketing voice. Tables, states and actions carry the product voice.

Positioning sentence: FORGE connects existing banking systems with programmable financial operations on Solana.

Short line: Banking infrastructure. Onchain.

Write FORGE in uppercase. Keep explanatory copy in sentence case. The prototype should be described as a sandbox until deployment and controls support stronger claims. Product illustrations use labeled sample data. Bank relationships, audits, certifications and performance figures appear only when independently established.

## Color system

Exact hexadecimal tokens in this document and forge-tokens.css are authoritative. Generated concept artwork illustrates the direction; it is not a color calibration reference.

### Light theme, default

| Token | Hex | Role |
| --- | --- | --- |
| Graphite | #17232D | Main text, primary monochrome mark, wordmark |
| Paper | #F4F2ED | Page canvas and brand presentation background |
| White | #FFFFFF | Working surfaces and white-on-copper button text |
| Copper | #A5482B | Brand accent, selected control, primary action |
| Copper hover | #873B24 | Hovered primary action |
| Copper wash | #F3E4DC | Selected navigation and quiet brand tint |
| Muted text | #58646C | Secondary readable text |
| Hairline | #D6D8D4 | Decorative separators and nonessential panel boundaries |
| Control border | #79838A | Inputs and controls requiring a visible boundary |
| Focus | #245BBA | Keyboard focus outline, kept distinct from brand copper |

### Dark theme

| Token | Hex | Role |
| --- | --- | --- |
| Night | #121B22 | Page canvas |
| Dark surface | #1B2730 | Panels, dialogs and table surface |
| Dark raised | #25333D | Hover and elevated controls |
| Paper | #F4F2ED | Main text and reverse wordmark |
| Dark muted text | #AAB6BC | Secondary text |
| Pale copper | #E6A080 | Brand detail and primary button surface |
| Pale copper hover | #F1B798 | Hovered action |
| Dark copper wash | #3D2B27 | Selected navigation surface |
| Dark hairline | #38454D | Decorative panel borders |
| Dark control border | #7C8A93 | Input and control boundaries |
| Dark focus | #8EB9FF | Keyboard focus |

### Semantic states

| Meaning | Light text / background | Dark text / background | Required label |
| --- | --- | --- | --- |
| Successful settlement | #216449 / #E8F2EB | #9CDEBB / #17382A | Finalized |
| Approval needed | #765400 / #FFF0CC | #F3CE78 / #3B3118 | 1 of 2 approvals |
| Failure | #A8263C / #FCECEF | #FFABB7 / #412630 | Failed |
| Informational | #245BBA / #E9F0FC | #ACC9FF / #1E304C | Submitted or Confirmed |

Approval and settlement are different state systems. A fully approved loan can still have an unsubmitted or failed disbursement. The UI must communicate both independently. Brand copper never means financially successful or profitable.

Measured contrast examples: graphite/paper 14.28:1; muted/paper 5.43:1; copper/paper 5.27:1; white/copper 5.90:1; paper/night 15.57:1; pale copper/night 8.05:1. These checks cover the named pairs, not an audit of all future interfaces. Check semantic states, hover, focus and control boundaries when implementing components. Decorative hairlines must not be the only sign of a required control boundary.

## Typography

Use IBM Plex Sans for interface and marketing text. Use IBM Plex Mono for identifiers, code and small technical metadata. Use tabular figures for financial values, with aligned decimals and explicit asset labels. The [IBM Plex project](https://github.com/IBM/plex) provides these families under the Open Font License.

| Role | Desktop / mobile | Weight | Line height |
| --- | --- | --- | --- |
| Marketing display | 72 / 40px | 300 | 1.08 |
| Section headline | 40 / 30px | 400 | 1.15 |
| Product page title | 28 / 24px | 500 | 1.25 |
| Financial figure | 32 / 26px | 400, tabular | 1.2 |
| Body | 16px | 400 | 1.5 |
| Table, controls | 14px | 400–500 | 1.4 |
| Metadata | 12px | 400 | 1.4 |
| Code / IDs | 12–14px | 400, Plex Mono | 1.5 |

Use letter spacing -0.025em on display type, normal tracking for product text and at most 0.08em on small uppercase brand labels. Critical approval details use at least 14px. Product amounts should never use light gray text or weight 300. Wordmark artwork is custom lettering; it is not replaced by a typeset heading when a finalized mark is available.

Self-host licensed font files in the application. The offline preview specifies Plex with system fallbacks and does not bundle font binaries. Its system-rendered fallback is illustrative when Plex is not installed.

## Logo directions

The generated comparison sheet contains three candidate concepts, not a finalized vector master. Recommendation: refine 01 Monolith.

| Direction | Construction | Intended character |
| --- | --- | --- |
| 01 Monolith | Compact continuous F silhouette, square inner counter and one chamfered top corner | Strong institutional core; readable app and SDK identity |
| 02 Keystone | Architectural gate with a structural opening and an F in the negative space | Controlled access and two approvals converging |
| 03 Cutword | Custom FORGE wordmark with deliberate diagonal cuts and a copper square | Typography-first identity for developer documentation and institutional materials |

For the selected direction, keep at least half a symbol height clear on every side. Horizontal lockup uses one symbol, one gap of roughly half the symbol width and FORGE. Prototype minimum symbol size is 24px; simplify only after testing at 16px and 20px. Use graphite on paper, paper on night, or copper on paper. Pale copper works on night. Use solid flat artwork for UI and document headers.

Before using a concept as the product asset, draw an editable vector master with consistent corners, optical spacing and a dedicated small-size variant. Test black-only, reversed, 16px and 24px versions. The current generated sheets are exploration references, not SVG or certified production assets. The most recent opaque comparison sheet supersedes the earlier transparent presentation drafts.

## Geometry, layout and depth

Use a 4px base spacing scale: 4, 8, 12, 16, 24, 32, 48, 64, 96. Main controls have 4px corners, status labels 2px and marketing image frames square corners. Use 1px separators and background changes to group content. Reserve shadows for overlays where they help indicate stacking.

Marketing content has a 1280px maximum width and a 12-column grid. Use generous heading space and tighter supporting details. The application uses a 224px navigation rail and a flexible content area. Page padding is 32px desktop, 24px tablet and 16px mobile.

Banking tables use 48–56px rows, right-aligned values, left-aligned names, clear asset units and persistent column labels. Expose copy and full-value views for abbreviated addresses. Technical identifiers may use monospace; prose remains sans.

## Components

| Component | Visual and behavioral rule |
| --- | --- |
| Primary button | Copper/white in light; pale copper/night in dark; 44px minimum height and 4px radius. One primary action per local task. |
| Secondary button | Transparent or working surface, 1px control border, main text. |
| Focus | 2px focus token outline with 3px offset; keyboard-visible on every interactive element. |
| Input | Persistent label, 44px minimum height, visible control border and distinct helper/error text. |
| Navigation | Selected item has a copper edge and tinted surface; text weight supports the state. |
| Summary metrics | Quiet labels above large tabular numbers; cash, receivables and pending operations remain separate. |
| Approval panel | Shows amount, asset, borrower, destination, term and current approvers before requesting a signature. |
| Transaction state | Submitted, Confirmed, Finalized and Failed are explicit; a signing event is not displayed as settlement. |
| Tables | Neutral rows, clear header, tabular amounts, text-plus-icon state badges and visible row action. |
| Empty state | Explains the missing prerequisite and offers the next useful action. |
| Error | Gives the failure reason and a specific recovery action while retaining entered details. |

Brand language should feel calm even when transactions are pending. Favor "Awaiting second approval", "Transaction submitted" and "Repayment finalized" over vague success messages.

## Motion and responsive behavior

Use 120–180ms opacity and color transitions for interaction feedback. Reduce or remove motion when the user requests reduced motion. Avoid animated balance counters and background motion inside approval workflows; display amounts immediately.

At 1024px collapse the navigation rail. At 768px stack metric groups and place metadata below primary content. On narrow displays put wide tables inside their own labeled horizontal-scroll region. Preserve the hierarchy of critical financial data. Interactive targets stay at least 44px; mobile primary controls can grow to 48px.

## Visual imagery

Use material and architectural references sparingly in marketing: crisp metal profiles, aligned channels, fitted parts and diagrammatic structure. The core identity itself stays flat. Functional diagrams must be drawn from actual system behavior and remain legible. Sample dashboards should be labeled as samples. Avoid speculative yield imagery, coins, mascots and atmospheric effects in operational screens.

## Implementation guide for a coding agent

Read this document before building FORGE screens. Use forge-tokens.css as the token source. Default to the light theme and support an explicit dark-theme preference. Implement the first bank-controlled lending sandbox with legible amounts, separate approval and settlement states, 4px geometry and persistent labels. Use the confirmed MVP specification for behavior and this document for presentation. Keep all displayed balances traceable to application state. Treat the reference preview as a visual specimen, not a working financial application. Preserve the traditional-bank and Solana-first positioning.

## Provenance

Reference repository: VoltAgent/awesome-design-md, MIT License, copyright 2026 VoltAgent. The accompanying reference license is included in the brand kit. IBM, Stripe, Coinbase and HashiCorp remain references, not partners or endorsers. FORGE's palette, concept briefs, financial workflow component rules and complete dark adaptation were authored for this project.

Source comparisons: [IBM](https://github.com/VoltAgent/awesome-design-md/blob/8147538b4226ae41e2487a9179e3bcc1f68e8554/design-md/ibm/DESIGN.md), [Stripe](https://github.com/VoltAgent/awesome-design-md/blob/8147538b4226ae41e2487a9179e3bcc1f68e8554/design-md/stripe/DESIGN.md), [Coinbase](https://github.com/VoltAgent/awesome-design-md/blob/8147538b4226ae41e2487a9179e3bcc1f68e8554/design-md/coinbase/DESIGN.md), [HashiCorp](https://github.com/VoltAgent/awesome-design-md/blob/8147538b4226ae41e2487a9179e3bcc1f68e8554/design-md/hashicorp/DESIGN.md).
