Feature: Programs

  Scenario: Create an empty program
    Given A profile with name "test" exists
    When I create a program with name "test program"
    And I fetch my programs
    Then My program has the name "test program"

  Scenario: Update a program
    Given A profile with name "test" exists
    And A program with name "test program" exists
    When I update my program to have name "test program 2"
    And I fetch my programs
    Then My program has the name "test program 2"

  Scenario: Delete a program
    Given A profile with name "test" exists
    And A program with name "test program" exists
    When I delete my program
    And I fetch my programs
    Then My program does not exist

  Scenario: Adding sets
    Given A profile with name "test" exists
    And A program with name "test program" exists
    And A movement with name "bench press" exists
    And A movement with name "squat" exists
    And I fetch all movements
    When I create a "bench press" set for Monday
    And I create a "squat" set for Monday
    And I fetch my program summary
    Then My program has ["bench press", "squat"] on Monday

  Scenario: Reordering sets
    Given A profile with name "test" exists
    And A program with name "test program" exists
    And A movement with name "bench press" exists
    And A movement with name "squat" exists
    And I fetch all movements
    And I have a "bench press" set for Monday
    And I have a "squat" set for Monday
    When I reorder Monday from 0 to 1
    And I fetch my program summary
    Then My program has ["squat", "bench press"] on Monday
