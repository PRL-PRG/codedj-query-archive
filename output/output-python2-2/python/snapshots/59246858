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

try:
    from xml.etree import ElementTree as ET
except ImportError:
    try:
        from elementtree import ElementTree as ET
    except ImportError:
        raise ImportError, "ElementTree (or py2.5) needed"

def debug(msg, level="debug"):
    if __debug__:
        print msg

def min2hmtime(mins):
    """
    Converte un intero che rappresenta i minuti
    in forma di orario hh:mm

    >>> min2hmtime(120)
    '02:00'
    >>> min2hmtime(234)
    '03:54'
    """
    return "%02d:%02d" % (mins/60, mins%60)

def hmtime2min(hmtime):
    """
    Converte un intero che rappresenta un orario hh:mm
    in forma di minuti

    >>> hmtime2min('02:00')
    120
    >>> hmtime2min('03:54')
    234
    """
    return int(hmtime.split(":")[0])*60 + int(hmtime.split(":")[1])

def timerange(max_hours, time_step, min_step=0):
    """
    Restituisce una lista di stringhe con orari da
    0:00 a max_hours con passo time_step

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
    Arrotonda la stringa hh:mm alla risoluzione inviata
    e restituisce un oggetto timedelta:

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
    Analizza una stringa e restisuisce un dizionario

    >>> parseSmartQuery("pro pha act 2:30 commento")
    {'in_prj': 'pro', 'in_hmtime': '2:30', 'in_remark': 'commento', 'in_act': 'act', 'in_pha': 'pha'}
    >>> parseSmartQuery("pro pha act 2 commento")
    {'in_prj': 'pro', 'in_hmtime': '2', 'in_remark': 'commento', 'in_act': 'act', 'in_pha': 'pha'}
    >>> parseSmartQuery("pro pha act commento")
    {'in_prj': 'pro', 'in_hmtime': '', 'in_remark': 'commento', 'in_act': 'act', 'in_pha': 'pha'}
    >>> parseSmartQuery("pro pha act")
    {'in_prj': 'pro', 'in_hmtime': '', 'in_remark': '', 'in_act': 'act', 'in_pha': 'pha'}
    >>> parseSmartQuery("pro pha")
    {'in_prj': 'pro', 'in_hmtime': '', 'in_remark': '', 'in_act': '', 'in_pha': 'pha'}
    """
    getsq = re.compile("""
        (?P<in_prj>[^ ]+|)\ *
        (?P<in_pha>[^ ]+|)\ *
        (?P<in_act>[^ ]+|)\ *
        (?P<in_hmtime>\d{1,2}:\d{1,2}|\d{1,2}|)\ *
        (?P<in_remark>.*|)
        """, re.VERBOSE + re.DOTALL)
    res = getsq.search(smartquery).groupdict()
    if res["in_hmtime"] and ":" not in res["in_hmtime"]:
        res["in_hmtime"] += ":00"
    return res
