package dupcode.case2;

import java.time.LocalDateTime;
import java.util.ArrayList;
import java.util.List;

enum ActionVariation {
  Tap, Scroll, Toggle;
}

class AnalyticEvent {
  private final ActionVariation action;
  private final String category;
  private final double value;

  private static final List<AnalyticEvent> db = new ArrayList<>();
  private static final StringBuilder logFile = new StringBuilder();

  public AnalyticEvent(ActionVariation action, String category, double value) {
    this.action = action;
    this.category = category;
    this.value = value;
  }

  public void captureAnalyticEvent(AnalyticEvent analyticEvent) {
    String summary = validateAndSummarize(analyticEvent, "CAPTURE");

    db.add(analyticEvent);
    System.out.println(summary);
    System.out.println("Saved to DB. Total events in DB: " + db.size() + "\n");
  }

  public void logAnalyticEvent(AnalyticEvent analyticEvent) {
    String summary = validateAndSummarize(analyticEvent, "LOG");

    String timestamp = LocalDateTime.now().toString();
    logFile.append(timestamp).append(" - ").append(summary).append("\n");
    System.out.println(summary);
    System.out.println("Appended to log.\n");
  }

  private String validateAndSummarize(AnalyticEvent event, String contextLabel) {
    if (event.value < 0) {
      System.err.println("Warning: Negative value for event");
    }

    return String.format("[%s] ActionVariation: %s, Category: %s, Value: %.2f",
      contextLabel, event.action, event.category, event.value);
  }

  public static void printLogFile() {
    System.out.println("📝 Full Log File:");
    System.out.println(logFile.toString());
  }

  public static void printDbContents() {
    System.out.println("📦 Simulated DB Contents:");
    for (AnalyticEvent e : db) {
      System.out.printf("- %s | %s | %.2f\n", e.action, e.category, e.value);
    }
  }
}