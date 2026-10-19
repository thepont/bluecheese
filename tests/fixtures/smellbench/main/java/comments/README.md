## Code Smell: Comments
### Description
A method is filled with explanatory comments.

#### Problem CBE1.java
The comments seem to explain the method's functionality as it is not clear from the name. It also describes an assertion conditon in which this method should work. Also due to the equation looking complicated, the developer has defined how and why the calculations are done for total amount calculation.
Primitives have also been used for fields such as `orderId` which should have strict format and `orderStatus` which could have been an enum.

```
Observed Code Smells:
- Comments (lines 17-25)
- Primitive Obsession (lines 2 & 7)
```

#### Solution CGE1.java
Applied `Extract Variable` to break the complicated expression into sub-expressions. Applied `Rename Method` from `solve()` to `calculateOrderAmount()` so that the functionality is clear by the method name. Applied `Introduce Assertion` since the comment regarding assertion will not force the system to check for this conditon.
```
Refactoring Applied:
- Comments
    - Extract Variable (lines 22-24)
    - Rename Method (solve -> calculateOrderAmount)
    - Introduce Assertion (line 18)
```

```
Observed Code Smells After Refactoring:
- Primitive Obsession (lines 2 & 7)
```

#### Problem CBE2.java
The comment explains that the section of code makes a transpose matrix.

```
Observed Code Smells:
- Comments (line 12)
```

#### Solution CGE2.java
Applied `Extract Method` to break the code into a separate method named `transposeMatrix()`. The name itself is indicative that it is for transpose matrix.

```
Refactoring Applied:
- Comments
    - Extract Method (transposeMatrix)
```

```
Observed Code Smells After Refactoring:
- None
```

#### Problem CBE3.java
The comment is defined to inform the user why the variables are final and what is the significance of the class and method.

```
Observed Code Smells:
- Comments (lines 1-3, 5, 16)
```

#### Solution
Ignore the code smell as the comments are informative and inform the developer why was something done in a certain way.

```
Refactoring Applied:
- None
```

```
Observed Code Smells After Refactoring:
- None (Ignore Comments)
```

#### Problem CBE4.java
The comments explain that this constructor should have assertions about the ranges of the RGB values but
the code does not implement the defensive block.
```
Observed Code Smells:
- Comments (lines 10-12)
```

#### Solution CGE4.java
Applied `Introduce Assertion` to clamp out the invalid ranges for the RGB values.

```
Refactoring Applied:
- Comments
    - Introduce Assertion (line 10-12)
```

```
Observed Code Smells After Refactoring:
- None
```

#### Problem CBE5.java
The comments explain the specific numeric conditions that the Mac could be upgraded.
These numeric conditions can be exacted as a series of 'ok' variables.
```
Observed Code Smells:
- Comments (lines 13-18)
```

#### Solution CGE5.java
Applied `Extract Variables` to set the conditions to be a series of 'ok' variables.

```
Refactoring Applied:
- Comments
    - Extract Variables (line 14-16)
```

```
Observed Code Smells After Refactoring:
- None
```

#### Problem CBE6.java
The comments explain that the upgrade method is checking if the conditions for upgrading the iPHONE
are met. However, these lines of code can be extracted as a separate method.
```
Observed Code Smells:
- Comments (lines 35)
```

#### Solution CGE6.java
Applied `Extract Method` to the check the conditions.

```
Refactoring Applied:
- Comments
    - Extract Variables (line 29-35)
```

```
Observed Code Smells After Refactoring:
- None
```

#### Problem CBE7.java
The comments are explaining the 3 conditions that are necessary to check before the firmware device can be 
updated.

This OOD scenario is similar to the example above, but the method to purely check the conditions has been 
extracted as a method, but not well-named yet, which results in a line of comments to explain the 
purpose of the checking. If we rename the method properly,  it will be no such comment needed.
```
Observed Code Smells:
- Comments (lines 29)
```

#### Solution CGE7.java
Applied `Rename Method` to the check the conditions.

```
Refactoring Applied:
- Comments
    - Rename Method (line 29-35)
```

```
Observed Code Smells After Refactoring:
- None
```

#### Problem CBE8.java
A lot of comments at the end of lines that bloat the code and are dispensable as the code is self-explanatory. Some comments are detailed and required which means methods can be extracted

```
Observed Code Smells:
- Comments (lines 117, 135, 138, 141, 144, 149, 154, 157)
```

#### Solution CGE8.java
Applied `Extract Method` to create a new method called `applyBrightnessToPixel` that better illustrates what that piece of code is to do.
```
Refactoring Applied:
- Comments
    - Extract Method (line 147-158)
```

```
Observed Code Smells After Refactoring:
- None
```