package middleman.case1;

import java.util.Map;
import java.util.List;
import java.util.HashMap;
import java.util.Random;
import java.util.ArrayList;

class FactorySupervisorGood{
  private final String name;
  private Map<FactoryWorkerGood, String> assignedTasks;
  private List<String> availableTasks;

  public FactorySupervisorGood(String name, List<String> availableTasks) {
    this.name = name;
    this.availableTasks = availableTasks;
    this.assignedTasks = new HashMap<>();
  }

  public String assignTask(FactoryWorkerGood worker) {
    Random random = new Random();
    int randomTaskIndex = random.nextInt(availableTasks.size());
    String task = availableTasks.get(randomTaskIndex);
    assignedTasks.put(worker, task);
    return task;
  }

  public void printAssignedTasks() {
    for(Map.Entry<FactoryWorkerGood, String> entry : assignedTasks.entrySet()) {
      FactoryWorkerGood worker = entry.getKey();
      String task = entry.getValue();
      System.out.print("Worker: ");
      worker.getName();
      System.out.print("Task: " + task);
    }
  }

  public void printSupervisorDetails() {
    System.out.println("Name: " + name);
  }
}

class FactoryWorkerGood {
  private String name;
  private List<String> tasks;
  private FactorySupervisorGood supervisor;

  public FactoryWorkerGood(String name, FactorySupervisorGood supervisor) {
    this.name = name;
    this.supervisor = supervisor;
    this.tasks = new ArrayList<>();
  }

  public void getTask() {
    String task = supervisor.assignTask(this);
    tasks.add(task);
  }

  public void printTasks() {
    for(String task : tasks) {
      System.out.println("Task: " + task);
    }
  }
  public void getName() {
    System.out.println(name);
  }
}
