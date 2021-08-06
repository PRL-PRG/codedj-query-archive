#! /usr/bin/python
import os.path
import uu
import sys

def decode(path):
    in_file = open(path)
    t = "#INCLUDED PROJECT:"
    write = False
    out_file = None
    file_name = os.path.split(path)[1].split('.')[0] 

    for line in in_file:
        if t in line:
            write = True
            out_file = open(file_name + '.temp', 'w')
        if write == True:
            out_file.write(line)  

    if  write == False:
        print "No project included in this error log..."
        return

    in_file.close()
    out_file.close()

    out_file = open(file_name + '.temp', 'r')
    project_file = open(file_name + '.frip', 'w')
    uu.decode(out_file, project_file)
    out_file.close()
    project_file.close()
    os.remove(os.path.abspath(os.path.split(path)[0] + file_name + '.temp'))
    print 'decoding done... extracted project saved in: ' + os.path.abspath(os.path.split(path)[0] + file_name + '.frip')


def usage():
    print sys.argv[0] + " - Extracts project from UML.fri error logs"
    print "Usage: python",sys.argv[0], "/path/to/umlfri/error_log"


def main():
    if len(sys.argv) == 2:
        path = sys.argv[1]
        if os.path.isfile(path):
            decode(path)
        else: 
            print 'File does not exists.'
    else: usage()

if __name__ == "__main__":
    main()

