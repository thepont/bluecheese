abstract class UnusedAbstraction {}

abstract class LightlyUsedAbstraction {}

abstract class UsedAbstraction {}

class UsesAbstraction {
  LightlyUsedAbstraction lightlyUsed;
  UsedAbstraction first;
  UsedAbstraction second;
}
