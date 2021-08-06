
import os
import re
import sys

pat_start_re = re.compile("^.* Program #\d+ not found in PAT!$")

assert pat_start_re.match(
    "2007-10-19 20:36:32.502 Program #14208 not found in PAT!") != None

start_entry_re = re.compile("^[-\d]+ [.\d:]+ .*$")

assert start_entry_re.match(
    "2007-10-19 20:36:33.252 Program #14208 not found in PAT!") != None

out = sys.stdout
#out = open(os.devnull, "w")

print_line = True

stop_printing = [pat_start_re]

start_printing = [start_entry_re]


for line in file(sys.argv[1]):

    if not print_line:
        for rx in start_printing:
            if rx.match(line):
                print_line = True
                break

    if print_line:
        out.write(line)
        for rx in stop_printing:
            if rx.match(line):
                print_line = False
                break
        
    

