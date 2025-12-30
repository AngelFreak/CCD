package commands

import (
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"os"

	"github.com/spf13/cobra"
)

func NewSwitchCommand(pbURL *string) *cobra.Command {
	cmd := &cobra.Command{
		Use:   "switch <project-slug>",
		Short: "Switch active project context",
		Args:  cobra.ExactArgs(1),
		RunE: func(cmd *cobra.Command, args []string) error {
			projectSlug := args[0]
			return switchProject(*pbURL, projectSlug)
		},
	}

	return cmd
}

func switchProject(pbURL, projectSlug string) error {
	// Get project by slug
	url := fmt.Sprintf("%s/api/collections/projects/records?filter=slug='%s'", pbURL, projectSlug)
	resp, err := http.Get(url)
	if err != nil {
		return fmt.Errorf("failed to fetch project: %w", err)
	}
	defer resp.Body.Close()

	body, err := io.ReadAll(resp.Body)
	if err != nil {
		return err
	}

	var result struct {
		Items []struct {
			ID       string `json:"id"`
			Name     string `json:"name"`
			Slug     string `json:"slug"`
			RepoPath string `json:"repo_path"`
		} `json:"items"`
	}

	if err := json.Unmarshal(body, &result); err != nil {
		return err
	}

	if len(result.Items) == 0 {
		return fmt.Errorf("project not found: %s", projectSlug)
	}

	project := result.Items[0]

	// Change to project directory
	if err := os.Chdir(project.RepoPath); err != nil {
		return fmt.Errorf("failed to change directory: %w", err)
	}

	// Pull context automatically
	if err := pullContext(pbURL, projectSlug, "CLAUDE.md"); err != nil {
		fmt.Printf("Warning: failed to pull context: %v\n", err)
	}

	fmt.Printf("✓ Switched to project: %s\n", project.Name)
	fmt.Printf("📍 Directory: %s\n", project.RepoPath)
	fmt.Printf("📄 Context written to CLAUDE.md\n")

	return nil
}
