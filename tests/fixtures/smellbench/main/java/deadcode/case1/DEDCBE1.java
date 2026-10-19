package deadcode.case1;

class PhysicsEngineBad {
  private double gravity;
  private double initialvelocity;
  private double time;
  private double finalVelocity;
  private double distance;

  public PhysicsEngineBad(double gravity, double velocity, double time) {
      this.gravity = 9.8;
      this.initialvelocity = velocity;
      this.time = time;
      this.finalVelocity = -1;
  }

  public void calculateDistance() {
      double distance = initialvelocity * time + 0.5 * gravity * time * time;
  }

  public void calculateFinalVelocity() {
      finalVelocity = initialvelocity + gravity * time;
  }

  public void printPhysicsParameters() {
        System.out.println("Gravity: " + gravity);
        System.out.println("Initial Velocity: " + initialvelocity);
        System.out.println("Time: " + time);
        System.out.println("Final Velocity: " + finalVelocity);
  }


}