use eframe::egui::{Color32, CornerRadius, Shadow, Stroke, Visuals};

// === Accent (Indigo) ===
// Tailwind indigo-500 / indigo-600, tuned per theme for contrast.
pub const ACCENT_DARK: Color32 = Color32::from_rgb(0x63, 0x66, 0xF1); // #6366F1
pub const ACCENT_DARK_HOVER: Color32 = Color32::from_rgb(0x81, 0x8C, 0xF8); // #818CF8
pub const ACCENT_LIGHT: Color32 = Color32::from_rgb(0x4F, 0x46, 0xE5); // #4F46E5

/// Text color that reads well on top of an accent-filled surface (both themes).
pub const ON_ACCENT: Color32 = Color32::from_rgb(0xFA, 0xFA, 0xFA);

/// A muted "success" green that fits the minimal palette.
pub const SUCCESS: Color32 = Color32::from_rgb(0x22, 0xC5, 0x5E); // #22C55E

/// Build the full egui visuals for the requested theme.
pub fn visuals(dark: bool) -> Visuals {
    if dark { dark_visuals() } else { light_visuals() }
}

fn apply_corner_radius(v: &mut Visuals, r: u8) {
    let cr = CornerRadius::same(r);
    v.widgets.noninteractive.corner_radius = cr;
    v.widgets.inactive.corner_radius = cr;
    v.widgets.hovered.corner_radius = cr;
    v.widgets.active.corner_radius = cr;
    v.widgets.open.corner_radius = cr;
}

fn dark_visuals() -> Visuals {
    let mut v = Visuals::dark();

    let accent = ACCENT_DARK;
    let accent_hover = ACCENT_DARK_HOVER;

    let bg = Color32::from_rgb(0x18, 0x18, 0x1B); // zinc-900
    let card = Color32::from_rgb(0x22, 0x22, 0x27);
    let surface = Color32::from_rgb(0x2A, 0x2A, 0x30);
    let surface_hover = Color32::from_rgb(0x33, 0x33, 0x3B);
    let extreme = Color32::from_rgb(0x0F, 0x0F, 0x11);
    let line = Color32::from_rgb(0x2E, 0x2E, 0x35);

    let text = Color32::from_rgb(0xD4, 0xD4, 0xD8); // zinc-300 (body)
    let text_strong = Color32::from_rgb(0xFA, 0xFA, 0xFA); // near-white (headings/kanji)
    let text_button = Color32::from_rgb(0xE4, 0xE4, 0xE7); // zinc-200

    v.dark_mode = true;
    v.panel_fill = bg;
    v.window_fill = bg;
    v.faint_bg_color = card;
    v.extreme_bg_color = extreme;
    v.window_stroke = Stroke::new(1.0, line);
    v.window_corner_radius = CornerRadius::same(12);
    v.hyperlink_color = accent_hover;

    v.selection.bg_fill = accent.gamma_multiply(0.40);
    v.selection.stroke = Stroke::new(1.0, accent_hover);

    // noninteractive — default label text + separators / borders
    v.widgets.noninteractive.bg_fill = bg;
    v.widgets.noninteractive.weak_bg_fill = bg;
    v.widgets.noninteractive.bg_stroke = Stroke::new(1.0, line);
    v.widgets.noninteractive.fg_stroke = Stroke::new(1.0, text);

    // inactive — default buttons / pills
    v.widgets.inactive.bg_fill = surface;
    v.widgets.inactive.weak_bg_fill = surface;
    v.widgets.inactive.bg_stroke = Stroke::NONE;
    v.widgets.inactive.fg_stroke = Stroke::new(1.0, text_button);

    // hovered
    v.widgets.hovered.bg_fill = surface_hover;
    v.widgets.hovered.weak_bg_fill = surface_hover;
    v.widgets.hovered.bg_stroke = Stroke::new(1.0, line);
    v.widgets.hovered.fg_stroke = Stroke::new(1.5, text_strong);

    // active — primary accent; fg_stroke also drives `strong_text_color()`
    v.widgets.active.bg_fill = accent;
    v.widgets.active.weak_bg_fill = accent;
    v.widgets.active.bg_stroke = Stroke::new(1.0, accent);
    v.widgets.active.fg_stroke = Stroke::new(1.5, text_strong);

    // open — combo boxes / menus
    v.widgets.open.bg_fill = surface;
    v.widgets.open.weak_bg_fill = surface;
    v.widgets.open.bg_stroke = Stroke::new(1.0, line);
    v.widgets.open.fg_stroke = Stroke::new(1.0, text_button);

    apply_corner_radius(&mut v, 8);

    v.window_shadow = Shadow {
        offset: [0, 8],
        blur: 24,
        spread: 0,
        color: Color32::from_black_alpha(120),
    };
    v.popup_shadow = Shadow {
        offset: [0, 4],
        blur: 16,
        spread: 0,
        color: Color32::from_black_alpha(110),
    };

    v
}

fn light_visuals() -> Visuals {
    let mut v = Visuals::light();

    let accent = ACCENT_LIGHT;

    let bg = Color32::from_rgb(0xFF, 0xFF, 0xFF);
    let card = Color32::from_rgb(0xF4, 0xF4, 0xF5); // zinc-100
    let surface = Color32::from_rgb(0xF0, 0xF0, 0xF2);
    let surface_hover = Color32::from_rgb(0xE7, 0xE7, 0xEB); // zinc-200-ish
    let extreme = Color32::from_rgb(0xFF, 0xFF, 0xFF);
    let line = Color32::from_rgb(0xE4, 0xE4, 0xE7); // zinc-200

    let text = Color32::from_rgb(0x3F, 0x3F, 0x46); // zinc-700 (body)
    let text_strong = Color32::from_rgb(0x18, 0x18, 0x1B); // zinc-900 (headings/kanji)
    let text_button = Color32::from_rgb(0x27, 0x27, 0x2A); // zinc-800

    v.dark_mode = false;
    v.panel_fill = bg;
    v.window_fill = bg;
    v.faint_bg_color = card;
    v.extreme_bg_color = extreme;
    v.window_stroke = Stroke::new(1.0, line);
    v.window_corner_radius = CornerRadius::same(12);
    v.hyperlink_color = accent;

    v.selection.bg_fill = accent.gamma_multiply(0.25);
    v.selection.stroke = Stroke::new(1.0, accent);

    v.widgets.noninteractive.bg_fill = bg;
    v.widgets.noninteractive.weak_bg_fill = bg;
    v.widgets.noninteractive.bg_stroke = Stroke::new(1.0, line);
    v.widgets.noninteractive.fg_stroke = Stroke::new(1.0, text);

    v.widgets.inactive.bg_fill = surface;
    v.widgets.inactive.weak_bg_fill = surface;
    v.widgets.inactive.bg_stroke = Stroke::NONE;
    v.widgets.inactive.fg_stroke = Stroke::new(1.0, text_button);

    v.widgets.hovered.bg_fill = surface_hover;
    v.widgets.hovered.weak_bg_fill = surface_hover;
    v.widgets.hovered.bg_stroke = Stroke::new(1.0, line);
    v.widgets.hovered.fg_stroke = Stroke::new(1.5, text_strong);

    v.widgets.active.bg_fill = accent;
    v.widgets.active.weak_bg_fill = accent;
    v.widgets.active.bg_stroke = Stroke::new(1.0, accent);
    v.widgets.active.fg_stroke = Stroke::new(1.5, text_strong);

    v.widgets.open.bg_fill = surface;
    v.widgets.open.weak_bg_fill = surface;
    v.widgets.open.bg_stroke = Stroke::new(1.0, line);
    v.widgets.open.fg_stroke = Stroke::new(1.0, text_button);

    apply_corner_radius(&mut v, 8);

    v.window_shadow = Shadow {
        offset: [0, 8],
        blur: 24,
        spread: 0,
        color: Color32::from_black_alpha(35),
    };
    v.popup_shadow = Shadow {
        offset: [0, 4],
        blur: 16,
        spread: 0,
        color: Color32::from_black_alpha(25),
    };

    v
}
