package dupcode.case2;

import java.time.LocalDateTime;
import java.util.ArrayList;
import java.util.List;

enum Action {
  Tap, Scroll, Toggle;
}

class AnalyticEventVariation {
  private final Action action;
  private final String category;
  private final double value;

  private static final List<AnalyticEventVariation> db = new ArrayList<>();
  private static final StringBuilder logFile = new StringBuilder();

  public AnalyticEventVariation(Action action, String category, double value) {
    this.action = action;
    this.category = category;
    this.value = value;
  }

  public void captureAnalyticEventVariation(AnalyticEventVariation AnalyticEventVariation) {
    if (AnalyticEventVariation.value < 0) {
      System.err.println("Warning: Negative value for event");
    }

    String summary = String.format(
      "[CAPTURE] Action: %s, Category: %s, Value: %.2f",
      AnalyticEventVariation.action, AnalyticEventVariation.category, AnalyticEventVariation.value
    );

    db.add(AnalyticEventVariation);
    System.out.println(summary);
    System.out.println("Saved to DB. Total events in DB: " + db.size() + "\n");
  }

  public void logAnalyticEventVariation(AnalyticEventVariation AnalyticEventVariation) {
    if (AnalyticEventVariation.value < 0) {
      System.err.println("Warning: Negative value for event");
    }

    String summary = String.format(
      "[LOG] Action: %s, Category: %s, Value: %.2f",
      AnalyticEventVariation.action, AnalyticEventVariation.category, AnalyticEventVariation.value
    );

    String timestamp = LocalDateTime.now().toString();
    logFile.append(timestamp).append(" - ").append(summary).append("\n");
    System.out.println(summary);
    System.out.println("Appended to log.\n");
  }

  public static void printLogFile() {
    System.out.println("📝 Full Log File:");
    System.out.println(logFile.toString());
  }

  public static void printDbContents() {
    System.out.println("📦 Simulated DB Contents:");
    for (AnalyticEventVariation e : db) {
      System.out.printf("- %s | %s | %.2f\n", e.action, e.category, e.value);
    }
  }
}
