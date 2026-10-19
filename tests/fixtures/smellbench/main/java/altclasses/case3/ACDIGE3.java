package altclasses.case3;

interface PersonaGood {

    public int attackWithMagic();

    public void block(int incomingDamage);

    public int attackWithMelee();

    public void restoreHealth();
}

class SimplePersonaGood implements PersonaGood {

    private String name;
    private int health;
    private int meleeDamage;
    private int magicDamage;
    private int defense;

    public SimplePersonaGood(String name, int health, int meleeDamage, int magicDamage, int defense) {
        this.name = name;
        this.health = health;
        this.meleeDamage = meleeDamage;
        this.magicDamage = magicDamage;
        this.defense = defense;
    }

    @Override
    public int attackWithMagic() {
        System.out.println(this.name + " used Magic and caused " + this.magicDamage + " damage!");
        return this.magicDamage;
    }

    @Override
    public void block(int incomingDamage) {
        int finalDamage = this.defense * incomingDamage / 100;
        System.out.println(this.name + " defended and suffered " + finalDamage + " damage!");
        this.health -= finalDamage;
    }

    @Override
    public int attackWithMelee() {
        System.out.println(this.name + " used Melee and caused " + this.meleeDamage + " damage!");
        return this.meleeDamage;
    }

    @Override
    public void restoreHealth() {
        System.out.println(this.name + " healed themselves " + this.defense / 100 + " points!");
        System.out.println("New health: " + this.health);
        this.health += this.defense / 100;
    }

}