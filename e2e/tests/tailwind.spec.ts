import { test, expect } from "./fixtures";

test.describe("Tailwind Example", () => {
  test.beforeEach(async ({ tailwind }) => {
    await tailwind.goto();
  });

  test("renders sidebar navigation", async ({ tailwind }) => {
    await expect(tailwind.page.locator("aside")).toBeVisible();
    await expect(
      tailwind.page.locator('aside .text-xl:has-text("Dashboard")')
    ).toBeVisible();
  });

  test("renders all navigation links", async ({ tailwind }) => {
    await expect(tailwind.navLink("Dashboard")).toBeVisible();
    await expect(tailwind.navLink("Analytics")).toBeVisible();
    await expect(tailwind.navLink("Settings")).toBeVisible();
  });

  test("Dashboard is active on initial load", async ({ tailwind }) => {
    await expect(tailwind.navLink("Dashboard")).toHaveClass(/active/);
    await expect(tailwind.heading()).toHaveText("Dashboard");
  });

  test("displays dashboard stats cards", async ({ tailwind }) => {
    await expect(tailwind.page.locator("text=Users")).toBeVisible();
    await expect(tailwind.page.locator("text=1,234")).toBeVisible();
    await expect(tailwind.page.locator("text=Revenue")).toBeVisible();
    await expect(tailwind.page.locator("text=$12,345")).toBeVisible();
    await expect(tailwind.page.locator("text=Orders")).toBeVisible();
    await expect(tailwind.page.locator("text=567")).toBeVisible();
  });

  test("NavLinks have nav-link class", async ({ tailwind }) => {
    const links = await tailwind.page.locator("nav a").all();

    for (const link of links) {
      await expect(link).toHaveClass(/nav-link/);
    }
  });

  test("navigates to Analytics", async ({ tailwind }) => {
    await tailwind.clickNav("Analytics");

    await expect(tailwind.navLink("Analytics")).toHaveClass(/active/);
    await expect(tailwind.navLink("Dashboard")).not.toHaveClass(/active/);
    await expect(tailwind.heading()).toHaveText("Analytics");
  });

  test("navigates to Settings", async ({ tailwind }) => {
    await tailwind.clickNav("Settings");

    await expect(tailwind.navLink("Settings")).toHaveClass(/active/);
    await expect(tailwind.navLink("Dashboard")).not.toHaveClass(/active/);
    await expect(tailwind.heading()).toHaveText("Settings");
  });

  test("full navigation cycle", async ({ tailwind }) => {
    await tailwind.clickNav("Analytics");
    await expect(tailwind.navLink("Analytics")).toHaveClass(/active/);

    await tailwind.clickNav("Settings");
    await expect(tailwind.navLink("Settings")).toHaveClass(/active/);

    await tailwind.clickNav("Dashboard");
    await expect(tailwind.navLink("Dashboard")).toHaveClass(/active/);
  });

  test("direct URL navigation", async ({ tailwind }) => {
    await tailwind.goto("/analytics");

    await expect(tailwind.navLink("Analytics")).toHaveClass(/active/);
    await expect(tailwind.heading()).toHaveText("Analytics");
  });

  test("handles 404 page", async ({ tailwind }) => {
    await tailwind.goto("/nonexistent");

    await expect(tailwind.page.locator("h1:has-text('404')")).toBeVisible();
    await expect(tailwind.page.locator("text=Page not found")).toBeVisible();
  });

  test("browser history navigation", async ({ tailwind }) => {
    await tailwind.clickNav("Analytics");
    await tailwind.clickNav("Settings");

    await tailwind.page.goBack();
    await expect(tailwind.navLink("Analytics")).toHaveClass(/active/);

    await tailwind.page.goBack();
    await expect(tailwind.navLink("Dashboard")).toHaveClass(/active/);

    await tailwind.page.goForward();
    await expect(tailwind.navLink("Analytics")).toHaveClass(/active/);
  });

  test("only one link is active at a time", async ({ tailwind }) => {
    const routes = ["Dashboard", "Analytics", "Settings"];

    for (const active of routes) {
      await tailwind.clickNav(active);

      for (const route of routes) {
        if (route === active) {
          await expect(tailwind.navLink(route)).toHaveClass(/active/);
        } else {
          await expect(tailwind.navLink(route)).not.toHaveClass(/active/);
        }
      }
    }
  });

  test("layout has correct structure", async ({ tailwind }) => {
    await expect(tailwind.page.locator(".min-h-screen.flex")).toBeVisible();
    await expect(tailwind.page.locator("aside.w-64")).toBeVisible();
    await expect(tailwind.page.locator("main.flex-1")).toBeVisible();
  });
});
