package altclasses.case2;

enum ColorGood {
  BLACK(1), WHITE(0);
  private final int value;
  ColorGood(int value) { this.value = value; }
  public int getValue() { return value; }
}

interface ChessPieceGood {
  int getRow();
  int getCol();
  ColorGood getColor();
  boolean canMove(int row, int col);
  boolean canKill(ChessPieceGood piece);
}

abstract class AbstractChessPieceGood implements ChessPieceGood {
  private int row, col;
  private ColorGood color;

  public AbstractChessPieceGood(int row, int col, ColorGood color) {
    if (row > 7 || row < 0 || col > 7 || col < 0) {
      throw new IllegalArgumentException("altclasses.case2.ChessPiece out of bounds");
    }
    this.row = row;
    this.col = col;
    this.color = color;
  }

  public AbstractChessPieceGood(AbstractChessPieceGood original) {
    this(original.row, original.col, original.color);
  }

  @Override
  public int getRow() {
    return this.row;
  }

  @Override
  public int getCol() {
    return this.col;
  }

  @Override
  public ColorGood getColor() {
    return this.color;
  }

  @Override
  public abstract boolean canMove(int row, int col);

  @Override
  public abstract boolean canKill(ChessPieceGood piece);
}

class BishopGood extends AbstractChessPieceGood {
  public BishopGood(int row, int col, ColorGood color) {
    super(row, col, color);
  }

  public BishopGood(BishopGood original) {
    this(original.getRow(), original.getCol(), original.getColor());
  }

  @Override
  public boolean canMove(int row, int col) {
    if (row < 0 || row > 7 || col < 0 || col > 7
            || row == this.getRow() || col == this.getCol()){
      return false;
    }
    int rowDiff = Math.abs(row - this.getRow());
    int colDiff =Math.abs(col - this.getCol());
    return (rowDiff == colDiff);
  }

  @Override
  public boolean canKill(ChessPieceGood piece) {
    return (piece.getColor() != this.getColor()
            && this.canMove(piece.getRow(), piece.getCol()));
  }
}

class KingGood extends AbstractChessPieceGood {
  private static final int VALID_KING_1 = 1;
  private static final int VALID_KING_0 = 0;

  public KingGood(int row, int col, ColorGood color) {
    super(row, col, color);
  }

  public KingGood(KingGood original) {
    this(original.getRow(), original.getCol(), original.getColor());
  }

  @Override
  public boolean canMove(int row, int col) {
    if (row < 0 || row > 7 || col < 0 || col > 7
            || (row == this.getRow() && col == this.getCol())){
      return false;
    }

    int rowDiff = Math.abs(row-this.getRow());
    int colDiff = Math.abs(col-this.getCol());

    return (rowDiff == VALID_KING_1 && colDiff == VALID_KING_1)
            || (rowDiff == VALID_KING_1 && colDiff == VALID_KING_0)
            || (rowDiff == VALID_KING_0 && colDiff == VALID_KING_1);
  }

  @Override
  public boolean canKill(ChessPieceGood piece) {
    return (piece.getColor() != this.getColor()
            && this.canMove(piece.getRow(), piece.getCol()));
  }
}

class KnightGood extends AbstractChessPieceGood {
  private final static int VALID_COL_DIFF_2 = 2;
  private final static int VALID_COL_DIFF_1 = 1;
  private final static int VALID_ROW_DIFF_2 = 2;
  private final static int VALID_ROW_DIFF_1 = 1;

  public KnightGood(int row, int col, ColorGood color) {
    super(row, col, color);
  }

  public KnightGood(KnightGood original) {
    this(original.getRow(), original.getCol(), original.getColor());
  }

  @Override
  public boolean canMove(int row, int col) {
    if (row < 0 || row > 7 || col < 0 || col > 7
            || row == this.getRow() || col == this.getCol()){
      return false;
    }

    int rowDiff = Math.abs(row-this.getRow());
    int colDiff = Math.abs(col-this.getCol());

    return ((rowDiff == VALID_ROW_DIFF_2 && colDiff == VALID_COL_DIFF_1)
            || (rowDiff == VALID_ROW_DIFF_1 && colDiff == VALID_COL_DIFF_2));
  }

  @Override
  public boolean canKill(ChessPieceGood piece) {
    return (piece.getColor() != this.getColor()
            && this.canMove(piece.getRow(), piece.getCol()));
  }
}

class PawnGood extends AbstractChessPieceGood {
  private static final int BLACK_START_ROW = 6;
  private static final int WHITE_START_ROW = 1;
  private static final int VALID_BLACK_ROW_DIFF_1 = -1;
  private static final int VALID_BLACK_ROW_DIFF_2 = -2;
  private static final int VALID_WHITE_ROW_DIFF_1 = 1;
  private static final int VALID_WHITE_ROW_DIFF_2 = 2;
  private static final int VALID_COL_DIFF = 0;
  private static final int VALID_COL_KILL_DIFF = 1;

  public PawnGood(int row, int col, ColorGood color) {
    super(row, col, color);
    if (color == ColorGood.WHITE && row != 1) {
      throw new IllegalArgumentException("Set white pawns at the wrong row!");
    } else if (color == ColorGood.BLACK && row != 6) {
      throw new IllegalArgumentException("Set black pawns at the wrong row!");
    }
  }

  public PawnGood(PawnGood original) {
    this(original.getRow(), original.getCol(), original.getColor());
  }

  @Override
  public boolean canMove(int row, int col) {
    if (row < 0 || row > 7 || col < 0 || col > 7
            || (row == this.getRow() && col == this.getCol())) {
      return false;
    }

    int rowDiff = row-this.getRow();
    int colDiff = col-this.getCol();

    if (this.getColor() == ColorGood.BLACK) {
      if (this.getRow() == BLACK_START_ROW) {
        return (rowDiff == VALID_BLACK_ROW_DIFF_1 && colDiff == VALID_COL_DIFF)
                || (rowDiff == VALID_BLACK_ROW_DIFF_2 && colDiff == VALID_COL_DIFF);
      } else {
        return rowDiff == VALID_BLACK_ROW_DIFF_1 && colDiff == VALID_COL_DIFF;
      }
    }

    if (this.getColor() == ColorGood.WHITE) {
      if (this.getRow() == WHITE_START_ROW) {
        return (rowDiff == VALID_WHITE_ROW_DIFF_1 && colDiff == VALID_COL_DIFF)
                || (rowDiff == VALID_WHITE_ROW_DIFF_2 && colDiff == VALID_COL_DIFF);
      } else {
        return rowDiff == VALID_WHITE_ROW_DIFF_1 && colDiff == VALID_COL_DIFF;
      }
    }
    return false;
  }

  @Override
  public boolean canKill(ChessPieceGood piece) {
    if (this.getColor() == piece.getColor()
            || piece.getRow() > 7 || piece.getRow() < 0 || piece.getCol() > 7 || piece.getCol() < 0) {
      return false;
    }

    int rowDiff = piece.getRow()-this.getRow();
    int colDiff = Math.abs(piece.getCol()-this.getCol());

    if (this.getColor() == ColorGood.BLACK) {
      return rowDiff == VALID_BLACK_ROW_DIFF_1 && colDiff == VALID_COL_KILL_DIFF;
    } else {
      return rowDiff == VALID_WHITE_ROW_DIFF_1 && colDiff == VALID_COL_KILL_DIFF;
    }
  }

  protected boolean hasMoved() {
    if (this.getColor() == ColorGood.BLACK && this.getRow() == BLACK_START_ROW) {
      return false;
    }
    if (this.getColor() == ColorGood.WHITE && this.getRow() == WHITE_START_ROW) {
      return false;
    }
    return true;
  }
}

class QueenGood extends AbstractChessPieceGood {
  public QueenGood(int row, int col, ColorGood color) {
    super(row, col, color);
  }

  public QueenGood(QueenGood original) {
    this(original.getRow(), original.getCol(), original.getColor());
  }

  @Override
  public boolean canMove(int row, int col) {
    if (row < 0 || row > 7 || col < 0 || col > 7
            || (row == this.getRow() && col == this.getCol())){
      return false;
    }

    int rowDiff = Math.abs(row-this.getRow());
    int colDiff = Math.abs(col-this.getCol());
    return (rowDiff == colDiff || rowDiff == 0 || colDiff == 0);
  }

  @Override
  public boolean canKill(ChessPieceGood piece) {
    return (piece.getColor() != this.getColor()
            && this.canMove(piece.getRow(), piece.getCol()));
  }
}

class RookGood extends AbstractChessPieceGood {
  private static final int VALID_ROOK_0 = 0;

  public RookGood(int row, int col, ColorGood color) {
    super(row, col, color);
  }

  public RookGood(RookGood original) {
    this(original.getRow(), original.getCol(), original.getColor());
  }

  @Override
  public boolean canMove(int row, int col) {
    if (row < 0 || row > 7 || col < 0 || col > 7
            || row == this.getRow() || col == this.getCol()){
      return false;
    }

    int rowDiff = Math.abs(row-this.getRow());
    int colDiff = Math.abs(col-this.getCol());
    return (rowDiff == VALID_ROOK_0 || colDiff == VALID_ROOK_0);
  }

  @Override
  public boolean canKill(ChessPieceGood piece) {
    return (piece.getColor() != this.getColor()
            && this.canMove(piece.getRow(), piece.getCol()));
  }
}