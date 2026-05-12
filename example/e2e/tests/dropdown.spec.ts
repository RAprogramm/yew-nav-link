// SPDX-FileCopyrightText: 2024-2026 RAprogramm <andrey.rozanov-vl@gmail.com>
// SPDX-License-Identifier: MIT

import { test, expect } from '@playwright/test';

// NavDropdown ships toggle-on-click semantics. Outside-click and Escape
// handlers are tracked in a follow-up: the current scenarios exercise the
// state machine that is implemented today.

test.describe('NavDropdown', () => {
  test('opens on toggle click and exposes the menu', async ({ page }) => {
    await page.goto('/components');

    const toggle = page.getByRole('button', { name: /Account/ }).first();
    await expect(toggle).toHaveAttribute('aria-expanded', 'false');

    await toggle.click();
    await expect(toggle).toHaveAttribute('aria-expanded', 'true');
    await expect(toggle.locator('xpath=..').locator('.nav-dropdown-menu')).toHaveClass(/open/);
  });

  test('closes when the toggle is clicked again', async ({ page }) => {
    await page.goto('/components');

    const toggle = page.getByRole('button', { name: /Account/ }).first();
    await toggle.click();
    await expect(toggle).toHaveAttribute('aria-expanded', 'true');

    await toggle.click();
    await expect(toggle).toHaveAttribute('aria-expanded', 'false');
  });
});
