#!/usr/bin/python
# -*- coding: utf-8 -*-

import os
import time
import datetime

os.environ['DJANGO_SETTINGS_MODULE'] = 'dlt.settings'

from sqlalchemy import orm
from dlt.downloader.models_sa2 import DownloadItem

dbsession = orm.create_session()
query = dbsession.query(DownloadItem)

for item in query.select_by(started_at=None):
    print u"Начинаем качать с %s" % item.url
    item.started_at = datetime.datetime.now()
    dbsession.flush()
    time.sleep(10)
    print u"Закончили качать с %s" % item.url
    item.finished_at = datetime.datetime.now()
    dbsession.flush()
    
