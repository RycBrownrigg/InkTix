/**
 * E2E tests for the smart contracts deployment page.
 *
 * Verifies that the page loads, shows prerequisite information, and
 * displays a connection prompt when no wallet is connected.
 */
import { test, expect } from "@playwright/test";

test.describe("Contract Deployment Page", () => {
  test("shows smart contracts page with prerequisite info", async ({
    page,
  }) => {
    await page.goto("/smart-contracts/");
    await expect(page.getByText("Smart Contract")).toBeVisible();
  });

  test("shows connect prompt when not connected", async ({ page }) => {
    await page.goto("/smart-contracts/");
    await expect(
      page.getByText("Connect to Deploy Contract")
    ).toBeVisible();
  });
});
