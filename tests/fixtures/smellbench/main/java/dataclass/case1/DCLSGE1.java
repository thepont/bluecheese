package dataclass.case1;

class RectangleGood {
  private final double height;
  private final double width;
  public RectangleGood(double height, double width) {
    this.height = height;
    this.width = width;
  }
  public double getPerimeter() {
    return 2 * (height + width);
  }
  public double getArea() {
      return height * width;
  }
}
