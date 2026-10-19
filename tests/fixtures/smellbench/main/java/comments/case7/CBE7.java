package comments.case7;

class SmartThermostat {
  private boolean isUpgraded;
  private String firmwareVersion;
  private int currentTemp;
  private int connectedDevices;
  private boolean powerStable;

  private final int MIN_TEMP = 18;
  private final int MAX_CONNECTED_DEVICES = 5;

  public SmartThermostat(String version) {
    this.firmwareVersion = version;
    this.isUpgraded = false;
  }

  public void setCurrentTemp(int temp) {
    this.currentTemp = temp;
  }

  public void setConnectedDevices(int count) {
    this.connectedDevices = count;
  }

  public void setPowerStability(boolean isStable) {
    this.powerStable = isStable;
  }

  public boolean getUCC() {
    // Three "upgrade condition check" has to be made before the device can be updated.
    // UCC stands for term - "upgrade condition check".
    boolean tempOk = currentTemp >= MIN_TEMP;
    boolean devicesOk = connectedDevices <= MAX_CONNECTED_DEVICES;
    boolean powerOk = powerStable;

    return tempOk && devicesOk && powerOk;
  }

  public void upgradeFirmware(String newVersion) {
    if (!(getUCC())) {
      System.out.println("Cannot upgrade firmware: Requirements not met.");
      return;
    }

    this.firmwareVersion = newVersion;
    this.isUpgraded = true;
    System.out.println("Thermostat upgraded to firmware version " + newVersion);
  }
}
