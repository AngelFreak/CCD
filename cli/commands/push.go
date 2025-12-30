package commands

import (
	"bytes"
	"encoding/json"
	"fmt"
	"net/http"
	"time"

	"github.com/spf13/cobra"
)

func NewPushCommand(pbURL *string) *cobra.Command {
	cmd := &cobra.Command{
		Use:   "push <project-slug> <summary>",
		Short: "Save session summary",
		Args:  cobra.ExactArgs(2),
		RunE: func(cmd *cobra.Command, args []string) error {
			projectSlug := args[0]
			summary := args[1]
			return pushSession(*pbURL, projectSlug, summary)
		},
	}

	return cmd
}

func pushSession(pbURL, projectSlug, summary string) error {
	// Get project by slug
	url := fmt.Sprintf("%s/api/collections/projects/records?filter=slug='%s'", pbURL, projectSlug)
	resp, err := http.Get(url)
	if err != nil {
		return fmt.Errorf("failed to fetch project: %w", err)
	}
	defer resp.Body.Close()

	var result struct {
		Items []struct {
			ID string `json:"id"`
		} `json:"items"`
	}

	if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
		return err
	}

	if len(result.Items) == 0 {
		return fmt.Errorf("project not found: %s", projectSlug)
	}

	projectID := result.Items[0].ID

	// Create session
	url = fmt.Sprintf("%s/api/collections/session_history/records", pbURL)
	data := map[string]interface{}{
		"project":       projectID,
		"summary":       summary,
		"session_start": time.Now().Format(time.RFC3339),
		"session_end":   time.Now().Format(time.RFC3339),
	}

	jsonData, err := json.Marshal(data)
	if err != nil {
		return err
	}

	resp, err = http.Post(url, "application/json", bytes.NewBuffer(jsonData))
	if err != nil {
		return fmt.Errorf("failed to create session: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK && resp.StatusCode != http.StatusCreated {
		return fmt.Errorf("failed to create session: status %d", resp.StatusCode)
	}

	fmt.Println("✓ Session summary saved")
	return nil
}
