// SPDX-FileCopyrightText: RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

import { test, expect } from '@playwright/test';

// The /hooks page renders live hook output: the query-params map and the
// breadcrumb trail react to the URL without a reload.

test.describe('hooks page', () => {
  test('use_query_params reflects the current query string', async ({ page }) => {
    await page.goto('/hooks?foo=bar&page=2');

    const dump = page.locator('.inline-pre').first();
    await expect(dump).toContainText('foo');
    await expect(dump).toContainText('bar');
    await expect(dump).toContainText('page');
  });

  test('breadcrumb trail marks only the last item current', async ({ page }) => {
    await page.goto('/hooks');

    const trail = page.locator('nav[aria-label="Breadcrumb"]').first();
    await expect(trail).toBeVisible();
    await expect(trail.locator('[aria-current="page"]')).toHaveCount(1);
  });
});
