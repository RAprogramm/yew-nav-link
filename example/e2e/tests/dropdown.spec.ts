// SPDX-FileCopyrightText: RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

import { test, expect } from '@playwright/test';

// NavDropdown supports toggle-on-click plus keyboard (Escape) and
// outside-click dismissal.

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

  test('closes on Escape', async ({ page }) => {
    await page.goto('/components');

    const toggle = page.getByRole('button', { name: /Account/ }).first();
    await toggle.click();
    await expect(toggle).toHaveAttribute('aria-expanded', 'true');

    await page.keyboard.press('Escape');
    await expect(toggle).toHaveAttribute('aria-expanded', 'false');
  });

  test('closes when focus moves outside the menu', async ({ page }) => {
    await page.goto('/components');

    const toggle = page.getByRole('button', { name: /Account/ }).first();
    await toggle.click();
    await expect(toggle).toHaveAttribute('aria-expanded', 'true');

    // Tab out of the dropdown; the focusout handler dismisses the menu.
    await page.locator('body').click({ position: { x: 5, y: 5 } });
    await expect(toggle).toHaveAttribute('aria-expanded', 'false');
  });

  test('arrow keys move focus over the menu links', async ({ page }) => {
    await page.goto('/components');

    const toggle = page.getByRole('button', { name: /Account/ }).first();
    await toggle.click();
    await expect(toggle).toHaveAttribute('aria-expanded', 'true');

    // Opening by click focuses the first menu link; ArrowDown moves on.
    const menu = toggle.locator('xpath=..').locator('.nav-dropdown-menu');
    const links = menu.locator('a[href]');
    await expect(links.first()).toBeFocused();

    await page.keyboard.press('ArrowDown');
    await expect(links.nth(1)).toBeFocused();

    await page.keyboard.press('End');
    await expect(links.last()).toBeFocused();

    await page.keyboard.press('Home');
    await expect(links.first()).toBeFocused();

    await page.keyboard.press('Escape');
    await expect(toggle).toHaveAttribute('aria-expanded', 'false');
    await expect(toggle).toBeFocused();
  });

  test('uses disclosure semantics instead of menu roles', async ({ page }) => {
    await page.goto('/components');

    const toggle = page.getByRole('button', { name: /Account/ }).first();
    const container = toggle.locator('xpath=..');
    await expect(container.locator('[role="menu"]')).toHaveCount(0);
    await expect(container.locator('[role="menuitem"]')).toHaveCount(0);
    await expect(toggle).not.toHaveAttribute('aria-haspopup');
  });
});
