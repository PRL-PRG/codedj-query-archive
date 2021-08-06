#!/usr/bin/env python
#-*- coding: utf-8 -*-
#
# Copyright 2006 Develer S.r.l. (http://www.develer.com/)
# All rights reserved.
#
# $Id:$
#
# Author: Matteo Bertini <naufraghi@develer.com>

import sys, re, datetime, time
from xml.parsers.expat import ExpatError

try:
    from xml.etree import ElementTree as ET
except ImportError:
    try:
        from elementtree import ElementTree as ET
    except ImportError:
        raise ImportError, "ElementTree (or py2.5) needed"

def debug(msg, level="debug"):
    if __debug__:
        if type(msg) != str:
            msg = msg.encode("utf-8")
        print msg

def min2hmtime(mins):
    """
    Converts an integer representing minutes in the hh:mm form

    >>> min2hmtime(120)
    '02:00'
    >>> min2hmtime(234)
    '03:54'
    """
    return "%02d:%02d" % (mins/60, mins%60)

def hmtime2min(hmtime):
    """
    Converts a string in the form hh:mm in minutes

    >>> hmtime2min('02:00')
    120
    >>> hmtime2min('03:54')
    234
    """
    return int(hmtime.split(":")[0])*60 + int(hmtime.split(":")[1])

def timerange(max_hours, time_step, min_step=0):
    """
    Returns a string list with times from
    *min_step* to *max_hours* with step *time_step*

    >>> [i for i in timerange(2, 15, 1)]
    ['00:15', '00:30', '00:45', '01:00', '01:15', '01:30', '01:45', '02:00']
    >>> [i for i in timerange(2, 20)]
    ['00:00', '00:20', '00:40', '01:00', '01:20', '01:40', '02:00']
    """
    time_max = max_hours*60/time_step
    for step in range(min_step, 1+time_max):
        yield min2hmtime(time_step*step)

def timeRound(in_time, step_time=15):
    """
    Rounds the string hh:mm to the provided resolution

    >>> timeRound("2:44")
    '02:45'
    >>> timeRound("13:10", 20)
    '13:20'
    """
    in_time_tuple = time.strptime(in_time, "%H:%M")
    step = datetime.timedelta(minutes=step_time)
    if step == datetime.timedelta():
        step = datetime.timedelta(minutes=1)
    pre = datetime.timedelta(hours=in_time_tuple.tm_hour,
                             minutes=in_time_tuple.tm_min)
    res = int(round(pre.seconds/float(step.seconds))*step.seconds)
    return "%02d:%02d" % (res/3600, (res%3600)/60)

def parseSmartQuery(smartquery):
    """
    Parse the provided string in a dictionary

    >>> parseSmartQuery("pro pha act 2:30 commento bla bla ")
    {'in_prj': 'pro', 'in_hmtime': '02:30', 'in_remark': 'commento bla bla', 'in_act': 'act', 'in_pha': 'pha'}
    >>> parseSmartQuery("pro pha act 2 commento bla bla ")
    {'in_prj': 'pro', 'in_hmtime': '02:00', 'in_remark': 'commento bla bla', 'in_act': 'act', 'in_pha': 'pha'}
    >>> parseSmartQuery("pro pha act commento bla bla ")
    {'in_prj': 'pro', 'in_hmtime': '', 'in_remark': 'commento bla bla', 'in_act': 'act', 'in_pha': 'pha'}
    >>> parseSmartQuery("pro2 pha act")
    {'in_prj': 'pro2', 'in_hmtime': '', 'in_remark': '', 'in_act': 'act', 'in_pha': 'pha'}
    >>> parseSmartQuery("pro pha")
    {'in_prj': 'pro', 'in_hmtime': '', 'in_remark': '', 'in_act': '', 'in_pha': 'pha'}
    >>> parseSmartQuery("pro 2:30 bla bla")
    {'in_prj': 'pro', 'in_hmtime': '02:30', 'in_remark': 'bla bla', 'in_act': '', 'in_pha': ''}
    >>> parseSmartQuery("")
    {'in_prj': '', 'in_hmtime': '', 'in_remark': '', 'in_act': '', 'in_pha': ''}
    >>> parseSmartQuery("pro pha act 120 commento")
    {'in_prj': 'pro', 'in_hmtime': '120:00', 'in_remark': 'commento', 'in_act': 'act', 'in_pha': 'pha'}    
    """
    res = {}
    gethmtime = re.compile("\s\d+:?\d*")
    getppa = re.compile("(?P<in_prj>[^\s]+|)\s*(?P<in_pha>[^\s]+|)\s*(?P<in_act>[^\s]+|)\s*(?P<in_remark>.*|)")
    parts = gethmtime.split(smartquery, 1)
    res = getppa.search(parts[0]).groupdict()
    try:
        res["in_hmtime"] = gethmtime.search(smartquery).group().strip()
        if ":" not in res["in_hmtime"]:
            res["in_hmtime"] += ":00"
        if len(res["in_hmtime"].split(":")[0]) < 2:
            res["in_hmtime"] = "0" + res["in_hmtime"]
    except AttributeError:
        res["in_hmtime"] = ""
    if len(parts) > 1:
        res["in_remark"] = parts[1].strip()
    else:
        res["in_remark"] = res["in_remark"].strip()
    return res

def strlike(str1, str2):
    op1 = str1.lower().find(str2.lower()) != -1
    op2 = str2.lower().find(str1.lower()) != -1
    return op1 or op2

def parse_hhmm(val):
    """
    Restituisce un oggetto datetime da una stringa che rappresenta un ora

    >>> print parse_hhmm("12:44")
    1900-01-01 12:44:00
    >>> print parse_hhmm("1612")
    1900-01-01 16:12:00
    >>> print parse_hhmm("321")
    1900-01-01 03:21:00
    """
    try:
        return datetime.datetime(*time.strptime(str(val), "%H:%M")[:-2])
    except:
        return datetime.datetime(*time.strptime(str(val), "%H%M")[:-2])

def parse_hinterval(val):
    """
    Parse a time intervall and returns a timedelta

    >>> print parse_hinterval("10:00-12:00")
    2:00:00
    >>> print parse_hinterval("300-530")
    2:30:00
    """
    val1, val2 = val.split("-")
    d1 = parse_hhmm(val1)
    d2 = parse_hhmm(val2)
    if d2 < d1:
        d2 += datetime.timedelta(1)
    return d2 - d1

def parse_wtime(val):
    vals = val.replace(" ","").split("+")
    res = datetime.timedelta(0)
    for ival in vals:
        if "-" in ival:
            res += parse_hinterval(ival)
        else:
            res += parse_hinterval("00:00-%s" % ival)
    return str(res)

def divide(total_time, days, time_step=15):
    """
    Return a list of string of the daily amount of hours.

    >>> for day in divide(25, 4):
    ...    print day
    ...
    06:15
    06:15
    06:15
    06:15
    >>> hours = 11
    >>> list(divide(hours, 4))
    ['02:45', '02:45', '02:45', '02:45']
    >>> sum(map(hmtime2min, divide(hours, 4))) == hours * 60
    True
    >>> list(divide(hours, 4, time_step=30))
    ['03:00', '02:30', '03:00', '02:30']
    >>> sum(map(hmtime2min, divide(hours, 4, time_step=30))) == hours * 60
    True
    """
    total_time_min = int(total_time) * 60
    while total_time_min >= 0 and days > 0:
        hour = timeRound(min2hmtime(total_time_min/days), time_step)
        days -= 1
        total_time_min -= hmtime2min(hour)
        yield hour

if __name__ == "__main__":
    print "Running doctests, -v to see more..."
    import doctest
    doctest.testmod()
