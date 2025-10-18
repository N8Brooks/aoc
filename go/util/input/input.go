package input

import (
	"fmt"
	"os"
	"sync"
)

const testdata = "../../../test_data"

var (
	cache   = map[problem]*cachedInput{}
	cacheMu sync.RWMutex
)

type cachedInput struct {
	once  sync.Once
	value string
}

// Load returns the input for the problem, caching contents in memory.
// Safe for concurrent callers. Panics if the file does not exist.
func Load(year uint16, day uint8) string {
	p := problem{year, day}

	cacheMu.RLock()
	entry := cache[p]
	cacheMu.RUnlock()

	if entry == nil {
		cacheMu.Lock()
		if entry = cache[p]; entry == nil {
			entry = &cachedInput{}
			cache[p] = entry
		}
		cacheMu.Unlock()
	}

	entry.once.Do(func() {
		entry.value = p.readInput()
	})

	return entry.value
}

type problem struct {
	Year uint16
	Day  uint8
}

func (p problem) readInput() string {
	b, err := os.ReadFile(p.inputName())
	if err != nil {
		panic(err)
	}
	return string(b)
}

func (p problem) inputName() string {
	return fmt.Sprintf("%s/year_%04d/day_%02d.txt", testdata, p.Year, p.Day)
}
