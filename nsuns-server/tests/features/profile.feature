Feature: Profiles

  Scenario: Creating a profile
    When I create a profile with name "test"
    And I fetch all profiles
    Then My profile has the name "test"

  Scenario: Updating a profile
    Given A profile with name "test" exists
    When I rename the profile to "test2"
    And I fetch all profiles
    Then My profile has the name "test2"

  Scenario: Deleting a profile
    Given A profile with name "test" exists
    When I delete the profile
    And I fetch all profiles
    Then My profile does not exist
