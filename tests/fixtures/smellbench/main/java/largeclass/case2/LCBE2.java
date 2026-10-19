package largeclass.case2;

interface IPersonBad {
  void printPersonDetails();
  void printStudentDetails();
}
class PersonBad implements IPersonBad {
  private String name;
  private String studentId;
  private String major;

  public PersonBad(String name, String studentId, String major) {
    this.name = name;
    this.studentId = studentId;
    this.major = major;
  }

  public void printPersonDetails() {
    System.out.println("Name: " + this.name);
  }

  public void printStudentDetails() {
    printPersonDetails();
    System.out.println("Student ID: " + this.studentId);
    System.out.println("Major: " + this.major);
  }
}

