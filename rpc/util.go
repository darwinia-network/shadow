package rpc

import (
	"strings"
	"unicode"
)

// Convert the ill-formed methods
func LowerCaseWithSlash(service *string) {
	s := *service

	// Check if request service is ill-formed
	slash := strings.LastIndex(s, "_")
	if slash == -1 {
		return
	}

	// Convert the service title
	if unicode.IsLower(rune(s[0])) {
		s = strings.Title(s)
	}

	// Convert the method title
	if unicode.IsLower(rune(s[slash+1])) {
		s = s[:slash] + strings.Title(s[(slash+1):])
	}

	// Replace slash with dot
	*service = s[:slash] + "." + s[slash:]
}
