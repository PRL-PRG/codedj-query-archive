
import sys
import os
import shutil

fName = sys.argv[1]

i = 0
while os.path.exists( fName + ".bak%d" % i ): i += 1
bakName = fName + ".bak%d" % i

shutil.move( fName, bakName )

r = open(bakName)
w = open(fName, "w" )
for line in r:
    line = line[:-1]
    while len(line) and (line[-1] == " " or line[-1] == "  "):
        line = line[:-1]
    w.write(line+"\n")

w.close()
r.close()


