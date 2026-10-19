package longparams.case4;

import java.util.Set;

class BlackFridayVariation {
  private final String marketSector;
  private double discount;
  private double spending;
  private boolean isPromotionActive;
  private int numberOfAvailableCoupons;
  private int couponCode;
  private int consumerID;

  private final double TECH_DISCOUNT = 0.30;
  private final double FASHION_DISCOUNT = 0.50;
  private final double FOOD_DISCOUNT = 0.10;
  private static final Set<Integer> VALID_COUPON_CODES = Set.of(12345, 67890, 11111);

  public BlackFridayVariation(String marketSector, int numberOfAvailableCoupons) {
    if (marketSector == null || marketSector.isEmpty()) {
      throw new IllegalArgumentException("Market sector must not be null or empty!");
    }

    this.marketSector = marketSector;
    this.discount = 0.0;
    this.spending = 0.0;
    this.isPromotionActive = false;
    this.numberOfAvailableCoupons = numberOfAvailableCoupons;
  }

  public void setSpending(double spending) {
    this.spending = spending;
  }

  public boolean couponAssigned() {
    return VALID_COUPON_CODES.contains(this.couponCode);
  }

  public double getConsumerID() {
    return consumerID;
  }

  public void setConsumerID(int consumerID) {
    this.consumerID = consumerID;
  }

  public void activatePromotion(boolean IDVerified, double spending) {
    this.spending = spending;

    if (couponAssigned() && IDVerified && spending >= 100.0 && numberOfAvailableCoupons > 0) {
      isPromotionActive = true;

      if (marketSector.equals("Tech")) {
        discount = TECH_DISCOUNT;
      } else if (marketSector.equals("Fashion")) {
        discount = FASHION_DISCOUNT;
      } else if (marketSector.equals("Food")) {
        discount = FOOD_DISCOUNT;
      } else {
        discount = 0.05;
      }

      this.numberOfAvailableCoupons--;
    } else {
      isPromotionActive = false;
      discount = 0.0;
    }
  }
}
