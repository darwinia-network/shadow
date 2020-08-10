package internal

import (
	"bufio"
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"strconv"
	"strings"

	"github.com/darwinia-network/shadow/internal/util"
)

const (
	INFURA_KEY     = "INFURA_KEY"
	SHADOW_GENESIS = "SHADOW_GENESIS"
	GETH_DATADIR   = "GETH_DATADIR"
)

type RawConfig struct {
	Eth Config `json:"eth"`
}

type Config struct {
	Api     string `json:"api"`
	Genesis uint64 `json:"genesis"`
	Root    string `json:"root"`
	Geth    string `json:"geth"`
}

// Common load config
func (c *Config) Load() error {
	// Init root directory
	var err error
	c.Root, err = RootDir()
	if err != nil {
		return err
	}

	// Load Geth datadir
	c.Geth = os.Getenv(GETH_DATADIR)

	// Load infura key
	gen := os.Getenv(SHADOW_GENESIS)
	if gen == "" {
		gen = "0"
	}

	// Construct shadow genesis
	c.Genesis, err = strconv.ParseUint(gen, 10, 64)
	if err != nil {
		return err
	}

	// Load api from env
	err = c.LoadEnv()
	if err != nil || c.Api == "" {
		c.readKeyWithPrompt()
	}

	return nil
}

// Load config from env
func (c *Config) LoadEnv() error {
	// load infura key
	api := os.Getenv(INFURA_KEY)
	if api == "" {
		return errors.New("Empty INFURA_KEY in env")
	}

	// construct config
	c.Api = ParseKey(api)
	return nil
}

// Parse infura api key
//
// return mainnet api if just inputs a infura key
func ParseKey(key string) string {
	if !strings.HasPrefix(key, "https") {
		key = "https://mainnet.infura.io/v3/" + key
	}

	return key
}

// Read infura key from command-line
func (c *Config) readKeyWithPrompt() {
	reader := bufio.NewReader(os.Stdin)
	fmt.Print("Please input your infura key: ")

	// Return infura key after parsing
	text, _ := reader.ReadString('\n')
	text = strings.Trim(text, "\n")
	c.Api = ParseKey(text)

	// Set INFURA_KEY to env
	os.Setenv("INFURA_KEY", c.Api)
}

// Get darwinia config root directory
func RootDir() (string, error) {
	home, err := os.UserHomeDir()
	util.Assert(err)

	// Create root dir if not exist
	root := filepath.Join(home, ".darwinia")
	if _, err := os.Stat(root); os.IsNotExist(err) {
		err = os.Mkdir(root, 0700)
		if err != nil {
			return "", err
		}
	}

	return root, nil
}
