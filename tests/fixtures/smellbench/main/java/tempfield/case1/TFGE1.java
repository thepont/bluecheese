package tempfield.case1;

class LoanGood {
  private double principal;
  private double interestRate;
  private double term;
  private double interestAmount;
  private MonthlyInterestCalculatorGood monthlyInterestCalculator;


  public LoanGood(double principal, double interestRate, double term) {
    this.principal = principal;
    this.interestRate = interestRate;
    this.term = term;
    this.interestAmount = 0;
    monthlyInterestCalculator = new MonthlyInterestCalculatorGood();
  }

  public void calculateInterest() {
    if(term < 1) {
      monthlyInterestCalculator = new MonthlyInterestCalculatorGood(interestRate, term);
      interestAmount = monthlyInterestCalculator.calculateInterestAmount(principal);
    }
    else {
      interestAmount = principal * interestRate * term;
    }
  }

  public double returnTotalAmount() {
    return principal + interestAmount;
  }
}

class MonthlyInterestCalculatorGood {

  private double monthlyInterestRate;
  private int numMonths;
  public MonthlyInterestCalculatorGood() {
    this.monthlyInterestRate = 0;
    this.numMonths = 0;
  }
  public MonthlyInterestCalculatorGood(double interestRate, double term) {
    this.monthlyInterestRate = interestRate / 12;
    this.numMonths = (int)Math.ceil(term * 12);
  }

  public double calculateInterestAmount(double principal) {
    return principal * monthlyInterestRate * numMonths;
  }
}