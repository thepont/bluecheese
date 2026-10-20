abstract class RepositoryBase {}
abstract class ControllerBase {}

class UserRepository extends RepositoryBase {}
class OrderRepository extends RepositoryBase {}
class UserController extends ControllerBase {}
class OrderController extends ControllerBase {}

abstract class VehicleBase {}
abstract class ShapeBase {}
class Bicycle extends VehicleBase {}
class Circle extends ShapeBase {}
