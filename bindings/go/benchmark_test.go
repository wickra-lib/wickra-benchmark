package wickra

import (
	"encoding/json"
	"math"
	"strings"
	"testing"
)

const strategy = `{"symbol":"BTCUSDT","timeframe":"1h",` +
	`"indicators":{"ema_fast":{"type":"Ema","params":[5]},"ema_slow":{"type":"Ema","params":[15]}},` +
	`"entry":{"cross_above":["ema_fast","ema_slow"]},"exit":{"cross_below":["ema_fast","ema_slow"]},` +
	`"sizing":{"type":"fixed_fraction","fraction":0.95},` +
	`"costs":{"taker_bps":5,"slippage":{"type":"fixed_bps","bps":2}}}`

func candles() []map[string]float64 {
	out := make([]map[string]float64, 0, 40)
	for i := 0; i < 40; i++ {
		base := 100.0 + math.Sin(float64(i)*0.4)*8.0
		out = append(out, map[string]float64{
			"time": float64(1_700_000_000 + i*3600), "open": base,
			"high": base + 1.0, "low": base - 1.0, "close": base + 0.5, "volume": 1000.0,
		})
	}
	return out
}

// runCaseRequest builds a run_case command with a placeholder expected/hash: the
// run recomputes the real report; these tests check the response shape, not that
// the case passes.
func runCaseRequest() string {
	c := map[string]any{
		"id":            "sma-crossover-01",
		"description":   "smoke",
		"strategy":      json.RawMessage(strategy),
		"dataset_ref":   "d.csv",
		"expected":      map[string]any{},
		"expected_hash": strings.Repeat("0", 64),
	}
	cmd, _ := json.Marshal(map[string]any{"cmd": "run_case", "case": c, "data": candles()})
	return string(cmd)
}

func TestVersion(t *testing.T) {
	if Version() == "" {
		t.Fatal("empty version")
	}
}

func TestRunCaseShape(t *testing.T) {
	b := New()
	defer b.Close()
	raw, err := b.Command(runCaseRequest())
	if err != nil {
		t.Fatal(err)
	}
	var result struct {
		ID        string `json:"id"`
		Passed    bool   `json:"passed"`
		HashMatch bool   `json:"hash_match"`
		Hash      string `json:"hash"`
	}
	if err := json.Unmarshal([]byte(raw), &result); err != nil {
		t.Fatal(err)
	}
	if result.ID != "sma-crossover-01" {
		t.Fatalf("expected id sma-crossover-01, got %s", raw)
	}
	if len(result.Hash) != 64 {
		t.Fatalf("expected a 64-hex hash, got %s", raw)
	}
}

func TestRunCaseByteStable(t *testing.T) {
	b := New()
	defer b.Close()
	req := runCaseRequest()
	first, err := b.Command(req)
	if err != nil {
		t.Fatal(err)
	}
	second, err := b.Command(req)
	if err != nil {
		t.Fatal(err)
	}
	if first != second {
		t.Fatal("expected byte-stable run_case output")
	}
}

func TestUnknownCommandIsInBandError(t *testing.T) {
	b := New()
	defer b.Close()
	raw, err := b.Command(`{"cmd":"nope"}`)
	if err != nil {
		t.Fatalf("unexpected hard error: %v", err)
	}
	if !strings.Contains(raw, `"ok":false`) {
		t.Fatalf("expected an in-band error, got: %s", raw)
	}
}
