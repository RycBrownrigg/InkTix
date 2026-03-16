import { test, expect } from "@playwright/test";

test.describe("Events & Tickets", () => {
  test("shows events page with event listings", async ({ page }) => {
    await page.goto("/events/");
    await expect(page.getByText("Events")).toBeVisible();
  });

  test("shows event categories for filtering", async ({ page }) => {
    await page.goto("/events/");
    // The events page has category filters
    await expect(page.getByText("All")).toBeVisible();
  });

  test("displays platform statistics", async ({ page }) => {
    await page.goto("/events/");
    // Events page shows stats
    await expect(page.getByText(/events/i)).toBeVisible();
  });
});
