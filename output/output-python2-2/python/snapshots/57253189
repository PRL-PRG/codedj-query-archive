import _settings

class database(_settings.settings):
    pass


middleware_database = None
class middleware():
    """
        This Django middleware loads up configuration settings for the current site and for the current machine
    """
    def process_request(self, request):
        global middleware_database
        if not middleware_database:
            middleware_database = database()
            middleware_database.set("Fost.settings.middleware", "Load files", "true")
        if middleware_database.get("Fost.settings.middleware", "Load files") == "true":
            middleware_database.set("Fost.settings.middleware", "Load files", "false")
            import os
            basepath = os.path.split(__import__(os.environ["DJANGO_SETTINGS_MODULE"]).__file__)[0]
            middleware_database.file(os.path.join(basepath, "settings.ini"))
            middleware_database.file(os.path.join(basepath, '%s.ini' % request.META['HTTP_HOST'].lower()))
            middleware_database.file(os.path.join(basepath, '%s-%s.ini' % (request.META['HTTP_HOST'].lower(), request.META['SERVER_PORT'])))
