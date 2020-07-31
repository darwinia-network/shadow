package cmd

import (
	"encoding/json"
	"fmt"

	"github.com/darwinia-network/shadow/internal"
	"github.com/darwinia-network/shadow/internal/core"
	"github.com/darwinia-network/shadow/mock"
	"github.com/spf13/cobra"
)

func init() {
	cmdTest.PersistentFlags().BoolVarP(
		&VERBOSE,
		"verbose",
		"v",
		false,
		"Enable all shadow logs",
	)
}

type Codec struct {
	Headers []string `json:"headers"`
}

var cmdTest = &cobra.Command{
	Use:   "test",
	Short: "Test command",
	Long:  "This command is for tests, will not port in production",
	Run: func(cmd *cobra.Command, args []string) {
		verboseCheck()
		conf := internal.Config{}
		_ = conf.Load()

		shadow, _ := core.NewShadow()
		members := []uint64{}
		for i := 0; i < 19; i++ {
			members = append(members, uint64(i))
		}

		var res []string = []string{}
		headers, _ := shadow.GetProposalHeaders(members)
		for idx, header := range headers {
			header.Root = mock.ROOTS[idx][2:]
			res = append(res, header.IntoProposalCodecWithExProof(mock.PROOFS[idx]))
		}

		h, _ := mock.Proposal(uint64(21), conf)
		h.Root = mock.ROOTS[19][2:]

		arr, _ := json.Marshal(
			Codec{
				Headers: append(
					res,
					h.IntoProposalCodecWithExProof(mock.PROOFS[19]),
				),
			},
		)
		fmt.Println(string(arr))
	},
}
