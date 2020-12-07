package shadow

import (
	"os"
	"path/filepath"
	"github.com/darwinia-network/shadow/pkg/log"
)

type RawConfig struct {
	Eth Config `json:"eth"`
}

type Config struct {
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

	return nil
}

// Get darwinia config root directory
func RootDir() (string, error) {
	home, err := os.UserHomeDir()
	log.Assert(err)

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
