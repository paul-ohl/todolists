# Todolist learning

## Context

There are quite a few web frameworks that I want to explore, both front and
backend.

Having a good, simple and standard problem to test them on is very important
for me to quickly assess whether I want to keep working with that framework or
not.

The todolist felt like the most obvious answer.

## Features

### User Authentication

Implement user registration and login functionality.

I am not particularly interested in password management, so no security
concerns here.

### Task CRUD Operations

Users should be able to perform basic CRUD (Create, Read, Update, Delete)
operations on tasks.

### Task Status

Implement task status.

I only want to manage TODO and DONE status.

### Task Categories

Allow users to categorize tasks into different categories (e.g., work,
personal, shopping).

Depending on my level of motivation the categories will be:
- A fixed list
- Created separately
- Created dynamically if not already existing when creating a task

### Search and Filter

Provide a way for users to search for tasks and filter them based on categories
and status.

The only filters I am interested in are category and status filters.

There will be no tag system.

### Responsive Design

Make sure the web app is responsive and looks good on both desktop and mobile
devices.

Unless I'm working with a frontend framework, this is a bonus.

### Database

Unless I am specifically learning with a specific database in mind, the
database used will always be sqlite.

### Deployment

I want each app to either run in a Docker container, or have its own deployment
strategy.
