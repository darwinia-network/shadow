package util

import (
	"encoding/hex"
	"fmt"
	"math/big"
	"path/filepath"
	"sort"
	"strconv"
	"strings"

	gmath "github.com/ethereum/go-ethereum/common/math"
	"github.com/huandu/xstrings"
	"github.com/shopspring/decimal"
	"github.com/ttacon/libphonenumber"
	"github.com/tuvistavie/securerandom"
)

func RandStr(length int) string {
	str, _ := securerandom.Hex(length)
	return str
}

func SnakeString(s string) string {
	return strings.ToLower(xstrings.ToSnakeCase(s))
}

func CamelString(s string) string {
	return xstrings.ToCamelCase(s)
}

func Padding(str string) string {
	if strings.HasPrefix(str, "0x") {
		str = str[2:]
	}
	return xstrings.RightJustify(str, 64, "0")
}

func PaddingF(str string) string {
	if strings.HasPrefix(str, "0x") {
		str = str[2:]
	}
	return xstrings.RightJustify(str, 64, "f")
}

func StringInSlice(a string, list []string) bool {
	for _, b := range list {
		if b == a {
			return true
		}
	}
	return false
}

func IntInSlice(a int, list []int) bool {
	for _, b := range list {
		if b == a {
			return true
		}
	}
	return false
}

func ToWei(v int64) *big.Int {
	return big.NewInt(v).Mul(big.NewInt(v), big.NewInt(int64(10)).Exp(big.NewInt(int64(10)), big.NewInt(int64(18)), nil))
}
func EthToWei(v decimal.Decimal) *big.Int {
	decimalStr := v.Mul(decimal.NewFromFloat(1e18)).String()
	wei, _ := new(big.Int).SetString(decimalStr, 10)
	return wei
}

func BigToDecimal(v *big.Int) decimal.Decimal {
	return decimal.NewFromBigInt(v, 0).Div(decimal.NewFromFloat(1e18))
}

func AddHex(s string, chain ...string) string {
	prefix := "0x"
	addr := s
	if len(chain) > 0 {
		switch chain[0] {
		case "Tron":
			prefix = "41"
		case "Eth":
			prefix = "0x"
		case "PChain":
			prefix = "0p"
		}
	}
	if strings.HasPrefix(addr, prefix) {
		return addr
	}
	return strings.ToLower(prefix + addr)
}

func U256(v string) *big.Int {
	v = strings.TrimPrefix(v, "0x")
	bn := new(big.Int)
	bn.SetString(v, 16)
	return gmath.U256(bn)
}

func DecodeInputU256(i *big.Int) string {
	if i.Sign() == -1 {
		pow := BigIntLog(new(big.Int).Abs(i), big.NewInt(16))
		newHex := big.NewInt(16).Exp(big.NewInt(16), pow, nil)
		newHex = newHex.Add(newHex, i)
		return PaddingF(fmt.Sprintf("%x", newHex))
	} else {
		return fmt.Sprintf("%064s", fmt.Sprintf("%x", i))
	}
}

func IntToString(i int) string {
	return strconv.Itoa(i)
}

func StringToInt(s string) int {
	if i, err := strconv.Atoi(s); err != nil {
		return 0
	} else {
		return i
	}

}

func StringToInt64(s string) int64 {
	if i, err := strconv.ParseInt(s, 10, 64); err != nil {
		return 0
	} else {
		return i
	}
}

func IntToHex(i interface{}) string {
	return fmt.Sprintf("0x%x", i)
}

func BigIntLog(i *big.Int, n *big.Int) *big.Int {
	c := big.NewInt(1)
	for {
		quotient := new(big.Int).Quo(i, n)
		if quotient.Sign() != 1 { // quot >= n
			break
		} else {
			c = c.Add(c, big.NewInt(1))
			i = quotient
		}
	}
	return c

}

func DealNegU256(s string) *big.Int {
	index := strings.IndexFunc(s, isSlash)
	fis := new(big.Int)
	exp := 0
	if index != -1 {
		fis = U256(s[index:])
		exp = len(s) - index
	}
	pos := new(big.Int).Exp(big.NewInt(16), big.NewInt(int64(exp)), nil)
	return new(big.Int).Sub(fis, pos)
}

func EncodeU256(s string) *big.Int {
	if s[0] == 'f' {
		return DealNegU256(s)
	} else {
		return U256(s)
	}
}
func ResourceDecode(r string) []int {
	var resource []int
	resourceEnum := [5]int{0, 1, 2, 3, 4}
	for _, index := range resourceEnum {
		b := EncodeU256(r)
		fs := new(big.Int).Lsh(big.NewInt(65535), uint(16*index))
		ss := new(big.Int).And(b, fs)
		ts := new(big.Int).Rsh(ss, uint(16*index))
		resource = append(resource, int(ts.Int64()))
	}
	return resource
}

func isSlash(r rune) bool {
	return r != 'f'
}

func GetFileType(uploadFilePath string) string {
	fileType := strings.TrimPrefix(filepath.Ext(uploadFilePath), ".")
	return fileType
}

func AppendIfMissing(slice []string, i string) []string {
	for _, ele := range slice {
		if ele == i {
			return slice
		}
	}
	return append(slice, i)
}

// mobile like +86 151 8888 9998
func BlurryMobile(mobile string) string {
	mobileSlice := strings.Split(mobile, " ")
	mobile = strings.Join(mobileSlice[1:], "")
	return mobile[0:3] + "*****" + mobile[len(mobile)-3:]
}

func AnalysisMobile(mobile, region string) string {
	if mobile == "" {
		return ""
	}
	if region == "" {
		region = "CN"
	}
	num, err := libphonenumber.Parse(mobile, region)
	if err != nil {
		return mobile
	}
	return libphonenumber.Format(num, libphonenumber.INTERNATIONAL)
}

func TrimHex(s string) string {
	return strings.TrimPrefix(s, "0x")
}
func TrimTronHex(s string) string {
	return strings.TrimPrefix(s, "41")
}

type Pair struct {
	Key   string
	Value int
}

type PairList []Pair

func (p PairList) Len() int           { return len(p) }
func (p PairList) Less(i, j int) bool { return p[i].Value < p[j].Value }
func (p PairList) Swap(i, j int)      { p[i], p[j] = p[j], p[i] }

func RankByWordCount(wordFrequencies map[string]int) PairList {
	pl := make(PairList, len(wordFrequencies))
	i := 0
	for k, v := range wordFrequencies {
		pl[i] = Pair{k, v}
		i++
	}
	sort.Sort(sort.Reverse(pl))
	return pl
}

func StringsIntersection(a []string, b []string) []string {
	var refresh []string
	for _, v := range a {
		if StringInSlice(v, b) {
			refresh = append(refresh, v)
		}
	}
	return refresh
}

func StringsExclude(a []string, b []string) {
	var refresh []string
	for _, v := range a {
		if StringInSlice(v, b) == false {
			refresh = append(refresh, v)
		}
	}
}

func BoolToString(b bool) string {
	if b {
		return "true"
	}
	return "false"
}

func HexToBytes(s string) []byte {
	s = strings.TrimPrefix(s, "0x")
	c := make([]byte, hex.DecodedLen(len(s)))
	_, _ = hex.Decode(c, []byte(s))
	return c
}

func BytesToHex(b []byte) string {
	c := make([]byte, hex.EncodedLen(len(b)))
	hex.Encode(c, b)
	return string(c)
}
