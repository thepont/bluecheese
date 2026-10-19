package dataclass.case3;

import java.util.ArrayList;
import java.util.List;

class TaskVariation {
    private String name;
    private String description;
    private boolean completed;

    public TaskVariation(String name, String description) {
        this.name = name;
        this.description = description;
        this.completed = false;
    }

    public String getName() {
        return this.name;
    }

    public String getDescription() {
        return this.description;
    }

    public boolean getStatus() {
        return this.completed;
    }

    public void markAsCompleted() {
        this.completed = true;
    }

}

class TaskVariationList {
    List<TaskVariation> tasks;

    public TaskVariationList() {
        this.tasks = new ArrayList<>();
    }

    public void printTaskList() {
        for (TaskVariation task : tasks) {
            System.out.println(" - " + task.getName() + " : " + task.getDescription() + " :: " + task.getStatus());
        }
    }

    public void addTask(String name, String description) {
        TaskVariation newTask = new TaskVariation(name, description);
        tasks.add(newTask);
    }

    public void markTaskAsCompleted(String name) {
        for (TaskVariation task : tasks) {
            if (task.getName().equals(name)) {
                task.markAsCompleted();
            }
        }
    }

}
