package primobsession.case2;

interface StaffGood {
  void printStaffDetails();
}
abstract class TeachingStaffGood implements StaffGood {
  private String empName;
  private String empId;
  private double salary;

  public TeachingStaffGood(String empName, String empId) {
    validateEmployeeId(empId);
    this.empName = empName;
    this.empId = empId;
    this.salary = setSalary();
  }

  public void printStaffDetails() {
    System.out.println("Employee Name: " + empName);
    System.out.println("Employee ID: " + empId);
    printEmployeeType();
    System.out.println("Employee Salary: " + salary);
  }

  protected double setSalary() {
    throw new IllegalArgumentException("Invalid employee type");
  }

  protected void printEmployeeType() {
    throw new IllegalArgumentException("Invalid employee type");
  }

  private void validateEmployeeId(String empId) {
    if(empId.length() != 6) {
      throw new IllegalArgumentException("Invalid employee id");
    }
  }
}

class ProfessorGood extends TeachingStaffGood {
  public ProfessorGood(String empName, String empId) {
    super(empName, empId);
  }

  @Override
  protected double setSalary() {
      return 100000;
  }

  @Override
  protected void printEmployeeType() {
      System.out.println("Employee Type: Professor");
  }
}

class AssistantProfessorGood extends TeachingStaffGood {
  public AssistantProfessorGood(String empName, String empId) {
    super(empName, empId);
  }

  @Override
  protected double setSalary() {
      return 80000;
  }
  @Override
  protected void printEmployeeType() {
    System.out.println("Employee Type: Assistant Professor");
  }

}

class AssociateProfessorGood extends TeachingStaffGood {
  public AssociateProfessorGood(String empName, String empId) {
    super(empName, empId);
  }

  @Override
  protected double setSalary() {
      return 50000;
  }

  @Override
  protected void printEmployeeType() {
    System.out.println("Employee Type: Associate Professor");
  }
}