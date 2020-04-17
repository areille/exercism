// This is a "stub" file.  It's a little start on your solution.
// It's not a complete solution though; you have to write some code.

// Package bob should have a package comment that summarizes what it's about.
// https://golang.org/doc/effective_go.html#commentary
package bob

import (
	"strings"
	"unicode"
)

// ContainsLetter returns true if str contains at least one letter
func ContainsLetter(str string) bool {
	for _, r := range str {
		if unicode.IsLetter(r) {
			return true
		}
	}
	return false
}

// Hey should have a comment documenting it.
func Hey(remark string) string {
	if strings.TrimSpace(remark) == "" {
		return "Fine. Be that way!"
	}
	remark = strings.TrimSpace(remark)
	if strings.HasSuffix(remark, "?") {
		if strings.ToUpper(remark) == remark && ContainsLetter(remark) {
			return "Calm down, I know what I'm doing!"
		}
		return "Sure."
	}
	if strings.ToUpper(remark) == remark && ContainsLetter(remark) {
		return "Whoa, chill out!"
	}
	return "Whatever."
}
