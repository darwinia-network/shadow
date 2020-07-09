package api

type ProposalParams struct {
	Members  []uint64 `json:"members"`
	LastLeaf uint64   `json:"last_leaf"`
}
