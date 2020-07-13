package core

type ShadowRPC struct {
	Shadow Shadow
}

func NewShadowRPC() (ShadowRPC, error) {
	shadow, err := NewShadow()
	return ShadowRPC{
		Shadow: shadow,
	}, err
}

/**
 * Get Eth Header By Hash
 */
func (s *ShadowRPC) GetEthHeaderByHash(
	params GetEthHeaderByHashParams,
	resp *GetEthHeaderResp,
) error {
	var err error
	resp.Header, err = s.Shadow.GetHeader(Ethereum, params.Hash)
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
	resp.Header, err = s.Shadow.GetHeader(Ethereum, params.Number)
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
	*resp, err = s.Shadow.GetHeaderWithProof(
		Ethereum,
		params.Number,
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
	*resp, err = s.Shadow.GetHeaderWithProof(
		Ethereum,
		params.Hash,
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
	*resp, err = s.Shadow.BatchHeaderWithProof(
		params.Number,
		params.Batch,
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
	*resp, err = s.Shadow.GetProposalHeaders(params.Numbers)
	return err
}
