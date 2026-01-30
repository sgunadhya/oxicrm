// @ts-check
const { test, expect } = require('@playwright/test');

test.describe('Opportunity Management', () => {
  test.beforeEach(async ({ page }) => {
    // Start at the home page
    await page.goto('/');
  });

  test('should display opportunities page', async ({ page }) => {
    // Navigate to opportunities page
    await page.goto('/opportunities');

    // Check page title
    await expect(page.locator('h2')).toContainText('Opportunities');

    // Check that Add Opportunity button exists
    await expect(page.locator('a[href="/opportunities/new"]')).toBeVisible();
  });

  test('should display opportunity creation form', async ({ page }) => {
    // Navigate to new opportunity form
    await page.goto('/opportunities/new');

    // Check form title
    await expect(page.locator('h2')).toContainText('Add New Opportunity');

    // Verify all required form fields exist
    await expect(page.locator('input[name="name"]')).toBeVisible();
    await expect(page.locator('select[name="stage"]')).toBeVisible();
    await expect(page.locator('input[name="amount_micros"]')).toBeVisible();
    await expect(page.locator('input[name="currency_code"]')).toBeVisible();
    await expect(page.locator('input[name="close_date"]')).toBeVisible();
    await expect(page.locator('select[name="company_id"]')).toBeVisible();
    await expect(page.locator('select[name="point_of_contact_id"]')).toBeVisible();

    // Check submit button
    await expect(page.locator('button[type="submit"]')).toContainText('Save');

    // Check cancel link
    await expect(page.locator('a[href="/opportunities"]')).toContainText('Cancel');
  });

  test('should verify stage options in dropdown', async ({ page }) => {
    await page.goto('/opportunities/new');

    const stageSelect = page.locator('select[name="stage"]');

    // Verify all stages are present
    const stages = ['Prospecting', 'Qualification', 'Negotiation', 'Won', 'Lost'];
    for (const stage of stages) {
      await expect(stageSelect.locator(`option[value="${stage}"]`)).toBeVisible();
    }
  });

  test('should create a new opportunity', async ({ page }) => {
    // Navigate to new opportunity form
    await page.goto('/opportunities/new');

    // Fill in the form
    await page.fill('input[name="name"]', 'Q1 Enterprise Deal');
    await page.selectOption('select[name="stage"]', 'Prospecting');
    await page.fill('input[name="amount_micros"]', '50000');
    await page.fill('input[name="currency_code"]', 'USD');
    await page.fill('input[name="close_date"]', '2026-03-31');

    // Submit the form
    await page.click('button[type="submit"]');

    // Wait for navigation/response
    await page.waitForTimeout(1000);

    // Verify we're on the opportunities list page
    await expect(page.locator('h2')).toContainText('Opportunities');

    // Verify the new opportunity appears in the list
    await expect(page.locator('table')).toContainText('Q1 Enterprise Deal');
    await expect(page.locator('table')).toContainText('Prospecting');
    await expect(page.locator('table')).toContainText('USD 50.00');
  });

  test('should display opportunities in table format', async ({ page }) => {
    await page.goto('/opportunities');

    // Check table headers
    const table = page.locator('table');
    await expect(table.locator('th:has-text("Name")')).toBeVisible();
    await expect(table.locator('th:has-text("Stage")')).toBeVisible();
    await expect(table.locator('th:has-text("Amount")')).toBeVisible();
    await expect(table.locator('th:has-text("Close Date")')).toBeVisible();
    await expect(table.locator('th:has-text("Actions")')).toBeVisible();
  });

  test('should delete an opportunity', async ({ page }) => {
    // First create an opportunity
    await page.goto('/opportunities/new');
    await page.fill('input[name="name"]', 'Test Delete Opportunity');
    await page.selectOption('select[name="stage"]', 'Prospecting');
    await page.click('button[type="submit"]');
    await page.waitForTimeout(1000);

    // Verify it exists
    await expect(page.locator('table')).toContainText('Test Delete Opportunity');

    // Find and click delete button for this opportunity
    const row = page.locator('tr:has-text("Test Delete Opportunity")');
    await row.locator('button:has-text("Delete")').click();

    // Wait for deletion
    await page.waitForTimeout(1000);

    // Verify the opportunity is removed from the table
    await expect(page.locator('table')).not.toContainText('Test Delete Opportunity');
  });

  test('should navigate to opportunity form and back', async ({ page }) => {
    await page.goto('/opportunities');

    // Click Add Opportunity button
    await page.click('a[href="/opportunities/new"]');

    // Verify we're on the form page
    await expect(page.locator('h2')).toContainText('Add New Opportunity');

    // Click Cancel
    await page.click('a[href="/opportunities"]');

    // Verify we're back on the list page
    await expect(page.locator('h2')).toContainText('Opportunities');
  });

  test('should validate required fields', async ({ page }) => {
    await page.goto('/opportunities/new');

    // Try to submit without filling required fields
    await page.click('button[type="submit"]');

    // Check that name field shows validation error (HTML5 validation)
    const nameInput = page.locator('input[name="name"]');
    const isInvalid = await nameInput.evaluate((el) => !el.validity.valid);
    expect(isInvalid).toBe(true);
  });

  test('should create opportunity with all fields', async ({ page }) => {
    // First, create a company and person for linking
    await page.goto('/companies/new');
    await page.fill('input[name="name"]', 'Test Company');
    await page.fill('input[name="domain_name"]', 'testcompany.com');
    await page.click('button[type="submit"]');
    await page.waitForTimeout(500);

    await page.goto('/people/new');
    await page.fill('input[name="name"]', 'Test Person');
    await page.fill('input[name="email"]', 'test@example.com');
    await page.click('button[type="submit"]');
    await page.waitForTimeout(500);

    // Now create opportunity with links
    await page.goto('/opportunities/new');

    await page.fill('input[name="name"]', 'Full Feature Opportunity');
    await page.selectOption('select[name="stage"]', 'Qualification');
    await page.fill('input[name="amount_micros"]', '100000');
    await page.fill('input[name="currency_code"]', 'EUR');
    await page.fill('input[name="close_date"]', '2026-06-30');

    // Select company (should be available in dropdown)
    const companyOptions = await page.locator('select[name="company_id"] option').count();
    if (companyOptions > 1) { // More than just the placeholder
      await page.selectOption('select[name="company_id"]', { index: 1 });
    }

    // Select person (should be available in dropdown)
    const personOptions = await page.locator('select[name="point_of_contact_id"] option').count();
    if (personOptions > 1) {
      await page.selectOption('select[name="point_of_contact_id"]', { index: 1 });
    }

    await page.click('button[type="submit"]');
    await page.waitForTimeout(1000);

    // Verify the opportunity was created with all details
    await expect(page.locator('table')).toContainText('Full Feature Opportunity');
    await expect(page.locator('table')).toContainText('Qualification');
    await expect(page.locator('table')).toContainText('EUR 100.00');
    await expect(page.locator('table')).toContainText('2026-06-30');
  });
});
