package comments.case2;

class MatrixBad {
  private final int[][] matrix;

  public MatrixBad(int[][] matrix) {
    this.matrix = matrix;
  }

  public void printDifferentMatrix() {
    System.out.println("Original Matrix:");
    printMatrix(matrix);

    // Code to make a temporary transposed matrix
    int[][] differentMatrix = new int[matrix[0].length][matrix.length];
    for(int i = 0; i < matrix.length; i++) {
      for (int j = 0; j < matrix[0].length; j++) {
        differentMatrix[j][i] = matrix[i][j];
      }
    }

    System.out.println("Transposed Matrix:");
    printMatrix(differentMatrix);
  }

  private void printMatrix(int[][] matrix) {
      for(int i = 0; i < matrix.length; i++) {
      for(int j = 0; j < matrix[0].length; j++) {
          System.out.print(matrix[i][j] + " ");
      }
      System.out.println();
      }
  }
}
