package util

import (
	"encoding/json"
	"errors"
	"io/ioutil"
	"os"
	"path/filepath"
)

type RawConfig struct {
	Eth Config `json:"eth"`
}

type Config struct {
	Api string `json:"api"`
}

// New config
func LoadConfig() (Config, error) {
	home, err := os.UserHomeDir()
	Assert(err)

	// Create root dir if not exist
	root := filepath.Join(home, ".darwinia")
	if _, err := os.Stat(root); os.IsNotExist(err) {
		err = os.Mkdir(root, 0700)
		if err != nil {
			return Config{}, errors.New(
				"please fill your infura key in `eth.api` at `~/.darwinia/config.json`",
			)
		}
	}

	// Check `config.json`
	conf := filepath.Join(root, "config.json")
	if _, err := os.Stat(conf); os.IsNotExist(err) {
		if err != nil {
			return Config{}, err
		}
	}

	// Read `config.json`
	confJson := RawConfig{}
	bytes, err := ioutil.ReadFile(conf)
	if err != nil {
		return Config{}, err
	}

	err = json.Unmarshal(bytes, &confJson)
	if err != nil {
		return Config{}, err
	}

	// Return eth config
	return confJson.Eth, nil
}
