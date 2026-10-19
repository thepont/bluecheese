package comments.case5;

class macOSSystemVariation {
  private boolean isUpgraded;
  private String osVersion;
  private int batteryLevel;
  private int runningProgramsCount;
  private int displayLockTimeSeconds;

  private final int BATTERY_LEVEL = 80;
  private final int MAX_RUNNING_PROGRAMS = 3;
  private final int DISPLAY_LOCKTIME_SECONDS = 60;

  public macOSSystemVariation(String osVersion) {
    this.osVersion = osVersion;
    this.isUpgraded = false;
  }

  public boolean canBeUpgraded() {
    boolean batteryOk = batteryLevel > BATTERY_LEVEL;
    boolean programsOk = runningProgramsCount < MAX_RUNNING_PROGRAMS;
    boolean displayLockOk = displayLockTimeSeconds <= DISPLAY_LOCKTIME_SECONDS;

    return batteryOk && programsOk && displayLockOk;
  }

  public void setRunningProgramsCount(int runningProgramsCount) {
    this.runningProgramsCount = runningProgramsCount;
  }

  public void setDisplayLockTime(int seconds) {
    this.displayLockTimeSeconds = seconds;
  }

  public void setBatteryLevel(int level) {
    this.batteryLevel = level;
  }

  public void upgradeOS(String newVersion) {
    if (!canBeUpgraded()) {
      System.out.println("Cannot upgrade macOS: Requirements not met.");
      return;
    }

    this.osVersion = newVersion;
    this.isUpgraded = true;
    System.out.println("macOS has been successfully upgraded to version " + newVersion);
  }
}

