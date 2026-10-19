package lazyclass.case4;

import java.time.LocalDate;
import java.time.Period;
import java.util.Objects;

class EmployeeVariation {
  private final String id;
  private String firstName;
  private String lastName;
  private String position;
  private double salary;
  private final LocalDate hireDate;
  private Integer badgeNumber;

  public EmployeeVariation(String id, String firstName, String lastName,
                  String position, double salary, LocalDate hireDate) {
    this.id = Objects.requireNonNull(id, "id must not be null");
    this.firstName = Objects.requireNonNull(firstName, "firstName must not be null");
    this.lastName = Objects.requireNonNull(lastName, "lastName must not be null");
    this.position = Objects.requireNonNull(position, "position must not be null");
    if (salary < 0) throw new IllegalArgumentException("salary must be non-negative");
    this.salary = salary;
    this.hireDate = Objects.requireNonNull(hireDate, "hireDate must not be null");
  }

  public EmployeeVariation(String id, String firstName, String lastName,
                  String position, double salary, LocalDate hireDate, Integer badgeNumber) {
    this.id = Objects.requireNonNull(id, "id must not be null");
    this.firstName = Objects.requireNonNull(firstName, "firstName must not be null");
    this.lastName = Objects.requireNonNull(lastName, "lastName must not be null");
    this.position = Objects.requireNonNull(position, "position must not be null");
    if (salary < 0) throw new IllegalArgumentException("salary must be non-negative");
    this.salary = salary;
    this.hireDate = Objects.requireNonNull(hireDate, "hireDate must not be null");
    this.badgeNumber = Objects.requireNonNull(badgeNumber, "badgeNumber must not be null");
  }

  public String getId() {
    return id;
  }

  public String getFirstName() {
    return firstName;
  }

  public void setFirstName(String firstName) {
    this.firstName = Objects.requireNonNull(firstName);
  }

  public String getLastName() {
    return lastName;
  }

  public void setLastName(String lastName) {
    this.lastName = Objects.requireNonNull(lastName);
  }

  public String getPosition() {
    return position;
  }

  public void setPosition(String position) {
    this.position = Objects.requireNonNull(position);
  }

  public double getSalary() {
    return salary;
  }

  public void giveRaise(double percent) {
    if (percent < 0) throw new IllegalArgumentException("percent must be non-negative");
    this.salary += this.salary * percent / 100.0;
  }

  public LocalDate getHireDate() {
    return hireDate;
  }

  public int getYearsEmployed() {
    return Period.between(hireDate, LocalDate.now()).getYears();
  }

  public String getFullName() {
    return firstName + " " + lastName;
  }

  public Integer getBadgeNumber() {
    return this.badgeNumber;
  }

  public void setBadgeNumber(Integer badgeNumber) {
    this.badgeNumber = Objects.requireNonNull(badgeNumber);
  }
}