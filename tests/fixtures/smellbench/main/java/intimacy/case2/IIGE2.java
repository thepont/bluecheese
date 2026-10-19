package intimacy.case2;

import java.util.ArrayList;
import java.util.List;

enum IngredientVariation {
  CREAM(0.10),
  SUGAR(0.05),
  ESPRESSO_SHOT(0.75),
  HOT_WATER(0.01),
  TAP_WATER(0.00),
  ICE(0.02),
  STEAMED_MILK(0.30),
  CHOCOLATE_SYRUP(0.50);

  private final double cost;

  IngredientVariation(double cost) {
    this.cost = cost;
  }

  public double getCost() {
    return cost;
  }
}

interface CoffeeVariation {
  void brew();
  void addCream(int extraPumps);
  void addSugar(int extraPumps);
  List<IngredientVariation> getIngredients();
  double getCost();
}

abstract class AbstractCoffeeVariation implements CoffeeVariation {
  protected List<IngredientVariation> ingredients = new ArrayList<>();

  @Override
  public void addCream(int extraPumps) {
    for (int i = 0; i <= extraPumps; i++) {
      ingredients.add(IngredientVariation.CREAM);
    }
  }

  @Override
  public void addSugar(int extraPumps) {
    for (int i = 0; i <= extraPumps; i++) {
      ingredients.add(IngredientVariation.SUGAR);
    }
  }

  @Override
  public double getCost() {
    return ingredients.stream()
      .mapToDouble(IngredientVariation::getCost)
      .sum();
  }

  @Override
  public List<IngredientVariation> getIngredients() {
    return ingredients;
  }
}

class EspressoVariation extends AbstractCoffeeVariation {

  public EspressoVariation() {
    this.ingredients = new ArrayList<>();
  }

  @Override
  public void brew() {
    System.out.println("Brewing espresso: grinding beans, extracting shot.");
    this.ingredients.add(IngredientVariation.ESPRESSO_SHOT);
  }
}

class AmericanoVariation extends EspressoVariation {
  protected boolean iced;

  public AmericanoVariation(boolean iced) {
    super();
    this.iced = false;
  }

  @Override
  public void brew() {
    super.brew();

    if (iced) {
      this.ingredients.add(IngredientVariation.TAP_WATER);
      this.ingredients.add(IngredientVariation.ICE);
    } else {
      this.ingredients.add(IngredientVariation.HOT_WATER);
    }
  }

  @Override
  public void addCream(int extraPumps) {
    super.addCream(0);

    for (int i=0; i<=extraPumps; i++) {
      this.ingredients.add(IngredientVariation.CREAM);
    }
  }

  @Override
  public void addSugar(int extraPumps) {
    super.addSugar(0);

    for (int i=0; i<=extraPumps; i++) {
      this.ingredients.add(IngredientVariation.SUGAR);
    }
  }
}

class LatteVariation extends AbstractCoffeeVariation {
  private boolean iced;

  public LatteVariation(Boolean iced) {
    this.iced = false;
    this.ingredients = new ArrayList<>();

    this.ingredients.add(IngredientVariation.CREAM);
    this.ingredients.add(IngredientVariation.CREAM);
    this.ingredients.add(IngredientVariation.SUGAR);
  }

  @Override
  public void brew() {
    System.out.println("Brewing Latte: pulling espresso and steamed milk.");
    this.ingredients.add(IngredientVariation.ESPRESSO_SHOT);
    this.ingredients.add(IngredientVariation.ESPRESSO_SHOT);
    this.ingredients.add(IngredientVariation.STEAMED_MILK);
  }
}

class CafeMochaVariation extends LatteVariation {

  public CafeMochaVariation(Boolean iced) {
    super(iced);

    this.ingredients.add(IngredientVariation.CHOCOLATE_SYRUP);
    this.ingredients.add(IngredientVariation.CHOCOLATE_SYRUP);
  }

  @Override
  public void brew() {
    super.brew();
  }

  @Override
  public void addCream(int extraPumps) {
    super.addCream(0);
  }

  @Override
  public void addSugar(int extraPumps) {
    super.addSugar(0);
  }

  private void addChocolate(int extraPumps) {
    for (int i = 0; i <= extraPumps; i++) {
      this.ingredients.add(IngredientVariation.CHOCOLATE_SYRUP);
    }
  }
}
