package intimacy.case2;

import java.util.ArrayList;
import java.util.List;

enum Ingredient {
  CREAM(0.10),
  SUGAR(0.05),
  ESPRESSO_SHOT(0.75),
  HOT_WATER(0.01),
  TAP_WATER(0.00),
  ICE(0.02),
  STEAMED_MILK(0.30),
  CHOCOLATE_SYRUP(0.50);

  private final double cost;

  Ingredient(double cost) {
    this.cost = cost;
  }

  public double getCost() {
    return cost;
  }
}

interface Coffee {
  void brew();
  void addCream(int extraPumps);
  void addSugar(int extraPumps);
  List<Ingredient> getIngredients();
  double getCost();
}

abstract class AbstractCoffee implements Coffee {
  protected List<Ingredient> ingredients = new ArrayList<>();

  @Override
  public void addCream(int extraPumps) {
    for (int i = 0; i <= extraPumps; i++) {
      ingredients.add(Ingredient.CREAM);
    }
  }

  @Override
  public void addSugar(int extraPumps) {
    for (int i = 0; i <= extraPumps; i++) {
      ingredients.add(Ingredient.SUGAR);
    }
  }

  @Override
  public double getCost() {
    return ingredients.stream()
      .mapToDouble(Ingredient::getCost)
      .sum();
  }

  @Override
  public List<Ingredient> getIngredients() {
    return ingredients;
  }
}

class Espresso extends AbstractCoffee {

  public Espresso() {}

  @Override
  public void brew() {
    System.out.println("Brewing espresso: grinding beans, extracting shot.");
    ingredients.add(Ingredient.ESPRESSO_SHOT);
  }
}

class Americano extends AbstractCoffee {
  private final boolean iced;
  private Espresso base = new Espresso();

  public Americano(boolean iced) {
    this.iced = iced;
    ingredients = base.getIngredients();

    ingredients.add(Ingredient.CREAM);
    ingredients.add(Ingredient.SUGAR);
  }

  @Override
  public void brew() {
    System.out.println("Brewing Americano: pulling Espresso and adding water.");
    base.brew();

    if (iced) {
      ingredients.add(Ingredient.ICE);
      ingredients.add(Ingredient.TAP_WATER);
    } else {
      ingredients.add(Ingredient.HOT_WATER);
    }
  }
}

class Latte extends AbstractCoffee {
  protected boolean iced;

  public Latte(boolean iced) {
    this.iced = iced;

    ingredients.add(Ingredient.CREAM);
    ingredients.add(Ingredient.CREAM);
    ingredients.add(Ingredient.SUGAR);
  }

  @Override
  public void brew() {
    System.out.println("Brewing Latte: pulling Espresso and steamed milk.");
    ingredients.add(Ingredient.ESPRESSO_SHOT);
    ingredients.add(Ingredient.ESPRESSO_SHOT);
    ingredients.add(Ingredient.STEAMED_MILK);

    if (iced) {
      ingredients.add(Ingredient.ICE);
      ingredients.add(Ingredient.TAP_WATER);
    } else {
      ingredients.add(Ingredient.HOT_WATER);
    }
  }
}

class CafeMocha extends AbstractCoffee {
  private boolean iced;
  private Latte base = new Latte(false);

  public CafeMocha(boolean iced) {
    this.iced = false;

    base.ingredients.add(Ingredient.CREAM);
    base.ingredients.add(Ingredient.CHOCOLATE_SYRUP);
    base.ingredients.add(Ingredient.CHOCOLATE_SYRUP);

    ingredients = base.getIngredients();
  }

  @Override
  public void brew() {
    System.out.println("Brewing Cafe Mocha: pulling Latte and steamed milk.");
    base.brew();
    ingredients.add(Ingredient.STEAMED_MILK);
  }

  private void addChocolate(int extraPumps) {
    for (int i = 0; i <= extraPumps; i++) {
      ingredients.add(Ingredient.CHOCOLATE_SYRUP);
    }
  }
}


