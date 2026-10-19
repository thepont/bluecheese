package primobsession.case5;

class ShippingVolumeCheck {
    private int[] boxDimensions;
    final private int ITEMVOLUME;

    private int calculateBoxVolume() {
        int volume = this.boxDimensions[0] * this.boxDimensions[1] * this.boxDimensions[2];
        return volume;
    }

    public boolean checkItemVolume() {
        return this.boxDimensions[3] > this.ITEMVOLUME;
    }

    ShippingVolumeCheck(int boxHeight, int boxDepth, int boxWidth) {
        this.boxDimensions = new int[4];
        this.ITEMVOLUME = 12;
        this.boxDimensions[0] = boxHeight;
        this.boxDimensions[1] = boxDepth;
        this.boxDimensions[2] = boxWidth;
        this.boxDimensions[3] = calculateBoxVolume();
    }

}
