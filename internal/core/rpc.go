package core

type ShadowRPC struct {
	Core Shadow
}

/**
 * Get Eth Header By Hash
 */
func (s *ShadowRPC) GetEthHeaderByHash(
	params GetEthHeaderByHashParams,
	resp *GetEthHeaderResp,
) error {
	var err error
	resp.Header, err = s.Core.GetHeader(Ethereum, params.Hash)
	return err
}

/**
 * Get Eth Header By Number
 */
func (s *ShadowRPC) GetEthHeaderByNumber(
	params GetEthHeaderByNumberParams,
	resp *GetEthHeaderResp,
) error {
	var err error
	resp.Header, err = s.Core.GetHeader(Ethereum, params.Number)
	return err
}

/**
 * GetEthHeaderWithProofByNumber
 */
func (s *ShadowRPC) GetEthHeaderWithProofByNumber(
	params GetEthHeaderWithProofByNumberParams,
	resp *interface{},
) error {
	var err error
	*resp, err = s.Core.GetHeaderWithProof(
		Ethereum,
		params.Number,
		new(ProofFormat).From(params.Options.Format),
	)
	return err
}

/**
 * GetEthHeaderWithProofByHash
 */
func (s *ShadowRPC) GetEthHeaderWithProofByHash(
	params GetEthHeaderWithProofByHashParams,
	resp *interface{},
) error {
	var err error
	*resp, err = s.Core.GetHeaderWithProof(
		Ethereum,
		params.Hash,
		new(ProofFormat).From(params.Options.Format),
	)
	return err
}

/**
 * BatchEthHeaderWithProofByNumber
 */
func (s *ShadowRPC) BatchEthHeaderWithProofByNumber(
	params BatchEthHeaderWithProofByNumberParams,
	resp *interface{},
) error {
	var err error
	*resp, err = s.Core.BatchHeaderWithProof(
		params.Number,
		params.Batch,
		new(ProofFormat).From(params.Options.Format),
	)

	return err
}

/**
 * BatchEthHeaderWithProofByNumber
 */
func (s *ShadowRPC) GetProposalEthHeaders(
	params GetProposalEthHeadersParams,
	resp *interface{},
) error {
	var err error
	*resp, err = s.Core.GetProposalHeaders(
		params.Numbers,
		new(ProofFormat).From(params.Options.Format),
	)

	return err
}
