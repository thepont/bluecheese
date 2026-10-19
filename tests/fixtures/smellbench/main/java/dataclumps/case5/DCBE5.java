package dataclumps.case5;

enum ConnectionStateVariation {
    CONNECTED,
    FAILURE,
    NOTCONNECTED
}

class MongoDBConnectorVariation {
    private ConnectionStateVariation state;

    public MongoDBConnectorVariation() {
        this.state = ConnectionStateVariation.NOTCONNECTED;
    }

    public boolean isCredentialsValid(int port, String host, String username, String password) {
        if (port < 0 || port > 65535) {
            throw new IllegalArgumentException("Port is invalid");
        }

        if (!host.startsWith("http")) {
            throw new IllegalArgumentException("Host is invalid");
        }

        return true;
    }

    public boolean testConnection(int port, String host, String username, String password) {
        if (Math.random() > 0.1) {
            System.out.println("Connection to: " + host + ":" + port + "/" + username + "&" + password + "successful");
            return true;
        } else {
            System.out.println("Connection to: " + host + ":" + port + "/" + username + "&" + password + "failed");
            return false;
        }
    }

    public void connectToDatabase(int port, String host, String username, String password)
            throws IllegalArgumentException {
        if (!isCredentialsValid(port, host, username, password)) {
            this.state = ConnectionStateVariation.FAILURE;
            throw new IllegalArgumentException("Invalid database credentials");
        }

        System.out.println("Testing connection...");

        if (!testConnection(port, host, username, password)) {
            this.state = ConnectionStateVariation.FAILURE;
            throw new IllegalArgumentException(
                    "Failed trying to connect to database: " + host + ":" + port + "/" + username + "&" + password);
        }

        System.out
                .println("Connection to database estabilished: " + host + ":" + port + "/" + username + "&" + password);
        this.state = ConnectionStateVariation.CONNECTED;
    }

    public String getCurrentState() {
        return "State: " + this.state;
    }
}