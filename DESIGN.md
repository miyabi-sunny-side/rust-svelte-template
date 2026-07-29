---
version: "alpha"
name: Rust + Svelte Field Kit
description: A practical field-note interface for a small production web service.
colors:
  primary: "#1c211c"
  surface: "#eee9df"
  signal: "#ec5b36"
  healthy: "#315c46"
typography:
  display:
    fontFamily: Iowan Old Style
    fontSize: 8.5rem
    fontWeight: 650
    lineHeight: 0.78
    letterSpacing: -0.06em
  body:
    fontFamily: Courier New
    fontSize: 1rem
    lineHeight: 1.8
rounded:
  square: 0px
spacing:
  grid: 24px
  panel: 1.4rem
components:
  service-panel:
    backgroundColor: "{colors.surface}"
    textColor: "{colors.primary}"
    rounded: "{rounded.square}"
    padding: "{spacing.panel}"
  retry-button:
    backgroundColor: "{colors.primary}"
    textColor: "{colors.surface}"
    rounded: "{rounded.square}"
    padding: 0.75rem
  pending-signal:
    backgroundColor: "{colors.signal}"
  healthy-signal:
    backgroundColor: "{colors.healthy}"
---

# Rust + Svelte Field Kit

## Overview

The starter screen feels like a practical field notebook: calm, precise, and ready to be marked up
by the next project. It proves that the frontend and backend are connected without pretending to be
a product dashboard. The service connection is the primary interactive state, and one deliberate
composition replaces a grid of ornamental demo cards.

## Colors

Warm paper and near-black ink form the dominant field-note palette. Safety orange identifies
attention states; moss indicates a healthy link. Color supplements visible status text and never
carries meaning alone.

## Typography

An editorial serif display stack contrasts with compact monospace labels and body copy. Both are
local system stacks: no runtime font, image, icon, or JavaScript dependency comes from a third-party
origin.

## Layout

Wide layouts use an asymmetric two-column composition with the service panel on the right. At 760
CSS pixels and below, sections become a single readable column. The layout works at 320 CSS pixels,
and body copy remains at least 0.78rem.

## Elevation & Depth

Fine rules, registration marks, graph-paper lines, and one hard offset shadow suggest a printed
technical folio. Gradients may draw the paper grid but must not imitate decorative glass surfaces.

## Shapes

Corners remain square. The interface earns hierarchy through spacing, rules, and type rather than
rounded containers.

## Components

On load, the service panel requests `/api/health` and announces the result through an `aria-live`
region. A failed request explains how to recover and exposes a keyboard-operable retry button. The
connected state uses status text, descriptive copy, and the healthy color token.

## Do's and Don'ts

- Do keep keyboard focus visible and interactive targets generous.
- Do remove the pending animation for `prefers-reduced-motion`.
- Do replace this identity when creating a real product from the template.
- Don't add project branding, navigation, authentication, data views, or shared component libraries
  to the template core.
- Don't introduce purple gradients, glass cards, or a generic dashboard grid.
