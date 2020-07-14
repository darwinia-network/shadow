package log

import (
	l "log"
)

func emit(label string, ctx string) {
	l.SetPrefix(label)
	l.SetFlags(l.LstdFlags | l.Lshortfile)
	l.Println(ctx)
}

func Info(ctx string) {
	emit("[ INFO ] ", ctx)
}

func Trace(ctx string) {
	emit("[ TRACE ] ", ctx)
}

func Warn(ctx string) {
	emit("[ WARN ] ", ctx)
}
