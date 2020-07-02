package core

// chain
type Chain string

const (
	Ethereum Chain = "Ethereum"
)

// proof
type ProofFormat string

const (
	JsonFormat  ProofFormat = "json"
	ScaleFormat ProofFormat = "scale"
	RawFormat   ProofFormat = "raw"
)

func (f *ProofFormat) From(s string) ProofFormat {
	switch s {
	case "json":
		return JsonFormat
	case "scale":
		return ScaleFormat
	default:
		return RawFormat
	}
}
