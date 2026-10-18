// island.mjs — the <docs-island> custom element (ADR-0004 §2.5).
//
// Emitted by docs-core as:
//   <docs-island component-url="/islands/nav-search.js" client="visible">
//     <script type="application/json" data-island-props>{"count":3}</script>
//   </docs-island>
//
// The module's default export receives { props } and returns an HTML string
// (or a Node); the shim swaps it in and fires `docs:hydrated`. Import this
// file once per page — it registers the element.

import {
  decideHydration,
  hydrationFor,
  componentUrlFor,
  parseProps,
  makeLoader,
} from "./island-core.mjs";

const load = makeLoader();

async function hydrate(el) {
  const url = componentUrlFor(el);
  if (!url || el.dataset.hydrated === "true") return;
  el.dataset.hydrating = "true";
  try {
    const module = await load(url);
    const render = module.default ?? module.render;
    if (typeof render !== "function") {
      throw new Error(`island module ${url} has no default export`);
    }
    const result = await render({ props: parseProps(el), element: el });
    if (typeof result === "string") {
      el.innerHTML = result;
    } else if (result instanceof Node) {
      el.replaceChildren(result);
    }
    el.dataset.hydrated = "true";
    el.dispatchEvent(new CustomEvent("docs:hydrated", { bubbles: true }));
  } catch (error) {
    console.error(`[docs-island] hydration failed for ${url}`, error);
    el.dataset.hydrationError = "true";
  } finally {
    delete el.dataset.hydrating;
  }
}

class DocsIsland extends HTMLElement {
  connectedCallback() {
    const client = hydrationFor(this);
    const start = () => hydrate(this);

    if (client === "load") {
      start();
    } else if (client === "idle") {
      const idle = globalThis.requestIdleCallback ?? ((fn) => setTimeout(fn, 1));
      idle(() => start());
    } else {
      // visible (default)
      if (typeof IntersectionObserver === "function") {
        const observer = new IntersectionObserver((entries) => {
          if (entries.some((entry) => entry.isIntersecting)) {
            observer.disconnect();
            start();
          }
        });
        observer.observe(this);
      } else {
        start();
      }
    }
  }
}

if (!customElements.get("docs-island")) {
  customElements.define("docs-island", DocsIsland);
}

export { hydrate };
export default DocsIsland;
