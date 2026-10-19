package longparams.case6;

class ShipmentVariation {
    private final int EARTHRADIUS;

    public double calculateDistance(LocationCoordinatesVariation origin, LocationCoordinatesVariation dest) {
        double p = Math.PI / 180;
        double a = 0.5 - Math.cos((dest.getLatitude() - origin.getLatitude()) * p) / 2
                + Math.cos(origin.getLatitude() * p) * Math.cos(dest.getLatitude() * p)
                        * (1 - Math.cos((dest.getLongitude() - origin.getLongitude()) * p)) / 2;
        return 2 * this.EARTHRADIUS * Math.asin(Math.sqrt(a));
    }

    public ShipmentVariation() {
        this.EARTHRADIUS = 6371;
    }
}

class LocationCoordinatesVariation {
    private double longitude;
    private double latitude;

    public double getLatitude() {
        return latitude;
    }

    public double getLongitude() {
        return longitude;
    }

    public LocationCoordinatesVariation(double longitude, double latitude) {
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