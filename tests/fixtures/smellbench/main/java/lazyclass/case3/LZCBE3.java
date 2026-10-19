package lazyclass.case3;

/**
 * This class will be used to represent an RGB color.
 */
class RGBBad {
  private final int red;
  private final int green;
  private final int blue;

  public RGBBad(int red, int green, int blue) {
      this.red = red;
      this.green = green;
      this.blue = blue;
  }

  public double averageScale() {
      return (red + green + blue) / 3.0;
  }
}
