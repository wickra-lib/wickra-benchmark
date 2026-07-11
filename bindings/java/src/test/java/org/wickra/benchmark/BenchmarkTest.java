package org.wickra.benchmark;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.util.regex.Matcher;
import java.util.regex.Pattern;
import org.junit.jupiter.api.Test;

class BenchmarkTest {
    private static final String STRATEGY =
            "{\"symbol\":\"BTCUSDT\",\"timeframe\":\"1h\","
                    + "\"indicators\":{\"ema_fast\":{\"type\":\"Ema\",\"params\":[5]},"
                    + "\"ema_slow\":{\"type\":\"Ema\",\"params\":[15]}},"
                    + "\"entry\":{\"cross_above\":[\"ema_fast\",\"ema_slow\"]},"
                    + "\"exit\":{\"cross_below\":[\"ema_fast\",\"ema_slow\"]},"
                    + "\"sizing\":{\"type\":\"fixed_fraction\",\"fraction\":0.95},"
                    + "\"costs\":{\"taker_bps\":5,\"slippage\":{\"type\":\"fixed_bps\",\"bps\":2}}}";

    private static String candles() {
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < 40; i++) {
            double b = 100.0 + Math.sin(i * 0.4) * 8.0;
            if (i > 0) {
                sb.append(',');
            }
            sb.append("{\"time\":").append(1_700_000_000L + i * 3600L)
                    .append(",\"open\":").append(b)
                    .append(",\"high\":").append(b + 1.0)
                    .append(",\"low\":").append(b - 1.0)
                    .append(",\"close\":").append(b + 0.5)
                    .append(",\"volume\":1000.0}");
        }
        return sb.append(']').toString();
    }

    // A placeholder expected/hash: the run recomputes the real report; these
    // tests check the response shape, not that the case passes.
    private static String runCaseRequest() {
        String theCase = "{\"id\":\"sma-crossover-01\",\"description\":\"smoke\","
                + "\"strategy\":" + STRATEGY + ",\"dataset_ref\":\"d.csv\","
                + "\"expected\":{},\"expected_hash\":\"" + "0".repeat(64) + "\"}";
        return "{\"cmd\":\"run_case\",\"case\":" + theCase + ",\"data\":" + candles() + "}";
    }

    @Test
    void versionIsNonEmpty() {
        assertFalse(Benchmark.version().isEmpty());
    }

    @Test
    void runCaseHasShape() {
        try (Benchmark bench = new Benchmark()) {
            String result = bench.command(runCaseRequest());
            assertTrue(result.contains("\"id\":\"sma-crossover-01\""), result);
            Matcher m = Pattern.compile("\"hash\":\"([0-9a-f]{64})\"").matcher(result);
            assertTrue(m.find(), "missing 64-hex hash in " + result);
        }
    }

    @Test
    void runCaseIsByteStable() {
        try (Benchmark bench = new Benchmark()) {
            String req = runCaseRequest();
            assertEquals(bench.command(req), bench.command(req));
        }
    }

    @Test
    void unknownCommandIsInBandError() {
        try (Benchmark bench = new Benchmark()) {
            // An unknown command is not a hard error: the ABI returns a length and
            // the error surfaces in-band as {"ok":false,...} JSON.
            String raw = bench.command("{\"cmd\":\"nope\"}");
            assertTrue(raw.contains("\"ok\":false"), raw);
        }
    }
}
