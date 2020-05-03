package util

import (
	"bufio"
	"encoding/json"
	"errors"
	"fmt"
	"io/ioutil"
	"os"
	"path/filepath"
	"strconv"
	"strings"
)

type RawConfig struct {
	Eth Config `json:"eth"`
}

type Config struct {
	Api     string `json:"api"`
	Genesis uint64 `json:"genesis"`
}

// Parse infura api key
//
// return mainnet api if just inputs a infura key
func parseKey(key string) string {
	if !strings.HasPrefix(key, "https") {
		key = "https://mainnet.infura.io/v3/" + key
	}

	return key
}

// Read infura key from command-line
func (c *Config) ReadKeyWithPrompt() {
	reader := bufio.NewReader(os.Stdin)
	fmt.Print("Please input your infura key: ")

	// Return infura key after parsing
	text, _ := reader.ReadString('\n')
	text = strings.Trim(text, "\n")
	c.Api = parseKey(text)
}

// Common load config
func (c *Config) Load() error {
	// Load infura key
	gen := os.Getenv("SHADOW_GENESIS")
	if gen == "" {
		gen = "0"
	}

	// Construct shadow genesis
	var err error
	c.Genesis, err = strconv.ParseUint(gen, 10, 64)
	if err != nil {
		return err
	}

	// Load api from env
	err = c.LoadEnv()
	if err != nil {
		err = c.LoadFromFile()
	}

	// Check load file result
	if err != nil || c.Api == "" {
		c.ReadKeyWithPrompt()
	}

	return nil
}

// Load config from env
func (c *Config) LoadEnv() error {
	// load infura key
	api := os.Getenv("INFURA_KEY")
	if api == "" {
		return errors.New("Empty INFURA_KEY in env")
	}

	// construct config
	c.Api = parseKey(api)
	return nil
}

// Load Config from `.darwinia/config.json`
func (c *Config) LoadFromFile() error {
	home, err := os.UserHomeDir()
	Assert(err)

	// Create root dir if not exist
	root := filepath.Join(home, ".darwinia")
	if _, err := os.Stat(root); os.IsNotExist(err) {
		err = os.Mkdir(root, 0700)
		if err != nil {
			return errors.New(
				"please fill your infura key in `eth.api` at `~/.darwinia/config.json`",
			)
		}
	}

	// Check `config.json`
	conf := filepath.Join(root, "config.json")
	if _, err := os.Stat(conf); os.IsNotExist(err) {
		if err != nil {
			return err
		}
	}

	// Read `config.json`
	confJson := RawConfig{}
	bytes, err := ioutil.ReadFile(conf)
	if err != nil {
		return err
	}

	err = json.Unmarshal(bytes, &confJson)
	if err != nil {
		return err
	}

	// Return eth config
	c.Api = parseKey(confJson.Eth.Api)
	return nil
}
