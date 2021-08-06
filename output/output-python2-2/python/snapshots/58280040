#!/usr/bin/python
"""
output lircrc configuration for the defined keys
"""

# http://www.mythtv.org/docs/mythtv-HOWTO-11.html#ss11.1

keys = """\
mute F9
power 
tv Esc
teletext P
av 
one 1
two 2
three 3
four 4
five 5
six 6
seven 7
eight 8
nine 9
-/-- Space
zero 0
info O
resize W
menu M
up Up
left Left
right Right
down Down
volup F11
voldown F10
progup Up
progdown Down
""" 


for line in keys.split("\n"):
    try:
        key, cmd = line.split()
        print "begin"
        print "prog = mythtv"
        print "button = %s" % key
        print "config = %s" % cmd
        print "end"
        print
    except ValueError:
        pass
    
