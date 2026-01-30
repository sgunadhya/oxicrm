// @ts-check
const { test, expect } = require('@playwright/test');

test.describe('Workflow Management', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
  });

  test('should display workflows page', async ({ page }) => {
    await page.goto('/workflows');

    await expect(page.locator('h2')).toContainText('Workflows');
    await expect(page.locator('a[href="/workflows/new"]')).toBeVisible();
  });

  test('should display workflow creation form', async ({ page }) => {
    await page.goto('/workflows/new');

    await expect(page.locator('h2')).toContainText('Add New Workflow');

    // Verify form fields
    await expect(page.locator('input[name="name"]')).toBeVisible();

    await expect(page.locator('button[type="submit"]')).toContainText('Save');
    await expect(page.locator('a[href="/workflows"]')).toContainText('Cancel');
  });

  test('should create a new workflow', async ({ page }) => {
    await page.goto('/workflows/new');

    await page.fill('input[name="name"]', 'Lead Qualification Automation');

    await page.click('button[type="submit"]');
    await page.waitForTimeout(1000);

    await expect(page.locator('h2')).toContainText('Workflows');
    await expect(page.locator('table')).toContainText('Lead Qualification Automation');
  });

  test('should display workflows in table format', async ({ page }) => {
    await page.goto('/workflows');

    const table = page.locator('table');
    await expect(table.locator('th:has-text("Name")')).toBeVisible();
    await expect(table.locator('th:has-text("Created")')).toBeVisible();
    await expect(table.locator('th:has-text("Actions")')).toBeVisible();
  });

  test('should delete a workflow', async ({ page }) => {
    // First create a workflow
    await page.goto('/workflows/new');
    await page.fill('input[name="name"]', 'Workflow to Delete');
    await page.click('button[type="submit"]');
    await page.waitForTimeout(1000);

    // Verify it exists
    await expect(page.locator('table')).toContainText('Workflow to Delete');

    // Delete it
    const row = page.locator('tr:has-text("Workflow to Delete")');
    await row.locator('button:has-text("Delete")').click();
    await page.waitForTimeout(1000);

    // Verify it's removed
    await expect(page.locator('table')).not.toContainText('Workflow to Delete');
  });

  test('should navigate to workflow form and back', async ({ page }) => {
    await page.goto('/workflows');

    await page.click('a[href="/workflows/new"]');
    await expect(page.locator('h2')).toContainText('Add New Workflow');

    await page.click('a[href="/workflows"]');
    await expect(page.locator('h2')).toContainText('Workflows');
  });

  test('should validate required name field', async ({ page }) => {
    await page.goto('/workflows/new');

    // Try to submit without name
    await page.click('button[type="submit"]');

    const nameInput = page.locator('input[name="name"]');
    const isInvalid = await nameInput.evaluate((el) => !el.validity.valid);
    expect(isInvalid).toBe(true);
  });

  test('should create multiple workflows', async ({ page }) => {
    const workflowNames = [
      'Email Campaign Workflow',
      'Lead Scoring Workflow',
      'Onboarding Workflow'
    ];

    for (const name of workflowNames) {
      await page.goto('/workflows/new');
      await page.fill('input[name="name"]', name);
      await page.click('button[type="submit"]');
      await page.waitForTimeout(500);
    }

    // Verify all workflows are listed
    for (const name of workflowNames) {
      await expect(page.locator('table')).toContainText(name);
    }
  });

  test('should handle long workflow names', async ({ page }) => {
    await page.goto('/workflows/new');

    const longName = 'This is a very long workflow name that contains a lot of text to test how the UI handles longer names in the table view';
    await page.fill('input[name="name"]', longName);
    await page.click('button[type="submit"]');
    await page.waitForTimeout(1000);

    await expect(page.locator('table')).toContainText(longName);
  });

  test('should display creation timestamp', async ({ page }) => {
    await page.goto('/workflows/new');

    await page.fill('input[name="name"]', 'Timestamped Workflow');
    await page.click('button[type="submit"]');
    await page.waitForTimeout(1000);

    const row = page.locator('tr:has-text("Timestamped Workflow")');

    // Check that created date is displayed (format: YYYY-MM-DD HH:MM)
    const rowText = await row.textContent();
    expect(rowText).toMatch(/\d{4}-\d{2}-\d{2}\s+\d{2}:\d{2}/);
  });
});
