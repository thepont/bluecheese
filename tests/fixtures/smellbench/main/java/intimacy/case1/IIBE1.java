package intimacy.case1;

import java.util.ArrayList;
import java.util.List;
import java.util.SortedMap;
import java.util.TreeMap;

class SubjectMarksBad {
    int marks;
    SortedMap<Character, Integer> gradeScale;
    public SubjectMarksBad() {
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
}

class EnglishBad extends SubjectMarksBad {
    SubjectMarksBad subject;
    List<String> subjectTeacherList;
    public EnglishBad() {
        subject = new SubjectMarksBad();
        subjectTeacherList = new ArrayList<String>();
    }

    public Character getGrade() {
      return subject.getGrade();
    }


    public void printMarksAndGrade() {
      System.out.println("English Marks: " + marks + " Grade: " + getGrade());
    }
}

class TeacherBad {
  private EnglishBad englishSubject;
  private String name;

  public TeacherBad(String name, EnglishBad englishSubject) {
    this.name = name;
    this.englishSubject = englishSubject;
  }

  public void setMarksPrintGrade(int marks) {
    System.out.println("Grading by English teacher: " + name);
    englishSubject.marks = marks;
    englishSubject.printMarksAndGrade();
  }

}