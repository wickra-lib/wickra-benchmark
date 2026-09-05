package wickra

// Batch equivalence: a suite must produce exactly what the cases produce alone.
//
// run_suite is the batch form of run_case. It fans the cases out -- over rayon
// when the parallel feature is on -- and re-sorts the results by id before
// tallying, so the two paths share an engine but not a control flow. Nothing
// else holds them to the same answer.

import (
	"encoding/json"
	"fmt"
	"math"
	"sort"
	"strings"
	"testing"
)

func batchStrategy(fast, slow int) json.RawMessage {
	return json.RawMessage(fmt.Sprintf(
		`{"symbol":"BTCUSDT","timeframe":"1h",`+
			`"indicators":{"ema_fast":{"type":"Ema","params":[%d]},"ema_slow":{"type":"Ema","params":[%d]}},`+
			`"entry":{"cross_above":["ema_fast","ema_slow"]},"exit":{"cross_below":["ema_fast","ema_slow"]},`+
			`"sizing":{"type":"fixed_fraction","fraction":0.95},`+
			`"costs":{"taker_bps":5,"slippage":{"type":"fixed_bps","bps":2}}}`, fast, slow))
}

func batchCandles(seed int) []map[string]float64 {
	out := make([]map[string]float64, 0, 40)
	for i := 0; i < 40; i++ {
		base := 100.0 + math.Sin(float64(i)*0.4+float64(seed))*8.0
		out = append(out, map[string]float64{
			"time": float64(1_700_000_000 + i*3600), "open": base,
			"high": base + 1.0, "low": base - 1.0, "close": base + 0.5, "volume": 1000.0,
		})
	}
	return out
}

// Deliberately out of id order, each on its own dataset: a suite that only ever
// sees sorted input cannot show that it sorts.
func batchCases() []map[string]any {
	cases := make([]map[string]any, 0, 3)
	for i, n := range []int{3, 1, 2} {
		cases = append(cases, map[string]any{
			"id":            fmt.Sprintf("case-0%d", n),
			"description":   "batch equivalence",
			"strategy":      batchStrategy(3+n, 12+n),
			"dataset_ref":   fmt.Sprintf("d%d.csv", i),
			"expected":      map[string]any{},
			"expected_hash": strings.Repeat("0", 64),
		})
	}
	return cases
}

func batchDatasets(cases []map[string]any) map[string][]map[string]float64 {
	sets := make(map[string][]map[string]float64, len(cases))
	for i, c := range cases {
		sets[c["dataset_ref"].(string)] = batchCandles(i)
	}
	return sets
}

func TestSuiteMatchesCasesRunAlone(t *testing.T) {
	b := New()
	defer b.Close()

	cases := batchCases()
	datasets := batchDatasets(cases)

	alone := map[string]map[string]any{}
	for _, c := range cases {
		cmd, err := json.Marshal(map[string]any{
			"cmd": "run_case", "case": c, "data": datasets[c["dataset_ref"].(string)],
		})
		if err != nil {
			t.Fatalf("marshal: %v", err)
		}
		out, err := b.Command(string(cmd))
		if err != nil {
			t.Fatalf("run_case: %v", err)
		}
		var result map[string]any
		if err := json.Unmarshal([]byte(out), &result); err != nil {
			t.Fatalf("unmarshal: %v", err)
		}
		alone[result["id"].(string)] = result
	}

	cmd, err := json.Marshal(map[string]any{
		"cmd":      "run_suite",
		"suite":    map[string]any{"name": "batch", "cases": cases},
		"datasets": datasets,
	})
	if err != nil {
		t.Fatalf("marshal: %v", err)
	}
	out, err := b.Command(string(cmd))
	if err != nil {
		t.Fatalf("run_suite: %v", err)
	}
	var report struct {
		Results []map[string]any `json:"results"`
		Passed  int              `json:"passed"`
		Failed  int              `json:"failed"`
	}
	if err := json.Unmarshal([]byte(out), &report); err != nil {
		t.Fatalf("unmarshal: %v", err)
	}

	want := make([]string, 0, len(alone))
	for id := range alone {
		want = append(want, id)
	}
	sort.Strings(want)

	if len(report.Results) != len(want) {
		t.Fatalf("suite returned %d results, want %d", len(report.Results), len(want))
	}
	passed := 0
	for i, result := range report.Results {
		id := result["id"].(string)
		if id != want[i] {
			t.Fatalf("result %d is %q, want %q -- the suite must sort by id", i, id, want[i])
		}
		// Compare the canonical JSON of each, so the whole result is held to the
		// single-case run rather than a field or two.
		gotJSON, _ := json.Marshal(result)
		wantJSON, _ := json.Marshal(alone[id])
		if string(gotJSON) != string(wantJSON) {
			t.Fatalf("%s differs between the suite and the case run alone", id)
		}
		if result["passed"] == true && result["hash_match"] == true {
			passed++
		}
	}
	if report.Passed != passed || report.Failed != len(report.Results)-passed {
		t.Fatalf("tally %d/%d disagrees with the %d results it counted",
			report.Passed, report.Failed, len(report.Results))
	}
}

func TestCaseOrderDoesNotChangeTheReport(t *testing.T) {
	b := New()
	defer b.Close()

	cases := batchCases()
	datasets := batchDatasets(cases)

	run := func(order []map[string]any) string {
		cmd, _ := json.Marshal(map[string]any{
			"cmd":      "run_suite",
			"suite":    map[string]any{"name": "batch", "cases": order},
			"datasets": datasets,
		})
		out, err := b.Command(string(cmd))
		if err != nil {
			t.Fatalf("run_suite: %v", err)
		}
		return out
	}

	reversed := make([]map[string]any, len(cases))
	for i, c := range cases {
		reversed[len(cases)-1-i] = c
	}
	if run(cases) != run(reversed) {
		t.Fatal("the report depends on the order the cases were listed in")
	}
}
