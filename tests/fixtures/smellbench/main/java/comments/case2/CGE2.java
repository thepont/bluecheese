package comments.case2;

class MatrixGood {
  private final int[][] matrix;

  public MatrixGood(int[][] matrix) {
    this.matrix = matrix;
  }

  public void printDifferentMatrix() {
    System.out.println("Original Matrix:");
    printMatrix(matrix);

    System.out.println("Transposed Matrix:");
    printMatrix(transposeMatrix());
  }

  private void printMatrix(int[][] matrix) {
    for(int i = 0; i < matrix.length; i++) {
      for(int j = 0; j < matrix[0].length; j++) {
        System.out.print(matrix[i][j] + " ");
      }
      System.out.println();
    }
  }

  private int [][] transposeMatrix() {
    int[][] transpose = new int[matrix[0].length][matrix.length];
    for(int i = 0; i < matrix.length; i++) {
      for (int j = 0; j < matrix[0].length; j++) {
        transpose[j][i] = matrix[i][j];
      }
    }
    return transpose;
  }
}