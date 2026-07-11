package wickra

// Cross-language golden parity: for each committed golden/commands/*.json (a
// full command envelope), drive command_json and assert the response equals
// golden/expected/<name>.json byte-for-byte. The binding returns the core's
// canonical command_json string verbatim, so byte equality is the exact
// cross-language parity check — the same blake3 hashes in every language. The
// fixtures arrive in a later phase; until then the test skips cleanly.

import (
	"os"
	"path/filepath"
	"strings"
	"testing"
)

func goldenDir() string {
	dir, err := os.Getwd()
	if err != nil {
		return ""
	}
	for i := 0; i < 8; i++ {
		g := filepath.Join(dir, "golden")
		if _, err := os.Stat(filepath.Join(g, "commands")); err == nil {
			return g
		}
		dir = filepath.Dir(dir)
	}
	return ""
}

func TestGoldenParity(t *testing.T) {
	g := goldenDir()
	if g == "" {
		t.Skip("golden fixtures not present yet")
	}
	commands, err := os.ReadDir(filepath.Join(g, "commands"))
	if err != nil {
		t.Skip("golden commands not present yet")
	}
	for _, entry := range commands {
		if !strings.HasSuffix(entry.Name(), ".json") {
			continue
		}
		name := entry.Name()
		t.Run(name, func(t *testing.T) {
			cmdRaw, err := os.ReadFile(filepath.Join(g, "commands", name))
			if err != nil {
				t.Fatal(err)
			}
			expected, err := os.ReadFile(filepath.Join(g, "expected", name))
			if err != nil {
				t.Fatal(err)
			}
			b := New()
			defer b.Close()
			got, err := b.Command(string(cmdRaw))
			if err != nil {
				t.Fatal(err)
			}
			if got != strings.TrimSpace(string(expected)) {
				t.Fatalf("golden mismatch for %s:\n got: %s\nwant: %s", name, got, expected)
			}
		})
	}
}
