// SPDX-FileCopyrightText: RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

import { test, expect } from '@playwright/test';

// Regression guard for the `html!` `{classes}` shorthand footgun: it renders
// a bogus `classes="…"` attribute instead of `class="…"`, so styling silently
// breaks. No element in the demo should ever carry a `classes` attribute.
// Complements the static guard (tests/html_class_shorthand_guard.rs) and the
// wasm DOM tests by proving the real, trunk-built demo is styled correctly.

const ROUTES = ['/', '/navlink', '/components', '/hooks', '/utilities'];

test.describe('class attribute integrity', () => {
  for (const route of ROUTES) {
    test(`no bogus "classes" attribute on ${route}`, async ({ page }) => {
      await page.goto(route);
      await expect(page.locator('nav').first()).toBeVisible();
      await expect(page.locator('[classes]')).toHaveCount(0);
    });
  }

  test('affected components expose real class names on /components', async ({ page }) => {
    await page.goto('/components');

    await expect(page.locator('ul.pagination').first()).toBeVisible();
    await expect(page.locator('.nav-badge').first()).toBeVisible();
    await expect(page.locator('.nav-dropdown').first()).toBeVisible();
  });
});
