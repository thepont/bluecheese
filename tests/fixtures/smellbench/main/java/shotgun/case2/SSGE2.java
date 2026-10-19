package shotgun.case2;

interface ISensorGood {
  double takeNewReading();
  double lastReading();
}

interface IDiscreteSensorGood extends shotgun.case2.ISensorGood {
  boolean status();
  void flipStatus();
  void setStatus(boolean value);

  @Override
  default double lastReading() {
    return 0;
  }

  @Override
  default double takeNewReading() {
    return 0;
  }
}

abstract class SensorGood implements shotgun.case2.ISensorGood {
  protected double currentValue;
  protected double lastValue;
  protected final shotgun.case2.BatteryGood battery;

  public SensorGood(double value, shotgun.case2.BatteryGood battery) {
    this.currentValue = this.lastValue = value;
    this.battery      = battery;
  }

  public SensorGood(double value) {
    this(value, new shotgun.case2.BatteryGood(1.0));
  }

  public SensorGood() {
    this(0);
  }

  @Override
  public double lastReading() {
    return lastValue;
  }

  public double getBatteryLevel() {
    return battery.getLevel();
  }
}

class BatteryGood {
  private double level;
  private final double minLevel;
  private final double drainPerUse;

  public BatteryGood(double initialLevel, double minLevel, double drainPerUse) {
    if (initialLevel < 0 || initialLevel > 1.0)
      throw new IllegalArgumentException("initialLevel must be 0.0–1.0");
    this.level       = initialLevel;
    this.minLevel    = minLevel;
    this.drainPerUse = drainPerUse;
  }

  public BatteryGood(double initialLevel) {
    this(initialLevel, 0.1, 0.05);
  }

  public void check() {
    if (level < minLevel) {
      throw new IllegalStateException(
              String.format("Battery too low (%.2f < %.2f)", level, minLevel)
      );
    }
  }

  public void use() {
    level = Math.max(0.0, level - drainPerUse);
  }

  public double getLevel() {
    return level;
  }
}

class AtmosphericSensorGood extends shotgun.case2.SensorGood {
  public AtmosphericSensorGood(double value, shotgun.case2.BatteryGood battery) {
    super(value, battery);
  }

  public AtmosphericSensorGood(double value) {
    super(value);
  }

  public AtmosphericSensorGood() {
    super();
  }

  @Override
  public double takeNewReading() {
    battery.check();
    lastValue     = currentValue;
    currentValue  = shotgun.case2.SensorDataGood.currentReading();
    battery.use();
    return currentValue;
  }
}

class WaterSensorGood extends shotgun.case2.SensorGood implements shotgun.case2.IDiscreteSensorGood {
  private boolean flooding;

  public WaterSensorGood(double value, shotgun.case2.BatteryGood battery) {
    super(value, battery);
    this.flooding = value > 0.5;
  }

  public WaterSensorGood(double value) {
    super(value);
  }

  public WaterSensorGood() {
    super();
  }

  @Override
  public double takeNewReading() {
    battery.check();
    lastValue     = currentValue;
    currentValue  = shotgun.case2.SensorDataGood.currentReading();
    flooding      = (currentValue > 0.5);
    battery.use();
    return currentValue;
  }

  @Override
  public boolean status() {
    return flooding;
  }

  @Override
  public void flipStatus() {
    flooding = !flooding;
  }

  @Override
  public void setStatus(boolean value) {
    flooding = value;
  }
}

class SmokeSensorGood extends shotgun.case2.SensorGood implements shotgun.case2.IDiscreteSensorGood {
  private boolean smokeDetected;
  private static final double SMOKE_THRESHOLD = 0.7;

  public SmokeSensorGood(double value, shotgun.case2.BatteryGood battery) {
    super(value, battery);
    this.smokeDetected = value > SMOKE_THRESHOLD;
  }

  public SmokeSensorGood(double value) {
    super(value);
  }

  public SmokeSensorGood() {
    super();
  }

  @Override
  public double takeNewReading() {
    battery.check();
    lastValue        = currentValue;
    currentValue     = shotgun.case2.SensorDataGood.currentReading();
    smokeDetected    = (currentValue > SMOKE_THRESHOLD);
    battery.use();
    return currentValue;
  }

  @Override
  public boolean status() {
    return smokeDetected;
  }

  @Override
  public void flipStatus() {
    smokeDetected = !smokeDetected;
  }

  @Override
  public void setStatus(boolean value) {
    smokeDetected = value;
  }
}


class SensorDataGood {
  private static double[] readings = {
          0.1, 0.4, 0.0, 0.51, 0.5, 0.7, 0.0, 2.2, 1.0
  };
  private static int counter = 0;

  public static double currentReading() {
    int idx = counter++;
    if (counter >= readings.length) counter = 0;
    return readings[idx];
  }

  public static void reset() {
    counter = 0;
  }
}
