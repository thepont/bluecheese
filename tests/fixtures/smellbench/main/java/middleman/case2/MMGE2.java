package middleman.case2;

import java.util.HashMap;
import java.util.Map;

class ProductGood {
    private String name;
    private String type;
    private int stock;

    public ProductGood(String name, String type, int stock) {
        this.name = name;
        this.type = type;
        this.stock = stock;
    }

    public int getStock() {
        return this.stock;
    }

    public void reduceStock() {
        this.stock -= 1;
    }

    public void showDetails(int id) {
        System.out.println("Name: " + this.name);
        System.out.println("Type: " + this.type);
        System.out.println("Stock: " + this.stock);
    }
}

class ProductManagerGood {
    Map<Integer, middleman.case2.ProductGood> products;

    public ProductManagerGood() {
        this.products = new HashMap<>();
    }

    public void addProduct(int id, middleman.case2.ProductGood product) {
        products.put(id, product);
    }

    public void removeProduct(int id) {
        products.remove(id);
    }

    public void showTotalStock() {
        int totalStock = 0;
        for (Map.Entry<Integer, middleman.case2.ProductGood> product : products.entrySet()) {
            totalStock += product.getValue().getStock();
        }
        System.out.println("Total Stock of all Products: " + totalStock);
    }

}
