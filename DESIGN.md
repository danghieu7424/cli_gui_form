# Minimal — TUI Design System

> Clean, focused, zero noise. Inspired by Vercel and Linear's terminal aesthetics.

## 1. Theme Overview

- **Mood**: Minimal, professional, calm
- **Density**: Balanced — generous whitespace without wasting terminal real estate
- **Target**: Developer tools, CLI utilities, AI agent interfaces
- **Terminal**: 256-color minimum, TrueColor recommended

## 2. Color Palette

### Semantic Roles

| Role | Hex | ANSI 256 | ANSI 16 | Usage |
|------|-----|----------|---------|-------|
| Background | `#0a0a0a` | `232` | `black` | Main background |
| Foreground | `#ededed` | `255` | `white` | Default text |
| Primary | `#ffffff` | `15` | `bright white` | Key actions, focus states |
| Secondary | `#888888` | `245` | `bright black` | Supporting text |
| Accent | `#0070f3` | `33` | `blue` | Links, highlights |
| Success | `#00c853` | `41` | `green` | Positive status |
| Warning | `#f5a623` | `214` | `yellow` | Caution status |
| Error | `#ee0000` | `196` | `red` | Error status |
| Muted | `#555555` | `240` | `bright black` | Disabled, hints |
| Surface | `#1a1a1a` | `234` | `black` | Panels, cards |

### Neutral Scale

| Step | Hex | Usage |
|------|-----|-------|
| 50 | `#1a1a1a` | Subtle backgrounds, surface |
| 100 | `#2a2a2a` | Borders, dividers |
| 200 | `#444444` | Disabled text |
| 300 | `#666666` | Placeholder text |
| 400 | `#888888` | Secondary text |
| 500 | `#ededed` | Body text |

## 3. Typography & ASCII Art

- **Header font**: `small` (figlet) — compact, not flashy
- **Body text**: plain terminal font
- **Emphasis**: `bold` only — avoid italic in terminals (poor support)
- **Code/values**: `dim` background or Accent color

### Text Hierarchy

| Level | Style | Example Usage |
|-------|-------|---------------|
| H1 | figlet `small` + Primary | App title |
| H2 | BOLD + Foreground | Section headers |
| H3 | BOLD + Secondary | Subsection headers |
| Body | Foreground | Content text |
| Caption | Muted + dim | Help text, timestamps |
| Label | BOLD + Secondary | Form labels |

## 4. Borders & Box Drawing

### Primary Border

```
┌──────────────┐
│   content    │
└──────────────┘
```

Single-line box drawing. Clean and lightweight.

### Parts Table

| Part | Character | Usage |
|------|-----------|-------|
| top_left | `┌` | Panel corners |
| top_right | `┐` | |
| bottom_left | `└` | |
| bottom_right | `┘` | |
| horizontal | `─` | Horizontal lines |
| vertical | `│` | Vertical lines |
| cross | `┼` | Table intersections |
| tee_down | `┬` | Table header separator |
| tee_up | `┴` | Table footer |
| tee_right | `├` | Left junction |
| tee_left | `┤` | Right junction |

### Secondary Border

For nested or less important containers, use a dimmed border with the same characters but `Muted` color.

### Dividers

- Horizontal: `────────────────────`
- Section break: `── · ──`

## 5. Components

### Buttons / Actions

```
 ▸ Submit    Cancel    Help
   ↑          ↑        ↑
 focused   unfocused  muted
```

- Focused: `reverse` (white bg, black fg) with `▸` prefix
- Unfocused: plain Foreground text
- Disabled: Muted + dim

### Input Fields

```
  Email: │user@example.com        │
         └────────────────────────┘
```

- Active: `Accent` color border, cursor visible
- Inactive: `Muted` color border
- Error: `Error` color border, error message below in Error color

### Tables

```
  Name              Status    Time
  ─────────────────────────────────
  deploy-api        ✓ Ready   2m ago
  deploy-web        ▶ Build   just now
  deploy-docs       ✗ Error   5m ago
```

No outer border. Header separated by `─`. Dim separator line.

### Lists / Menus

```
    api/routes.ts
  ▸ api/handler.ts
    lib/utils.ts
    config.json
```

- Selected: `▸` prefix + BOLD + Primary
- Normal: 4-space indent + Foreground
- Disabled: 4-space indent + Muted + dim

### Panels / Cards

```
┌─ Deploy Status ──────────────┐
│                               │
│  Production    ✓ Ready        │
│  Preview       ▶ Building     │
│  Staging       ✓ Ready        │
│                               │
└───────────────────────────────┘
```

Title embedded in top border. 1-space padding inside.

### Tabs

```
  Overview │ Logs │ Settings
  ─────────┘      └─────────
```

Active tab: BOLD + Primary. Inactive: Secondary. Connected by box drawing.

### Status Bar

```
 main ─ 3 files changed ─ ✓ All checks passed         127.0.0.1:3000
```

Single line at bottom. Left-aligned info, right-aligned status. Separated by ` ─ `.

## 6. Layout & Spacing

- **Min terminal width**: `80`
- **Ideal terminal width**: `120`
- **Padding inside panels**: 1 line top/bottom, 1 char left/right
- **Gap between components**: 1 empty line
- **Indent level**: 2 spaces

### Alignment Principles

- Left-align all content
- Right-align timestamps and numeric values in tables
- Center only splash screen / logo

## 7. Icons & Indicators

| Purpose | Icon | Unicode | Fallback (ASCII) |
|---------|------|---------|-------------------|
| Success | `✔` | `U+2714` | `+` |
| Error / Fail | `✗` | `U+2716` | `x` |
| Warning | `⚠` | `U+26A0` | `!` |
| Info | `ℹ` | `U+2139` | `i` |
| Running / Exec | `▶` | `U+25B6` | `>` |
| Build / Work | `⚙` | `U+2699` | `*` |
| Stop | `■` | `U+25A0` | `[#]` |
| Pause | `⏸` | `U+23F8` | `||` |
| Thinking Pulse | `·,•,●,•,·, ` | `U+00B7...` | `o` |
| Braille Spinner | `⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏` | `U+280B...` | `\|/-` |
| Arrow Right | `→` | `U+2192` | `->` |
| Arrow Up | `▲` | `U+25B2` | `^` |
| Branch | `⤷` | `U+21B3` | `\_` |
| Bullet | `▪` | `U+25AA` | `-` |
| Selected | `▹` / `▸` | `U+25B8` | `>` |
| Checkbox on | `☑` | `U+2611` | `[x]` |
| Checkbox off | `☐` | `U+2610` | `[ ]` |
| Radio on | `●` | `U+25CF` | `(•)` |
| Radio off | `○` | `U+25CB` | `( )` |
| Sparkle Filled | `✦` | `U+2726` | `*` |
| Sparkle Empty | `✧` | `U+2727` | `*` |
| Star Outline | `⚝` | `U+269D` | `*` |
| Diamond | `◇` | `U+25C7` | `<>` |
| Snowflake | `❅` | `U+2745` | `*` |

Keep icons to single-width characters (or 2-cell standard with trailing space). No emoji.

## 8. Animation & Motion

### Spinners

- Default: `⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏` at 80ms — braille dots, smooth rotation
- Thinking/AI: `·` → `•` → `●` → `•` → `·` → ` ` at 150ms

### Transitions

- No animated transitions. State changes are instant.
- Loading states use spinners only, no progress simulation.

### Progress

```
  ━━━━━━━━━━────────── 50%
```

- Filled: `━`, Empty: `─`,( Caps: `▕` `▏` )
- Show percentage, no ETA
- Accent color for filled portion

## 9. Agent Prompt Guide

### Quick Reference

```
Background: #0a0a0a  (ANSI 232)
Foreground: #ededed  (ANSI 255)
Accent:     #0070f3  (ANSI 33)
Border:     ┌─┐│└─┘  (single line)
Style:      minimal, monochrome with blue accent, generous spacing
```

### Example Prompts

- "Build a status dashboard: single-line borders, white text on near-black bg, blue accent for active items, ✓/✗ status icons, no emoji"
- "Create a file picker: ▸ selector, dim unselected items, bold selected item, single-line box panel, embedded title in top border"
- "Design a form: bottom-bordered inputs, blue highlight on focus, red on error, reverse-video submit button, 2-space indent"

## Do's and Don'ts

### Do

- Use the neutral scale for text hierarchy — avoid coloring body text
- Leave generous whitespace — let the terminal breathe
- Use single-line box drawing for all borders
- Keep status indicators to 1 character width
- Test at 80 columns minimum

### Don't

- Don't use emoji — inconsistent widths break alignment
- Don't use more than 1 accent color per view
- Don't use double-line or heavy borders — they fight the minimal aesthetic
- Don't use background colors for emphasis — use bold or reverse sparingly
- Don't animate anything except spinners and progress bars
