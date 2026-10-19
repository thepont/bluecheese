package divergentchange.case3;

interface ConnectorVariation {
    public boolean checkValidUrl();

    public String getDomain();
}

class HTTPConnectorVariation implements ConnectorVariation {

    private String url;

    public HTTPConnectorVariation(String url) {
        this.url = url;
    }

    @Override
    public boolean checkValidUrl() {
        if(this.url.startsWith("http")) {
            System.out.println("Valid HTTP url");
            return true;
        }
        return false;
    }

    @Override
    public String getDomain() {
        String domain = this.url.substring(7, this.url.indexOf("/", 8));
        return domain;
    }

}

class FTPConnectorVariation implements ConnectorVariation {

    private String url;

    public FTPConnectorVariation(String url) {
        this.url = url;
    }

    @Override
    public boolean checkValidUrl() {
        if (this.url.startsWith("ftp")) {
            System.out.println("Valid FTP url");
            return true;
        }
        return false;
    }

    @Override
    public String getDomain() {
        String domain = this.url.substring(this.url.indexOf("@")+1, this.url.lastIndexOf(":"));
        return domain;
    }

}
