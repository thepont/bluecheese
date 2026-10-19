package lazyclass.case1;

enum OperationGood {
  ADD, SUBTRACT
}

class MatrixOperationGood {
  private final int [][]matrix1;
  private final int [][]matrix2;
  private final int [][]result;

  public MatrixOperationGood(int [][]matrix1, int [][]matrix2) throws IllegalArgumentException {
    if(!canMatrixOperationBePerformed(matrix1, matrix2)) {
      throw new IllegalArgumentException("Matrix dimensions are not same");
    }
    this.matrix1 = matrix1;
    this.matrix2 = matrix2;
    result = new int[matrix1.length][matrix1[0].length];
  }

  public void addMatrices() {
    performAddSubtractOperation(OperationGood.ADD);
  }

  public void subtractMatrices() {
    performAddSubtractOperation(OperationGood.SUBTRACT);
  }

  public void printResult() {
    for(int i = 0; i < result.length; i++) {
      for(int j = 0; j < result[0].length; j++) {
        System.out.print(result[i][j] + " ");
      }
      System.out.println();
    }
  }

  private void performAddSubtractOperation(OperationGood operation) {
    for(int i = 0; i < matrix1.length; i++) {
      for(int j = 0; j < matrix1[0].length; j++) {
        if(operation.equals(OperationGood.ADD)) {
          result[i][j] = matrix1[i][j] + matrix2[i][j];
        } else if(operation.equals(OperationGood.SUBTRACT)) {
          result[i][j] = matrix1[i][j] - matrix2[i][j];
        }
      }
    }
  }

  private boolean canMatrixOperationBePerformed(int [][]matrix1, int [][]matrix2) {
    return matrix1.length == matrix2.length && matrix1[0].length == matrix2[0].length;
  }
}
