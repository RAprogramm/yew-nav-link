// SPDX-FileCopyrightText: RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

import { test, expect } from '@playwright/test';

test.describe('Pagination', () => {
  test('marks the current page item active', async ({ page }) => {
    await page.goto('/components');

    // The pagination container is the `<nav aria-label="pagination">`
    // wrapper. Anchor on that role rather than internal class names so
    // the test survives stylesheet renames.
    const pagination = page.locator('nav[aria-label="pagination"]').first();
    await expect(pagination).toBeVisible();

    // The active page button is the only descendant carrying
    // aria-current="page" (Pagination emits "page" / "false" per item).
    const active = pagination.locator('button[aria-current="page"]');
    await expect(active).toHaveCount(1);
  });

  test('prev/next buttons expose an accessible name', async ({ page }) => {
    await page.goto('/components');

    const pagination = page.locator('nav[aria-label="pagination"]').first();
    await expect(pagination).toBeVisible();

    // The glyph-only prev/next controls must announce a real name to
    // assistive tech, not the raw "‹"/"›" characters.
    await expect(
      pagination.getByRole('button', { name: 'Previous page' })
    ).toHaveCount(1);
    await expect(
      pagination.getByRole('button', { name: 'Next page' })
    ).toHaveCount(1);
  });
});
