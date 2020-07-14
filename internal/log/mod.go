package log

import (
	"fmt"
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
func Info(ctx string, a ...interface{}) {
	if checkMode([]string{"INFO", "ALL"}) {
		emit("[ INFO ] ", fmt.Sprintf(ctx, a...))
	}

}

// Trace Logs
func Trace(ctx string, a ...interface{}) {
	if checkMode([]string{"TRACE", "ALL"}) {
		emit("[ TRACE ] ", fmt.Sprintf(ctx, a...))
	}
}

// Warn Logs
func Warn(ctx string, a ...interface{}) {
	if checkMode([]string{"WARN", "ALL"}) {
		emit("[ WARN ] ", fmt.Sprintf(ctx, a...))
	}
}

// Warn Logs
func Error(ctx string, a ...interface{}) {
	emit("[ Error ] ", fmt.Sprintf(ctx, a...))
}
