package eth

import (
	"fmt"
	"strconv"
	"strings"
)

func (h *DarwiniaEthHeader) Ser() string {
	return fmt.Sprintf(
		"%s|%v|%v|%s|%s|%s|%s|%s|%s|%s|%v|%v|%v|%s|%s",
		h.ParentHash,
		h.TimeStamp,
		h.Number,
		h.Author,
		h.TransactionsRoot,
		h.UnclesHash,
		h.ExtraData,
		h.StateRoot,
		h.ReceiptsRoot,
		h.LogBloom,
		h.GasUsed,
		h.GasLimited,
		h.Difficulty,
		fmt.Sprintf("%s.%s", h.Seal[0], h.Seal[1]),
		h.Hash,
	)
}

func (h *DarwiniaEthHeader) De(str string) error {
	sa := strings.Split(str, "|")
	if len(sa) != 15 {
		return fmt.Errorf(
			"Deserialize DarwiniaEthHeader failed, the lenght is %v, expect 15",
			len(sa),
		)
	}

	// timestamp
	ts, err := strconv.ParseUint(sa[1], 10, 64)
	if err != nil {
		return fmt.Errorf("Parse timestamp failed: %v", sa[1])
	}

	// number
	num, err := strconv.ParseUint(sa[2], 10, 64)
	if err != nil {
		return fmt.Errorf("Parse block number failed: %v", sa[2])
	}

	// gas used
	gasUsed, err := strconv.ParseUint(sa[10], 10, 64)
	if err != nil {
		return fmt.Errorf("Parse block gas used failed: %v", sa[10])
	}

	// gas limited
	gasLimited, err := strconv.ParseUint(sa[11], 10, 64)
	if err != nil {
		return fmt.Errorf("Parse block gas limited failed: %v", sa[11])
	}

	// difficulty
	diff, err := strconv.ParseUint(sa[12], 10, 64)
	if err != nil {
		return fmt.Errorf("Parse block gas used failed: %v", sa[12])
	}

	h.ParentHash = sa[0]
	h.TimeStamp = ts
	h.Number = num
	h.Author = sa[3]
	h.TransactionsRoot = sa[4]
	h.UnclesHash = sa[5]
	h.ExtraData = sa[6]
	h.StateRoot = sa[7]
	h.ReceiptsRoot = sa[8]
	h.LogBloom = sa[9]
	h.GasUsed = gasUsed
	h.GasUsed = gasLimited
	h.Difficulty = diff
	h.Seal = strings.Split(sa[13], ".")
	h.Hash = sa[14]

	return nil
}
