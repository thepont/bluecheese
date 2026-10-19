package divergentchange.case3;

class Connector {
    private String url;

    public Connector(String url) {
        this.url = url;
    }

    public boolean checkValidUrl() {
        if (this.url.startsWith("http")) {
            System.out.println("Valid HTTP url");
            return true;
        } else if (this.url.startsWith("ftp")) {
            System.out.println("Valid FTP url");
            return true;
        }
        return false;
    }

    public String getDomain() {
        String domain = "";
        if (this.url.startsWith("http")) {
            domain = this.url.substring(7, this.url.indexOf("/", 8));
        } else if(this.url.startsWith("ftp")) {
            domain = this.url.substring(this.url.indexOf("@")+1, this.url.lastIndexOf(":"));
        }
        return domain;
    }
}