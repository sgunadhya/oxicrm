// @ts-check
const { test, expect } = require('@playwright/test');

test.describe('Calendar Event Management', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
  });

  test('should display calendar events page', async ({ page }) => {
    await page.goto('/calendar-events');

    await expect(page.locator('h2')).toContainText('Calendar Events');
    await expect(page.locator('a[href="/calendar-events/new"]')).toBeVisible();
  });

  test('should display calendar event creation form', async ({ page }) => {
    await page.goto('/calendar-events/new');

    await expect(page.locator('h2')).toContainText('Add New Calendar Event');

    // Verify form fields
    await expect(page.locator('input[name="title"]')).toBeVisible();
    await expect(page.locator('input[name="start_time"]')).toBeVisible();
    await expect(page.locator('input[name="end_time"]')).toBeVisible();
    await expect(page.locator('textarea[name="description"]')).toBeVisible();

    await expect(page.locator('button[type="submit"]')).toContainText('Save');
    await expect(page.locator('a[href="/calendar-events"]')).toContainText('Cancel');
  });

  test('should create a new calendar event', async ({ page }) => {
    await page.goto('/calendar-events/new');

    // Fill in all required fields
    await page.fill('input[name="title"]', 'Team Meeting');
    await page.fill('input[name="start_time"]', '2024-03-15T10:00');
    await page.fill('input[name="end_time"]', '2024-03-15T11:00');
    await page.fill('textarea[name="description"]', 'Weekly team sync');

    await page.click('button[type="submit"]');
    await page.waitForTimeout(1000);

    await expect(page.locator('h2')).toContainText('Calendar Events');
    await expect(page.locator('table')).toContainText('Team Meeting');
  });

  test('should display calendar events in table format', async ({ page }) => {
    await page.goto('/calendar-events');

    const table = page.locator('table');
    await expect(table.locator('th:has-text("Title")')).toBeVisible();
    await expect(table.locator('th:has-text("Start Time")')).toBeVisible();
    await expect(table.locator('th:has-text("End Time")')).toBeVisible();
    await expect(table.locator('th:has-text("Actions")')).toBeVisible();
  });

  test('should delete a calendar event', async ({ page }) => {
    // First create an event
    await page.goto('/calendar-events/new');
    await page.fill('input[name="title"]', 'Event to Delete');
    await page.fill('input[name="start_time"]', '2024-03-20T14:00');
    await page.fill('input[name="end_time"]', '2024-03-20T15:00');
    await page.click('button[type="submit"]');
    await page.waitForTimeout(1000);

    // Verify it exists
    await expect(page.locator('table')).toContainText('Event to Delete');

    // Delete it
    const row = page.locator('tr:has-text("Event to Delete")');
    await row.locator('button:has-text("Delete")').click();
    await page.waitForTimeout(1000);

    // Verify it's removed
    await expect(page.locator('table')).not.toContainText('Event to Delete');
  });

  test('should navigate to event form and back', async ({ page }) => {
    await page.goto('/calendar-events');

    await page.click('a[href="/calendar-events/new"]');
    await expect(page.locator('h2')).toContainText('Add New Calendar Event');

    await page.click('a[href="/calendar-events"]');
    await expect(page.locator('h2')).toContainText('Calendar Events');
  });

  test('should validate required fields', async ({ page }) => {
    await page.goto('/calendar-events/new');

    // Try to submit without required fields
    await page.click('button[type="submit"]');

    const titleInput = page.locator('input[name="title"]');
    const isTitleInvalid = await titleInput.evaluate((el) => !el.validity.valid);
    expect(isTitleInvalid).toBe(true);
  });

  test('should create multiple calendar events', async ({ page }) => {
    const events = [
      { title: 'Morning Standup', start: '2024-03-21T09:00', end: '2024-03-21T09:30' },
      { title: 'Client Call', start: '2024-03-21T14:00', end: '2024-03-21T15:00' },
      { title: 'Project Review', start: '2024-03-22T16:00', end: '2024-03-22T17:00' }
    ];

    for (const event of events) {
      await page.goto('/calendar-events/new');
      await page.fill('input[name="title"]', event.title);
      await page.fill('input[name="start_time"]', event.start);
      await page.fill('input[name="end_time"]', event.end);
      await page.click('button[type="submit"]');
      await page.waitForTimeout(500);
    }

    // Verify all events are listed
    for (const event of events) {
      await expect(page.locator('table')).toContainText(event.title);
    }
  });

  test('should display times correctly in table', async ({ page }) => {
    await page.goto('/calendar-events/new');

    await page.fill('input[name="title"]', 'Time Format Test');
    await page.fill('input[name="start_time"]', '2024-03-25T10:30');
    await page.fill('input[name="end_time"]', '2024-03-25T11:45');
    await page.click('button[type="submit"]');
    await page.waitForTimeout(1000);

    const row = page.locator('tr:has-text("Time Format Test")');

    // Check that times are displayed (format: YYYY-MM-DD HH:MM)
    const rowText = await row.textContent();
    expect(rowText).toMatch(/\d{4}-\d{2}-\d{2}\s+\d{2}:\d{2}/);
  });

  test('should create event with description', async ({ page }) => {
    await page.goto('/calendar-events/new');

    await page.fill('input[name="title"]', 'Described Event');
    await page.fill('input[name="start_time"]', '2024-03-26T13:00');
    await page.fill('input[name="end_time"]', '2024-03-26T14:00');
    await page.fill('textarea[name="description"]', 'This event has a detailed description');
    await page.click('button[type="submit"]');
    await page.waitForTimeout(1000);

    await expect(page.locator('table')).toContainText('Described Event');
  });
});
