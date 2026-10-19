package shotgun.case3;

import java.time.LocalDateTime;
import java.util.HashMap;
import java.util.Map;

interface IBankAccountGood {
    public void deposit(double depositAmount) throws IllegalArgumentException;

    public double getBalance();

    public void printAccountDetails();
}

class BankAccountGood implements shotgun.case3.IBankAccountGood {
    private String name;
    private int id;
    private double balance;

    public BankAccountGood(String name, int id, double balance) {
        this.name = name;
        this.id = id;
        this.balance = balance;
    }

    public void withdraw(double withdrawAmount) throws IllegalArgumentException {
        if (withdrawAmount > balance) {
            throw new IllegalArgumentException("Not enough balance");
        }
        this.balance -= withdrawAmount;
        System.out.println("Successful deposit. New balance: " + this.balance);
    }

    @Override
    public void deposit(double depositAmount) throws IllegalArgumentException {
        if (depositAmount < 0) {
            throw new IllegalArgumentException("Negative deposit amount");
        }
        this.balance += depositAmount;
        System.out.println("Successful withdrawal. New balance: " + this.balance);
    }

    @Override
    public double getBalance() {
        return this.balance;
    }

    public int getId() {
        return this.id;
    }

    @Override
    public void printAccountDetails() {
        System.out.println("Id: " + this.id + "\nName: " + this.name);
    }

}

class DirectDepositProcessorGood {
    private shotgun.case3.BankAccountGood account;
    private Map<LocalDateTime, Double> depositHistory;

    public DirectDepositProcessorGood(shotgun.case3.BankAccountGood account) {
        this.account = account;
        this.depositHistory = new HashMap<>();
    }

    public void depositSalary(double salary) {
        LocalDateTime currentDateTime = LocalDateTime.now();
        account.deposit(salary);
        depositHistory.put(currentDateTime, salary);
        System.out.println("Successfully put salary into account: ");
        account.printAccountDetails();
        System.out.println("New balance: " + account.getBalance());
    }

    public void printSalaryHistory() {
        for (Map.Entry<LocalDateTime, Double> entry : depositHistory.entrySet()) {
            System.out.println(entry.getKey() + " : " + entry.getValue());
        }
    }

}

class CheckProcessorGood {
    private shotgun.case3.BankAccountGood account;

    public CheckProcessorGood(shotgun.case3.BankAccountGood account) {
        this.account = account;
    }

    public void validateCheque(shotgun.case3.BankAccountGood sender, double amount) throws IllegalArgumentException {
        if (sender.getBalance() < amount) {
            throw new IllegalArgumentException("Insufficient funds. Cheque bounced");
        }
        if (sender.getId() == account.getId()) {
            throw new IllegalArgumentException("Cannot send cheque to receiver");
        }
        sender.withdraw(amount);
        account.deposit(amount);
        generateReceipt(sender, amount);
    }

    private void generateReceipt(shotgun.case3.BankAccountGood sender, double amount) {
        System.out.println("-------------------");
        System.out.println(LocalDateTime.now().toString() + "\n");
        System.out.println("===================");
        System.out.println("Sender Information:");
        sender.printAccountDetails();
        System.out.println("Receiver Information:");
        account.printAccountDetails();
        System.out.println("Amount transferred: " + amount);
        System.out.println("-------------------");
    }

}