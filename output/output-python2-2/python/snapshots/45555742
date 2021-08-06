#!/usr/bin/env python
# relativeDates.py by Jehiah Czebotar
# http://jehiah.com/
# released under whatever license you want; modify as you see fit
#
# added russian pluralize support by Pythy <the.pythy@gmail.com>

import math,time

# russian plural forms:
tomorrow = u'\u0437\u0430\u0432\u0442\u0440\u0430'
yesterday = u'\u0432\u0447\u0435\u0440\u0430'
the_day_after_tomorrow = u'\u043f\u043e\u0441\u043b\u0435\u0437\u0430\u0432\u0442\u0440\u0430'
the_day_before_yesterday = u'\u043f\u043e\u0437\u0430\u0432\u0447\u0435\u0440\u0430'
day_alternates = (u'\u0434\u0435\u043d\u044c',
                  u'\u0434\u043d\u044f',
                  u'\u0434\u043d\u0435\u0439')
hour_alternates = (u'\u0447\u0430\u0441',
                   u'\u0447\u0430\u0441\u0430',
                   u'\u0447\u0430\u0441\u043e\u0432')
minute_alternates = (u'\u043c\u0438\u043d\u0443\u0442\u0430',
                     u'\u043c\u0438\u043d\u0443\u0442\u044b',
                     u'\u043c\u0438\u043d\u0443\u0442')
prefix_in = u'\u0447\u0435\u0440\u0435\u0437'
suffix_ago = u'\u043d\u0430\u0437\u0430\u0434'

def getRelativeTimeStr(str_time,time_format="%m/%d/%y %H%M",accuracy=1,cmp_time=None,alternative_past=None):
    # convert str_time to date
    t = time.mktime(time.strptime(str_time,time_format))
    return getRelativeTime(t,accuracy=accuracy,cmp_time=cmp_time,alternative_past=alternative_past)

def getPluralRussian(number, alternates):
    assert isinstance(number, int)
    assert len(alternates) == 3
    unity = number % 10
    tens = number / 10

    if unity == 1 and tens != 1:
        return alternates[0]
    if unity > 1 and unity <5 and tens != 1:
        return alternates[1]
    return alternates[2]

def getRelativeTime(t,accuracy=1, cmp_time=None, alternative_past=None):
    if cmp_time==None:
        cmp_time = time.time()
    diff_seconds = (t - cmp_time) + 20 # unknown why it's off by 20 seconds
    diff_minutes = int(math.floor(diff_seconds/60))
    relative_time = ""
    relative_time_alt = ""

    sign = diff_minutes > 0
    diff_minutes = math.fabs(diff_minutes)
    # return in minutes
    if diff_minutes > (60 * 24):
        val = int(math.floor(diff_minutes / (60*24)))
        if val == 1 and accuracy == 1:
            if sign:
                relative_time_alt = tomorrow
            else:
                relative_time_alt = yesterday
        elif val == 2 and accuracy == 1:
            if sign:
                relative_time_alt = the_day_after_tomorrow
            else:
                relative_time_alt = the_day_before_yesterday
        relative_time = str(val) + " " + getPluralRussian(val, day_alternates)
        if accuracy > 1:
            val = int(math.floor((diff_minutes % (60*24))) / 60)
            relative_time +=" "+ str(val) + " " + getPluralRussian(val, hour_alternates)
    elif diff_minutes > 60 :
        val = int(math.floor(diff_minutes / 60))
        relative_time = str(val) + " " + getPluralRussian(val, hour_alternates)
        if accuracy > 1:
            val = int(diff_minutes % 60)
            relative_time +=" "+ str(val) + " " + getPluralRussian(val, minute_alternates)
    else:
        val = int(diff_minutes)
        relative_time = str(val) + " " + getPluralRussian(val, minute_alternates)

    if relative_time_alt:
        relative_time = relative_time_alt
    elif sign:
        relative_time = prefix_in + " " + relative_time
    else:
        if alternative_past:
            return alternative_past
        relative_time += " " + suffix_ago
    return relative_time    
