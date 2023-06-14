<!-- @format -->

# Expense Tracker

This project is an implementation of an Expense Tracker web application using Rust for both the backend and frontend, developed by Cheng Lin.

## Project Overview

The Expense Tracker is a full-stack Rust application that allows users to monitor and manage their financial transactions and budgets. It provides the ability to create, view, and delete transactions within budget groups. The application uses `yew` for frontend development and `actix-web` for the backend. `gloo-net` is used for fetch requests on the frontend. SQLite for persistent storage.

This project was developed as part of a Rust programming class and aims to showcase the power and efficiency of Rust in full-stack development.

## Features

- CRUD operations for budget groups and transactions.
- Display of a summary of budget, total expenses, and remaining budget for each group.
- Persistent storage with SQLite.
- User-friendly interface built with `yew`.
- Frontend fetch requests powered by `gloo-net`.
- Backend server powered by `actix-web`.

## Run the Project

To run the project, follow these steps:

### Backend

Navigate to the backend directory:

```
cd backend
```

Then run the server:

```
cargo run
```

The server will be running at `http://localhost:5001`.

### Frontend

Navigate to the frontend directory:

```
cd frontend
```

Then run the frontend:

```
trunk serve
```

Open a browser and go to `http://localhost:8080`.

## Example of Operation

Navigating to the expense page, users can see the budget groups. Clicking on a group will display more detailed information, including the budget, total expenses, and remaining budget. Users can add new transactions and groups and delete existing ones as needed.

## Project Reflectin

The project was a successful demonstration of full-stack development in Rust. The application is functional and the user interface is intuitive and responsive. However, some challenges arose during development, especially around handling HTTP requests and responses. Also, Yew framework and understanding the intricacies of database interactions posed a significant challenge.

One area that's still a bit beyond my grasp is user authentication and authorization. It's a vital part of any secure application, but it's also a complex topic. I'm still in the process of learning and understanding its intricacies. In the future, I'd like to improve error handling, add more comprehensive tests, and complete features such as user authentication and more detailed budget analysis.

## Acknowledgments

- Rust programming language
- Yew library for frontend development
- Gloo-net library for fetch requests
- Actix-web framework for backend server
