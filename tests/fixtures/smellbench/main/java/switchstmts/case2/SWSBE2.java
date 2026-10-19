package switchstmts.case2;

enum RouteTypeBad {
    CAR, BIKE, WALK, BUS
}
interface RouteBad {
    void calculateTimeToDestination();
    void setRouteType(RouteTypeBad routeType);
}

class UserRouteBad implements RouteBad {
    private RouteTypeBad routeType;
    @Override
    public void calculateTimeToDestination() {
        switch (routeType) {
            case CAR:
                calculateCarRoute();
                break;
            case BIKE:
                calculateBikeRoute();
                break;
            case WALK:
                calculateWalkRoute();
                break;
            case BUS:
                calculateBusRoute();
                break;
        }
    }

    @Override
    public void setRouteType(RouteTypeBad routeType) {
        this.routeType = routeType;
    }

    private void calculateCarRoute() {
        System.out.println("Time to destination by car: 30 minutes");
    }

    private void calculateBikeRoute() {
        System.out.println("Time to destination by bike: 45 minutes");
    }

    private void calculateWalkRoute() {
        System.out.println("Time to destination by walk: 2 hours");
    }

    private void calculateBusRoute() {
        System.out.println("Time to destination by bus: 1 hour");
    }
}