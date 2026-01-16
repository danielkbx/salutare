type GreetingResponse = {
    greeting: string;
    language: { de: string; en: string };
};

function isGermanBrowser(): boolean {
    const lang = (navigator.language || "").toLowerCase();
    return lang === "de" || lang.startsWith("de-");
}

function $(id: string): HTMLElement {
    const el = document.getElementById(id);
    if (!el) throw new Error(`Missing element #${id}`);
    return el;
}

function getApiBase(): string {
    const meta = document.querySelector<HTMLMetaElement>('meta[name="salutare-api-base"]');
    const value = meta?.content?.trim() ?? "";
    // If empty, we use relative URLs (production default).
    return value.replace(/\/+$/, ""); // strip trailing slash
}

async function loadGreeting(): Promise<void> {
    const greetingEl = $("greeting");
    const languageEl = $("language");
    const errorEl = $("error");

    try {
        const base = getApiBase();
        const url = `${base}/api/v1/greeting`;
        const res = await fetch(url, {
            method: "GET",
            headers: {"Accept": "application/json"},
            credentials: "omit"
        });

        if (!res.ok) {
            const text = await res.text().catch(() => "");
            throw new Error(`HTTP ${res.status} ${res.statusText}${text ? ` – ${text}` : ""}`);
        }

        const data = (await res.json()) as GreetingResponse;

        greetingEl.textContent = data.greeting;
        languageEl.textContent = isGermanBrowser() ? data.language.de : data.language.en;

        errorEl.hidden = true;
        errorEl.textContent = "";
    } catch (err) {
        greetingEl.textContent = "Salutare";
        languageEl.textContent = "";

        errorEl.hidden = false;
        errorEl.textContent =
            err instanceof Error ? `Could not load greeting. ${err.message}` : "Could not load greeting.";
    }
}

document.addEventListener("DOMContentLoaded", () => {
    void loadGreeting();
});