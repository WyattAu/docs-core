// Headless tests for the island shim's pure half (bun test; the estate has
// bun + nodejs-lts and no npm).
import { test, expect, describe } from "bun:test";
import {
  HYDRATION,
  parseProps,
  hydrationFor,
  componentUrlFor,
  decideHydration,
  makeLoader,
} from "./island-core.mjs";

// Minimal element stand-ins: the pure functions only need attributes and
// querySelector, so no DOM shim is required.
const el = (attrs = {}, props = null) => ({
  attrs,
  querySelector: (sel) =>
    sel === "script[data-island-props]" && props !== null
      ? { textContent: props }
      : null,
  getAttribute: (name) => (name in attrs ? attrs[name] : null),
});

describe("props", () => {
  test("parses JSON object props", () => {
    expect(parseProps(el({}, '{"count":3}'))).toEqual({ count: 3 });
  });

  test("missing props script yields an empty object", () => {
    expect(parseProps(el())).toEqual({});
  });

  test("malformed props degrade to empty, never throw", () => {
    expect(parseProps(el({}, "{not json"))).toEqual({});
    expect(parseProps(el({}, "[1,2]"))).toEqual({});
    expect(parseProps(el({}, '"a string"'))).toEqual({});
  });
});

describe("hydration hints", () => {
  test("explicit hints pass through; unknown falls back to visible", () => {
    expect(hydrationFor(el({ client: "load" }))).toBe(HYDRATION.LOAD);
    expect(hydrationFor(el({ client: "idle" }))).toBe(HYDRATION.IDLE);
    expect(hydrationFor(el({}))).toBe(HYDRATION.VISIBLE);
    expect(hydrationFor(el({ client: "whenever" }))).toBe(HYDRATION.VISIBLE);
  });

  test("decisions follow the hint", () => {
    expect(decideHydration({ client: "load", visible: false, idle: false })).toBe(true);
    expect(decideHydration({ client: "visible", visible: true, idle: false })).toBe(true);
    expect(decideHydration({ client: "visible", visible: false, idle: true })).toBe(false);
    expect(decideHydration({ client: "idle", visible: true, idle: false })).toBe(false);
    expect(decideHydration({ client: "idle", visible: false, idle: true })).toBe(true);
  });
});

describe("component urls", () => {
  test("reads and validates the attribute", () => {
    expect(componentUrlFor(el({ "component-url": "/islands/a.js" }))).toBe("/islands/a.js");
    expect(componentUrlFor(el({ "component-url": "" }))).toBe(null);
    expect(componentUrlFor(el())).toBe(null);
  });
});

describe("loader", () => {
  test("imports each url exactly once", async () => {
    let calls = 0;
    const loader = makeLoader(async (url) => {
      calls += 1;
      return { default: () => url };
    });
    const a = await loader("/islands/a.js");
    const b = await loader("/islands/a.js");
    expect(calls).toBe(1);
    expect(a).toBe(b);
  });

  test("a failed load does not poison the cache", async () => {
    let calls = 0;
    const loader = makeLoader(async (url) => {
      calls += 1;
      if (calls === 1) throw new Error("network");
      return { default: () => url };
    });
    await expect(loader("/islands/x.js")).rejects.toThrow("network");
    const ok = await loader("/islands/x.js");
    expect(ok.default()).toBe("/islands/x.js");
    expect(calls).toBe(2);
  });

  test("different urls are different promises", () => {
    const loader = makeLoader(async (url) => ({ default: () => url }));
    expect(loader("/a.js")).not.toBe(loader("/b.js"));
  });
});
