// A runnable C# example: load a curated benchmark case and its dataset,
// recompute the report with the wickra-benchmark C ABI binding, and assert it
// reproduces — both `passed` (the report matches the frozen expectation) and
// `hash_match` (its canonical hash matches).
//
//   cargo build --release -p wickra-benchmark-c
//   dotnet run --project examples/csharp/Run
using System.Globalization;
using System.Text;
using Wickra.Benchmark;

static string FindDataDir()
{
    var dir = new DirectoryInfo(AppContext.BaseDirectory);
    for (var i = 0; i < 10 && dir is not null; i++, dir = dir.Parent)
    {
        var candidate = Path.Combine(dir.FullName, "examples", "data");
        if (Directory.Exists(candidate))
        {
            return candidate;
        }
    }
    throw new DirectoryNotFoundException("examples/data not found");
}

static string CandlesJson(string csvPath)
{
    var sb = new StringBuilder("[");
    var first = true;
    foreach (var line in File.ReadLines(csvPath).Skip(1)) // skip the header
    {
        var c = line.Split(',');
        if (c.Length < 6)
        {
            continue;
        }
        double F(int i) => double.Parse(c[i], CultureInfo.InvariantCulture);
        if (!first)
        {
            sb.Append(',');
        }
        first = false;
        sb.Append(CultureInfo.InvariantCulture,
            $"{{\"time\":{long.Parse(c[0], CultureInfo.InvariantCulture)},\"open\":{F(1):R},\"high\":{F(2):R},\"low\":{F(3):R},\"close\":{F(4):R},\"volume\":{F(5):R}}}");
    }
    return sb.Append(']').ToString();
}

var dataDir = FindDataDir();
var caseJson = File.ReadAllText(Path.Combine(dataDir, "cases", "sma-crossover-01.json"));
var candles = CandlesJson(Path.Combine(dataDir, "datasets", "sma-uptrend.csv"));

using var benchmark = new Benchmark();
var response = benchmark.Command($"{{\"cmd\":\"run_case\",\"case\":{caseJson},\"data\":{candles}}}");

Console.WriteLine($"wickra-benchmark {Benchmark.Version()}");
var ok = response.Contains("\"passed\":true") && response.Contains("\"hash_match\":true");
Console.WriteLine($"sma-crossover-01: {(ok ? "REPRODUCED (passed + hash_match)" : "MISMATCH")}");
if (!ok)
{
    Console.Error.WriteLine("the curated case must reproduce");
    Environment.Exit(1);
}
