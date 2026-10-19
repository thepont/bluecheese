package comments.case5;

class macOSSystem {
  private boolean isUpgraded;
  private String osVersion;
  private int batteryLevel;
  private int runningProgramsCount;
  private int displayLockTimeSeconds;

  private final int BATTERY_LEVEL = 80;
  private final int MAX_RUNNING_PROGRAMS = 3;
  private final int DISPLAY_LOCKTIME_SECONDS = 60;

  public macOSSystem(String osVersion) {
    this.osVersion = osVersion;
    this.isUpgraded = false;
  }

  /**
   * The method canBeUpgraded lists the conditions that the MAC can upgrade itself.
   * Only and if only when,
   *        batteryLevel > 80 and runningProgramCount < 3 and displayLockTimeSeconds <= 60.
   * @return True if the conditions are met; otherwise false.
   */
  public boolean canBeUpgraded() {
    return batteryLevel > BATTERY_LEVEL
      && runningProgramsCount < MAX_RUNNING_PROGRAMS
      && displayLockTimeSeconds <= DISPLAY_LOCKTIME_SECONDS;
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
