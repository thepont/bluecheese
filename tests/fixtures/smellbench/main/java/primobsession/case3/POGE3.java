package primobsession.case3;

import java.util.HashSet;
import java.util.Set;

class TutorialModalManagerGood {
  private boolean isAssigned;
  private String message;
  private Integer numOfWalkThru;
  private String bannerMessage;

  private final Integer MAX_NUM_OF_WALK_THRU = 10;
  private final Integer MIN_NUM_OF_WALK_THRU = 3;

  public TutorialModalManagerGood(boolean isAssigned) { this.isAssigned = false; }

  public void setGreetingModal(String message) {
    this.message = message;
  }

  public void setNumOfWalkThru(Integer numOfWalkThru) {
    this.numOfWalkThru = numOfWalkThru;
  }

  public void setOpenSystemSettings(String bannerMessage) {
    this.bannerMessage = bannerMessage;
  }

  public void reset() {
    this.isAssigned = false;
    this.message = "";
    this.numOfWalkThru = 0;
    this.bannerMessage = "";
  }

  public void adjustNumOfWalkThru(Integer number) {
    int res = this.numOfWalkThru + number;

    if (res <= MAX_NUM_OF_WALK_THRU && res >= MIN_NUM_OF_WALK_THRU) {
      this.numOfWalkThru += number;
    } else {
      System.out.println("Please change to another number since this adjustment will break the range!");
    }
  }
}

class LoginModelGood {
  private String email;
  private Set<String> registeredEmails = new HashSet<>();
  private primobsession.case3.TutorialModalManagerGood tutorialModals = new primobsession.case3.TutorialModalManagerGood(true);

  private final String GREETING_MSG = "hello world!";
  private final Integer NUM_OF_WALKTHRU = 5;
  private final String BANNER_MSG = "Go to Settings";

  public LoginModelGood(String email) {
    this.email = email;
  }

  public void setTutorialModals() {
    tutorialModals.setGreetingModal(GREETING_MSG);
    tutorialModals.setNumOfWalkThru(NUM_OF_WALKTHRU);
    tutorialModals.setOpenSystemSettings(BANNER_MSG);
  }

  public void registerEmail(String email) {
    registeredEmails.add(email);
    System.out.println("Registered email: " + email);
  }

  public boolean isRegistered(String email) {
    return registeredEmails.contains(email);
  }
}