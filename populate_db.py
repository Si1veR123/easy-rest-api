# FOR TESTING

import sqlite3

connection = sqlite3.connect("database.db")
cursor = connection.cursor()

cursor.execute('INSERT INTO people (name, age) VALUES ("john", 8)')
cursor.execute('INSERT INTO people (name, age) VALUES ("mary", 54)')

connection.commit()
connection.close()
