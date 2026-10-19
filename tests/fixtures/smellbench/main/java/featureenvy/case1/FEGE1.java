package featureenvy.case1;

class Coordinates2DGood {
  private int x;
  private int y;

  public Coordinates2DGood(int x, int y) {
    this.x = x;
    this.y = y;
  }

  public int getX() { return x; }
  public int getY() { return y; }
  public int calculateDistance(Coordinates2DGood point) {
    return (int) Math.sqrt(Math.pow(x - point.getX(), 2) + Math.pow(y - point.getY(), 2));
  }

  public boolean sameX(Coordinates2DGood point) {
    return x == point.getX();
  }

  public boolean sameY(Coordinates2DGood point) {
      return y == point.getY();
  }
}

class Rectangle2DSpaceGood {
  private final int length;
  private final int breadth;

  public Rectangle2DSpaceGood(Coordinates2DGood [] points) throws IllegalArgumentException {
    checkPointsArraySize(points);
    checkRectangle(points);
    this.length = points[0].calculateDistance(points[1]);
    this.breadth = points[0].calculateDistance(points[3]);
  }

  public int calculateArea() {
    return length * breadth;
  }

  public int calculatePerimeter() {
    return 2 * (length + breadth);
  }

  private void checkPointsArraySize(Coordinates2DGood [] points) throws IllegalArgumentException {
    if(points.length != 4) {
      throw new IllegalArgumentException("Rectangle should have 4 points");
    }
  }

  private void checkRectangle(Coordinates2DGood [] points) throws IllegalArgumentException {
    if(!points[0].sameX(points[1])  || !points[0].sameY(points[3])  ||
            !points[1].sameY(points[2]) || !points[2].sameX(points[3]) ) {
      throw new IllegalArgumentException("Points do not form a rectangle");
    }
  }


}