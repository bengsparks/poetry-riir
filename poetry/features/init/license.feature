Feature: Create a new project with a license
  Scenario Outline: Project with LICENSE is specified
    Given Licensed project with <license>
    When Project creation is attempted
    Then Creation of project directory succeeds
    And License key is set in pyproject.toml as <license>

    Examples:
      | license |
      | MIT     |
      | Apache-2.0 |