#!/usr/bin/env python
#-*- coding: utf-8 -*-
#
# Copyright 2006 Develer S.r.l. (http://www.develer.com/)
# All rights reserved.
#
# $Id$
#
# Author: Lorenzo Berni <duplo@develer.com>

from PyQt4.QtCore import QDate

DEFAULT_DAYS = (True,  True,  True,  True,  True,  False,  False)

def daterange(start_day,  end_day,  days=DEFAULT_DAYS):
    """
    Returns a QDate generator from
    *start_day* to *end_day* with one day step
    """
    current_day = start_day
    while True:
        if current_day > end_day:
            break
        tmp = current_day
        current_day = current_day.addDays(1)
        if days[tmp.dayOfWeek() - 1]:
            #TODO: Eliminare i festivi e le ferie
            yield tmp
