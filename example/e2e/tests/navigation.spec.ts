// SPDX-FileCopyrightText: RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

import { test, expect } from '@playwright/test';

test.describe('top-nav navigation', () => {
  test('clicking Components updates the URL and marks the link active', async ({ page }) => {
    await page.goto('/');

    const componentsLink = page.getByRole('link', { name: 'Components', exact: true });
    await expect(componentsLink).toHaveAttribute('class', /nav-link/);
    await expect(componentsLink).not.toHaveAttribute('aria-current', 'page');

    await componentsLink.click();
    await expect(page).toHaveURL(/\/components$/);

    await expect(componentsLink).toHaveAttribute('aria-current', 'page');
    await expect(componentsLink).toHaveAttribute('class', /\bactive\b/);
  });

  test('back/forward restores the active state', async ({ page }) => {
    await page.goto('/');

    await page.getByRole('link', { name: 'Components', exact: true }).click();
    await expect(page).toHaveURL(/\/components$/);

    await page.getByRole('link', { name: 'Utilities', exact: true }).click();
    await expect(page).toHaveURL(/\/utilities$/);

    await page.goBack();
    await expect(page).toHaveURL(/\/components$/);
    await expect(
      page.getByRole('link', { name: 'Components', exact: true })
    ).toHaveAttribute('aria-current', 'page');

    await page.goForward();
    await expect(page).toHaveURL(/\/utilities$/);
    await expect(
      page.getByRole('link', { name: 'Utilities', exact: true })
    ).toHaveAttribute('aria-current', 'page');
  });

  test('partial matching keeps the parent active under a nested route', async ({ page }) => {
    await page.goto('/navlink/lab/widget');

    const parent = page
      .locator('nav.top-nav')
      .getByRole('link', { name: 'NavLink', exact: true });
    await expect(parent).toHaveAttribute('aria-current', 'page');
    await expect(parent).toHaveAttribute('class', /\bactive\b/);
  });
});
