package org.wickra.benchmark;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.util.ArrayList;
import java.util.List;
import org.junit.jupiter.api.Test;

/**
 * Batch equivalence: a suite must produce exactly what the cases produce alone.
 *
 * <p>{@code run_suite} is the batch form of {@code run_case}. It fans the cases out -- over rayon
 * when the parallel feature is on -- and re-sorts the results by id before tallying, so the two
 * paths share an engine but not a control flow. Nothing else holds them to the same answer.
 */
class BatchEquivalenceTest {
    private static final String ZERO_HASH = "0".repeat(64);

    // Deliberately out of id order: a suite that only ever sees sorted input
    // cannot show that it sorts.
    private static final int[] ORDER = {3, 1, 2};

    private static String strategy(int fast, int slow) {
        return "{\"symbol\":\"BTCUSDT\",\"timeframe\":\"1h\","
                + "\"indicators\":{\"ema_fast\":{\"type\":\"Ema\",\"params\":[" + fast + "]},"
                + "\"ema_slow\":{\"type\":\"Ema\",\"params\":[" + slow + "]}},"
                + "\"entry\":{\"cross_above\":[\"ema_fast\",\"ema_slow\"]},"
                + "\"exit\":{\"cross_below\":[\"ema_fast\",\"ema_slow\"]},"
                + "\"sizing\":{\"type\":\"fixed_fraction\",\"fraction\":0.95},"
                + "\"costs\":{\"taker_bps\":5,\"slippage\":{\"type\":\"fixed_bps\",\"bps\":2}}}";
    }

    private static String candles(int seed) {
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < 40; i++) {
            double b = 100.0 + Math.sin(i * 0.4 + seed) * 8.0;
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

    private static String caseJson(int n) {
        return "{\"id\":\"case-0" + n + "\",\"description\":\"batch equivalence\","
                + "\"strategy\":" + strategy(3 + n, 12 + n) + ","
                + "\"dataset_ref\":\"d" + n + ".csv\","
                + "\"expected\":{},\"expected_hash\":\"" + ZERO_HASH + "\"}";
    }

    private static String suiteJson(boolean reversed) {
        StringBuilder sb = new StringBuilder("{\"name\":\"batch\",\"cases\":[");
        for (int i = 0; i < ORDER.length; i++) {
            int n = reversed ? ORDER[ORDER.length - 1 - i] : ORDER[i];
            if (i > 0) {
                sb.append(',');
            }
            sb.append(caseJson(n));
        }
        return sb.append("]}").toString();
    }

    private static String datasetsJson() {
        StringBuilder sb = new StringBuilder("{");
        for (int i = 0; i < ORDER.length; i++) {
            if (i > 0) {
                sb.append(',');
            }
            sb.append("\"d").append(ORDER[i]).append(".csv\":").append(candles(i));
        }
        return sb.append('}').toString();
    }

    @Test
    void suiteMatchesTheCasesRunAlone() {
        try (Benchmark benchmark = new Benchmark()) {
            List<String> alone = new ArrayList<>();
            for (int i = 0; i < ORDER.length; i++) {
                alone.add(benchmark.command("{\"cmd\":\"run_case\",\"case\":"
                        + caseJson(ORDER[i]) + ",\"data\":" + candles(i) + "}"));
            }

            String report = benchmark.command("{\"cmd\":\"run_suite\",\"suite\":"
                    + suiteJson(false) + ",\"datasets\":" + datasetsJson() + "}");

            // Sorted by id, whatever order the cases were listed in.
            assertTrue(report.indexOf("case-01") < report.indexOf("case-02"),
                    "the suite must sort by id");
            assertTrue(report.indexOf("case-02") < report.indexOf("case-03"),
                    "the suite must sort by id");

            // run_case returns exactly one CaseResult, and run_suite returns those
            // same objects in an array -- both canonical, both whitespace-free. So
            // the standalone response must appear verbatim inside the report.
            // Substring containment beats slicing the array apart: the recomputed
            // report is deeply nested, and a regex that has to find its closing
            // brace is a second implementation of a JSON parser.
            for (int i = 0; i < ORDER.length; i++) {
                assertTrue(report.contains(alone.get(i)),
                        "case-0" + ORDER[i] + " differs between the suite and the case run alone");
            }

            assertTrue(report.contains("\"failed\":3"),
                    "three placeholder cases must fail: " + report);
            assertTrue(report.contains("\"passed\":0"), "none of them can pass: " + report);
        }
    }

    @Test
    void caseOrderDoesNotChangeTheReport() {
        try (Benchmark benchmark = new Benchmark()) {
            String datasets = datasetsJson();
            String forwards = benchmark.command("{\"cmd\":\"run_suite\",\"suite\":"
                    + suiteJson(false) + ",\"datasets\":" + datasets + "}");
            String backwards = benchmark.command("{\"cmd\":\"run_suite\",\"suite\":"
                    + suiteJson(true) + ",\"datasets\":" + datasets + "}");
            assertEquals(forwards, backwards,
                    "the report depends on the order the cases were listed in");
        }
    }
}
