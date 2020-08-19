package internal

import (
	"io/ioutil"
	"os"
	"path/filepath"
	"strings"
	"time"
)

// Lock enum
type Lock string

const (
	PROOF_LOCK Lock = "proof.lock"
	EPOCH_LOCK Lock = "epoch.lock"
)

func (l *Lock) toString() string {
	return string(*l)
}

// Check if lock exists
func (c *Config) CheckLock(lock Lock) bool {
	p := filepath.Join(c.Root, lock.toString())
	if stat, err := os.Stat(p); os.IsNotExist(err) {
		return false
	} else if time.Since(stat.ModTime()).Minutes() > 30 {
		if err = c.RemoveLock(lock); err != nil {
			return true
		}
	}

	return true

}

// Create lock
func (c *Config) CreateLock(lock Lock) error {
	p := filepath.Join(c.Root, lock.toString())
	if _, err := os.Stat(p); os.IsNotExist(err) {
		_, err = os.Create(p)
		if err != nil {
			return err
		}

		err = ioutil.WriteFile(p, []byte(""), 0600)
		if err != nil {
			return err
		}
	}

	return nil
}

// Remove lock
func (c *Config) removeLockByString(lock string) error {
	p := filepath.Join(c.Root, lock)
	_, err := os.Stat(p)
	if err == nil {
		err = os.Remove(p)
		if err != nil {
			return err
		}
	}

	return nil
}

// Remove lock
func (c *Config) RemoveLock(lock Lock) error {
	return c.removeLockByString(lock.toString())
}

// Remove all locks
func (c *Config) RemoveAllLocks() (err error) {
	files, err := ioutil.ReadDir(c.Root)
	if err != nil {
		return
	}

	for _, f := range files {
		name := f.Name()
		if strings.HasSuffix(name, ".lock") {
			err = c.removeLockByString(name)
			if err != nil {
				return
			}
		}
	}

	return
}
