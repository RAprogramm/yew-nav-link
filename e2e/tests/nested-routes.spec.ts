import { test, expect } from "./fixtures";

test.describe("Nested Routes Example", () => {
  test.beforeEach(async ({ nestedRoutes }) => {
    await nestedRoutes.goto();
  });

  test("renders main navigation", async ({ nestedRoutes }) => {
    await expect(nestedRoutes.mainNavLink("Home")).toBeVisible();
    await expect(nestedRoutes.mainNavLink("Documentation")).toBeVisible();
    await expect(nestedRoutes.mainNavLink("Blog")).toBeVisible();
  });

  test("Home is active on initial load", async ({ nestedRoutes }) => {
    await expect(nestedRoutes.mainNavLink("Home")).toHaveClass(/active/);
    await expect(nestedRoutes.heading()).toHaveText("Nested Routes Example");
  });

  test.describe("Documentation Section", () => {
    test.beforeEach(async ({ nestedRoutes }) => {
      await nestedRoutes.mainNavLink("Documentation").click();
      await nestedRoutes.page.waitForLoadState("networkidle");
    });

    test("shows sub-navigation", async ({ nestedRoutes }) => {
      await expect(nestedRoutes.subNavLink("Overview")).toBeVisible();
      await expect(nestedRoutes.subNavLink("Getting Started")).toBeVisible();
      await expect(nestedRoutes.subNavLink("API Reference")).toBeVisible();
    });

    test("main nav Documentation stays active", async ({ nestedRoutes }) => {
      await expect(nestedRoutes.mainNavLink("Documentation")).toHaveClass(
        /active/
      );
      await expect(nestedRoutes.mainNavLink("Home")).not.toHaveClass(/active/);
    });

    test("Overview is active by default", async ({ nestedRoutes }) => {
      await expect(nestedRoutes.subNavLink("Overview")).toHaveClass(/active/);
      await expect(nestedRoutes.heading()).toHaveText("Documentation Overview");
    });

    test("navigates to Getting Started", async ({ nestedRoutes }) => {
      await nestedRoutes.subNavLink("Getting Started").click();
      await nestedRoutes.page.waitForLoadState("networkidle");

      await expect(nestedRoutes.subNavLink("Getting Started")).toHaveClass(
        /active/
      );
      await expect(nestedRoutes.subNavLink("Overview")).not.toHaveClass(
        /active/
      );
      // Main nav Documentation stays active due to partial matching
      await expect(nestedRoutes.mainNavLink("Documentation")).toHaveClass(
        /active/
      );
      await expect(nestedRoutes.heading()).toHaveText("Getting Started");
    });

    test("navigates to API Reference", async ({ nestedRoutes }) => {
      await nestedRoutes.subNavLink("API Reference").click();
      await nestedRoutes.page.waitForLoadState("networkidle");

      await expect(nestedRoutes.subNavLink("API Reference")).toHaveClass(
        /active/
      );
      await expect(nestedRoutes.heading()).toHaveText("API Reference");
    });
  });

  test.describe("Blog Section", () => {
    test.beforeEach(async ({ nestedRoutes }) => {
      await nestedRoutes.mainNavLink("Blog").click();
      await nestedRoutes.page.waitForLoadState("networkidle");
    });

    test("shows sub-navigation", async ({ nestedRoutes }) => {
      await expect(nestedRoutes.subNavLink("Latest Posts")).toBeVisible();
      await expect(nestedRoutes.subNavLink("Archive")).toBeVisible();
      await expect(nestedRoutes.subNavLink("Categories")).toBeVisible();
    });

    test("main nav Blog stays active", async ({ nestedRoutes }) => {
      await expect(nestedRoutes.mainNavLink("Blog")).toHaveClass(/active/);
      await expect(nestedRoutes.mainNavLink("Documentation")).not.toHaveClass(
        /active/
      );
    });

    test("Latest Posts is active by default", async ({ nestedRoutes }) => {
      await expect(nestedRoutes.subNavLink("Latest Posts")).toHaveClass(
        /active/
      );
      await expect(nestedRoutes.heading()).toHaveText("Latest Posts");
    });

    test("navigates through all blog pages", async ({ nestedRoutes }) => {
      await nestedRoutes.subNavLink("Archive").click();
      await nestedRoutes.page.waitForLoadState("networkidle");
      await expect(nestedRoutes.subNavLink("Archive")).toHaveClass(/active/);
      await expect(nestedRoutes.heading()).toHaveText("Archive");

      await nestedRoutes.subNavLink("Categories").click();
      await nestedRoutes.page.waitForLoadState("networkidle");
      await expect(nestedRoutes.subNavLink("Categories")).toHaveClass(/active/);
      await expect(nestedRoutes.heading()).toHaveText("Categories");
    });
  });

  test("switching between sections updates both navs", async ({
    nestedRoutes,
  }) => {
    await nestedRoutes.mainNavLink("Documentation").click();
    await nestedRoutes.page.waitForLoadState("networkidle");
    await expect(nestedRoutes.mainNavLink("Documentation")).toHaveClass(
      /active/
    );
    await expect(nestedRoutes.subNavLink("Overview")).toHaveClass(/active/);

    await nestedRoutes.mainNavLink("Blog").click();
    await nestedRoutes.page.waitForLoadState("networkidle");
    await expect(nestedRoutes.mainNavLink("Blog")).toHaveClass(/active/);
    await expect(nestedRoutes.mainNavLink("Documentation")).not.toHaveClass(
      /active/
    );
    await expect(nestedRoutes.subNavLink("Latest Posts")).toHaveClass(/active/);
  });

  test("direct URL to nested route works", async ({ nestedRoutes }) => {
    await nestedRoutes.goto("/docs/api");

    // Main nav stays active due to partial matching
    await expect(nestedRoutes.mainNavLink("Documentation")).toHaveClass(
      /active/
    );
    await expect(nestedRoutes.subNavLink("API Reference")).toHaveClass(/active/);
    await expect(nestedRoutes.heading()).toHaveText("API Reference");
  });

  test("handles 404 page", async ({ nestedRoutes }) => {
    await nestedRoutes.goto("/nonexistent");

    await expect(nestedRoutes.heading()).toHaveText("404 - Not Found");
  });
});
