package primobsession.case3;

import java.util.HashSet;
import java.util.Set;

class LoginModel {
  private String email;
  private Set<String> registeredEmails = new HashSet<>();
  private final String[] tutorialModals = new String[3];

  public LoginModel(String email) {
    this.email = email;
  }

  public void setTutorialModals() {
    tutorialModals[0] = "Greeting Message";
    tutorialModals[1] = "4";
    tutorialModals[2] = "System Settings";
  }

  public void registerEmail(String email) {
    registeredEmails.add(email);
    System.out.println("Registered email: " + email);
  }

  public boolean isRegistered(String email) {
    return registeredEmails.contains(email);
  }
}