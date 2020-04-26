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
		os.Mkdir(root, 0700)
	}

	// Check `config.json`
	conf := filepath.Join(root, "config.json")
	if _, err := os.Stat(conf); os.IsNotExist(err) {
		return Config{}, errors.New("config.json does not exist")
	}

	// Read `config.json`
	confJson := RawConfig{}
	bytes, err := ioutil.ReadFile(conf)
	Assert(err)

	err = json.Unmarshal(bytes, &confJson)
	Assert(err)

	// Return eth config
	return confJson.Eth, nil
}
