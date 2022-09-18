Feature: Add a dependency to a project
  Background: An already initialised project exists
    Given A location for the new project
    Given The project exists

  Scenario Outline: A named dependency without a version constraint is specified
    Given The dependency <dependency> is to be added to the project
    When The dependencies are added to the project
    Then Adding dependencies succeeds
    And <dependency> is named in pyproject.toml

    Examples:
      | dependency  |
      | tqdm        |
      | urllib3     |
      | requests    |

  Scenario Outline: A named dependency with version constraint is specified
    Given The versioned dependency <dependency>@<version> is to be added to the project
    When The dependencies are added to the project
    Then Adding dependencies succeeds
    And <dependency> is named in pyproject.toml with version <version>

    Examples:
      | dependency  | version |
      | tqdm        | ^4.0.0  |
      | urllib3     | >1.0.0  |
      | requests    | *       |