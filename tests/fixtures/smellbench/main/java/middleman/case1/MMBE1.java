package middleman.case1;

import java.util.Map;
import java.util.List;
import java.util.HashMap;
import java.util.Random;
import java.util.ArrayList;

class FactorySupervisorBad{
  private final String name;
  private Map<FactoryWorkerBad, String> assignedTasks;
  private List<String> availableTasks;

  public FactorySupervisorBad(String name, List<String> availableTasks) {
      this.name = name;
      this.availableTasks = availableTasks;
      this.assignedTasks = new HashMap<>();
  }

  public String assignTask(FactoryWorkerBad worker) {
      Random random = new Random();
      int randomTaskIndex = random.nextInt(availableTasks.size());
      String task = availableTasks.get(randomTaskIndex);
      assignedTasks.put(worker, task);
      return task;
  }

  public void printAssignedTasks() {
      for(Map.Entry<FactoryWorkerBad, String> entry : assignedTasks.entrySet()) {
          FactoryWorkerBad worker = entry.getKey();
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

class FactoryBad {
    private FactorySupervisorBad supervisor;

    public FactoryBad(FactorySupervisorBad supervisor) {
        this.supervisor = supervisor;
    }

    public void printFactoryDetails() {
        System.out.println("Supervisor Details: ");
        supervisor.printSupervisorDetails();
        System.out.println("Tasks remaining: ");
        supervisor.printAssignedTasks();
    }

    public String assignTasks(FactoryWorkerBad worker) {
      return supervisor.assignTask(worker);
    }
}

class FactoryWorkerBad {
    private String name;
    private List<String> tasks;
    private FactoryBad factory;

    public FactoryWorkerBad(String name, FactoryBad factory) {
        this.name = name;
        this.factory = factory;
        this.tasks = new ArrayList<>();
    }

    public void getTask() {
        String task = factory.assignTasks(this);
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