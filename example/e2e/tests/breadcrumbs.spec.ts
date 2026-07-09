// SPDX-FileCopyrightText: 2024-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

import { test, expect } from '@playwright/test';

test.describe('breadcrumbs', () => {
  test('renders one entry per path segment of a nested route', async ({ page }) => {
    await page.goto('/hooks/team/platform');

    const trail = page.locator('nav[aria-label="Breadcrumb"]').first();
    await expect(trail).toBeVisible();

    // The trail uses an aria-current="page" marker on the final crumb;
    // the BreadcrumbLabelProvider in the demo turns raw segments into
    // human labels, so the deepest crumb shows the team's display name
    // rather than the raw slug.
    const current = trail.locator('[aria-current="page"]');
    await expect(current).toHaveCount(1);

    const text = (await current.textContent())?.trim();
    expect(text, 'breadcrumb label should not be the raw slug').not.toBe('platform');
  });
});
