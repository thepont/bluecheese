package shotgun.case1;

class SubjectsGradingGood {
  private char subjectGrad;

  public SubjectsGradingGood() {
    this.subjectGrad = '\0';
  }

  public void setGradeBasedOnMarks(int marks) {
    if(marks >= 90) {
      setGrade('A');
    } else if(marks >= 80) {
      setGrade('B');
    } else if(marks >= 70) {
      setGrade('C');
    } else if(marks >= 60) {
      setGrade('D');
    } else {
      setGrade('F');
    }
  }

  private void setGrade(char grade) {
    this.subjectGrad = grade;
  }


  public void checkGrade() {
    if (this.subjectGrad == 'A') {
      System.out.println("Excellent");
    } else if (this.subjectGrad == 'B') {
      System.out.println("Good");
    } else if (this.subjectGrad == 'C') {
      System.out.println("Average");
    } else if (this.subjectGrad == 'D') {
      System.out.println("Poor");
    } else {
      System.out.println("Fail");
    }
  }
}

class MathClassGood {
  private SubjectsGradingGood subjectsGrading;

  public MathClassGood() {
    this.subjectsGrading = new SubjectsGradingGood();
  }

  public void setMathMarks(int marks) {
    subjectsGrading.setGradeBasedOnMarks(marks);
  }

  public void checkMathsGrade() {
    this.subjectsGrading.checkGrade();
  }
}

class EnglishClassGood {
  private SubjectsGradingGood subjectsGrading;

  public EnglishClassGood() {
    this.subjectsGrading = new SubjectsGradingGood();
  }

  public void setEnglishMarks(int marks) {
    subjectsGrading.setGradeBasedOnMarks(marks);
  }

  public void checkEnglishGrade() {
    this.subjectsGrading.checkGrade();
  }
}
