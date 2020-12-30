package config

import (
	"os"
    "strings"
	"path/filepath"
)

type EthProof struct {
    RootPath string
    Limitepochsize int
    LimitCPU int
}

func NormalizePath(path string) string {
    pathList := strings.Split(path, string(os.PathSeparator))
    normalizedPath := ""
    for _, dir := range pathList {
        if len(dir) > 0 && dir[0] == '$' {
            expanded := os.Getenv(dir[1:])
            normalizedPath = filepath.Join(normalizedPath, expanded)
        } else {
            normalizedPath = filepath.Join(normalizedPath, dir)
        }
    }
    return normalizedPath
}

