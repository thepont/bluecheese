package featureenvy.case2;

import java.time.LocalDateTime;
import java.util.*;

class RemoteControllerVariation {
  private final String controllerId;
  private final Map<String, Command> commandBindings;
  private final List<String> pressHistory;
  private boolean enabled;

  public RemoteControllerVariation(String controllerId) {
    this.controllerId = controllerId;
    this.commandBindings = new HashMap<>();
    this.pressHistory = new ArrayList<>();
    this.enabled = true;
  }

  public Map<String, Command> getCommandBindings() {
    return Collections.unmodifiableMap(commandBindings);
  }

  public List<String> getPressHistory() {
    return Collections.unmodifiableList(pressHistory);
  }

  public void setEnabled(boolean enabled) {
    this.enabled = enabled;
  }

  public void setCommand(String buttonName, Command command) {
    commandBindings.put(buttonName, command);
  }

  public void pressButton(String buttonName) {
    if (!enabled) {
      System.out.println("RemoteControllerVariation '" + controllerId + "' is disabled.");
      return;
    }
    pressHistory.add(buttonName);
    Command cmd = commandBindings.get(buttonName);
    if (cmd != null) {
      System.out.println("Button '" + buttonName + "' pressed on '" + controllerId + "'");
      cmd.execute();
    } else {
      System.out.println("No command assigned to button '" + buttonName + "'");
    }
  }

  public void generateUsageReport() {
    System.out.println("-- Usage Report for Controller '" + controllerId + "' --");
    System.out.println("Enabled: " + enabled);

    Set<String> buttons = commandBindings.keySet();
    List<String> history = pressHistory;

    System.out.println("Total buttons configured: " + buttons.size());
    System.out.println("Total button presses: " + history.size());

    Map<String, Long> counts = new HashMap<>();
    for (String b : history) {
      counts.put(b, counts.getOrDefault(b, 0L) + 1);
    }

    System.out.println("Presses by button:");
    for (String b : buttons) {
      System.out.println("  " + b + ": " + counts.getOrDefault(b, 0L));
    }

    List<String> unused = new ArrayList<>();
    for (String b : buttons) {
      if (!counts.containsKey(b)) {
        unused.add(b);
      }
    }
    System.out.println("Unused buttons: " + (unused.isEmpty() ? "none" : unused));
  }

  static class Command {
    private final Runnable action;

    public Command(Runnable action) {
      this.action = action;
    }

    public void execute() {
      action.run();
    }
  }
}

class SignalProcessorVariation {
  private final Map<String, Long> pressCounts;
  private final Set<String> configuredButtons;
  private int totalPresses = 0;
  private LocalDateTime lastProcessedAt;

  public SignalProcessorVariation() {
    this.pressCounts = new HashMap<>();
    this.configuredButtons = new HashSet<>();
  }

  public void process(RemoteControllerVariation remote) {
    configuredButtons.clear();
    configuredButtons.addAll(remote.getCommandBindings().keySet());
    pressCounts.clear();
    totalPresses = 0;

    for (String btn : remote.getPressHistory()) {
      pressCounts.put(btn, pressCounts.getOrDefault(btn, 0L) + 1);
      totalPresses++;
    }

    lastProcessedAt = LocalDateTime.now();
  }

  public List<String> getInactiveButtons() {
    List<String> inactive = new ArrayList<>();
    for (String btn : configuredButtons) {
      if (!pressCounts.containsKey(btn)) {
        inactive.add(btn);
      }
    }
    return inactive;
  }

  public void reset() {
    pressCounts.clear();
    configuredButtons.clear();
    totalPresses = 0;
    lastProcessedAt = null;
    System.out.println("SignalProcessorVariation metrics have been reset.");
  }
}

enum ButtonVariation {
  LIGHTS_ON("LightsOn"),
  LIGHTS_OFF("LightsOff"),
  TEMP_UP("TempUp");

  private final String label;

  ButtonVariation(String label) {
    this.label = label;
  }

  public String label() {
    return label;
  }
}

