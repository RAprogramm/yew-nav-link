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
    // aria-current="page" (inactive items omit the attribute entirely).
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

  test('clicking next advances the active page', async ({ page }) => {
    await page.goto('/components');

    const pagination = page.locator('nav[aria-label="pagination"]').first();
    await expect(
      pagination.locator('button[aria-current="page"]')
    ).toHaveText('1');

    await pagination.getByRole('button', { name: 'Next page' }).click();
    await expect(
      pagination.locator('button[aria-current="page"]')
    ).toHaveText('2');

    await pagination.getByRole('button', { name: 'Last page' }).click();
    await expect(
      pagination.locator('button[aria-current="page"]')
    ).toHaveText('20');
  });

  test('ellipsis gaps are not interactive', async ({ page }) => {
    await page.goto('/components');

    const pagination = page.locator('nav[aria-label="pagination"]').first();
    await expect(
      pagination.locator('.pagination-ellipsis').first()
    ).toBeVisible();
    await expect(
      pagination.locator('.pagination-ellipsis button')
    ).toHaveCount(0);
  });
});
