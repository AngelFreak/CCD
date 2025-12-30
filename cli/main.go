package main

import (
	"fmt"
	"os"

	"github.com/angelfreak/ccd/cli/commands"
	"github.com/spf13/cobra"
)

var (
	version = "0.1.0"
	pbURL   string
)

func main() {
	rootCmd := &cobra.Command{
		Use:   "cct",
		Short: "Claude Context Tracker CLI",
		Long:  `A command-line tool for managing Claude Code project contexts`,
	}

	// Global flags
	rootCmd.PersistentFlags().StringVar(&pbURL, "pb-url", "http://localhost:8090", "PocketBase URL")

	// Add commands
	rootCmd.AddCommand(commands.NewPullCommand(&pbURL))
	rootCmd.AddCommand(commands.NewPushCommand(&pbURL))
	rootCmd.AddCommand(commands.NewStatusCommand(&pbURL))
	rootCmd.AddCommand(commands.NewSwitchCommand(&pbURL))
	rootCmd.AddCommand(&cobra.Command{
		Use:   "version",
		Short: "Print version information",
		Run: func(cmd *cobra.Command, args []string) {
			fmt.Printf("cct version %s\n", version)
		},
	})

	if err := rootCmd.Execute(); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
}
