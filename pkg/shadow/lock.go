package shadow

import (
	"errors"
	"fmt"
	"io/ioutil"
	"os"
	"path/filepath"
	"strconv"
	"strings"
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

func (c *Config) checkLockExists(filename string, epoch uint64) (bool, error) {
	// Read the content of the lock
	content, err := ioutil.ReadFile(filename)
	if err != nil {
		return false, err
	}

	// Get file content
	text := string(content)
	epochString := strconv.Itoa(int(epoch))
	epochs := strings.Split(text, ",")

	// Check if element exists
	for _, e := range epochs {
		if e == epochString {
			return true, nil
		}
	}

	/// Append new lock
	f, err := os.OpenFile(filename, os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)
	if err != nil {
		return false, err
	}
	defer f.Close()
	if _, err := f.WriteString(fmt.Sprintf(",%d", epoch)); err != nil {
		return false, err
	}

	return false, nil
}

// Create lock
func (c *Config) CreateLock(lock Lock, epoch uint64) error {
	p := filepath.Join(c.Root, lock.toString())
	if _, err := os.Stat(p); os.IsNotExist(err) {
		_, err = os.Create(p)
		if err != nil {
			return err
		}

		err = ioutil.WriteFile(p, []byte(fmt.Sprintf("%d", epoch)), 0600)
		if err != nil {
			return err
		}

	} else {
		exists, err := c.checkLockExists(p, epoch)
		if err != nil {
			return err
		}

		if exists {
			return errors.New("The target ethash epoch is in process, shadow service is busy")
		}
	}

	return nil
}

// Remove lock
func (c *Config) RemoveLock(lock Lock, epoch uint64) error {
	p := filepath.Join(c.Root, lock.toString())
	if _, err := os.Stat(p); os.IsNotExist(err) {
		return nil
	}

	// Get content
	content, err := ioutil.ReadFile(p)
	if err != nil {
		return err
	}

	epochs := strings.Split(string(content), ",")
	if len(epochs) < 1 {
		err = os.Remove(p)
		if err != nil {
			return err
		}
	}

	// Compare epoch
	epochString := strconv.Itoa(int(epoch))
	newEpochs := []string{}
	for _, e := range epochs {
		if e != epochString {
			newEpochs = append(newEpochs, fmt.Sprintf(",%s", epochString))
		}
	}

	// Update lock file
	err = ioutil.WriteFile(p, []byte(strings.Join(newEpochs, ",")), 0600)
	if err != nil {
		return err
	}

	return nil
}
