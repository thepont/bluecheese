package featureenvy.case3;

import java.time.Year;

enum GenreGood {
    POP,
    ROCK,
    HIPHOP,
    RNB,
    EDM
}

enum PlaybackStateGood {
    STOPPED,
    PLAYING,
    PAUSED
}

class RecordGood {
    private String name;
    private featureenvy.case3.GenreGood genre;
    private int duration;
    private Year releaseYear;

    public RecordGood(String name, featureenvy.case3.GenreGood genre, int duration, Year releaseYear) {
        this.name = name;
        this.genre = genre;
        this.duration = duration;
        this.releaseYear = releaseYear;
    }

    public String getName() {
        return this.name;
    }

    public boolean isReleased() {
        return this.releaseYear.isBefore(Year.now()) || this.releaseYear.equals(Year.now());
    }

    public int getDuration() {
        return this.duration;
    }

    public featureenvy.case3.GenreGood getGenre() {
        return this.genre;
    }

    private String getFormattedDuration() {
        int minutes = this.duration / 60;
        int seconds = this.duration % 60;
        return String.format("%d min %02d sec", minutes, seconds);
    }

    public String getRecordDetails() {
        String details = this.name + " - " + this.genre + " - " + getFormattedDuration();
        return details;
    }

}

class RecordPlayerGood {
    private featureenvy.case3.RecordGood currentRecord;
    private featureenvy.case3.PlaybackStateGood state;

    public RecordPlayerGood(featureenvy.case3.RecordGood initialRecord) {
        this.currentRecord = initialRecord;
        this.state = featureenvy.case3.PlaybackStateGood.STOPPED;
    }

    public void playRecord() {
        if (this.state == featureenvy.case3.PlaybackStateGood.PLAYING) {
            System.out.println("Already playing");
        } else {
            System.out.println("Playing: " + currentRecord.getName());
            this.state = featureenvy.case3.PlaybackStateGood.PLAYING;
        }
    }

    public void stopRecord() {
        if (this.state == featureenvy.case3.PlaybackStateGood.STOPPED) {
            System.out.println("Already stopped");
        } else {
            System.out.println("Stopped playing: " + currentRecord.getName());
            this.state = featureenvy.case3.PlaybackStateGood.STOPPED;
        }
    }

    public void pauseRecord() {
        if (this.state == featureenvy.case3.PlaybackStateGood.PAUSED) {
            System.out.println("Already paused");
        } else {
            System.out.println("Paused: " + currentRecord.getName());
            this.state = featureenvy.case3.PlaybackStateGood.PAUSED;
        }
    }

    public void changeRecord(featureenvy.case3.RecordGood newRecord) {
        if (this.state != featureenvy.case3.PlaybackStateGood.STOPPED) {
            System.out.println("Please stop playing to change the record");
        } else {
            this.currentRecord = newRecord;
            this.state = featureenvy.case3.PlaybackStateGood.PLAYING;
            System.out.println("Started playing: " + this.currentRecord.getName());
        }
    }
}
