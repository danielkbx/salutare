"use strict";
/**
 * Salutare landing page client
 * ============================
 *
 * Responsibilities
 * - Fetch the greeting of the day from the Salutare API.
 * - Render greeting + language name into the DOM.
 * - Pick German/English language name depending on browser language.
 * - Apply a deterministic daily theme (accent + background) based on the greeting `id`.
 * - Provide an elegant background transition using a CSS-variable-driven overlay.
 *
 * Key design goals
 * - No external JS libraries.
 * - Keep things deterministic and stable.
 * - Respect user accessibility preferences (prefers-reduced-motion).
 * - Work in production (same origin) and locally (optional API base override).
 */
/**
 * Determine whether the browser UI language is German.
 * - de
 * - de-DE, de-AT, etc.
 *
 * In all other cases we use English.
 */
function isGermanBrowser() {
    const lang = (navigator.language || "").toLowerCase();
    return lang === "de" || lang.startsWith("de-");
}
/**
 * Minimal DOM helper.
 */
function $(id) {
    const el = document.getElementById(id);
    if (!el)
        throw new Error(`Missing element #${id}`);
    return el;
}
/**
 * Optional API base override (useful for local development).
 *
 * Reads from:
 *   <meta name="salutare-api-base" content="http://localhost:8080">
 *
 * - If missing or empty, returns "" => use relative URLs (production default).
 * - Strips trailing slashes for clean concatenation.
 */
function getApiBase() {
    var _a, _b;
    const meta = document.querySelector('meta[name="salutare-api-base"]');
    const value = (_b = (_a = meta === null || meta === void 0 ? void 0 : meta.content) === null || _a === void 0 ? void 0 : _a.trim()) !== null && _b !== void 0 ? _b : "";
    return value.replace(/\/+$/, "");
}
/**
 * Build the greeting endpoint URL.
 *
 * - Prod: "" + "/api/v1/greeting" => "/api/v1/greeting"
 * - Local override: "http://localhost:8080" + "/api/v1/greeting"
 */
function buildGreetingUrl() {
    const base = getApiBase();
    return `${base}/api/v1/greeting`;
}
/**
 * Parse JSON safely (for best-effort error handling).
 * If parsing fails, return null.
 */
async function parseJsonSafely(res) {
    try {
        return (await res.json());
    }
    catch (_a) {
        return null;
    }
}
function setText(id, text) {
    $(id).textContent = text;
}
function showError(message) {
    const errorEl = $("error");
    errorEl.hidden = false;
    errorEl.textContent = message;
}
function hideError() {
    const errorEl = $("error");
    errorEl.hidden = true;
    errorEl.textContent = "";
}
/**
 * Deterministic theming
 * --------------------
 *
 * The API returns the selected greeting row `id`.
 * We deliberately use `id` as the theme key because:
 * - It already reflects the `offset` parameter (different offsets => typically different ids).
 * - It stays stable for the selected greeting.
 *
 * Theme consists of:
 * - accent color (quotes + language label)
 * - background gradients
 *
 * Both are selected from curated arrays using the same palette index.
 */
const ACCENT_PALETTE = [
    "rgba(225, 145, 85, 0.78)", // 0: golden dawn
    "rgba(215, 110, 125, 0.78)", // 1: rosy sunrise
    "rgba(210, 160, 70, 0.78)", // 2: honey
    "rgba(185, 115, 95, 0.78)", // 3: terracotta
    "rgba(120, 165, 115, 0.78)", // 4: fresh green
    "rgba(110, 135, 185, 0.78)", // 5: cool sky
];
/**
 * Background variants aligned by index with ACCENT_PALETTE.
 *
 * Each entry is a full CSS `background` value.
 * We intentionally make them clearly different while staying in a “morning” spectrum.
 */
const BACKGROUND_VARIANTS = [
    // 0 — Golden Dawn
    `
  radial-gradient(1000px 600px at 20% 18%, rgba(255, 200, 120, 0.55), transparent 60%),
  radial-gradient(900px 520px at 80% 20%, rgba(255, 160, 90, 0.45), transparent 60%),
  radial-gradient(1100px 650px at 50% 85%, rgba(170, 230, 200, 0.30), transparent 65%),
  linear-gradient(180deg, rgba(255,255,255,0.98), rgba(255,245,235,0.82))
  `,
    // 1 — Rosy Sunrise
    `
  radial-gradient(1000px 600px at 20% 18%, rgba(255, 165, 175, 0.55), transparent 60%),
  radial-gradient(900px 520px at 80% 20%, rgba(255, 120, 145, 0.48), transparent 60%),
  radial-gradient(1100px 650px at 50% 85%, rgba(190, 220, 210, 0.28), transparent 65%),
  linear-gradient(180deg, rgba(255,255,255,0.98), rgba(255,240,245,0.82))
  `,
    // 2 — Honey Morning
    `
  radial-gradient(1000px 600px at 20% 18%, rgba(255, 215, 120, 0.60), transparent 60%),
  radial-gradient(900px 520px at 80% 20%, rgba(255, 175, 80, 0.50), transparent 60%),
  radial-gradient(1100px 650px at 50% 85%, rgba(185, 235, 190, 0.32), transparent 65%),
  linear-gradient(180deg, rgba(255,255,255,0.98), rgba(255,250,230,0.82))
  `,
    // 3 — Terracotta Sky
    `
  radial-gradient(1000px 600px at 20% 18%, rgba(225, 155, 125, 0.55), transparent 60%),
  radial-gradient(900px 520px at 80% 20%, rgba(205, 120, 95, 0.48), transparent 60%),
  radial-gradient(1100px 650px at 50% 85%, rgba(170, 215, 200, 0.30), transparent 65%),
  linear-gradient(180deg, rgba(255,255,255,0.98), rgba(250,240,235,0.82))
  `,
    // 4 — Fresh Morning
    `
  radial-gradient(1000px 600px at 20% 18%, rgba(170, 225, 170, 0.55), transparent 60%),
  radial-gradient(900px 520px at 80% 20%, rgba(130, 200, 160, 0.48), transparent 60%),
  radial-gradient(1100px 650px at 50% 85%, rgba(200, 235, 210, 0.38), transparent 65%),
  linear-gradient(180deg, rgba(255,255,255,0.98), rgba(240,255,245,0.82))
  `,
    // 5 — Cool Sunrise
    `
  radial-gradient(1000px 600px at 20% 18%, rgba(160, 185, 255, 0.55), transparent 60%),
  radial-gradient(900px 520px at 80% 20%, rgba(130, 160, 235, 0.48), transparent 60%),
  radial-gradient(1100px 650px at 50% 85%, rgba(200, 220, 245, 0.40), transparent 65%),
  linear-gradient(180deg, rgba(255,255,255,0.98), rgba(235,245,255,0.82))
  `,
];
/**
 * Map greeting id -> palette index.
 * This provides a stable theme per selected greeting and implicitly includes `offset`.
 */
function paletteIndexForGreetingId(id) {
    const n = ACCENT_PALETTE.length;
    const safe = Math.abs(Math.trunc(id));
    return safe % n;
}
/**
 * Apply accent color by setting the CSS variable.
 */
function applyAccentColor(accent) {
    document.documentElement.style.setProperty("--accent-muted", accent);
}
/**
 * Apply background using a mini-transition (edle Übergänge).
 *
 * Why the overlay technique?
 * - CSS cannot reliably interpolate between complex gradient values.
 * - Instead, we layer two backgrounds:
 *   - ::before shows the current background
 *   - ::after fades in the next background
 * - After the fade completes, we commit the new background as the base layer.
 *
 * Reduced motion:
 * - If the user prefers reduced motion, we apply immediately without animation.
 */
function applyBackground(backgroundCss) {
    const root = document.documentElement;
    const reduceMotion = typeof window !== "undefined" &&
        window.matchMedia &&
        window.matchMedia("(prefers-reduced-motion: reduce)").matches;
    if (reduceMotion) {
        root.style.setProperty("--bg-gradients", backgroundCss);
        root.style.setProperty("--bg-next", backgroundCss);
        root.style.setProperty("--bg-transition", "0");
        return;
    }
    // Stage new background in the overlay layer.
    root.style.setProperty("--bg-next", backgroundCss);
    // Trigger fade-in of the overlay.
    root.style.setProperty("--bg-transition", "1");
    // After fade, commit new background and reset overlay.
    // CSS transition duration is 900ms; keep a small buffer.
    window.setTimeout(() => {
        root.style.setProperty("--bg-gradients", backgroundCss);
        root.style.setProperty("--bg-transition", "0");
        root.style.setProperty("--bg-next", backgroundCss);
    }, 950);
}
/**
 * Fetch and render the greeting of the day.
 *
 * The API is public; we do not send cookies (credentials: "omit").
 * We request JSON and do best-effort parsing of error bodies.
 */
async function loadGreeting() {
    setText("greeting", "Salutare");
    setText("language", "Loading…");
    hideError();
    const url = buildGreetingUrl();
    const res = await fetch(url, {
        method: "GET",
        headers: { Accept: "application/json" },
        credentials: "omit",
        cache: "no-store",
    });
    if (!res.ok) {
        const apiErr = await parseJsonSafely(res);
        const fallbackText = await res.text().catch(() => "");
        const apiMessage = apiErr === null || apiErr === void 0 ? void 0 : apiErr.error;
        const message = apiMessage !== null && apiMessage !== void 0 ? apiMessage : (fallbackText && fallbackText.trim().length > 0
            ? fallbackText
            : `HTTP ${res.status} ${res.statusText}`);
        throw new Error(message);
    }
    const data = (await res.json());
    // Apply deterministic theme based on the selected greeting id.
    const idx = paletteIndexForGreetingId(data.id);
    applyAccentColor(ACCENT_PALETTE[idx]);
    applyBackground(BACKGROUND_VARIANTS[idx]);
    // Render content
    setText("greeting", data.greeting);
    setText("language", isGermanBrowser() ? data.language.de : data.language.en);
    hideError();
}
/**
 * App entry point.
 * All errors are caught and shown as a simple message on the page.
 */
async function main() {
    try {
        await loadGreeting();
    }
    catch (err) {
        const message = err instanceof Error
            ? `Could not load greeting. ${err.message}`
            : "Could not load greeting.";
        setText("greeting", "Salutare");
        setText("language", "");
        showError(message);
    }
}
document.addEventListener("DOMContentLoaded", () => {
    void main();
});
