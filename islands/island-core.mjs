// island-core.mjs — the pure half of the <docs-island> shim (ADR-0004 §2.5).
//
// No DOM, no globals: everything here is testable headlessly, and
// island.mjs is the thin wiring around it. The whole point is that a Rust
// docs core (docs-core) can emit static HTML with islands while the
// estate's existing SolidJS bundles do the hydrating — the seam neither
// Leptos nor Dioxus can cross.

/** Hydration hints, matching the Rust `Client` enum. */
export const HYDRATION = Object.freeze({
  LOAD: "load",
  VISIBLE: "visible",
  IDLE: "idle",
});

/**
 * Props for an island: the JSON in its <script data-island-props>.
 * Malformed props are a warning, not a crash — a broken island must not
 * take the page down.
 */
export function parseProps(el) {
  const script = el.querySelector('script[data-island-props]');
  if (!script) return {};
  try {
    const parsed = JSON.parse(script.textContent ?? "{}");
    // Plain objects only — the Rust emitter validates `{…}` too, and the two
    // halves must agree or an array slips through on one side.
    return parsed && typeof parsed === "object" && !Array.isArray(parsed)
      ? parsed
      : {};
  } catch (error) {
    console.warn("[docs-island] malformed props, continuing with {}", error);
    return {};
  }
}

/** The hydration hint; anything unknown falls back to `visible`. */
export function hydrationFor(el) {
  const client = el.getAttribute("client");
  if (
    client === HYDRATION.LOAD ||
    client === HYDRATION.IDLE ||
    client === HYDRATION.VISIBLE
  ) {
    return client;
  }
  return HYDRATION.VISIBLE;
}

/** The component URL, or null when the element forgot it. */
export function componentUrlFor(el) {
  const url = el.getAttribute("component-url");
  return url && url.length > 0 ? url : null;
}

/** Should we hydrate given the hint and the environment's answers? */
export function decideHydration({ client, visible, idle }) {
  switch (client) {
    case HYDRATION.LOAD:
      return true;
    case HYDRATION.IDLE:
      return Boolean(idle);
    case HYDRATION.VISIBLE:
      return Boolean(visible);
    default:
      return false;
  }
}

/**
 * A loader that imports each component URL at most once, ever. Two islands
 * using the same component share one network request and one module
 * instance — the behaviour Astro's island runtime guarantees and the thing
 * a hand-rolled `import()` in every element gets wrong.
 */
export function makeLoader(importer = (url) => import(/* @vite-ignore */ url)) {
  const cache = new Map();
  return function load(url) {
    if (!cache.has(url)) {
      cache.set(
        url,
        Promise.resolve(importer(url)).catch((error) => {
          // A failed load must not poison the cache: a retry should work.
          cache.delete(url);
          throw error;
        }),
      );
    }
    return cache.get(url);
  };
}
