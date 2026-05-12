// SPDX-FileCopyrightText: 2024-2026 RAprogramm <andrey.rozanov-vl@gmail.com>
// SPDX-License-Identifier: MIT

import { test, expect } from '@playwright/test';

test.describe('Pagination', () => {
  test('marks the current page item active', async ({ page }) => {
    await page.goto('/components');

    const pagination = page.locator('ul.pagination').first();
    await expect(pagination).toBeVisible();

    const active = pagination.locator('li.pagination-item.active');
    await expect(active).toHaveCount(1);
  });
});
