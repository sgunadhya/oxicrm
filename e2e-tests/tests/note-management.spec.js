// @ts-check
const { test, expect } = require('@playwright/test');

test.describe('Note Management', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
  });

  test('should display notes page', async ({ page }) => {
    await page.goto('/notes');

    await expect(page.locator('h2')).toContainText('Notes');
    await expect(page.locator('a[href="/notes/new"]')).toBeVisible();
  });

  test('should display note creation form', async ({ page }) => {
    await page.goto('/notes/new');

    await expect(page.locator('h2')).toContainText('Add New Note');

    // Verify form fields
    await expect(page.locator('input[name="title"]')).toBeVisible();
    await expect(page.locator('textarea[name="body_v2"]')).toBeVisible();

    await expect(page.locator('button[type="submit"]')).toContainText('Save');
    await expect(page.locator('a[href="/notes"]')).toContainText('Cancel');
  });

  test('should create a new note', async ({ page }) => {
    await page.goto('/notes/new');

    await page.fill('input[name="title"]', 'Client meeting notes');
    await page.fill('textarea[name="body_v2"]', 'Discussed Q1 requirements and timeline');

    await page.click('button[type="submit"]');
    await page.waitForTimeout(1000);

    await expect(page.locator('h2')).toContainText('Notes');
    await expect(page.locator('table')).toContainText('Client meeting notes');
  });

  test('should create note with only title', async ({ page }) => {
    await page.goto('/notes/new');

    await page.fill('input[name="title"]', 'Quick Note');
    // Leave body empty

    await page.click('button[type="submit"]');
    await page.waitForTimeout(1000);

    await expect(page.locator('table')).toContainText('Quick Note');
  });

  test('should display notes in table format', async ({ page }) => {
    await page.goto('/notes');

    const table = page.locator('table');
    await expect(table.locator('th:has-text("Title")')).toBeVisible();
    await expect(table.locator('th:has-text("Created")')).toBeVisible();
    await expect(table.locator('th:has-text("Actions")')).toBeVisible();
  });

  test('should delete a note', async ({ page }) => {
    // First create a note
    await page.goto('/notes/new');
    await page.fill('input[name="title"]', 'Note to Delete');
    await page.click('button[type="submit"]');
    await page.waitForTimeout(1000);

    // Verify it exists
    await expect(page.locator('table')).toContainText('Note to Delete');

    // Delete it
    const row = page.locator('tr:has-text("Note to Delete")');
    await row.locator('button:has-text("Delete")').click();
    await page.waitForTimeout(1000);

    // Verify it's removed
    await expect(page.locator('table')).not.toContainText('Note to Delete');
  });

  test('should navigate to note form and back', async ({ page }) => {
    await page.goto('/notes');

    await page.click('a[href="/notes/new"]');
    await expect(page.locator('h2')).toContainText('Add New Note');

    await page.click('a[href="/notes"]');
    await expect(page.locator('h2')).toContainText('Notes');
  });

  test('should validate required title field (INV-INT-002)', async ({ page }) => {
    await page.goto('/notes/new');

    // Try to submit without title
    await page.click('button[type="submit"]');

    const titleInput = page.locator('input[name="title"]');
    const isInvalid = await titleInput.evaluate((el) => !el.validity.valid);
    expect(isInvalid).toBe(true);
  });

  test('should create note with rich text content', async ({ page }) => {
    await page.goto('/notes/new');

    await page.fill('input[name="title"]', 'Meeting Summary');
    await page.fill('textarea[name="body_v2"]', `
# Meeting Notes

## Attendees
- Alice
- Bob
- Carol

## Discussion Points
1. Project timeline
2. Budget allocation
3. Resource requirements

## Action Items
- [ ] Review proposal
- [ ] Schedule follow-up
    `.trim());

    await page.click('button[type="submit"]');
    await page.waitForTimeout(1000);

    await expect(page.locator('table')).toContainText('Meeting Summary');
  });

  test('should display creation timestamp', async ({ page }) => {
    await page.goto('/notes/new');

    await page.fill('input[name="title"]', 'Timestamped Note');
    await page.click('button[type="submit"]');
    await page.waitForTimeout(1000);

    const row = page.locator('tr:has-text("Timestamped Note")');

    // Check that created date is displayed (format: YYYY-MM-DD HH:MM)
    const rowText = await row.textContent();
    expect(rowText).toMatch(/\d{4}-\d{2}-\d{2}\s+\d{2}:\d{2}/);
  });

  test('should create multiple notes', async ({ page }) => {
    const notesTitles = [
      'First Note',
      'Second Note',
      'Third Note'
    ];

    for (const title of notesTitles) {
      await page.goto('/notes/new');
      await page.fill('input[name="title"]', title);
      await page.click('button[type="submit"]');
      await page.waitForTimeout(500);
    }

    // Verify all notes are listed
    for (const title of notesTitles) {
      await expect(page.locator('table')).toContainText(title);
    }
  });

  test('should handle long note titles', async ({ page }) => {
    await page.goto('/notes/new');

    const longTitle = 'This is a very long note title that contains a lot of text to test how the UI handles longer titles in the table view';
    await page.fill('input[name="title"]', longTitle);
    await page.click('button[type="submit"]');
    await page.waitForTimeout(1000);

    await expect(page.locator('table')).toContainText(longTitle);
  });

  test('should handle special characters in title', async ({ page }) => {
    await page.goto('/notes/new');

    const specialTitle = 'Note with "quotes" & <special> characters!';
    await page.fill('input[name="title"]', specialTitle);
    await page.click('button[type="submit"]');
    await page.waitForTimeout(1000);

    // Title should be properly escaped and displayed
    await expect(page.locator('table')).toContainText('special');
  });
});
