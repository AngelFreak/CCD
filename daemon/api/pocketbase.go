package api

import (
	"bytes"
	"encoding/json"
	"fmt"
	"net/http"

	"github.com/angelfreak/ccd/daemon/extractor"
)

type Client struct {
	baseURL string
	client  *http.Client
}

type Project struct {
	ID       string `json:"id"`
	Name     string `json:"name"`
	RepoPath string `json:"repo_path"`
}

func NewClient(baseURL string) *Client {
	return &Client{
		baseURL: baseURL,
		client:  &http.Client{},
	}
}

func (c *Client) VerifyProject(projectID string) error {
	_, err := c.GetProject(projectID)
	return err
}

func (c *Client) GetProject(projectID string) (*Project, error) {
	url := fmt.Sprintf("%s/api/collections/projects/records/%s", c.baseURL, projectID)
	resp, err := c.client.Get(url)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		return nil, fmt.Errorf("project not found: %s", projectID)
	}

	var project Project
	if err := json.NewDecoder(resp.Body).Decode(&project); err != nil {
		return nil, err
	}

	return &project, nil
}

func (c *Client) CreateFact(projectID string, fact extractor.Fact) error {
	url := fmt.Sprintf("%s/api/collections/extracted_facts/records", c.baseURL)

	data := map[string]interface{}{
		"project":    projectID,
		"fact_type":  fact.Type,
		"content":    fact.Content,
		"importance": fact.Importance,
		"stale":      false,
	}

	jsonData, err := json.Marshal(data)
	if err != nil {
		return err
	}

	resp, err := c.client.Post(url, "application/json", bytes.NewBuffer(jsonData))
	if err != nil {
		return err
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK && resp.StatusCode != http.StatusCreated {
		return fmt.Errorf("failed to create fact: status %d", resp.StatusCode)
	}

	return nil
}

func (c *Client) CreateSession(projectID, summary string, tokenCount int) error {
	url := fmt.Sprintf("%s/api/collections/session_history/records", c.baseURL)

	data := map[string]interface{}{
		"project":       projectID,
		"summary":       summary,
		"token_count":   tokenCount,
		"session_start": "now",
	}

	jsonData, err := json.Marshal(data)
	if err != nil {
		return err
	}

	resp, err := c.client.Post(url, "application/json", bytes.NewBuffer(jsonData))
	if err != nil {
		return err
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK && resp.StatusCode != http.StatusCreated {
		return fmt.Errorf("failed to create session: status %d", resp.StatusCode)
	}

	return nil
}

func (c *Client) UpdateFactStale(factID string, stale bool) error {
	url := fmt.Sprintf("%s/api/collections/extracted_facts/records/%s", c.baseURL, factID)

	data := map[string]interface{}{
		"stale": stale,
	}

	jsonData, err := json.Marshal(data)
	if err != nil {
		return err
	}

	req, err := http.NewRequest(http.MethodPatch, url, bytes.NewBuffer(jsonData))
	if err != nil {
		return err
	}
	req.Header.Set("Content-Type", "application/json")

	resp, err := c.client.Do(req)
	if err != nil {
		return err
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		return fmt.Errorf("failed to update fact: status %d", resp.StatusCode)
	}

	return nil
}
