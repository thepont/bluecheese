package messagechains.case2;

import java.time.LocalDateTime;
import java.util.List;
import java.time.LocalDate;
import java.util.Map;
import java.time.LocalTime;

class AttendanceManager {
  private final List<Employee> employees;
  private LocalDateTime lastAccessTime;

  public AttendanceManager(List<Employee> employees) {
    this.employees = employees;
  }

  public Employee getEmployee(int employeeId) {
    updateAccessTime();
    return employees.stream()
            .filter(e -> e.getId() == employeeId)
            .findFirst()
            .orElseThrow(() ->
                    new IllegalArgumentException("No employee with ID " + employeeId)
            );
  }

  public LocalDateTime getLastAccessTime() {
    return lastAccessTime;
  }

  private void updateAccessTime() {
    this.lastAccessTime = LocalDateTime.now();
  }
}

class Employee {

  private final int id;
  private final String name;
  private final Map<LocalDate, Schedule> schedules;

  public Employee(int id, String name, Map<LocalDate, Schedule> schedules) {
    this.id = id;
    this.name = name;
    this.schedules = schedules;
  }

  public int getId() {
    return id;
  }

  public Schedule getSchedule(LocalDate date) {
    return schedules.getOrDefault(date, Schedule.offDay(date));
  }
}

class Schedule {
  private final LocalDateTime start;
  private final LocalDateTime end;

  private Schedule(LocalDateTime start, LocalDateTime end) {
    this.start = start;
    this.end   = end;
  }

  public static Schedule forShift(LocalDate date, LocalTime startTime, LocalTime endTime) {
    return new Schedule(
            LocalDateTime.of(date, startTime),
            LocalDateTime.of(date, endTime)
    );
  }

  public static Schedule offDay(LocalDate date) {
    LocalDateTime dt = date.atStartOfDay();
    return new Schedule(dt, dt);
  }

  public boolean isOnShift(LocalDateTime asOf) {
    return !asOf.isBefore(start) && !asOf.isAfter(end);
  }

  public boolean isOnShift() {
    return isOnShift(LocalDateTime.now());
  }
}

class PayrollSystem {
  private final AttendanceManager manager;

  public PayrollSystem(AttendanceManager manager) {
    this.manager = manager;
  }

  public boolean isEmployeeWorking(int employeeId, LocalDate date) {
    return manager.getEmployee(employeeId).getSchedule(date).isOnShift();
  }
}