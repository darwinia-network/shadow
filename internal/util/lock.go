package util

import (
	"io/ioutil"
	"os"
	"path/filepath"
)

// Common load config
func (c *Config) CheckLock(filename string) bool {
	p := filepath.Join(c.Root, filename)
	if _, err := os.Stat(p); os.IsNotExist(err) {
		return false
	}

	return true

}

// Common load config
func (c *Config) CreateLock(filename string, ctx []byte) error {
	p := filepath.Join(c.Root, filename)
	if _, err := os.Stat(p); os.IsNotExist(err) {
		_, err = os.Create(p)
		if err != nil {
			return err
		}

		err = ioutil.WriteFile(p, ctx, 0600)
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
