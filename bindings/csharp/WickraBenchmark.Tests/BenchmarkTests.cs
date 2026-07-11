using System.Text.Json;
using Wickra.Benchmark;
using Xunit;

namespace WickraBenchmark.Tests;

public class BenchmarkTests
{
    private static object Strategy() => new
    {
        symbol = "BTCUSDT",
        timeframe = "1h",
        indicators = new
        {
            ema_fast = new { type = "Ema", @params = new[] { 5 } },
            ema_slow = new { type = "Ema", @params = new[] { 15 } },
        },
        entry = new { cross_above = new[] { "ema_fast", "ema_slow" } },
        exit = new { cross_below = new[] { "ema_fast", "ema_slow" } },
        sizing = new { type = "fixed_fraction", fraction = 0.95 },
        costs = new { taker_bps = 5, slippage = new { type = "fixed_bps", bps = 2 } },
    };

    private static object[] Candles()
    {
        var list = new List<object>();
        for (int i = 0; i < 40; i++)
        {
            double b = 100.0 + Math.Sin(i * 0.4) * 8.0;
            list.Add(new { time = 1_700_000_000 + i * 3600, open = b, high = b + 1.0, low = b - 1.0, close = b + 0.5, volume = 1000.0 });
        }
        return [.. list];
    }

    // A placeholder expected/hash: the run recomputes the real report; these
    // tests check the response shape, not that the case passes.
    private static string RunCaseRequest()
    {
        var theCase = new
        {
            id = "sma-crossover-01",
            description = "smoke",
            strategy = Strategy(),
            dataset_ref = "d.csv",
            expected = new { },
            expected_hash = new string('0', 64),
        };
        return JsonSerializer.Serialize(new { cmd = "run_case", @case = theCase, data = Candles() });
    }

    [Fact]
    public void Version_IsNonEmpty()
    {
        Assert.False(string.IsNullOrEmpty(Benchmark.Version()));
    }

    [Fact]
    public void RunCase_HasShape()
    {
        using var bench = new Benchmark();
        JsonElement result = JsonDocument.Parse(bench.Command(RunCaseRequest())).RootElement;
        Assert.Equal("sma-crossover-01", result.GetProperty("id").GetString());
        Assert.Equal(64, result.GetProperty("hash").GetString()!.Length);
    }

    [Fact]
    public void RunCase_IsByteStable()
    {
        using var bench = new Benchmark();
        string req = RunCaseRequest();
        Assert.Equal(bench.Command(req), bench.Command(req));
    }

    [Fact]
    public void UnknownCommand_IsInBandError()
    {
        using var bench = new Benchmark();
        // An unknown command is not a hard error: the ABI returns a length and the
        // error surfaces in-band as {"ok":false,...} JSON.
        string raw = bench.Command("{\"cmd\":\"nope\"}");
        Assert.Contains("\"ok\":false", raw);
    }
}
