#!/usr/bin/env python

from hotshot import stats
import sys

s = stats.load(sys.argv[1])

s.sort_stats("cumulative").print_stats()
#s.sort_stats("time").print_callers()


