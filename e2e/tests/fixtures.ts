import { test as base, type Page, type Locator } from "@playwright/test";

const PORTS = {
  basic: 8080,
  "nested-routes": 8081,
  bootstrap: 8082,
  tailwind: 8083,
} as const;

type ExampleName = keyof typeof PORTS;

export class NavLinkPage {
  readonly page: Page;
  readonly baseUrl: string;

  constructor(page: Page, example: ExampleName) {
    this.page = page;
    this.baseUrl = `http://localhost:${PORTS[example]}`;
  }

  async goto(path = "/"): Promise<void> {
    await this.page.goto(`${this.baseUrl}${path}`);
    await this.waitForWasm();
  }

  async waitForWasm(): Promise<void> {
    await this.page.waitForFunction(() => {
      return document.querySelector("nav") !== null;
    });
  }

  navLink(text: string): Locator {
    return this.page.locator(`nav a:has-text("${text}")`);
  }

  mainNavLink(text: string): Locator {
    return this.page.locator(`.main-nav a:has-text("${text}")`);
  }

  subNavLink(text: string): Locator {
    return this.page.locator(`.sub-nav a:has-text("${text}")`);
  }

  heading(level: 1 | 2 | 3 = 1): Locator {
    return this.page.locator(`h${level}`).first();
  }

  async clickNav(text: string): Promise<void> {
    await this.navLink(text).click();
    await this.page.waitForLoadState("networkidle");
  }

  async expectActive(text: string): Promise<void> {
    await this.navLink(text).waitFor({ state: "visible" });
    const link = this.navLink(text);
    await link.evaluate((el) => el.classList.contains("active"));
  }

  async getActiveLinks(): Promise<string[]> {
    const links = await this.page.locator("nav a.active").all();
    const texts: string[] = [];
    for (const link of links) {
      const text = await link.textContent();
      if (text) texts.push(text.trim());
    }
    return texts;
  }
}

export const test = base.extend<{
  basic: NavLinkPage;
  nestedRoutes: NavLinkPage;
  bootstrap: NavLinkPage;
  tailwind: NavLinkPage;
}>({
  basic: async ({ page }, use) => {
    await use(new NavLinkPage(page, "basic"));
  },
  nestedRoutes: async ({ page }, use) => {
    await use(new NavLinkPage(page, "nested-routes"));
  },
  bootstrap: async ({ page }, use) => {
    await use(new NavLinkPage(page, "bootstrap"));
  },
  tailwind: async ({ page }, use) => {
    await use(new NavLinkPage(page, "tailwind"));
  },
});

export { expect } from "@playwright/test";
