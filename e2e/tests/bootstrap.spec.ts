import { test, expect } from "./fixtures";

test.describe("Bootstrap Example", () => {
  test.beforeEach(async ({ bootstrap }) => {
    await bootstrap.goto();
  });

  test("renders Bootstrap navbar", async ({ bootstrap }) => {
    await expect(bootstrap.page.locator(".navbar")).toBeVisible();
    await expect(bootstrap.page.locator(".navbar-brand")).toHaveText("MyApp");
  });

  test("renders all navigation links", async ({ bootstrap }) => {
    await expect(bootstrap.navLink("Home")).toBeVisible();
    await expect(bootstrap.navLink("Products")).toBeVisible();
    await expect(bootstrap.navLink("Pricing")).toBeVisible();
    await expect(bootstrap.navLink("Contact")).toBeVisible();
  });

  test("Home is active on initial load", async ({ bootstrap }) => {
    await expect(bootstrap.navLink("Home")).toHaveClass(/active/);
    await expect(bootstrap.page.locator(".card-title")).toHaveText("Welcome");
  });

  test("NavLinks have Bootstrap-compatible classes", async ({ bootstrap }) => {
    const links = await bootstrap.page.locator(".navbar-nav a").all();

    for (const link of links) {
      await expect(link).toHaveClass(/nav-link/);
    }
  });

  test("navigates to Products", async ({ bootstrap }) => {
    await bootstrap.clickNav("Products");

    await expect(bootstrap.navLink("Products")).toHaveClass(/active/);
    await expect(bootstrap.navLink("Home")).not.toHaveClass(/active/);
    await expect(bootstrap.page.locator(".card-title")).toHaveText("Products");
  });

  test("navigates to Pricing", async ({ bootstrap }) => {
    await bootstrap.clickNav("Pricing");

    await expect(bootstrap.navLink("Pricing")).toHaveClass(/active/);
    await expect(bootstrap.page.locator(".card-title")).toHaveText("Pricing");
  });

  test("navigates to Contact", async ({ bootstrap }) => {
    await bootstrap.clickNav("Contact");

    await expect(bootstrap.navLink("Contact")).toHaveClass(/active/);
    await expect(bootstrap.page.locator(".card-title")).toHaveText("Contact");
  });

  test("full navigation cycle", async ({ bootstrap }) => {
    const routes = ["Products", "Pricing", "Contact", "Home"];

    for (const route of routes) {
      await bootstrap.clickNav(route);
      await expect(bootstrap.navLink(route)).toHaveClass(/active/);
    }
  });

  test("direct URL navigation", async ({ bootstrap }) => {
    await bootstrap.goto("/pricing");

    await expect(bootstrap.navLink("Pricing")).toHaveClass(/active/);
    await expect(bootstrap.page.locator(".card-title")).toHaveText("Pricing");
  });

  test("handles 404 with Bootstrap alert", async ({ bootstrap }) => {
    await bootstrap.goto("/nonexistent");

    await expect(bootstrap.page.locator(".alert-warning")).toBeVisible();
    await expect(bootstrap.page.locator(".alert-heading")).toHaveText(
      "404 - Not Found"
    );
  });

  test("browser history navigation", async ({ bootstrap }) => {
    await bootstrap.clickNav("Products");
    await bootstrap.clickNav("Pricing");

    await bootstrap.page.goBack();
    await expect(bootstrap.navLink("Products")).toHaveClass(/active/);

    await bootstrap.page.goBack();
    await expect(bootstrap.navLink("Home")).toHaveClass(/active/);

    await bootstrap.page.goForward();
    await expect(bootstrap.navLink("Products")).toHaveClass(/active/);
  });

  test("only one link is active at a time", async ({ bootstrap }) => {
    const routes = ["Home", "Products", "Pricing", "Contact"];

    for (const active of routes) {
      await bootstrap.clickNav(active);

      for (const route of routes) {
        if (route === active) {
          await expect(bootstrap.navLink(route)).toHaveClass(/active/);
        } else {
          await expect(bootstrap.navLink(route)).not.toHaveClass(/active/);
        }
      }
    }
  });
});
