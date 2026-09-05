using System.Text.Json;
using Wickra.Benchmark;
using Xunit;

namespace WickraBenchmark.Tests;

/// <summary>
/// Batch equivalence: a suite must produce exactly what the cases produce alone.
///
/// <para><c>run_suite</c> is the batch form of <c>run_case</c>. It fans the cases out — over rayon
/// when the parallel feature is on — and re-sorts the results by id before tallying, so the two
/// paths share an engine but not a control flow. Nothing else holds them to the same answer.</para>
/// </summary>
public class BatchEquivalenceTests
{
    private const string ZeroHash =
        "0000000000000000000000000000000000000000000000000000000000000000";

    // Deliberately out of id order: a suite that only ever sees sorted input
    // cannot show that it sorts.
    private static readonly int[] Order = [3, 1, 2];

    private static object Strategy(int fast, int slow) => new
    {
        symbol = "BTCUSDT",
        timeframe = "1h",
        indicators = new
        {
            ema_fast = new { type = "Ema", @params = new[] { fast } },
            ema_slow = new { type = "Ema", @params = new[] { slow } },
        },
        entry = new { cross_above = new[] { "ema_fast", "ema_slow" } },
        exit = new { cross_below = new[] { "ema_fast", "ema_slow" } },
        sizing = new { type = "fixed_fraction", fraction = 0.95 },
        costs = new { taker_bps = 5, slippage = new { type = "fixed_bps", bps = 2 } },
    };

    private static object[] Candles(int seed)
    {
        var list = new List<object>();
        for (int i = 0; i < 40; i++)
        {
            double b = 100.0 + Math.Sin(i * 0.4 + seed) * 8.0;
            list.Add(new { time = 1_700_000_000 + i * 3600, open = b, high = b + 1.0, low = b - 1.0, close = b + 0.5, volume = 1000.0 });
        }
        return [.. list];
    }

    private static object Case(int n) => new
    {
        id = $"case-0{n}",
        description = "batch equivalence",
        strategy = Strategy(3 + n, 12 + n),
        dataset_ref = $"d{n}.csv",
        expected = new { },
        expected_hash = ZeroHash,
    };

    private static Dictionary<string, object[]> Datasets()
    {
        var sets = new Dictionary<string, object[]>();
        for (int i = 0; i < Order.Length; i++)
        {
            sets[$"d{Order[i]}.csv"] = Candles(i);
        }
        return sets;
    }

    private static string RunSuite(Benchmark bench, IEnumerable<object> cases) =>
        bench.Command(JsonSerializer.Serialize(new
        {
            cmd = "run_suite",
            suite = new { name = "batch", cases = cases.ToArray() },
            datasets = Datasets(),
        }));

    [Fact]
    public void SuiteMatchesTheCasesRunAlone()
    {
        using var bench = new Benchmark();

        var alone = new List<string>();
        for (int i = 0; i < Order.Length; i++)
        {
            alone.Add(bench.Command(JsonSerializer.Serialize(new
            {
                cmd = "run_case",
                @case = Case(Order[i]),
                data = Candles(i),
            })));
        }

        var report = RunSuite(bench, Order.Select(Case));

        // Sorted by id, whatever order the cases were listed in.
        Assert.True(report.IndexOf("case-01") < report.IndexOf("case-02"), "the suite must sort by id");
        Assert.True(report.IndexOf("case-02") < report.IndexOf("case-03"), "the suite must sort by id");

        // run_case returns exactly one CaseResult, and run_suite returns those same
        // objects in an array — both canonical, both whitespace-free. So the
        // standalone response must appear verbatim inside the report. Substring
        // containment beats slicing the array apart: the recomputed report is
        // deeply nested, and a matcher that has to find its closing brace is a
        // second implementation of a JSON parser.
        for (int i = 0; i < Order.Length; i++)
        {
            Assert.Contains(alone[i], report);
        }

        Assert.Contains("\"failed\":3", report);
        Assert.Contains("\"passed\":0", report);
    }

    [Fact]
    public void CaseOrderDoesNotChangeTheReport()
    {
        using var bench = new Benchmark();
        var forwards = RunSuite(bench, Order.Select(Case));
        var backwards = RunSuite(bench, Order.Reverse().Select(Case));
        Assert.Equal(forwards, backwards);
    }
}
