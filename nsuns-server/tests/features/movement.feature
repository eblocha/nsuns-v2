Feature: Movements

  Scenario: Create a movement
    When I create a movement with name "bench press"
    And I fetch all movements
    Then My movement has the name "bench press"

  Scenario: Update a movement
    Given A movement with name "bench press" exists
    When I update the movement to have name "squat"
    And I fetch all movements
    Then My movement has the name "squat"

  Scenario: Create a movement with non-unique name
    Given A movement with name "bench press" exists
    Then I cannot create a movement with name "bench press"
