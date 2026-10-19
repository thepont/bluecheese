package largeclass.case2;

interface IPersonGood {
   void printPersonDetails();
}

interface IStudentGood extends IPersonGood {
  void printStudentDetails();
}

class PersonGood implements IPersonGood {
  private String name;

  public PersonGood(String name) {
    this.name = name;
  }

  public void printPersonDetails() {
    System.out.println("Name: " + this.name);
  }
}

class StudentGood extends PersonGood implements IStudentGood {
  private String studentId;
  private String major;

  public StudentGood(String name, String studentId, String major) {
    super(name);
    this.studentId = studentId;
    this.major = major;
  }

  public void printStudentDetails() {
    printPersonDetails();
    System.out.println("Student ID: " + this.studentId);
    System.out.println("Major: " + this.major);
  }
}

