const { test, expect } = require('@playwright/test');
const axios = require('axios');

const API_BASE_URL = 'http://localhost:3001';

test.describe('Lead Management System', () => {
  let createdLeadId;

  test('should create a lead via API', async () => {
    const response = await axios.post(`${API_BASE_URL}/api/leads`, {
      first_name: 'John',
      last_name: 'Doe',
      email: 'john.doe@example.com',
      phone: '+1234567890',
      company_name: 'Acme Corp',
      job_title: 'CTO',
      source: 'manual_entry',
      notes: 'Met at tech conference'
    });

    expect(response.status).toBe(201);
    expect(response.data).toHaveProperty('id');
    expect(response.data.first_name).toBe('John');
    expect(response.data.last_name).toBe('Doe');
    expect(response.data.email).toBe('john.doe@example.com');
    expect(response.data.status).toBe('New');
    expect(response.data.score).toBeGreaterThan(0); // Should have calculated score

    createdLeadId = response.data.id;
  });

  test('should calculate lead score correctly', async () => {
    // Create lead with email only (score = 10)
    const response1 = await axios.post(`${API_BASE_URL}/api/leads`, {
      first_name: 'Jane',
      last_name: 'Smith',
      email: 'jane.smith@example.com',
      source: 'web_form'
    });
    expect(response1.data.score).toBe(10);

    // Create lead with email + phone (score = 10 + 30 = 40)
    const response2 = await axios.post(`${API_BASE_URL}/api/leads`, {
      first_name: 'Bob',
      last_name: 'Johnson',
      email: 'bob.johnson@example.com',
      phone: '+9876543210',
      source: 'email'
    });
    expect(response2.data.score).toBe(40);

    // Create lead with email + phone + company (score = 10 + 30 + 20 = 60)
    const response3 = await axios.post(`${API_BASE_URL}/api/leads`, {
      first_name: 'Alice',
      last_name: 'Williams',
      email: 'alice.williams@example.com',
      phone: '+1122334455',
      company_name: 'Tech Industries',
      source: 'referral'
    });
    expect(response3.data.score).toBe(60);

    // Create lead with all fields (score = 10 + 30 + 20 + 15 = 75)
    const response4 = await axios.post(`${API_BASE_URL}/api/leads`, {
      first_name: 'Charlie',
      last_name: 'Brown',
      email: 'charlie.brown@example.com',
      phone: '+5566778899',
      company_name: 'Innovation Inc',
      job_title: 'VP of Sales',
      source: 'web_form'
    });
    expect(response4.data.score).toBe(75);
  });

  test('should list all leads', async () => {
    const response = await axios.get(`${API_BASE_URL}/api/leads`);

    expect(response.status).toBe(200);
    expect(Array.isArray(response.data)).toBe(true);
    expect(response.data.length).toBeGreaterThan(0);
  });

  test('should get a specific lead', async () => {
    const response = await axios.get(`${API_BASE_URL}/api/leads/${createdLeadId}`);

    expect(response.status).toBe(200);
    expect(response.data.id).toBe(createdLeadId);
    expect(response.data.first_name).toBe('John');
    expect(response.data.last_name).toBe('Doe');
  });

  test('should update lead status', async () => {
    // Update to Contacted
    const response1 = await axios.put(
      `${API_BASE_URL}/api/leads/${createdLeadId}/status`,
      { status: 'contacted' }
    );

    expect(response1.status).toBe(200);
    expect(response1.data.status).toBe('Contacted');
    expect(response1.data.last_contacted_at).not.toBeNull();

    // Update to Qualified
    const response2 = await axios.put(
      `${API_BASE_URL}/api/leads/${createdLeadId}/status`,
      { status: 'qualified' }
    );

    expect(response2.status).toBe(200);
    expect(response2.data.status).toBe('Qualified');
  });

  test('should create lead via webhook', async () => {
    const response = await axios.post(`${API_BASE_URL}/webhooks/lead-capture`, {
      first_name: 'Webhook',
      last_name: 'Test',
      email: 'webhook.test@example.com',
      phone: '+1111222233',
      company_name: 'Web Company',
      job_title: 'Marketing Director'
    });

    expect(response.status).toBe(201);
    expect(response.data).toHaveProperty('id');
    expect(response.data.source).toBe('WebForm'); // Webhook always uses WebForm source
    expect(response.data.first_name).toBe('Webhook');
  });

  test('should prevent duplicate email addresses', async () => {
    try {
      await axios.post(`${API_BASE_URL}/api/leads`, {
        first_name: 'Duplicate',
        last_name: 'Test',
        email: 'john.doe@example.com', // Same email as first test
        source: 'manual_entry'
      });
      // Should not reach here
      expect(true).toBe(false);
    } catch (error) {
      expect(error.response.status).toBe(500);
      expect(error.response.data.error).toContain('Email already exists');
    }
  });

  test('should convert lead to Person only', async () => {
    // Create a fresh lead for conversion
    const leadResponse = await axios.post(`${API_BASE_URL}/api/leads`, {
      first_name: 'Convert',
      last_name: 'Person',
      email: 'convert.person@example.com',
      source: 'manual_entry'
    });

    const leadId = leadResponse.data.id;

    // Convert to Person only
    const convertResponse = await axios.post(
      `${API_BASE_URL}/api/leads/${leadId}/convert`,
      {
        create_person: true,
        create_company: false,
        create_opportunity: false
      }
    );

    expect(convertResponse.status).toBe(200);
    expect(convertResponse.data.lead.status).toBe('Converted');
    expect(convertResponse.data.person_id).not.toBeNull();
    expect(convertResponse.data.company_id).toBeNull();
    expect(convertResponse.data.opportunity_id).toBeNull();
  });

  test('should convert lead to Person + Company', async () => {
    // Create a lead with company info
    const leadResponse = await axios.post(`${API_BASE_URL}/api/leads`, {
      first_name: 'Convert',
      last_name: 'Company',
      email: 'convert.company@example.com',
      company_name: 'Test Company Inc',
      source: 'web_form'
    });

    const leadId = leadResponse.data.id;

    // Convert to Person + Company
    const convertResponse = await axios.post(
      `${API_BASE_URL}/api/leads/${leadId}/convert`,
      {
        create_person: true,
        create_company: true,
        create_opportunity: false
      }
    );

    expect(convertResponse.status).toBe(200);
    expect(convertResponse.data.lead.status).toBe('Converted');
    expect(convertResponse.data.person_id).not.toBeNull();
    expect(convertResponse.data.company_id).not.toBeNull();
    expect(convertResponse.data.opportunity_id).toBeNull();
  });

  test('should convert lead to Person + Company + Opportunity', async () => {
    // Create a lead with all info
    const leadResponse = await axios.post(`${API_BASE_URL}/api/leads`, {
      first_name: 'Convert',
      last_name: 'Opportunity',
      email: 'convert.opportunity@example.com',
      phone: '+9998887777',
      company_name: 'Big Deal Corp',
      job_title: 'CEO',
      source: 'referral'
    });

    const leadId = leadResponse.data.id;

    // Convert to full opportunity
    const convertResponse = await axios.post(
      `${API_BASE_URL}/api/leads/${leadId}/convert`,
      {
        create_person: true,
        create_company: true,
        create_opportunity: true,
        opportunity_name: 'Big Enterprise Deal',
        opportunity_amount: 100000000000 // 100k in micros
      }
    );

    expect(convertResponse.status).toBe(200);
    expect(convertResponse.data.lead.status).toBe('Converted');
    expect(convertResponse.data.person_id).not.toBeNull();
    expect(convertResponse.data.company_id).not.toBeNull();
    expect(convertResponse.data.opportunity_id).not.toBeNull();
  });

  test('should not allow status change on converted lead', async () => {
    // Get any converted lead
    const leadsResponse = await axios.get(`${API_BASE_URL}/api/leads`);
    const convertedLead = leadsResponse.data.find(lead => lead.status === 'Converted');

    if (convertedLead) {
      try {
        await axios.put(
          `${API_BASE_URL}/api/leads/${convertedLead.id}/status`,
          { status: 'qualified' }
        );
        // Should not reach here
        expect(true).toBe(false);
      } catch (error) {
        expect(error.response.status).toBe(500);
        expect(error.response.data.error).toContain('Cannot change status of converted lead');
      }
    }
  });

  test('should delete a lead (soft delete)', async () => {
    // Create a lead to delete
    const leadResponse = await axios.post(`${API_BASE_URL}/api/leads`, {
      first_name: 'Delete',
      last_name: 'Me',
      email: 'delete.me@example.com',
      source: 'manual_entry'
    });

    const leadId = leadResponse.data.id;

    // Delete the lead
    const deleteResponse = await axios.delete(`${API_BASE_URL}/api/leads/${leadId}`);
    expect(deleteResponse.status).toBe(204);

    // Try to get the deleted lead - should return 404
    try {
      await axios.get(`${API_BASE_URL}/api/leads/${leadId}`);
      expect(true).toBe(false); // Should not reach here
    } catch (error) {
      expect(error.response.status).toBe(404);
    }
  });

  test('should validate required fields', async () => {
    // Missing first_name
    try {
      await axios.post(`${API_BASE_URL}/api/leads`, {
        last_name: 'Doe',
        email: 'test@example.com',
        source: 'manual_entry'
      });
      expect(true).toBe(false);
    } catch (error) {
      expect(error.response.status).toBe(500);
      expect(error.response.data.error).toContain('First name cannot be empty');
    }

    // Invalid email format
    try {
      await axios.post(`${API_BASE_URL}/api/leads`, {
        first_name: 'John',
        last_name: 'Doe',
        email: 'invalid-email',
        source: 'manual_entry'
      });
      expect(true).toBe(false);
    } catch (error) {
      expect(error.response.status).toBe(500);
      expect(error.response.data.error).toContain('Invalid email');
    }
  });
});
