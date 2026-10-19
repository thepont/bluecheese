package messagechains.case3;

import java.util.LinkedList;
import java.util.Queue;

class OrderGood {
    private String item;
    private String destination;
    private boolean isDelivered;

    public OrderGood(String item, String destination) {
        this.item = item;
        this.destination = destination;
        this.isDelivered = false;
    }

    public String getOrderDetails() {
        return "Item: " + this.item + " to: " + this.destination + "\nDelivered: " + this.isDelivered;
    }

    public String getDestination() {
        return destination;
    }

    public void markOrderDelivered() {
        this.isDelivered = true;
    }

    public boolean getDeliveredStatus() {
        return this.isDelivered;
    }
}

class WarehouseGood {
    private String address;
    private String owningEntity;
    private Queue<messagechains.case3.OrderGood> orders;

    public WarehouseGood(String address, String owningEntity) {
        this.address = address;
        this.owningEntity = owningEntity;
        this.orders = new LinkedList<>();
    }

    public void addOrder(int orderId, messagechains.case3.OrderGood newOrder) {
        orders.add(newOrder);
    }

    public messagechains.case3.OrderGood getLatestOrder() {
        return orders.peek();
    }

    public String getWarehouseDetails() {
        return "Owned by " + this.owningEntity + "located at " + this.address;
    }

    public int getPendingOrderSize() {
        return orders.size();
    }

    public void checkAndEmptyWarehouse() throws Error {
        for (messagechains.case3.OrderGood order : orders) {
            if (!order.getDeliveredStatus()) {
                throw new Error("Warehouse is not empty!");
            }
        }
        orders.clear();
    }
}

class DistributorGood {
    private String owner;
    private String address;
    private messagechains.case3.WarehouseGood warehouse;

    public DistributorGood(String owner, String address, messagechains.case3.WarehouseGood warehouse) {
        this.owner = owner;
        this.address = address;
        this.warehouse = warehouse;
    }

    public messagechains.case3.OrderGood getLatestOrder() {
        return this.warehouse.getLatestOrder();
    }

    public String getDistributorDetails() {
        return "Owned by " + this.owner + " located at " + this.address;
    }

    public String getLatestOrderDestination() {
        return this.warehouse.getLatestOrder().getDestination();
    }

}

class VehicleGood {
    private String vehicleNumber;
    private String model;
    private messagechains.case3.DistributorGood distributor;

    public VehicleGood(messagechains.case3.DistributorGood distributor, String vehicleNumber, String model) {
        this.vehicleNumber = vehicleNumber;
        this.model = model;
        this.distributor = distributor;
    }

    public String getVehicleDetails() {
        return this.vehicleNumber + "\n" + this.model;
    }

    public void stationVehicle() {
        System.out.println(this.vehicleNumber + " is now stationed");
    }

    public void startVehicle() {
        System.out.println("Vehicle has been started");
    }

    public void getOrderRoute() {
        String destination = distributor.getLatestOrderDestination();
        System.out.println(vehicleNumber + " inbound to " + destination);
    }

}