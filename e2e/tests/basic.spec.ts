import { test, expect } from "./fixtures";

test.describe("Basic Example", () => {
  test.beforeEach(async ({ basic }) => {
    await basic.goto();
  });

  test("renders navigation with all links", async ({ basic }) => {
    await expect(basic.navLink("Home")).toBeVisible();
    await expect(basic.navLink("About")).toBeVisible();
    await expect(basic.navLink("Contact")).toBeVisible();
  });

  test("Home link is active on initial load", async ({ basic }) => {
    await expect(basic.navLink("Home")).toHaveClass(/active/);
    await expect(basic.navLink("About")).not.toHaveClass(/active/);
    await expect(basic.navLink("Contact")).not.toHaveClass(/active/);
  });

  test("displays Home page content", async ({ basic }) => {
    await expect(basic.heading()).toHaveText("Home");
  });

  test("navigates to About and updates active state", async ({ basic }) => {
    await basic.clickNav("About");

    await expect(basic.navLink("About")).toHaveClass(/active/);
    await expect(basic.navLink("Home")).not.toHaveClass(/active/);
    await expect(basic.heading()).toHaveText("About");
  });

  test("navigates to Contact and updates active state", async ({ basic }) => {
    await basic.clickNav("Contact");

    await expect(basic.navLink("Contact")).toHaveClass(/active/);
    await expect(basic.navLink("Home")).not.toHaveClass(/active/);
    await expect(basic.heading()).toHaveText("Contact");
  });

  test("navigation sequence maintains correct active states", async ({
    basic,
  }) => {
    await basic.clickNav("About");
    await expect(basic.navLink("About")).toHaveClass(/active/);

    await basic.clickNav("Contact");
    await expect(basic.navLink("Contact")).toHaveClass(/active/);
    await expect(basic.navLink("About")).not.toHaveClass(/active/);

    await basic.clickNav("Home");
    await expect(basic.navLink("Home")).toHaveClass(/active/);
    await expect(basic.navLink("Contact")).not.toHaveClass(/active/);
  });

  test("direct URL navigation sets correct active state", async ({ basic }) => {
    await basic.goto("/about");

    await expect(basic.navLink("About")).toHaveClass(/active/);
    await expect(basic.navLink("Home")).not.toHaveClass(/active/);
    await expect(basic.heading()).toHaveText("About");
  });

  test("handles 404 page", async ({ basic }) => {
    await basic.goto("/nonexistent");

    await expect(basic.heading()).toHaveText("404 - Not Found");
  });

  test("all NavLinks have nav-link class", async ({ basic }) => {
    const links = await basic.page.locator("nav a").all();

    for (const link of links) {
      await expect(link).toHaveClass(/nav-link/);
    }
  });

  test("browser back/forward navigation works", async ({ basic }) => {
    await basic.clickNav("About");
    await expect(basic.navLink("About")).toHaveClass(/active/);

    await basic.clickNav("Contact");
    await expect(basic.navLink("Contact")).toHaveClass(/active/);

    await basic.page.goBack();
    await expect(basic.navLink("About")).toHaveClass(/active/);

    await basic.page.goForward();
    await expect(basic.navLink("Contact")).toHaveClass(/active/);
  });
});
