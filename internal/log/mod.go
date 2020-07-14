package log

import (
	"fmt"
	l "log"
	"os"
	"runtime"
	"strings"
	"time"

	"github.com/fatih/color"
)

const (
	GO_LOG = "GO_LOG"
)

func emit(label string, ctx string) {
	l.SetFlags(0)

	// get time
	t := time.Now().Format(time.RFC3339)
	t = t[:len(t)-6]
	t += "Z"

	// get file
	_, file, _, _ := runtime.Caller(2)
	file = file[:len(file)-3]
	file = strings.ReplaceAll(file, "/", "::")

	// prints log
	l.Println(fmt.Sprintf(
		"%s%s %v %s%s %s",
		color.New(color.FgHiBlack).Sprintf("["),
		t,
		label,
		strings.SplitAfter(file, "shadow::")[1],
		color.New(color.FgHiBlack).Sprintf("]"),
		ctx,
	))
}

func checkMode(modes []string) bool {
	for _, m := range modes {
		if strings.Contains(strings.ToLower(os.Getenv(GO_LOG)), strings.ToLower(m)) {
			return true
		}
	}

	return false
}

// Info logs
func Info(ctx string, a ...interface{}) {
	if checkMode([]string{"INFO", "ALL"}) {
		emit(color.GreenString("INFO "), fmt.Sprintf(ctx, a...))
	}

}

// Trace Logs
func Trace(ctx string, a ...interface{}) {
	if checkMode([]string{"TRACE", "ALL"}) {
		emit(color.New(color.FgHiBlack).Sprintf("TRACE"), fmt.Sprintf(ctx, a...))
	}
}

// Warn Logs
func Warn(ctx string, a ...interface{}) {
	if checkMode([]string{"WARN", "ALL"}) {
		emit(color.YellowString("WARN "), fmt.Sprintf(ctx, a...))
	}
}

// Warn Logs
func Error(ctx string, a ...interface{}) {
	emit(color.RedString("ERROR"), fmt.Sprintf(ctx, a...))
}
