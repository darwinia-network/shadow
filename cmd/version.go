package cmd

import (
	"fmt"

	"github.com/spf13/cobra"
)

var cmdVersion = &cobra.Command{
	Use:   "version",
	Short: "Print the version number of dargo",
	Long:  `All software has versions. This is dargo's`,
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println("shadow 0.1.6")
	},
}
