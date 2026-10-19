package largeclass.case3;

import java.time.LocalDate;
import java.time.Period;
import java.util.ArrayList;
import java.util.List;
import java.util.Objects;
import java.util.UUID;

class Employee {
  private final UUID id;
  private String firstName, lastName, position;
  private double salary;
  private final LocalDate hireDate;
  private Integer badgeNumber;
  private String department;
  private List<Employee> teamMembers;

  public Employee(UUID id,
                  String firstName,
                  String lastName,
                  String position,
                  double salary,
                  LocalDate hireDate) {
    this.id = Objects.requireNonNull(id, "id must not be null");
    this.firstName = Objects.requireNonNull(firstName, "firstName must not be null");
    this.lastName = Objects.requireNonNull(lastName, "lastName must not be null");
    this.position = Objects.requireNonNull(position, "position must not be null");
    if (salary < 0) throw new IllegalArgumentException("salary must be non-negative");
    this.salary = salary;
    this.hireDate = Objects.requireNonNull(hireDate, "hireDate must not be null");
  }

  public Employee(UUID id,
                  String firstName,
                  String lastName,
                  String position,
                  double salary,
                  LocalDate hireDate,
                  Integer badgeNumber) {
    this.id = Objects.requireNonNull(id, "id must not be null");
    this.firstName = Objects.requireNonNull(firstName, "firstName must not be null");
    this.lastName = Objects.requireNonNull(lastName, "lastName must not be null");
    this.position = Objects.requireNonNull(position, "position must not be null");
    if (salary < 0) throw new IllegalArgumentException("salary must be non-negative");
    this.salary = salary;
    this.hireDate = Objects.requireNonNull(hireDate, "hireDate must not be null");
    this.badgeNumber = Objects.requireNonNull(badgeNumber, "badgeNumber must not be null");
  }

  public Employee(UUID id,
                  String firstName,
                  String lastName,
                  String position,
                  double salary,
                  LocalDate hireDate,
                  Integer badgeNumber,
                  String department) {
    this(id, firstName, lastName, position, salary, hireDate, badgeNumber);
    this.department   = Objects.requireNonNull(department,   "department must not be null");
    this.teamMembers  = new ArrayList<>();
  }

  public UUID getId() {
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

  public boolean hasBadge() {
    return badgeNumber != null;
  }

  public void swipeBadge(String location) {
    if (!hasBadge()) {
      throw new IllegalStateException("Cannot swipe badge: no badge assigned");
    }
    System.out.println(getFullName() + " swiped badge #" + badgeNumber + " at " + location);
  }

  public void renewBadge(Integer newBadgeNumber) {
    if (!hasBadge()) {
      throw new IllegalStateException("Cannot renew badge: no existing badge assigned");
    }
    this.badgeNumber = Objects.requireNonNull(newBadgeNumber, "newBadgeNumber must not be null");
    System.out.println(getFullName() + " renewed badge: new badge #" + badgeNumber);
  }

  public String badgeInfo() {
    return hasBadge() ? "Badge #" + badgeNumber : "No badge assigned";
  }

  public boolean isManager() {
    return department != null;
  }

  public String getDepartment() {
    if (!isManager()) {
      throw new IllegalStateException("Cannot get department: not a manager");
    }
    return department;
  }

  public void addTeamMember(Employee member) {
    if (!isManager()) {
      throw new IllegalStateException("Cannot add team member: not a manager");
    }
    Objects.requireNonNull(member, "member must not be null");
    teamMembers.add(member);
  }

  public boolean removeTeamMember(Employee member) {
    if (!isManager()) {
      throw new IllegalStateException("Cannot remove team member: not a manager");
    }
    return teamMembers.remove(member);
  }

  public int getTeamSize() {
    if (!isManager()) {
      throw new IllegalStateException("Cannot get team size: not a manager");
    }
    return teamMembers.size();
  }

  public void conductTeamMeeting(String agenda) {
    if (!isManager()) {
      throw new IllegalStateException("Cannot conduct meeting: not a manager");
    }
    System.out.println("Manager " + getFullName()
      + " is holding a meeting for department ‘" + department
      + "’ with agenda: " + Objects.requireNonNull(agenda));
  }
}