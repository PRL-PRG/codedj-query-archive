import pages

def get_wsgi_app():
    from nevow.wsgi import createWSGIApplication
    return createWSGIApplication(pages.Root())

def get_nevow_site():
    from nevow.appserver import NevowSite
    return NevowSite(pages.Root())
