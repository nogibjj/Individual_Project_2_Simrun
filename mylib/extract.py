"""
Extract a dataset from a URL like Kaggle or data.gov.
JSON or CSV formats tend to work well

Jeopardy dataset
"""
import sqlite3
import pandas as pd


def create(Database_Name):
    con = sqlite3.connect(Database_Name)
    df = pd.read_csv("/workspaces/Individual_Project_2_Simrun-/data/auto_data.csv")

    df.to_sql("auto", con, if_exists="replace", index=False)

    cursor = con.execute("SELECT * FROM auto")
    result = cursor.fetchall()
    con.commit()
    con.close()

    return result

def read(database_name):
    cnt = sqlite3.connect(database_name)

    # "Name of model with this car name  :"

    cursor = cnt.execute(
        """SELECT "model year" FROM auto WHERE
                        "car name" == "amc hornet";"""
    )

    result1 = cursor.fetchall()
    print("")  # Print new line

    # "Name of University where No of student per staff is less than 40.0"

    cursor = cnt.execute(
        """SELECT "car name", "cylinders" FROM
    auto WHERE "cylinders" > 6;"""
    )
    result2 = cursor.fetchall()

    print("")

    cursor = cnt.execute(
        """SELECT "car name", "cylinders" FROM
auto WHERE ("cylinders" < 8) AND ("car name" == "amc hornet");"""
    )
    result3 = cursor.fetchall()
    cnt.commit()

    return result1, result2, result3


def update(Database_Name):
    con = sqlite3.connect(Database_Name)

    con.execute(
        """UPDATE auto SET "cylinders" = '4' WHERE
    "cylinders" = '3';"""
    )

    cursor = con.execute("""SELECT * FROM auto""")

    results = cursor.fetchall()
    con.commit()
    return results

def delete(Database_Name):
    con = sqlite3.connect(Database_Name)
    con.execute("""DELETE FROM auto WHERE "cylinders" = 5;""")

    cursor = con.execute("""SELECT "car name" FROM auto""")
    con.commit()

    results = cursor.fetchall()
    return results


create("auto.db")
read("auto.db")
update("auto.db")
delete("auto.db")
