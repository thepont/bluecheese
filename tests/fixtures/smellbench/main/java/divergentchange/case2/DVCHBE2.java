package divergentchange.case2;

import java.util.Scanner;
import java.util.logging.Logger;

enum ShapeType {
  CIRCLE, RECTANGLE, SQUARE, TRIANGLE
}

class Shape {
  private final ShapeType type;
  private int x, y;
  private double sizeParam;

  public Shape(ShapeType type, int x, int y, double sizeParam) {
    this.type = type;
    this.x = x;
    this.y = y;
    this.sizeParam = sizeParam;
  }

  public ShapeType getType() {
    return type;
  }

  public int getX() {
    return x;
  }

  public int getY() {
    return y;
  }

  public double getSizeParam() {
    return sizeParam;
  }

  public void setSizeParam(double s) {
    this.sizeParam = s;
  }

  public void setPosition(int nx, int ny) {
    x = nx; y = ny;
  }

  public String toString() {
    return String.format(
      "divergentchange.case2.Shape[type=%s, x=%d, y=%d, sizeParam=%.2f]",
      type, x, y, sizeParam
    );
  }
}

class LucyChartService {
  private String userId, password;
  private Integer numberOfOperations;

  private static final Logger logger = Logger.getLogger(LucyChartService.class.getName());

  public LucyChartService() {
    this.numberOfOperations = 0;
    Scanner scanner = new Scanner(System.in);

    logger.info("Prompting for userId");
    System.out.print("Enter userId: ");
    this.userId = scanner.nextLine().trim();

    logger.info("Prompting for password");
    System.out.print("Enter password: ");
    this.password = scanner.nextLine().trim();

    logger.info("Initialization complete for userId=" + this.userId);
  }

  public void draw(Shape s) {
    this.numberOfOperations += 1;

    switch(s.getType()) {
      case CIRCLE:
        System.out.println("Draw circle at (" + s.getX() + "," + s.getY()
          + ") with radius " + s.getSizeParam());
        break;
      case RECTANGLE:
        System.out.println("Draw rectangle at (" + s.getX() + "," + s.getY()
          + ") with width/height " + s.getSizeParam());
        break;
      case SQUARE:
        System.out.println("Draw square at (" + s.getX() + "," + s.getY()
          + ") with side " + s.getSizeParam());
      case TRIANGLE:
        System.out.println("Draw triangle at (" + s.getX() + "," + s.getY()
          + ") with side length " + s.getSizeParam());
        break;

      default:
        throw new IllegalArgumentException("Unknown shape: " + s.getType());
    }
  }

  public void resize(Shape s, double factor) {
    this.numberOfOperations += 1;

    switch(s.getType()) {
      case CIRCLE:
        System.out.println("Resizing circle by a factor of " + factor);
        s.setSizeParam(s.getSizeParam() * factor);
        break;
      case RECTANGLE:
        System.out.println("Resizing rectangle by a factor of " + factor);
        s.setSizeParam(s.getSizeParam() * factor);
        break;
      case SQUARE:
        System.out.println("Resizing square by a factor of " + factor);
        s.setSizeParam(s.getSizeParam() * factor);
        break;
      case TRIANGLE:
        System.out.println("Resizing triangle by a factor of " + factor);
        s.setSizeParam(s.getSizeParam() * factor);
        break;
      default:
        throw new IllegalArgumentException("Cannot resize unknown shape: " + s.getType());
    }
  }

  public void move(Shape s, int deltaX, int deltaY) {
    this.numberOfOperations += 1;

    switch (s.getType()) {
      case CIRCLE:
        System.out.println("Moving circle from ("
          + s.getX() + "," + s.getY() + ") by ("
          + deltaX + "," + deltaY + ")");
        s.setPosition(s.getX() + deltaX, s.getY() + deltaY);
        break;
      case RECTANGLE:
        System.out.println("Moving rectangle from ("
          + s.getX() + "," + s.getY() + ") by ("
          + deltaX + "," + deltaY + ")");
        s.setPosition(s.getX() + deltaX, s.getY() + deltaY);
        break;
      case SQUARE:
        System.out.println("Moving square from ("
          + s.getX() + "," + s.getY() + ") by ("
          + deltaX + "," + deltaY + ")");
        s.setPosition(s.getX() + deltaX, s.getY() + deltaY);
        break;
      case TRIANGLE:
        System.out.println("Moving triangle from ("
          + s.getX() + "," + s.getY() + ") by ("
          + deltaX + "," + deltaY + ")");
        s.setPosition(s.getX() + deltaX, s.getY() + deltaY);
        break;
      default:
        throw new IllegalArgumentException(
          "Cannot move unknown shape: " + s.getType()
        );
    }
  }

  public void printOutUsageFact() {
    System.out.printf(
      "Fact: user %s has %d operations on LucyChart.%n",
      userId,
      numberOfOperations
    );
  }
}

