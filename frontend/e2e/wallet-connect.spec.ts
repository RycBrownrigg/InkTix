import { test, expect } from "@playwright/test";

test.describe("Wallet Connection", () => {
  test("shows connect wallet button on connect page", async ({ page }) => {
    await page.goto("/connect/");
    await expect(page.getByText("Connect Your Wallet")).toBeVisible();
  });

  test("shows getting started steps", async ({ page }) => {
    await page.goto("/connect/");
    await expect(page.getByText("Install Extension")).toBeVisible();
    await expect(page.getByText("Connect Wallet")).toBeVisible();
  });

  test("navigates to connect page from home", async ({ page }) => {
    await page.goto("/");
    const connectLink = page.getByRole("link", { name: /connect/i }).first();
    await connectLink.click();
    await expect(page).toHaveURL(/\/connect/);
  });
});
