package longparams.case2;

import java.util.HashMap;
import java.util.Map;

class StoreMemberPricingGood {
  private double memberDiscount;
  private double stateTax;

  public StoreMemberPricingGood(double stateTax) {
    this.memberDiscount = 0.05;
    this.stateTax = stateTax;
  }

  public double getMemberDiscount() {
    return memberDiscount;
  }

  public double getTax() {
    return stateTax;
  }

  public void printMemberDiscount() {
    System.out.println("Thank you for being a member.");
    System.out.println("Member Discount: " + memberDiscount);
    System.out.println("State Tax: " + stateTax);
  }
}
class StorePricingGood {
  private double storeDiscount;
  private double fees;
  private Map<String, Double> productPricePerQuantity;

  public StorePricingGood(Map<String, Double> productPricePerQuantity) {
    this.productPricePerQuantity = productPricePerQuantity;
    this.storeDiscount = 0.1;
    this.fees = 10;
  }

  public Map<String, Double> getProductPricePerQuantity() {
    return productPricePerQuantity;
  }

  public double getStoreDiscount() {
    return storeDiscount;
  }

  public double getFees() {
    return fees;
  }

  public void printProductTypesAndPrice() {
    for (Map.Entry<String, Double> entry : productPricePerQuantity.entrySet()) {
      System.out.println("Product: " + entry.getKey() + ", Price: " + entry.getValue());
    }
  }
}
class ShoppingCartGood {

  private Map<String, Integer> productsBought;
  private StorePricingGood storePricing;
  private StoreMemberPricingGood storeMemberPricing;

  public ShoppingCartGood(StorePricingGood storePricing, StoreMemberPricingGood storeMemberPricing) {
    this.productsBought = new HashMap<>();
    this.storePricing = storePricing;
    this.storeMemberPricing = storeMemberPricing;
  }

  public void addProduct(String product, int quantity) {
    productsBought.put(product, quantity);
  }

  public double calculateCartPrice() {
    Map <String, Double> priceList = storePricing.getProductPricePerQuantity();
    double storeDiscount = storePricing.getStoreDiscount();
    double memberDiscount = storeMemberPricing.getMemberDiscount();
    double tax = storeMemberPricing.getTax();
    double fees = storePricing.getFees();
    double totalPrice = 0;
    for (Map.Entry<String, Integer> entry : productsBought.entrySet()) {
      String product = entry.getKey();
      int quantity = entry.getValue();
      double price = priceList.get(product);
      totalPrice += price * quantity;
    }
    totalPrice = totalPrice * (1 - storeDiscount) * (1 - memberDiscount) * (1 + tax) + fees;
    return totalPrice;
  }

}
