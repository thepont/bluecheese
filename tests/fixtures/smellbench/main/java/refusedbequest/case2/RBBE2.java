package refusedbequest.case2;

interface NoteTakerBad {
    void writeNotes();
    void shareNotesAsPdf();
}

class IpadNotesBad implements NoteTakerBad {
    @Override
    public void writeNotes() {
        System.out.println("Notes written on iPad.");
    }

    @Override
    public void shareNotesAsPdf() {
        System.out.println("Notes shared as PDF.");
    }
}

class NotebookNotesBad implements NoteTakerBad {
    @Override
    public void writeNotes() {
        System.out.println("Notes written on Notebook.");
    }

    @Override
    public void shareNotesAsPdf() {
        throw new UnsupportedOperationException("Notebook does not support sharing notes as PDF.");
    }
}