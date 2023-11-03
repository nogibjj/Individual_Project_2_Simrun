# CSV to SQLite Importer
Youtube Link : https://youtu.be/jTOhYgCWTu0
[![Rust CI/CD](https://github.com/nogibjj/Individual_Project_2_Simrun/actions/workflows/cicd.yml/badge.svg)](https://github.com/nogibjj/Individual_Project_2_Simrun/actions/workflows/cicd.yml)

This is a command-line tool that imports data from a CSV file into an SQLite database, allows you to rename columns, and execute SQL queries on the imported data.

## Prerequisites

Before running this tool, make sure you have the following prerequisites installed:

- Rust programming language and Cargo
- SQLite
- CSV file to import
- create a virtual environment

### Creating a Python Virtual Environment

To create a virtual environment in Python for your project, follow these steps:

1. **Open a Terminal or Command Prompt:**

   First, open a terminal or command prompt on your system.

2. **Navigate to Your Project Directory:**

   Use the `cd` command to navigate to your project directory. For example:

   ```sh
   cd /path/to/your/project
3. **Create the Virtual Environment:**
   Run the following command to create a virtual environment in your project directory. Replace venv with the desired name for your virtual environment:
   ```sh
   python3 -m venv venv
   This command uses Python 3 to create a virtual environment named "venv."

5. **Activate the Virtual Environment:**
   Depending on your operating system, use one of the following commands to activate the virtual environment:
   ```sh
   .\venv\Scripts\Activate

7. Verify the Virtual Environment:

To verify that the virtual environment is active, you can run the which command on macOS and Linux or the where command on Windows to check which Python interpreter is being used. It should point to the Python interpreter within your virtual environment.

8. Deactivate the Virtual Environment (Optional):

When you're done working in the virtual environment, you can deactivate it by running the following command:
```sh
deactivate
```
### CRUD Operations
**Create:**
  The provided code enables the creation of an SQLite database and the insertion of data from a CSV file into the database. First, it establishes an SQLite database       connection and defines a table structure called "data" to store the CSV data. Subsequently, it reads the data from the specified CSV file and inserts it into the "data" table in the database. The create operation ensures that the data is ready for further manipulation and retrieval.

**Read:**
  The code supports the reading operation by executing SQL queries to retrieve specific data from the SQLite database. It allows users to interactively enter SQL queries from the terminal, which can include filtering criteria. For example, the code can retrieve the "model year" of cars with the name "amc hornet" or gather information about cars with more than six cylinders. The read operation facilitates the extraction of relevant data from the database based on user-defined criteria.

**Update:**
  With the update operation, the code offers the ability to modify data stored in the SQLite database. In this case, the code updates the "cylinders" column, changing any instances of '3' to '4'. After applying the update, the code retrieves the updated data from the database. This operation ensures that data can be adjusted or corrected as needed while preserving the database's integrity and structure.

**Delete:**
  The delete operation involves the removal of specific data from the SQLite database. The code executes SQL queries to delete rows that match specific criteria, such as cars with '5' cylinders. Once the deletion is complete, the code retrieves and displays the remaining data. This operation enables the removal of unwanted or irrelevant data from the database, maintaining data quality and consistency.

### Github Copilot
  **How did Github Copilot help with this project**
  - SQL Queries: Copilot provided SQL query suggestions and to help construct complex SQL statements. Which was beneficial to extract specific data from the SQLite database or perform more advanced operations.
  - Understanding Rust: Since I am new to certain aspects of Rust, SQLite, or CSV handling, Copilot chat provided explanations, examples, and references to documentation, aid in understanding and learning process. Copilot provided Rust code snippets, SQL statements, and CSV file handling code, reducing the chances of syntax errors and saving time.

### Usage

1. Clone or download this repository to your local machine.
   
2. Cargo new "project name". I called my project "rust_crud". Cargo new is a Rust command that creates a new project directory, including essential project files like Cargo.toml and an initial source code file. It streamlines the process of setting up a new Rust project by providing a basic project structure.
   
3. Build the project using `cargo build`. This checks the Cargo.toml and determines which dependencies to use. When you run cargo build in your Rust project directory, it compiles your Rust code and its dependencies into an executable binary file. By default, the resulting binary is placed in the target/debug directory of your project.

4. You can then run the compiled binary to execute your Rust program with the following command:
  ```sh
   cargo run
```

## Examples of how the command line tool works:
Query : select mpg, cylinders from data where cylinders > 6

<img width="533" alt="image" src="https://github.com/nogibjj/Individual_Project_2_Simrun/assets/141798228/4265ffdb-40c3-4629-af97-9c117348308d">


<img width="609" alt="image" src="https://github.com/nogibjj/Individual_Project_2_Simrun/assets/141798228/31d91ea3-4320-493b-bcf5-cef35cf2566f">


<img width="477" alt="image" src="https://github.com/nogibjj/Individual_Project_2_Simrun/assets/141798228/3e03bcc1-e1bf-4890-9a8e-13a91b456c29">







