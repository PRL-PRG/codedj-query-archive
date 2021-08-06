#!/usr/bin/python
# -*- coding: utf-8 -*-

import os
import time
import datetime

os.environ['DJANGO_SETTINGS_MODULE'] = 'dlt.settings'

from dlt.downloader import models

for item in models.DownloadItem.objects.all().filter(started_at__isnull=True):
    print "Начинаем качать с %s" % item.url
    item.started_at = datetime.datetime.now()
    item.save()
    time.sleep(10)
    print "Закончили качать с %s" % item.url
    item.finished_at = datetime.datetime.now()
    item.save()
