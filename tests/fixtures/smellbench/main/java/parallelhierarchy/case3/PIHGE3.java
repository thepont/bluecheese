package parallelhierarchy.case3;

interface EnemyGood {
    public void attack();

    public void defend();
}

class VampireGood implements parallelhierarchy.case3.EnemyGood {
    private parallelhierarchy.case3.WeaponGood shotgun;
    private int defense;

    public VampireGood(parallelhierarchy.case3.WeaponGood shotgun, int defense) {
        this.shotgun = shotgun;
        this.defense = defense;
    }

    @Override
    public void attack() {
        if (shotgun.inspectHealth() > 0) {
            System.out.println("Vampire attacks. Damage done: " + this.shotgun.getDamage());
        } else {
            System.out.println("Weapon broken. Vampire can't repair weapon");
        }
    }

    @Override
    public void defend() {
        System.out.println("Blocking attack with defense: " + (this.defense + 10));
    }

}

class GhoulGood implements parallelhierarchy.case3.EnemyGood {
    private parallelhierarchy.case3.WeaponGood bazooka;
    private int defense;

    public GhoulGood(parallelhierarchy.case3.WeaponGood bazooka, int defense) {
        this.bazooka = bazooka;
        this.defense = defense;
    }

    @Override
    public void attack() {
        if (bazooka.inspectHealth() > 0) {
            System.out.println("Ghoul attacks. Damage done: " + this.bazooka.getDamage());
        } else {
            System.out.println("Weapon broken. Repairing...");
            this.bazooka.repair();
        }
    }

    @Override
    public void defend() {
        System.out.println("Blocking attack with defense: " + (this.defense + 20));
    }

}

interface WeaponGood {
    public int getDamage();

    public void repair();

    public int inspectHealth();
}

class BasicWeaponGood implements parallelhierarchy.case3.WeaponGood {
    private int damage;
    private int health;

    public BasicWeaponGood(int damage, int health) {
        this.damage = damage;
        this.health = health;
    }

    @Override
    public int getDamage() {
        this.health -= 10;
        return damage;
    }

    @Override
    public void repair() {
        this.health = 100;
    }

    @Override
    public int inspectHealth() {
        return this.health;
    }

}
