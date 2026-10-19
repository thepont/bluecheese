package longparams.case3;

class Coordinates2DGood {
  private double x;
  private double y;
  Coordinates2DGood(double x, double y) {
    this.x = x;
    this.y = y;
  }

  public double getDistance(Coordinates2DGood other) {
    return Math.sqrt(Math.pow(this.x - other.x, 2) + Math.pow(this.y - other.y, 2));
  }
}
class TriangleGood {
  private Coordinates2DGood point1;
  private Coordinates2DGood point2;
  private Coordinates2DGood point3;

  TriangleGood(Coordinates2DGood point1, Coordinates2DGood point2, Coordinates2DGood point3) {
    this.point1 = point1;
    this.point2 = point2;
    this.point3 = point3;
  }

  public double getPerimeter() {
    return point1.getDistance(point2) + point2.getDistance(point3) + point3.getDistance(point1);
  }

  public double getArea() {
    double a = point1.getDistance(point2);
    double b = point2.getDistance(point3);
    double c = point3.getDistance(point1);
    double s = (a + b + c) / 2;
    return Math.sqrt(s * (s - a) * (s - b) * (s - c));
  }
}
