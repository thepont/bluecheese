package longmethod.case1;

import java.util.ArrayDeque;
import java.util.Date;
import java.util.Deque;

enum StatusGood {
  ACTIVE,
  INACTIVE
}

enum StandingGood {
  GOOD,
  BAD
}

enum TransactionStatusGood {
  SUCCESS,
  FAILURE,
  PENDING
}

class CardDetailsGood {
    private final String cardNumber;
    private final String cardHolder;
    private final Date expiryDate;
    private final int cvv;

    public CardDetailsGood(String cardNumber, String cardHolder, Date expiryDate, int cvv) {
        this.cardNumber = cardNumber;
        this.cardHolder = cardHolder;
        this.expiryDate = expiryDate;
        this.cvv = cvv;
    }

    public boolean equals(CardDetailsGood cardDetails) {
        return cardNumber.equals(cardDetails.cardNumber) && cardHolder.equals(cardDetails.cardHolder) && expiryDate.equals(cardDetails.expiryDate) && cvv == cardDetails.cvv;
    }
}

class TransactionGood {
  private String txnId;
  private float amount;
  private Date date;
  private TransactionStatusGood status;

  public TransactionGood(String txnId, float amount, Date date) {
    this.txnId = txnId;
    this.amount = amount;
    this.date = date;
    this.status = TransactionStatusGood.PENDING;
  }

  public void updateStatus(TransactionStatusGood status) {
    this.status = status;
  }

  public TransactionStatusGood getStatus() {
    return this.status;
  }

  public  void printTransactionDetails() {
    System.out.println("TransactionBad ID: " + txnId);
    System.out.println("Amount: " + amount);
    System.out.println("Date: " + date);
    System.out.println("Status: " + status);
}
}

class CreditCardGood {
  private CardDetailsGood cardDetails;
  private float creditLimit;
  private float availableBalance;
  private StatusGood status;
  private StandingGood standing;
  private Deque<TransactionGood> txnHistory;

  public CreditCardGood(CardDetailsGood cardDetails) {
    this.cardDetails = cardDetails;
    this.creditLimit = 10000f;
    this.availableBalance = 10000f;
    this.status = StatusGood.ACTIVE;
    this.standing = StandingGood.GOOD;
    txnHistory = new ArrayDeque<>();
  }

  public TransactionGood makeTransaction(float amount, CardDetailsGood cardDetails, Date transactionDate) {
    String txnId = "TXN" + (int) (Math.random() * 1000);
    TransactionGood txn = new TransactionGood(txnId, amount, transactionDate);
    TransactionStatusGood transactionStatus = checkTransactionValidity(amount, cardDetails);

    if(transactionStatus == TransactionStatusGood.SUCCESS) {
      performTransaction(amount);
    }

    txn.updateStatus(transactionStatus);
    txnHistory.push(txn);
    return txn;
  }

  private void performTransaction(float amount) {
    availableBalance -= amount;
    System.out.println("longmethod.case1.TransactionGood successful. Available balance: " + availableBalance);
  }

  private TransactionStatusGood checkTransactionValidity(float amount, CardDetailsGood cardDetails) {
    if(!verifyCardDetails(cardDetails) || !checkAccountStandingAndStatus() || !checkTransactionAmountWithinLimit(amount)) {
      return TransactionStatusGood.FAILURE;
    }
    return TransactionStatusGood.SUCCESS;
  }

  private boolean verifyCardDetails(CardDetailsGood cardDetails) {
    if(!cardDetails.equals(this.cardDetails)) {
      System.out.println("Invalid card details. Please check and try again.");
        return false;
    }
    return true;
  }

  private boolean checkAccountStandingAndStatus() {
    if(status == StatusGood.INACTIVE) {
      System.out.println("Card is inactive. Please contact customer service.");
      return false;
    }
    if(standing == StandingGood.BAD) {
      System.out.println("Card is in bad standing. Please contact customer service.");
      return false;
    }
    return true;
  }

  private boolean checkTransactionAmountWithinLimit(float amount) {
    if(amount > creditLimit) {
      System.out.println("longmethod.case1.TransactionGood amount exceeds credit limit. Please try again.");
      return false;
    }
    if(amount > availableBalance) {
      System.out.println("longmethod.case1.TransactionGood amount exceeds available balance. Please try again.");
      return false;
    }
    return true;
  }
}