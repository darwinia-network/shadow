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

var LogLevel = map[string]int{
    "trace": 0,
    "info": 1,
    "warn": 2,
    "error": 3,
}

func emit(label string, ctx string) {
	l.SetFlags(0)

	// get time
	t := time.Now().UTC().Format(time.RFC3339)

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
		"go::" + strings.SplitAfter(file, "pkg::")[1],
		color.New(color.FgHiBlack).Sprintf("]"),
		ctx,
	))
}

func checkMode(l int) bool {
    setted := strings.ToLower(os.Getenv(GO_LOG))
    if setted == "" {
        setted = "info"
    }
    level, ok := LogLevel[setted]
    return ok && l >= level
}

// Trace Logs
func Trace(ctx string, a ...interface{}) {
	if checkMode(LogLevel["trace"]) {
		emit(color.New(color.FgHiBlack).Sprintf("TRACE"), fmt.Sprintf(ctx, a...))
	}
}

// Info logs
func Info(ctx string, a ...interface{}) {
	if checkMode(LogLevel["info"]) {
		emit(color.GreenString("INFO "), fmt.Sprintf(ctx, a...))
	}

}

// Warn Logs
func Warn(ctx string, a ...interface{}) {
	if checkMode(LogLevel["warn"]) {
		emit(color.YellowString("WARN "), fmt.Sprintf(ctx, a...))
	}
}

// Warn Logs
func Error(ctx string, a ...interface{}) {
	emit(color.RedString("ERROR"), fmt.Sprintf(ctx, a...))
}
