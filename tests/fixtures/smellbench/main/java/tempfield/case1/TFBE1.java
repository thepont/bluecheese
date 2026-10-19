package tempfield.case1;

class LoanBad {
    private double principal;
    private double interestRate;
    private double term;
    private double interestAmount;
    private double monthlyInterestRate;
    private int numMonths;

    public LoanBad(double principal, double interestRate, double term) {
        this.principal = principal;
        this.interestRate = interestRate;
        this.term = term;
        this.interestAmount = 0;
        this.monthlyInterestRate = interestRate / 12;
        this.numMonths = 12;
    }

    public void calculateInterest() {
        if(term < 1) {
            numMonths = (int)Math.ceil(term * 12);
            interestAmount = principal * monthlyInterestRate * numMonths;
        }
        else {
            interestAmount = principal * interestRate * term;
        }
    }

    public double returnTotalAmount() {
        return principal + interestAmount;
    }
}