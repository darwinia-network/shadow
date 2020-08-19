package internal

import (
	"os"
	"path/filepath"
	"strconv"

	"github.com/darwinia-network/shadow/internal/util"
)

const (
	ETHEREUM_RPC         = "ETHEREUM_RPC"
	SHADOW_GENESIS       = "SHADOW_GENESIS"
	GETH_DATADIR         = "GETH_DATADIR"
	DEFAULT_ETHEREUM_RPC = "https://mainnet.infura.io/v3/0bfb9acbb13c426097aabb1d81a9d016"
)

type RawConfig struct {
	Eth Config `json:"eth"`
}

type Config struct {
	Api     string `json:"api"`
	Genesis uint64 `json:"genesis"`
	Root    string `json:"root"`
}

// Common load config
func (c *Config) Load() error {
	// Init root directory
	var err error
	c.Root, err = RootDir()
	if err != nil {
		return err
	}

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
	c.Api = os.Getenv(ETHEREUM_RPC)
	if c.Api == "" {
		c.Api = DEFAULT_ETHEREUM_RPC
	}

	return nil
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
