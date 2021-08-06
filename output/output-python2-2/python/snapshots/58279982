
"""
populate multiplex table in mythtv's database with contents of 
dvb util's scan configuration file
"""


import doctest
import sys

import MySQLdb


def none_field(s):
    if s == "NONE":
        return "none"
    else:
        return s


def mod_field(s):
    """
    >>> mod_field("QAM64")
    'qam_64'
    """
    if s.startswith("QAM"):
        return "qam_%s" % s[len("QAM"):]


def parse_mysql_txt():
    d = {}
    
    pairs = (("DBHostName", "host"),
             ("DBUserName", "user"),
             ("DBName", "db"),
             ("DBPassword", "passwd"))
    
    for line in file("/etc/mythtv/mysql.txt"):
        for name, parameter in pairs:
            if line.startswith("%s=" % name):
                d[parameter] = line.rstrip().split("=", 1)[1]
    return d

doctest.testmod()
    
connection_parameters = parse_mysql_txt()
assert connection_parameters["db"] == "mythconverg", connection_parameters
db = MySQLdb.connect(**connection_parameters)
cursor = db.cursor()

for line in file(sys.argv[1]):
    if line.startswith("#"):
        continue
    fields = line.rstrip().split(" ", 8)

    frequency = int(fields[1])
    bandwidth = fields[2][0]
    fec_hi = none_field(fields[3])
    fec_lo = none_field(fields[4])
    mod = mod_field(fields[5]) # constellation
    transmission_mode = fields[6][0]
    guard_interval = fields[7]
    hierarchy = none_field(fields[8])

    cursor.execute("""
        DELETE FROM dtv_multiplex WHERE frequency = %s""", (frequency,))
    cursor.execute("""
        INSERT INTO dtv_multiplex
            (frequency, bandwidth, hp_code_rate, lp_code_rate,
             constellation, transmission_mode, guard_interval, hierarchy,
             sourceid)
        VALUES (%s, %s, %s, %s, %s, %s, %s, %s, %s)""", 
            (frequency, bandwidth, fec_hi, fec_lo,
             mod, transmission_mode, guard_interval, hierarchy,
             1))

db.commit()


