package lazyclass.case5;

class CatVariation {
    private String name;
    private String cry;

    public CatVariation(String name, String cry) {
        this.name = name;
        this.cry = cry;
    }

    public void purr() {
        System.out.println(this.cry);
    }

    public void getCatName() {
        System.out.println(this.name);
    }
}

class TigerVariation extends CatVariation {
    public TigerVariation(String name, String cry) {
        super(name, cry);
    }

    public void isDangerous() {
        System.out.println("Yes");
    }
}