package refusedbequest.case2;

interface NoteTakerGood {
  void writeNotes();
}

class IpadNotesGood implements NoteTakerGood {
  @Override
  public void writeNotes() {
    System.out.println("Notes written on iPad.");
  }

  public void shareNotesAsPdf() {
    System.out.println("Notes shared as PDF.");
  }
}

class NotebookNotesGood implements NoteTakerGood {
  @Override
  public void writeNotes() {
    System.out.println("Notes written on Notebook.");
  }

}
