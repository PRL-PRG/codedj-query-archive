#!/usr/bin/env python

import os

activities = ['../' + rep for rep in os.listdir('../') if rep.endswith('activity')]
for activity in activities:
    for f in sorted(os.listdir(os.path.join(activity,'po'))):
        print 'po/' + f
        