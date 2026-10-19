package shotgun.case1;

class SubjectsGradingBad {
    private char subjectGrad;

    public SubjectsGradingBad() {
        this.subjectGrad = '\0';
    }

    public void setSubjectGrade(char grade) {
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

class MathClassBad {
    private SubjectsGradingBad subjectsGrading;

    public MathClassBad() {
        this.subjectsGrading = new SubjectsGradingBad();
    }

    public void setMathMarks(int marks) {
        setMathGrade(marks);
    }

    public void checkMathsGrade() {
        this.subjectsGrading.checkGrade();
    }

    private void setMathGrade(int marks) {
        if(marks >= 90) {
            this.subjectsGrading.setSubjectGrade('A');
        } else if(marks >= 80) {
            this.subjectsGrading.setSubjectGrade('B');
        } else if(marks >= 70) {
            this.subjectsGrading.setSubjectGrade('C');
        } else if(marks >= 60) {
            this.subjectsGrading.setSubjectGrade('D');
        } else {
            this.subjectsGrading.setSubjectGrade('F');
        }
    }
}

class EnglishClassBad {
    private SubjectsGradingBad subjectsGrading;

    public EnglishClassBad() {
        this.subjectsGrading = new SubjectsGradingBad();
    }

    public void setEnglishMarks(int marks) {
        setEnglishGrade(marks);
    }

    public void checkEnglishGrade() {
        this.subjectsGrading.checkGrade();
    }

    private void setEnglishGrade(int marks) {
        if(marks >= 90) {
            this.subjectsGrading.setSubjectGrade('A');
        } else if(marks >= 80) {
            this.subjectsGrading.setSubjectGrade('B');
        } else if(marks >= 70) {
            this.subjectsGrading.setSubjectGrade('C');
        } else if(marks >= 60) {
            this.subjectsGrading.setSubjectGrade('D');
        } else {
            this.subjectsGrading.setSubjectGrade('F');
        }
    }
}
public class SSBE1 {
    public static void main(String[] args) {
        MathClassBad math = new MathClassBad();
        math.setMathMarks(85);
        EnglishClassBad english = new EnglishClassBad();
        english.setEnglishMarks(75);
        math.checkMathsGrade();
        english.checkEnglishGrade();
    }
}