import sys

def argument(short = None, long = None, parametercount = 1):
    def tmp(fnc):
        fnc.argv_opts = (short, long, parametercount)
        return fnc
    return tmp
