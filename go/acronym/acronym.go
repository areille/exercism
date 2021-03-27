// Package acronym should have a package comment that summarizes what it's about.
// https://golang.org/doc/effective_go.html#commentary
package acronym

import "strings"

// Abbreviate should have a comment documenting it.
func Abbreviate(s string) string {
	var abbreviation string
	s = strings.ReplaceAll(s, "_", "")
	s = strings.ReplaceAll(s, "-", " ")
	for _, word := range strings.Fields(s) {
		if len(word) != 0 {
			abbreviation += strings.ToUpper(word[:1])
		}
	}
	return abbreviation
}
