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
	Root    string `json:"root"`
}

// Common load config
func (c *Config) Load() error {
	// Init root directory
	var err error
	c.Root, err = c.rootDir()
	if err != nil {
		return err
	}

	// Load infura key
	gen := os.Getenv("SHADOW_GENESIS")
	if gen == "" {
		gen = "0"
	}

	// Construct shadow genesis
	c.Genesis, err = strconv.ParseUint(gen, 10, 64)
	if err != nil {
		return err
	}

	// Load api from env
	err = c.loadEnv()
	if err != nil {
		err = c.loadFromFile()
	}

	// Check load file result
	if err != nil || c.Api == "" {
		c.readKeyWithPrompt()
	}

	return nil
}

// Common load config
func (c *Config) CheckLock(filename string) bool {
	p := filepath.Join(c.Root, filename)
	if _, err := os.Stat(p); os.IsNotExist(err) {
		return false
	}

	return true

}

// Common load config
func (c *Config) CreateLock(filename string) error {
	p := filepath.Join(c.Root, filename)
	if _, err := os.Stat(p); os.IsNotExist(err) {
		_, err = os.Create(p)
		if err != nil {
			return err
		}
	}

	return nil
}

// Common load config
func (c *Config) RemoveLock(filename string) error {
	p := filepath.Join(c.Root, filename)
	_, err := os.Stat(p)
	if err == nil {
		err = os.Remove(p)
		if err != nil {
			return err
		}
	}

	return nil
}

// Load config from env
func (c *Config) loadEnv() error {
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
func (c *Config) loadFromFile() error {
	root, err := c.rootDir()
	if err != nil {
		return err
	}

	// Check `config.json`
	conf := filepath.Join(root, "config.json")
	if _, err = os.Stat(conf); os.IsNotExist(err) {
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
func (c *Config) readKeyWithPrompt() {
	reader := bufio.NewReader(os.Stdin)
	fmt.Print("Please input your infura key: ")

	// Return infura key after parsing
	text, _ := reader.ReadString('\n')
	text = strings.Trim(text, "\n")
	c.Api = parseKey(text)
}

// Get darwinia config root directory
func (c *Config) rootDir() (string, error) {
	home, err := os.UserHomeDir()
	Assert(err)

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
