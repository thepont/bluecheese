package primobsession.case4;

class CardboardBox {
    private int height;
    private int width;
    private int depth;

    CardboardBox(int height, int width, int depth) {
        this.depth = depth;
        this.height = height;
        this.width = width;
    }

    public int calculateBoxVolume() {
        int volume = this.height * this.depth * this.width;
        return volume;
    }
}

class ShippingVolumeCheckGood {
    final private int ITEMVOLUME;
    private CardboardBox box;

    public boolean checkItemVolume() {
        return box.calculateBoxVolume() > this.ITEMVOLUME;
    }

    ShippingVolumeCheckGood(int boxHeight, int boxDepth, int boxWidth) {
        this.ITEMVOLUME = 12;
        this.box = new CardboardBox(boxHeight, boxWidth, boxDepth);
    }
}
