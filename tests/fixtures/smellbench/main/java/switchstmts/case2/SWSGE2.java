package switchstmts.case2;

interface RouteStrategy {
  void calculateTimeToDestination();
}

class CarRouteStrategy implements RouteStrategy {
  @Override
  public void calculateTimeToDestination() {
    System.out.println("Time to destination by car: 30 minutes");
  }
}

class BikeRouteStrategy implements RouteStrategy {
  @Override
  public void calculateTimeToDestination() {
    System.out.println("Time to destination by bike: 45 minutes");
  }
}

class WalkRouteStrategy implements RouteStrategy {
  @Override
  public void calculateTimeToDestination() {
    System.out.println("Time to destination by walk: 2 hours");
  }
}

class BusRouteStrategy implements RouteStrategy {
  @Override
  public void calculateTimeToDestination() {
    System.out.println("Time to destination by bus: 1 hour");
  }
}

interface RouteGood {
  void calculateTimeToDestination();
  void setRouteType(RouteStrategy routeStrategy);
}

class UserRouteGood implements RouteGood {
  private RouteStrategy routeStrategy;

  @Override
  public void calculateTimeToDestination() {
    routeStrategy.calculateTimeToDestination();
  }

  @Override
  public void setRouteType(RouteStrategy routeStrategy) {
    this.routeStrategy = routeStrategy;
  }

}
