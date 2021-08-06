#!/usr/bin/env python
#-*- coding: utf-8 -*-
#
# Copyright 2006 Develer S.r.l. (http://www.develer.com/)
# All rights reserved.
#
# $Id:$
#
# Author: Lorenzo Berni <duplo@develer.com>
from PyQt4.QtCore import QDate

from PyQt4.QtCore import QDate

def daterange(start_day,  end_day):
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
        #TODO: Eliminare i sabati e le domeniche
        yield tmp
