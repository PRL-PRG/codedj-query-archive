import os
import sqlalchemy as sa
import sqlalchemy.orm as orm
from django.conf import settings

from dlt.downloader.models import DownloadItem

def get_db_uri():
    db_engines = {
      'sqlite3': 'sqlite',
      'postgresql_psycopg2': 'postgres',
      'mysql': 'mysql',
        }
    sa_db_engine = db_engines.get(settings.DATABASE_ENGINE)
    if sa_db_engine is None:
        raise ValueError("Unsupported db engine: %s" %
                         settings.DATABASE_ENGINE)
    # SQLite needs only the name of db
    if sa_db_engine == 'sqlite':
        sa_db_uri = "%s:///%s" % \
                    (sa_db_engine,
                     os.path.abspath(settings.DATABASE_NAME))
    else:
        sa_db_host = settings.DATABASE_HOST or 'localhost'
        if settings.DATABASE_PORT:
            sa_db_port = ':%s' % settings.DATABASE_PORT
        else:
            sa_db_port = ''
        sa_db_uri = "%(engine)s://%(user)s:%(password)s@%(host)s%(port)s/%(name)s" % \
                    {
             'engine': sa_db_engine, 
             'user': settings.DATABASE_USER,
             'password': settings.DATABASE_PASSWORD,
             'host': sa_db_host,
             'port': sa_db_port,
             'name': settings.DATABASE_NAME,
                    }
    return sa_db_uri

meta = sa.BoundMetaData(get_db_uri())
if settings.DEBUG:
    meta.engine.echo = True
download_item_table = sa.Table(DownloadItem._meta.db_table, meta, autoload=True)

orm.mapper(DownloadItem, download_item_table)
