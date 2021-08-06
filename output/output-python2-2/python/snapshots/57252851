from Fost.settings import database
import os
from socket import gethostname

settingsdb = database()
basepath = os.path.split(__import__(os.environ.get("DJANGO_SETTINGS_MODULE", "settings")).__file__)[0]
settingsdb.file(os.path.join(basepath, "settings.ini"))
settingsdb.file(os.path.join(basepath, "settings-%s.ini" % gethostname()))

DEBUG = settingsdb["Django", "DEBUG"]
TEMPLATE_DEBUG = settingsdb["Django", "TEMPLATE_DEBUG"]
ADMINS =settingsdb["Django", "ADMINS"]
MANAGERS = settingsdb["Django", "MANAGERS"]
DATABASE_ENGINE = settingsdb["Django", "DATABASE_ENGINE"] or ''
DATABASE_NAME = settingsdb["Django", "DATABASE_NAME"] or ''
DATABASE_USER = settingsdb["Django", "DATABASE_USER"] or ''
DATABASE_PASSWORD = settingsdb["Django", "DATABASE_PASSWORD"] or ''
DATABASE_HOST = settingsdb["Django", "DATABASE_HOST"] or ''
DATABASE_PORT = settingsdb["Django", "DATABASE_PORT"] or ''
TIME_ZONE = settingsdb["Django", "TIME_ZONE"]
LANGUAGE_CODE = settingsdb["Django", "LANGUAGE_CODE"]
SITE_ID = settingsdb["Django", "SITE_ID"]
USE_I18N = settingsdb["Django", "USE_I18N"]
MEDIA_ROOT = settingsdb["Django", "MEDIA_ROOT"] or ''
MEDIA_URL = settingsdb["Django", "MEDIA_URL"] or ''
ADMIN_MEDIA_PREFIX = settingsdb["Django", "ADMIN_MEDIA_PREFIX"]
SECRET_KEY = settingsdb["Django", "SECRET_KEY"]
TEMPLATE_LOADERS = settingsdb["Django", "TEMPLATE_LOADERS"]
MIDDLEWARE_CLASSES =settingsdb["Django", "MIDDLEWARE_CLASSES"]
ROOT_URLCONF = settingsdb["Django", "ROOT_URLCONF"]
TEMPLATE_DIRS = settingsdb["Django", "TEMPLATE_DIRS"]
INSTALLED_APPS = [str(a) for a in settingsdb["Django", "INSTALLED_APPS"]]
