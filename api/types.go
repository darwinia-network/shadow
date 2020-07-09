package api

type ProposalParams struct {
	Headers  []uint64 `json:"headers"`
	LastLeaf uint64   `json:"last_leaf"`
}
