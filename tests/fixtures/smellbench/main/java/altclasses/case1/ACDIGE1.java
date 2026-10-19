package java.altclasses.case1;

interface Animal {
    void animalVoice();
    void animalInfo();
}

abstract class AbstractAnimal implements Animal {
    private String name;
    private int age;

    public AbstractAnimal(String name, int age) {
        this.name = name;
        this.age = age;
    }

    abstract public void animalVoice();

    public void animalInfo() {
        System.out.println("Name: " + name + ", Age: " + age);
    }
}

class LionGood extends AbstractAnimal implements Animal {
    public LionGood(String name, int age) {
        super(name, age);
    }

    @Override
    public void animalVoice() {
        System.out.println("Roar");
    }
}

class RabbitGood extends AbstractAnimal implements Animal {
    public RabbitGood(String name, int age) {
        super(name, age);
    }

    @Override
    public void animalVoice() {
        System.out.println("Squeak");
    }
}