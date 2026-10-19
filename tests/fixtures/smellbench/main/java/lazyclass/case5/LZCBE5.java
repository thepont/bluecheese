package lazyclass.case5;

class Cat {
    private String name;
    private String cry;

    public Cat(String name, String cry) {
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

class Tiger extends Cat {
    public Tiger(String name, String cry) {
        super(name, cry);
    }

    public void isDangerous() {
        System.out.println("Yes");
    }
}

class Cheetah extends Cat {
    public Cheetah(String name, String cry) {
        super(name, cry);
    }
}
