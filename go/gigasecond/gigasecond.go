package gigasecond

import "time"

// AddGigasecond adds a Gigasecond to a time.
func AddGigasecond(t time.Time) time.Time {
	return t.Add(time.Second * 1e9)
}
