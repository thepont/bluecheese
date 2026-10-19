package messagechains.case2;

import java.time.LocalDate;
import java.util.Map;
import java.util.function.Function;
import java.util.stream.Collectors;
import java.util.List;
import java.time.LocalDateTime;
import java.time.LocalTime;

class AttendanceManagerGood {
  private final Map<Integer, messagechains.case2.EmployeeGood> employeesById;
  private LocalDateTime lastAccessTime;

  public AttendanceManagerGood(List<messagechains.case2.EmployeeGood> employees) {
    this.employeesById = employees.stream()
            .collect(Collectors.toMap(messagechains.case2.EmployeeGood::getId, Function.identity()));
  }

  public messagechains.case2.ScheduleGood getSchedule(int employeeId, LocalDate date) {
    updateAccessTime();
    messagechains.case2.EmployeeGood emp = employeesById.get(employeeId);
    if (emp == null) {
      throw new IllegalArgumentException("No employee with ID " + employeeId);
    }
    return emp.getSchedule(date);
  }

  public boolean isEmployeeOnShift(int employeeId, LocalDate date) {
    updateAccessTime();
    return getSchedule(employeeId, date).isOnShift();
  }

  public boolean isEmployeeOnShift(int employeeId, LocalDate date, java.time.LocalDateTime asOf) {
    updateAccessTime();
    return getSchedule(employeeId, date).isOnShift(asOf);
  }

  public LocalDateTime getLastAccessTime() {
    return lastAccessTime;
  }

  private void updateAccessTime() {
    this.lastAccessTime = LocalDateTime.now();
  }
}

class EmployeeGood {
  private final int id;
  private final String name;
  private final Map<LocalDate, messagechains.case2.ScheduleGood> schedules;

  public EmployeeGood(int id, String name, Map<LocalDate, messagechains.case2.ScheduleGood> schedules) {
    this.id = id;
    this.name = name;
    this.schedules = schedules;
  }

  public int getId() {
    return id;
  }

  messagechains.case2.ScheduleGood getSchedule(LocalDate date) {
    return schedules.getOrDefault(date, messagechains.case2.ScheduleGood.offDay(date));
  }
}

class ScheduleGood {
  private final LocalDateTime start;
  private final LocalDateTime end;

  private ScheduleGood(LocalDateTime start, LocalDateTime end) {
    this.start = start;
    this.end   = end;
  }

  public static messagechains.case2.ScheduleGood forShift(LocalDate date, LocalTime startTime, LocalTime endTime) {
    return new messagechains.case2.ScheduleGood(
            LocalDateTime.of(date, startTime),
            LocalDateTime.of(date, endTime)
    );
  }

  public static messagechains.case2.ScheduleGood offDay(LocalDate date) {
    LocalDateTime dt = date.atStartOfDay();
    return new messagechains.case2.ScheduleGood(dt, dt);
  }

  protected boolean isOnShift(LocalDateTime asOf) {
    return !asOf.isBefore(start) && !asOf.isAfter(end);
  }

  protected boolean isOnShift() {
    return isOnShift(LocalDateTime.now());
  }
}