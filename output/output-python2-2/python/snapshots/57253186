# Copyright 2008, Felspar Co Ltd. http://fost.3.felspar.com/
# Distributed under the Boost Software License, Version 1.0.
# See accompanying file LICENSE_1_0.txt or copy at
#     http://www.boost.org/LICENSE_1_0.txt
import _settings
from Fost.utils.simplejson.decoder import JSONDecoder

class database(_settings.settings):
    def __getitem__(self, key):
        return JSONDecoder().decode(self.get(key[0], key[1]))
    def __setitem__(self, key, item):
        print key


middleware_database = None
class middleware():
    """
        This Django middleware loads up configuration settings for the current site and for the current machine
    """
    def process_request(self, request):
        global middleware_database
        if not middleware_database:
            middleware_database = _settings.settings()
            middleware_database["Fost.settings.middleware", "Load files"] = True
        if middleware_database["Fost.settings.middleware", "Load files"]:
            middleware_database["Fost.settings.middleware", "Load files"] = False
            import os
            basepath = os.path.split(__import__(os.environ["DJANGO_SETTINGS_MODULE"]).__file__)[0]
            middleware_database.file(os.path.join(basepath, "settings.ini"))
            middleware_database.file(os.path.join(basepath, '%s.ini' % request.META['HTTP_HOST'].lower()))
            middleware_database.file(os.path.join(basepath, '%s-%s.ini' % (request.META['HTTP_HOST'].lower(), request.META['SERVER_PORT'])))
