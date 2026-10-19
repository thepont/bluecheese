package lazyclass.case1;

enum OperationBad {
  ADD, SUBTRACT
}
class MatrixOperationValidatorBad {
    public boolean canMatrixOperationBePerformed(int [][]matrix1, int [][]matrix2) {
      return matrix1.length == matrix2.length && matrix1[0].length == matrix2[0].length;
    }
}
class MatrixOperationBad {
  private final int [][]matrix1;
  private final int [][]matrix2;
  private final int [][]result;

  public MatrixOperationBad(int [][]matrix1, int [][]matrix2) throws IllegalArgumentException {
    MatrixOperationValidatorBad validator = new MatrixOperationValidatorBad();
      if(!validator.canMatrixOperationBePerformed(matrix1, matrix2)) {
          throw new IllegalArgumentException("Matrix dimensions are not same");
      }
      this.matrix1 = matrix1;
      this.matrix2 = matrix2;
      result = new int[matrix1.length][matrix1[0].length];
  }

  public void addMatrices() {
    performAddSubtractOperation(OperationBad.ADD);
  }

  public void subtractMatrices() {
    performAddSubtractOperation(OperationBad.SUBTRACT);
  }

  public void printResult() {
      for(int i = 0; i < result.length; i++) {
          for(int j = 0; j < result[0].length; j++) {
              System.out.print(result[i][j] + " ");
          }
          System.out.println();
      }
  }

  private void performAddSubtractOperation(OperationBad operation) {
    for(int i = 0; i < matrix1.length; i++) {
      for(int j = 0; j < matrix1[0].length; j++) {
        if(operation.equals(OperationBad.ADD)) {
          result[i][j] = matrix1[i][j] + matrix2[i][j];
        } else if(operation.equals(OperationBad.SUBTRACT)) {
          result[i][j] = matrix1[i][j] - matrix2[i][j];
        }
      }
    }
  }
}