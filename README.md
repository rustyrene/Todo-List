# Todo-List
A simple todo list that allows you to log in to your account and 
create update or delete todos. Also allowing you to have different 
categories to sort them to your wishes.

# 💻 Tech Stack:

## Backend:
The Api is build using ![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white) with the Actix Web framework
and sqlx to perform the nessecary operations on the Database. It uses JWT-Cookies for authentication.

## Frontend:
The Frontend is build using ![Svelte](https://img.shields.io/badge/svelte-%23f1413d.svg?style=for-the-badge&logo=svelte&logoColor=white) and the 
svelte-spa-router.

## Database
All users and todos are saved in a ![Postgres](https://img.shields.io/badge/postgres-%23316192.svg?style=for-the-badge&logo=postgresql&logoColor=white) Database
that is hosted inside a ![Docker](https://img.shields.io/badge/docker-%230db7ed.svg?style=for-the-badge&logo=docker&logoColor=white) Container.
