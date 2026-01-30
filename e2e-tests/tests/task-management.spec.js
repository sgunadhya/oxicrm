// @ts-check
const { test, expect } = require('@playwright/test');

test.describe('Task Management', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
  });

  test('should display tasks page', async ({ page }) => {
    await page.goto('/tasks');

    await expect(page.locator('h2')).toContainText('Tasks');
    await expect(page.locator('a[href="/tasks/new"]')).toBeVisible();
  });

  test('should display task creation form', async ({ page }) => {
    await page.goto('/tasks/new');

    await expect(page.locator('h2')).toContainText('Add New Task');

    // Verify form fields
    await expect(page.locator('input[name="title"]')).toBeVisible();
    await expect(page.locator('textarea[name="body"]')).toBeVisible();
    await expect(page.locator('select[name="status"]')).toBeVisible();
    await expect(page.locator('input[name="due_at"]')).toBeVisible();

    await expect(page.locator('button[type="submit"]')).toContainText('Save');
    await expect(page.locator('a[href="/tasks"]')).toContainText('Cancel');
  });

  test('should verify status options', async ({ page }) => {
    await page.goto('/tasks/new');

    const statusSelect = page.locator('select[name="status"]');

    // Verify all status options exist
    await expect(statusSelect.locator('option[value="TODO"]')).toHaveCount(1);
    await expect(statusSelect.locator('option[value="IN_PROGRESS"]')).toHaveCount(1);
    await expect(statusSelect.locator('option[value="DONE"]')).toHaveCount(1);
  });

  test('should create a new task', async ({ page }) => {
    await page.goto('/tasks/new');

    await page.fill('input[name="title"]', 'Follow up with client');
    await page.fill('textarea[name="body"]', 'Need to discuss Q1 requirements');
    await page.selectOption('select[name="status"]', 'TODO');

    await page.click('button[type="submit"]');
    await page.waitForTimeout(1000);

    await expect(page.locator('h2')).toContainText('Tasks');
    await expect(page.locator('table')).toContainText('Follow up with client');
    await expect(page.locator('table')).toContainText('TODO');
  });

  test('should create task with due date', async ({ page }) => {
    await page.goto('/tasks/new');

    await page.fill('input[name="title"]', 'Scheduled Task');
    await page.fill('input[name="due_at"]', '2026-03-15T14:30');
    await page.selectOption('select[name="status"]', 'TODO');

    await page.click('button[type="submit"]');
    await page.waitForTimeout(1000);

    await expect(page.locator('table')).toContainText('Scheduled Task');
    await expect(page.locator('table')).toContainText('2026-03-15');
  });

  test('should display tasks in table format', async ({ page }) => {
    await page.goto('/tasks');

    const table = page.locator('table');
    await expect(table.locator('th:has-text("Title")')).toBeVisible();
    await expect(table.locator('th:has-text("Status")')).toBeVisible();
    await expect(table.locator('th:has-text("Due Date")')).toBeVisible();
    await expect(table.locator('th:has-text("Actions")')).toBeVisible();
  });

  test('should delete a task', async ({ page }) => {
    // First create a task
    await page.goto('/tasks/new');
    await page.fill('input[name="title"]', 'Task to Delete');
    await page.click('button[type="submit"]');
    await page.waitForTimeout(1000);

    // Verify it exists
    await expect(page.locator('table')).toContainText('Task to Delete');

    // Delete it
    const row = page.locator('tr:has-text("Task to Delete")');
    await row.locator('button:has-text("Delete")').click();
    await page.waitForTimeout(1000);

    // Verify it's removed
    await expect(page.locator('table')).not.toContainText('Task to Delete');
  });

  test('should navigate to task form and back', async ({ page }) => {
    await page.goto('/tasks');

    await page.click('a[href="/tasks/new"]');
    await expect(page.locator('h2')).toContainText('Add New Task');

    await page.click('a[href="/tasks"]');
    await expect(page.locator('h2')).toContainText('Tasks');
  });

  test('should validate required title field', async ({ page }) => {
    await page.goto('/tasks/new');

    // Try to submit without title
    await page.click('button[type="submit"]');

    const titleInput = page.locator('input[name="title"]');
    const isInvalid = await titleInput.evaluate((el) => !el.validity.valid);
    expect(isInvalid).toBe(true);
  });

  test('should create task with different statuses', async ({ page }) => {
    // Test TODO status
    await page.goto('/tasks/new');
    await page.fill('input[name="title"]', 'TODO Task');
    await page.selectOption('select[name="status"]', 'TODO');
    await page.click('button[type="submit"]');
    await page.waitForTimeout(500);

    // Test IN_PROGRESS status
    await page.goto('/tasks/new');
    await page.fill('input[name="title"]', 'In Progress Task');
    await page.selectOption('select[name="status"]', 'IN_PROGRESS');
    await page.click('button[type="submit"]');
    await page.waitForTimeout(500);

    // Test DONE status
    await page.goto('/tasks/new');
    await page.fill('input[name="title"]', 'Completed Task');
    await page.selectOption('select[name="status"]', 'DONE');
    await page.click('button[type="submit"]');
    await page.waitForTimeout(500);

    // Verify all tasks are created with correct statuses
    await expect(page.locator('table')).toContainText('TODO Task');
    await expect(page.locator('table')).toContainText('In Progress Task');
    await expect(page.locator('table')).toContainText('Completed Task');

    // Verify status labels
    const tableContent = await page.locator('table').textContent();
    expect(tableContent).toContain('TODO');
    expect(tableContent).toContain('IN PROGRESS');
    expect(tableContent).toContain('DONE');
  });

  test('should create task with full details', async ({ page }) => {
    await page.goto('/tasks/new');

    await page.fill('input[name="title"]', 'Complete Project Documentation');
    await page.fill('textarea[name="body"]', 'Write comprehensive documentation for the new API endpoints');
    await page.selectOption('select[name="status"]', 'IN_PROGRESS');
    await page.fill('input[name="due_at"]', '2026-04-20T17:00');

    await page.click('button[type="submit"]');
    await page.waitForTimeout(1000);

    await expect(page.locator('table')).toContainText('Complete Project Documentation');
    await expect(page.locator('table')).toContainText('IN PROGRESS');
    await expect(page.locator('table')).toContainText('2026-04-20');
  });
});
