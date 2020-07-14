package log

import (
	l "log"
	"os"
	"strings"
)

const (
	LOG_GO = "LOG_GO"
)

func emit(label string, ctx string) {
	l.SetPrefix(label)
	l.SetFlags(l.LstdFlags | l.Lshortfile)
	l.Println(ctx)
}

func checkMode(modes []string) bool {
	for _, m := range modes {
		if strings.Contains(strings.ToLower(os.Getenv(LOG_GO)), strings.ToLower(m)) {
			return true
		}
	}

	return false
}

// Info logs
func Info(ctx string) {
	if checkMode([]string{"INFO", "ALL"}) {
		emit("[ INFO ] ", ctx)
	}

}

// Trace Logs
func Trace(ctx string) {
	if checkMode([]string{"TRACE", "ALL"}) {
		emit("[ TRACE ] ", ctx)
	}
}

// Warn Logs
func Warn(ctx string) {
	if checkMode([]string{"WARN", "ALL"}) {
		emit("[ WARN ] ", ctx)
	}
}
