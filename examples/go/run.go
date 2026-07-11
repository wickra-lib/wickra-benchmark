// A runnable Go example: load a curated benchmark case and its dataset,
// recompute the report with the wickra-benchmark C ABI binding, and assert it
// reproduces — both `passed` (the report matches the frozen expectation) and
// `hash_match` (its canonical hash matches).
//
//	cargo build --release -p wickra-benchmark-c
//	# stage the library under bindings/go/lib/<goos>_<goarch>/ (CI does this)
//	cd examples/go && go run .
package main

import (
	"encoding/csv"
	"fmt"
	"os"
	"path/filepath"
	"strconv"
	"strings"

	wickra "github.com/wickra-lib/wickra-benchmark-go"
)

// dataDir walks up from the working directory to find examples/data.
func dataDir() string {
	dir, _ := os.Getwd()
	for i := 0; i < 8; i++ {
		candidate := filepath.Join(dir, "examples", "data")
		if _, err := os.Stat(candidate); err == nil {
			return candidate
		}
		dir = filepath.Dir(dir)
	}
	panic("examples/data not found")
}

// candlesJSON turns a time,open,high,low,close,volume CSV into a JSON array.
func candlesJSON(path string) string {
	f, err := os.Open(path)
	if err != nil {
		panic(err)
	}
	defer f.Close()
	rows, err := csv.NewReader(f).ReadAll()
	if err != nil {
		panic(err)
	}
	var parts []string
	for _, row := range rows[1:] { // skip the header
		fields := make([]float64, 6)
		for i, cell := range row {
			fields[i], _ = strconv.ParseFloat(cell, 64)
		}
		parts = append(parts, fmt.Sprintf(
			`{"time":%d,"open":%v,"high":%v,"low":%v,"close":%v,"volume":%v}`,
			int64(fields[0]), fields[1], fields[2], fields[3], fields[4], fields[5]))
	}
	return "[" + strings.Join(parts, ",") + "]"
}

func main() {
	data := dataDir()
	caseJSON, err := os.ReadFile(filepath.Join(data, "cases", "sma-crossover-01.json"))
	if err != nil {
		panic(err)
	}
	candles := candlesJSON(filepath.Join(data, "datasets", "sma-uptrend.csv"))

	b := wickra.New()
	defer b.Close()

	cmd := fmt.Sprintf(`{"cmd":"run_case","case":%s,"data":%s}`, caseJSON, candles)
	resp, err := b.Command(cmd)
	if err != nil {
		panic(err)
	}

	fmt.Println("wickra-benchmark", wickra.Version())
	ok := strings.Contains(resp, `"passed":true`) && strings.Contains(resp, `"hash_match":true`)
	if !ok {
		panic("the curated case must reproduce, got: " + resp)
	}
	fmt.Println("sma-crossover-01: REPRODUCED (passed + hash_match)")
}
