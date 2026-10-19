package featureenvy.case1;

class Coordinates2DBad {
    private final int x;
    private final int y;

    public Coordinates2DBad(int x, int y) {
        this.x = x;
        this.y = y;
    }

    public int getX() { return x; }
    public int getY() { return y; }
}

class Rectangle2DSpaceBad {
    private final int length;
    private final int breadth;

    public Rectangle2DSpaceBad(Coordinates2DBad [] points) throws IllegalArgumentException {
        checkPointsArraySize(points);
        checkRectangle(points);
        this.length = calculateDistance(points[0], points[1]);
        this.breadth = calculateDistance(points[0], points[3]);
    }

    public int calculateArea() {
        return length * breadth;
    }

    public int calculatePerimeter() {
        return 2 * (length + breadth);
    }

    private void checkPointsArraySize(Coordinates2DBad [] points) throws IllegalArgumentException {
      if(points.length != 4) {
        throw new IllegalArgumentException("Rectangle should have 4 points");
      }
    }

    private void checkRectangle(Coordinates2DBad [] points) throws IllegalArgumentException {
      if(points[0].getX() != points[1].getX() || points[0].getY() != points[3].getY() ||
         points[1].getY() != points[2].getY() || points[2].getX() != points[3].getX()) {
        throw new IllegalArgumentException("Points do not form a rectangle");
      }
    }

    private int calculateDistance(Coordinates2DBad point1, Coordinates2DBad point2) {
      return (int) Math.sqrt(Math.pow(point1.getX() - point2.getX(), 2) + Math.pow(point1.getY() - point2.getY(), 2));
    }


}