package longparams.case6;

class Shipment {
    private final int EARTHRADIUS;

    public double calculateDistance(double originLat, double originLong, double destLat, double destLong) {
        double p = Math.PI / 180;
        double a = 0.5 - Math.cos((destLat - originLat) * p) / 2
                + Math.cos(originLat * p) * Math.cos(destLat * p)
                        * (1 - Math.cos((destLong - originLong) * p)) / 2;
        return 2 * this.EARTHRADIUS * Math.asin(Math.sqrt(a));
    }

    public Shipment() {
        this.EARTHRADIUS = 6371;
    }
}

class LocationCoordinates {
    private double longitude;
    private double latitude;

    public double getLatitude() {
        return latitude;
    }

    public double getLongitude() {
        return longitude;
    }

    public LocationCoordinates(double longitude, double latitude) {
        this.longitude = longitude;
        this.latitude = latitude;
    }

    public double getRelativePositionToPoles(String direction) {
        if (direction.equals("North")) {
            return 90 - latitude;
        } else if (direction.equals("South")) {
            return 90 + latitude;
        } else {
            return 181;
        }
    }
}