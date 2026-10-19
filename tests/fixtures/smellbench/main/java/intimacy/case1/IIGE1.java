package intimacy.case1;

import java.util.SortedMap;
import java.util.TreeMap;

class SubjectMarksGood {
  private int marks;
  private SortedMap<Character, Integer> gradeScale;
  public SubjectMarksGood() {
    this.marks = 0;
    this.gradeScale = new TreeMap<>();
    gradeScale.put('A', 90);
    gradeScale.put('B', 80);
    gradeScale.put('C', 70);
    gradeScale.put('D', 60);
    gradeScale.put('E', 50);
    gradeScale.put('F', 0);
  }
  public Character getGrade() {
    for(SortedMap.Entry<Character, Integer> entry : gradeScale.entrySet()) {
      if(marks >= entry.getValue()) {
        return entry.getKey();
      }
    }
    return 'F';
  }

  public int getMarks() {
    return marks;
  }

  public void setMarks(int marks) {
    this.marks = marks;
  }
}

class EnglishGood extends SubjectMarksGood {
  public EnglishGood() {
    super();
  }

  public void printMarksAndGrade() {
    System.out.println("English Marks: " + getMarks() + " Grade: " + getGrade());
  }
}

class TeacherGood {
  private EnglishGood englishSubject;
  private String name;

  public TeacherGood(String name, EnglishGood englishSubject) {
    this.name = name;
    this.englishSubject = englishSubject;
  }

  public void setMarksPrintGrade(int marks) {
    System.out.println("Grading by English teacher: " + name);
    englishSubject.setMarks(marks);
    englishSubject.printMarksAndGrade();
  }

}
