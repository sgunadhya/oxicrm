const { test, expect } = require('@playwright/test');
const axios = require('axios');

const API_BASE_URL = 'http://localhost:3001';

test.describe('Email System', () => {
  test('should create an email template', async () => {
    const response = await axios.post(`${API_BASE_URL}/api/email-templates`, {
      name: 'welcome_email',
      subject: 'Welcome to {{company_name}}!',
      body_text: 'Hello {{name}},\n\nWelcome to {{company_name}}. We are excited to have you on board!\n\nBest regards,\nThe Team',
      category: 'manual'
    });

    expect(response.status).toBe(201);
    expect(response.data).toHaveProperty('id');
    expect(response.data.name).toBe('welcome_email');
    expect(response.data.subject).toBe('Welcome to {{company_name}}!');
  });

  test('should list email templates', async () => {
    const response = await axios.get(`${API_BASE_URL}/api/email-templates`);

    expect(response.status).toBe(200);
    expect(Array.isArray(response.data)).toBe(true);
  });

  test('should send an email without template', async () => {
    const response = await axios.post(`${API_BASE_URL}/api/emails`, {
      from_email: 'sender@example.com',
      to_email: 'recipient@example.com',
      subject: 'Test Email',
      body_text: 'This is a test email sent via the API.'
    });

    expect(response.status).toBe(201);
    expect(response.data).toHaveProperty('id');
    expect(response.data.status).toMatch(/sent|pending/);
  });

  test('should send an email with template', async () => {
    // First create a template
    const templateResponse = await axios.post(`${API_BASE_URL}/api/email-templates`, {
      name: 'test_template_' + Date.now(),
      subject: 'Hello {{name}}',
      body_text: 'Dear {{name}}, this is a {{type}} message.',
      category: 'manual'
    });

    const templateId = templateResponse.data.id;

    // Send email using template
    const emailResponse = await axios.post(`${API_BASE_URL}/api/emails`, {
      from_email: 'sender@example.com',
      to_email: 'recipient@example.com',
      subject: 'Will be replaced by template',
      body_text: 'Will be replaced by template',
      template_id: templateId,
      template_variables: {
        name: 'John Doe',
        type: 'test'
      }
    });

    expect(emailResponse.status).toBe(201);
    expect(emailResponse.data).toHaveProperty('id');
    expect(emailResponse.data.status).toMatch(/sent|pending/);
  });

  test('should list emails', async () => {
    const response = await axios.get(`${API_BASE_URL}/api/emails`);

    expect(response.status).toBe(200);
    expect(Array.isArray(response.data)).toBe(true);
  });

  test('should retrieve a specific email', async () => {
    // First send an email
    const sendResponse = await axios.post(`${API_BASE_URL}/api/emails`, {
      from_email: 'sender@example.com',
      to_email: 'recipient@example.com',
      subject: 'Retrievable Email',
      body_text: 'This email will be retrieved.'
    });

    const emailId = sendResponse.data.id;

    // Retrieve the email
    const getResponse = await axios.get(`${API_BASE_URL}/api/emails/${emailId}`);

    expect(getResponse.status).toBe(200);
    expect(getResponse.data.id).toBe(emailId);
    expect(getResponse.data.subject).toBe('Retrievable Email');
  });

  test('should handle inbound email webhook', async () => {
    const response = await axios.post(`${API_BASE_URL}/webhooks/inbound-email`, {
      from_email: 'customer@example.com',
      to_email: 'support@oxicrm.com',
      subject: 'Inbound Test Email',
      body_text: 'This is an inbound email from a customer.'
    });

    expect(response.status).toBe(201);
    expect(response.data).toHaveProperty('id');
    expect(response.data.status).toBe('received');
  });

  test('should update an email template', async () => {
    // Create a template
    const createResponse = await axios.post(`${API_BASE_URL}/api/email-templates`, {
      name: 'updatable_template_' + Date.now(),
      subject: 'Original Subject',
      body_text: 'Original body',
      category: 'manual'
    });

    const templateId = createResponse.data.id;

    // Update the template
    const updateResponse = await axios.put(`${API_BASE_URL}/api/email-templates/${templateId}`, {
      subject: 'Updated Subject',
      body_text: 'Updated body text'
    });

    expect(updateResponse.status).toBe(200);
    expect(updateResponse.data.subject).toBe('Updated Subject');
    expect(updateResponse.data.body_text).toBe('Updated body text');
  });

  test('should delete an email template', async () => {
    // Create a template
    const createResponse = await axios.post(`${API_BASE_URL}/api/email-templates`, {
      name: 'deletable_template_' + Date.now(),
      subject: 'To Be Deleted',
      body_text: 'This template will be deleted',
      category: 'manual'
    });

    const templateId = createResponse.data.id;

    // Delete the template
    const deleteResponse = await axios.delete(`${API_BASE_URL}/api/email-templates/${templateId}`);

    expect(deleteResponse.status).toBe(204);

    // Verify deletion
    try {
      await axios.get(`${API_BASE_URL}/api/email-templates/${templateId}`);
      throw new Error('Template should have been deleted');
    } catch (error) {
      expect(error.response.status).toBe(404);
    }
  });

  test('should send email with CC and BCC', async () => {
    const response = await axios.post(`${API_BASE_URL}/api/emails`, {
      from_email: 'sender@example.com',
      to_email: 'recipient@example.com',
      cc_emails: ['cc1@example.com', 'cc2@example.com'],
      bcc_emails: ['bcc@example.com'],
      subject: 'Email with CC and BCC',
      body_text: 'This email has CC and BCC recipients.'
    });

    expect(response.status).toBe(201);
    expect(response.data).toHaveProperty('id');
  });

  test('should reject invalid email addresses', async () => {
    try {
      await axios.post(`${API_BASE_URL}/api/emails`, {
        from_email: 'invalid-email',
        to_email: 'also-invalid',
        subject: 'Test',
        body_text: 'Test'
      });
      throw new Error('Should have rejected invalid email');
    } catch (error) {
      expect(error.response.status).toBe(500);
    }
  });
});
