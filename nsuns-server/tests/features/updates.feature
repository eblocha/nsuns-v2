Feature: Updates

  Scenario: Updating maxes with no reps
    Given I am an anonymous user
    Given A profile with name "test" exists
    And A program with name "test program" exists
    And A movement with name "bench press" exists
    And I fetch all movements
    And I have a max of 100 in "bench press"
    When I run updates
    And I fetch my maxes
    Then My "bench press" max is 100

  Scenario: Updating maxes with 1 rep
    Given I am an anonymous user
    Given A profile with name "test" exists
    And A program with name "test program" exists
    And A movement with name "bench press" exists
    And I fetch all movements
    And I have 1 rep in "bench press"
    And I have a max of 100 in "bench press"
    When I run updates
    And I fetch my maxes
    Then My "bench press" max is 100

  Scenario: Updating maxes with 3 reps
    Given I am an anonymous user
    Given A profile with name "test" exists
    And A program with name "test program" exists
    And A movement with name "bench press" exists
    And I fetch all movements
    And I have 3 reps in "bench press"
    And I have a max of 100 in "bench press"
    When I run updates
    And I fetch my maxes
    Then My "bench press" max is 105
  
  Scenario: Updating maxes with 5 reps
    Given I am an anonymous user
    Given A profile with name "test" exists
    And A program with name "test program" exists
    And A movement with name "bench press" exists
    And I fetch all movements
    And I have 5 reps in "bench press"
    And I have a max of 100 in "bench press"
    When I run updates
    And I fetch my maxes
    Then My "bench press" max is 110
  
  Scenario: Updating maxes with 6 reps
    Given I am an anonymous user
    Given A profile with name "test" exists
    And A program with name "test program" exists
    And A movement with name "bench press" exists
    And I fetch all movements
    And I have 6 reps in "bench press"
    And I have a max of 100 in "bench press"
    When I run updates
    And I fetch my maxes
    Then My "bench press" max is 115

  Scenario: Undoing updates
    Given I am an anonymous user
    Given A profile with name "test" exists
    And A program with name "test program" exists
    And A movement with name "bench press" exists
    And I fetch all movements
    And I have a max of 100 in "bench press"
    And I have a max of 105 in "bench press"
    When I undo updates
    And I fetch my maxes
    Then My "bench press" max is 100
