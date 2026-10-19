## Code Smell: Long Method
### Description
A method contains too many lines of code.

#### Problem LMBE1.java
A single method `makeTransaction` has too many lines of code (30+). Also there are too many switch statements in the method.
Also `cardNumber` and `txnId` are defined in primitives without any validation logic for them.
```
Observed Code Smells:
- Long Method (line 86-121)
- Switch Statements (line 90-119)
- Primitive Obsession (lines 23, 41)
- Duplicate Code (lines 92, 95, 98, 101, 104)
```

#### Solution LMGE1.java
Applied `Extract Method` to break it down into multiple helper methods. Also applied `Decompose Conditional` to wrap complicated parts of the switch logic into their own methods. 

```
Refactoring Applied:
- Long Method
    - Extract Method (performTransaction, checkTransactionValidity, verifyCardDetails, checkAccountStandingAndStatus, checkTransactionAmountWithinLimit)
    - Decompose Conditional (performTransaction, checkTransactionValidity, verifyCardDetails, checkAccountStandingAndStatus, checkTransactionAmountWithinLimit)
```

```
Observed Code Smells After Refactoring:
- Primitive Obsession (lines 23, 41)
```

#### Problem LMBE2.java
The method `calculateAllMotion` is a method with too many lines of code with a lot of temp variables in it.

```
Observed Code Smells:
- Long Method (line 12-25)
```

#### Solution LPGE2.java
Applied `Replace Temp with Query` to replace the temp variables with private method calls. Safe to ignore the data clumps created because making mass and time into a single object would not make a relevant object leading to data class and increase dependency on a new class.

```
Refactoring Applied:
- Long Method
    - Replace Temp with Query (calculateFinalVelocity, calculateFinalDisplacement, calculateTimeToReachGround, calculateMomentum, calculateWeight, calculateKineticEnergy, calculatePower)
```

```
Observed Code Smells After Refactoring:
- None (Ignore Data Clumps)
```

#### Problem LMBE2.java
The class TextAnalyzer is a utility class that contains a useful method to calculate information about a .txt file.
The 'analyze' method is now doing everything: 
1. Calculate the word count,
2. Calculate the sentence count, 
3. Calculate the average word length,
4. Identify the most common word.

which, makes the method extremely long.
```
Observed Code Smells:
- Long Method (lines 15-51)
```

#### Solution LMGE2.java
Applied `Extract Method` to take each calculation out as a separate method.

```
Refactoring Applied:
- Long Method
    - Extract Method (line 13-58)
```

```
Observed Code Smells After Refactoring:
- None
```

#### Problem LMBE3.java
The method `boardFlight()` does a lot of work that can make it confusing to understand and should be broken into smaller methods.
```
Observed Code Smells:
- Long Method (line 62-92)
```

#### Solution LMGE3.java
Applied `Extract Method`s to the method and created three new methods `securityCheck`, `noFlyListCheck`, `passengerCanBoard`.

```
Refactoring Applied:
- Long Method
    - Extract Method (securityCheck, noFlyListCheck, passengerCanBoard)
```

```
Observed Code Smells After Refactoring:
- None
```

The if-else-if statements in boardAirplane() method are not considered a code smell because of the ignore case in Switch Statements code smell `When a switch operator performs simple actions, there’s no reason to make code changes`