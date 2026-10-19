## Code Smell: Temporary Field

### Description
Temporary fields get their values (and thus are needed by objects) only under certain circumstances. Outside of these circumstances, they’re empty.

#### Problem TFBE1.java
`monthlyInterestRate` and `numMonths` are fields only used if term is less than 1 year. Or else there is no use for them.

```
Observed Code Smells:
- Temporary Field (lines 20-21)
```

#### Solution TFGE1.java
Applied `Extract Class` to extract into a `MonthlyInterestCalculatorGood` and performed action similar to `Replace Method with Method Object`.

```
Refactoring Applied:
- Temporary Field
    - Extract Class (MonthlyInterestCalculatorGood)
```

```
Observed Code Smells After Refactoring:
- None
```

#### Problem TFBE2.java
The line 50 is checking if a customer is null, considering this case of nullable customer is reasonable 
in such a scenario, some object should be created to capture this situation and prevents program crashes
from NullPointer issues.
```
Observed Code Smells:
- Temporary field (line 40, 47, 54)
```

#### Solution TFGE2.java
Apply `Introduce Null Object` by creating a new subclass of `Customer` called `NullCustomer`, 
mostly just an informative temp variable for checking for null.

```
Refactoring Applied:
- Temporary Field:
    - Introduce Null Object (line 22, line 40-48, line 56, line 65, line 74)
```

```
Observed Code Smells After Refactoring:
- None
```

#### Problem TFBE3.java
Variables a and b are always used but c is used only when Quadratic/Cubic equations need to be solved and d only when Cubic equations need to be solved.

```
Observed Code Smells:
- Temporary Field (lines 4-5)
```

#### Solution TFGE3.java
Applied `Extract Class` to extract all the methods into their own Classes 'LinearSolver', 'QuadraticSolver' and 'CubicSolver' and deleted the original 'PolynomialSolver' class

```
Refactoring Applied:
- Temporary Field
    - Extract Class (LinearSolver, QuadraticSolver, CubicSolver)
```

```
Observed Code Smells After Refactoring:
- None
```