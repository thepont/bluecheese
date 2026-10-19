package longmethod.case1;

import java.util.ArrayDeque;
import java.util.Date;
import java.util.Deque;

enum StatusBad {
    ACTIVE,
    INACTIVE
}

enum StandingBad {
    GOOD,
    BAD
}

enum TransactionStatusBad {
    SUCCESS,
    FAILURE,
    PENDING
}

class CardDetailsBad {
    private final String cardNumber;
    private final String cardHolder;
    private final Date expiryDate;
    private final int cvv;

    public CardDetailsBad(String cardNumber, String cardHolder, Date expiryDate, int cvv) {
        this.cardNumber = cardNumber;
        this.cardHolder = cardHolder;
        this.expiryDate = expiryDate;
        this.cvv = cvv;
    }

    public boolean equals(CardDetailsBad cardDetails) {
        return cardNumber.equals(cardDetails.cardNumber) && cardHolder.equals(cardDetails.cardHolder) && expiryDate.equals(cardDetails.expiryDate) && cvv == cardDetails.cvv;
    }
}

class TransactionBad {
    private String txnId;
    private float amount;
    private Date date;
    private TransactionStatusBad status;

    public TransactionBad(String txnId, float amount, Date date) {
        this.txnId = txnId;
        this.amount = amount;
        this.date = date;
        this.status = TransactionStatusBad.PENDING;
    }

    public void updateStatus(TransactionStatusBad status) {
        this.status = status;
    }

    public TransactionStatusBad getStatus() {
        return this.status;
    }

    public  void printTransactionDetails() {
        System.out.println("longmethod.case1.TransactionBad ID: " + txnId);
        System.out.println("Amount: " + amount);
        System.out.println("Date: " + date);
        System.out.println("Status: " + status);
    }
}

class CreditCardBad {
    private CardDetailsBad cardDetails;
    private float creditLimit;
    private float availableBalance;
    private StatusBad status;
    private StandingBad standing;
    private Deque<TransactionBad> txnHistory;

    public CreditCardBad(CardDetailsBad cardDetails) {
        this.cardDetails = cardDetails;
        this.creditLimit = 10000f;
        this.availableBalance = 10000f;
        this.status = StatusBad.ACTIVE;
        this.standing = StandingBad.GOOD;
        txnHistory = new ArrayDeque<>();
    }

    public TransactionBad makeTransaction(float amount, CardDetailsBad cardDetails, Date transactionDate) {
        String txnId = "TXN" + (int) (Math.random() * 1000);
        TransactionBad txn = new TransactionBad(txnId, amount, transactionDate);
        TransactionStatusBad transactionStatus = TransactionStatusBad.PENDING;
        if(!this.cardDetails.equals(cardDetails)) {
            System.out.println("Invalid card details. Please check and try again.");
            transactionStatus = TransactionStatusBad.FAILURE;
        } else if(status == StatusBad.INACTIVE) {
            System.out.println("Card is inactive. Please contact customer service.");
            transactionStatus = TransactionStatusBad.FAILURE;
        } else if(standing == StandingBad.BAD) {
            System.out.println("Card is in bad standing. Please contact customer service.");
            transactionStatus = TransactionStatusBad.FAILURE;
        } else if(amount > creditLimit) {
            System.out.println("Amount exceeds credit limit. Please try again.");
            transactionStatus = TransactionStatusBad.FAILURE;
        } else if(amount > availableBalance) {
            System.out.println("Amount exceeds available balance. Please try again.");
            transactionStatus = TransactionStatusBad.FAILURE;
        }

        if(transactionStatus == TransactionStatusBad.FAILURE) {
            txn.updateStatus(transactionStatus);
            txnHistory.push(txn);
            return txn;
        }
        else {
            availableBalance -= amount;
            transactionStatus = TransactionStatusBad.SUCCESS;
            txn.updateStatus(transactionStatus);
            txnHistory.push(txn);
            System.out.println("longmethod.case1.TransactionBad successful. Available balance: " + availableBalance);
            return txn;
        }
    }
}