package eth

// The post api of fetching eth header
const (
	GETBLOCK        = "{\"jsonrpc\":\"2.0\",\"method\":\"eth_getBlockByNumber\",\"params\": [\"0x%x\", false],\"id\":1}\n"
	GETBLOCK_BYHASH = "{\"jsonrpc\":\"2.0\",\"method\":\"eth_getBlockByHash\",\"params\": [\"%s\", false],\"id\":1}\n"

	// uncle block
	GET_UNCLE_BLOCK = "{\"jsonrpc\":\"2.0\",\"method\":\"eth_getUncleByBlockNumberAndIndex\",\"params\": [\"0x%x\", \"0x0\"],\"id\":1}\n"
)
