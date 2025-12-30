package commands

import (
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"os"
	"path/filepath"

	"github.com/spf13/cobra"
)

func NewStatusCommand(pbURL *string) *cobra.Command {
	cmd := &cobra.Command{
		Use:   "status",
		Short: "Show active project and session info",
		RunE: func(cmd *cobra.Command, args []string) error {
			return showStatus(*pbURL)
		},
	}

	return cmd
}

func showStatus(pbURL string) error {
	// Try to determine current project from git repo
	cwd, err := os.Getwd()
	if err != nil {
		return err
	}

	// Get all active projects
	url := fmt.Sprintf("%s/api/collections/projects/records?filter=status='active'&sort=-updated", pbURL)
	resp, err := http.Get(url)
	if err != nil {
		return fmt.Errorf("failed to fetch projects: %w", err)
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
			Status   string `json:"status"`
		} `json:"items"`
	}

	if err := json.Unmarshal(body, &result); err != nil {
		return err
	}

	if len(result.Items) == 0 {
		fmt.Println("No active projects")
		return nil
	}

	// Find project matching current directory
	var currentProject *struct {
		ID       string `json:"id"`
		Name     string `json:"name"`
		Slug     string `json:"slug"`
		RepoPath string `json:"repo_path"`
		Status   string `json:"status"`
	}

	for _, project := range result.Items {
		absPath, err := filepath.Abs(project.RepoPath)
		if err != nil {
			continue
		}
		if absPath == cwd || filepath.Dir(cwd) == absPath {
			currentProject = &project
			break
		}
	}

	if currentProject != nil {
		fmt.Printf("📂 Current Project: %s (%s)\n", currentProject.Name, currentProject.Slug)
		fmt.Printf("📍 Path: %s\n", currentProject.RepoPath)
		fmt.Printf("🟢 Status: %s\n", currentProject.Status)

		// Get latest session
		url = fmt.Sprintf("%s/api/collections/session_history/records?filter=project='%s'&sort=-created&limit=1", pbURL, currentProject.ID)
		resp, err = http.Get(url)
		if err == nil {
			defer resp.Body.Close()
			var sessions struct {
				Items []struct {
					Summary    string `json:"summary"`
					TokenCount int    `json:"token_count"`
					Created    string `json:"created"`
				} `json:"items"`
			}

			if err := json.NewDecoder(resp.Body).Decode(&sessions); err == nil && len(sessions.Items) > 0 {
				fmt.Printf("\n📝 Last Session:\n")
				fmt.Printf("   Summary: %s\n", sessions.Items[0].Summary)
				if sessions.Items[0].TokenCount > 0 {
					fmt.Printf("   Tokens: %d\n", sessions.Items[0].TokenCount)
				}
			}
		}
	} else {
		fmt.Println("📂 No project matching current directory")
		fmt.Printf("\nActive Projects:\n")
		for _, project := range result.Items {
			fmt.Printf("  • %s (%s)\n", project.Name, project.Slug)
		}
	}

	return nil
}
