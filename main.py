"""
ETL-Query script
"""
import fire
from mylib.extract import extract
from mylib.transform_load import load
from mylib.query import query


def main(query):
    """Run the ETL process"""
    # Extract
    print("Extracting data...")
    extract()

    # Transform and load
    print("Transforming data...")
    load(query)

    # Query
    print("Querying data...")
    query(query)


if __name__ == "__main__":
    query = "SELECT COUNT(*) FROM table1"
    #load("SELECT COUNT(*) FROM table1")
    fire.Fire(main(query))
    #query("SELECT COUNT(*) FROM table1")