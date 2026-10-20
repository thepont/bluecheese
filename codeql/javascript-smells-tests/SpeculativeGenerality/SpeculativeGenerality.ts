abstract class UnusedBase {}

interface UnusedContract {}

interface ReferencedContract {}

class FirstUser implements ReferencedContract {}

class SecondUser implements ReferencedContract {}

abstract class ReferencedBase {}

class Derived extends ReferencedBase {}

class AnotherDerived extends ReferencedBase {}
