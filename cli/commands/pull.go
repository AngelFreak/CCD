package commands

import (
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"os"

	"github.com/spf13/cobra"
)

func NewPullCommand(pbURL *string) *cobra.Command {
	var output string

	cmd := &cobra.Command{
		Use:   "pull <project-slug>",
		Short: "Pull project context and write to CLAUDE.md",
		Args:  cobra.ExactArgs(1),
		RunE: func(cmd *cobra.Command, args []string) error {
			projectSlug := args[0]
			return pullContext(*pbURL, projectSlug, output)
		},
	}

	cmd.Flags().StringVarP(&output, "output", "o", "CLAUDE.md", "Output file")

	return cmd
}

func pullContext(pbURL, projectSlug, output string) error {
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
			ID          string   `json:"id"`
			Name        string   `json:"name"`
			Slug        string   `json:"slug"`
			RepoPath    string   `json:"repo_path"`
			Status      string   `json:"status"`
			Priority    int      `json:"priority"`
			TechStack   []string `json:"tech_stack"`
			Description string   `json:"description"`
		} `json:"items"`
	}

	if err := json.Unmarshal(body, &result); err != nil {
		return err
	}

	if len(result.Items) == 0 {
		return fmt.Errorf("project not found: %s", projectSlug)
	}

	project := result.Items[0]

	// Get context sections
	url = fmt.Sprintf("%s/api/collections/context_sections/records?filter=project='%s'&sort=order", pbURL, project.ID)
	resp, err = http.Get(url)
	if err != nil {
		return fmt.Errorf("failed to fetch context sections: %w", err)
	}
	defer resp.Body.Close()

	body, err = io.ReadAll(resp.Body)
	if err != nil {
		return err
	}

	var sections struct {
		Items []struct {
			Title   string `json:"title"`
			Content string `json:"content"`
		} `json:"items"`
	}

	if err := json.Unmarshal(body, &sections); err != nil {
		return err
	}

	// Generate markdown
	markdown := fmt.Sprintf("# %s\n\n", project.Name)

	if project.Description != "" {
		markdown += fmt.Sprintf("%s\n\n", project.Description)
	}

	markdown += "## Project Info\n"
	markdown += fmt.Sprintf("- **Status**: %s\n", project.Status)
	markdown += fmt.Sprintf("- **Priority**: %d\n", project.Priority)
	markdown += fmt.Sprintf("- **Repo Path**: %s\n", project.RepoPath)

	if len(project.TechStack) > 0 {
		markdown += fmt.Sprintf("- **Tech Stack**: %s\n", joinStrings(project.TechStack, ", "))
	}

	markdown += "\n"

	for _, section := range sections.Items {
		markdown += fmt.Sprintf("## %s\n\n%s\n\n", section.Title, section.Content)
	}

	// Write to file
	if err := os.WriteFile(output, []byte(markdown), 0644); err != nil {
		return fmt.Errorf("failed to write file: %w", err)
	}

	fmt.Printf("✓ Context written to %s\n", output)
	return nil
}

func joinStrings(strs []string, sep string) string {
	result := ""
	for i, s := range strs {
		if i > 0 {
			result += sep
		}
		result += s
	}
	return result
}
