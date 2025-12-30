package commands

import (
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"time"

	"github.com/spf13/cobra"
)

func NewDiffCommand(pbURL *string) *cobra.Command {
	var count int

	cmd := &cobra.Command{
		Use:   "diff <project-slug>",
		Short: "Show differences between recent sessions",
		Args:  cobra.ExactArgs(1),
		RunE: func(cmd *cobra.Command, args []string) error {
			projectSlug := args[0]
			return showDiff(*pbURL, projectSlug, count)
		},
	}

	cmd.Flags().IntVarP(&count, "count", "n", 5, "Number of sessions to compare")

	return cmd
}

func showDiff(pbURL, projectSlug string, count int) error {
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
			ID string `json:"id"`
		} `json:"items"`
	}

	if err := json.Unmarshal(body, &result); err != nil {
		return err
	}

	if len(result.Items) == 0 {
		return fmt.Errorf("project not found: %s", projectSlug)
	}

	projectID := result.Items[0].ID

	// Get session history
	url = fmt.Sprintf("%s/api/collections/session_history/records?filter=project='%s'&sort=-created&limit=%d",
		pbURL, projectID, count)
	resp, err = http.Get(url)
	if err != nil {
		return fmt.Errorf("failed to fetch sessions: %w", err)
	}
	defer resp.Body.Close()

	body, err = io.ReadAll(resp.Body)
	if err != nil {
		return err
	}

	var sessions struct {
		Items []struct {
			ID         string `json:"id"`
			Summary    string `json:"summary"`
			TokenCount int    `json:"token_count"`
			Created    string `json:"created"`
		} `json:"items"`
	}

	if err := json.Unmarshal(body, &sessions); err != nil {
		return err
	}

	if len(sessions.Items) == 0 {
		fmt.Println("No session history found")
		return nil
	}

	fmt.Printf("📊 Session Diff for %s\n\n", projectSlug)

	// Calculate and display diffs
	for i := 1; i < len(sessions.Items); i++ {
		current := sessions.Items[i-1]
		previous := sessions.Items[i]

		tokenDelta := current.TokenCount - previous.TokenCount

		fmt.Printf("Session: %s\n", formatTime(current.Created))
		fmt.Printf("Summary: %s\n", current.Summary)

		if tokenDelta > 0 {
			fmt.Printf("Tokens:  +%d (increased)\n", tokenDelta)
		} else if tokenDelta < 0 {
			fmt.Printf("Tokens:  %d (decreased)\n", tokenDelta)
		} else {
			fmt.Printf("Tokens:  no change\n")
		}

		fmt.Println()
	}

	return nil
}

func formatTime(timeStr string) string {
	t, err := time.Parse(time.RFC3339, timeStr)
	if err != nil {
		return timeStr
	}
	return t.Format("Jan 2, 2006 3:04 PM")
}
