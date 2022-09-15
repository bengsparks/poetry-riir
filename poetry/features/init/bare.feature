Feature: Create a new bare project
	Background: Bare project is specified
		Given Bare project settings

	Scenario: Create bare project at a currently non-existent directory
		When Project creation is attempted
		Then Creation of project directory succeeds
		And Project directory contains pyproject.toml

	Scenario: Create bare project in an existent, empty directory
		Given Project directory exists in working directory
		When Project creation is attempted
		Then Creation of project directory succeeds
		And Project directory contains pyproject.toml

	Scenario: Attempt to create bare project in an existent, non-empty directory
		Given Project directory exists in working directory
		And Project directory is not empty
		When Project creation is attempted
		Then Creation of project directory fails

