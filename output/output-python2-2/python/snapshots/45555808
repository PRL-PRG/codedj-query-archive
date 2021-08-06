import datetime
import sqlalchemy as sa
import sqlalchemy.orm as orm
from django.conf import settings

meta = sa.BoundMetaData(settings.SQLALCHEMY_DB_URI)
if settings.DEBUG:
    meta.engine.echo = True
download_item_table = sa.Table('downloader_downloaditem', meta, autoload=True)

class DownloadItem(object):
    
    def __str__(self):
        return self.url

    def __repr__(self):
        return "%s(%s)" % (self.__class__.__name__, self.url)

    def is_started(self):
        return self.started_at is not None and datetime.datetime.now() > self.started_at
    
    def is_finished(self):
        return self.finished_at is not None and datetime.datetime.now() > self.finished_at

orm.mapper(DownloadItem, download_item_table)
