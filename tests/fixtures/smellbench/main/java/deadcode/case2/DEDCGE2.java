package deadcode.case2;

import java.util.HashMap;
import java.util.Map;

class IkeaCartVariation {
  private final Map<String, Integer> items = new HashMap<>();
  private final Map<String, Double> priceList;

  public IkeaCartVariation(Map<String, Double> priceList) {
    this.priceList = priceList;
  }

  public void addItem(String itemCode) {
    if (!priceList.containsKey(itemCode)) {
      throw new IllegalArgumentException("Unknown item: " + itemCode);
    }
    items.merge(itemCode, 1, Integer::sum);
  }

  public void removeItem(String itemCode) {
    items.remove(itemCode);
  }

  public double calculateTotal() {
    double total = 0;
    for (Map.Entry<String, Integer> e : items.entrySet()) {
      double price = priceList.get(e.getKey());
      total += price * e.getValue();
    }
    return total;
  }

  @Deprecated
  public double calculate2018BlackFridayPrice() {
    double original = calculateTotal();
    double discounted = original * 0.80;
    if (items.size() >= 3) {
      discounted *= 0.95;
    }
    int hundreds = (int) (original / 100);
    double coupon = Math.min(hundreds * 10, 50);
    discounted -= coupon;
    discounted = Math.max(discounted, 0);

    return Math.round(discounted * 100.0) / 100.0;
  }

  public void printReceipt() {
    System.out.println("---- IKEA Cart Receipt ----");
    for (Map.Entry<String, Integer> e : items.entrySet()) {
      double lineTotal = priceList.get(e.getKey()) * e.getValue();
      System.out.printf("%s x%d = $%.2f%n", e.getKey(), e.getValue(), lineTotal);
    }
    System.out.printf("TOTAL: $%.2f%n", calculateTotal());
    System.out.println("---------------------------");
  }
}
