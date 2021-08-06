import xmlrpclib
from django.conf import settings
from django.core.management.base import NoArgsCommand

PYPI_URL = "http://cheeseshop.python.org/pypi"
PYPI_KEYWORD = "django"

from djangoapps.models import DjangoApp

class Command(NoArgsCommand):
    help = 'Used to update the feeds of the aggregator app.'

    def handle_noargs(self, **options):
        """
        queries PyPI for package with the default keyword and updates the app
        database accordingly
        """
        specs = {'keywords': PYPI_KEYWORD}
        pypi = xmlrpclib.Server(PYPI_URL)
        query = pypi.search(specs)
        results = sorted(query, key=lambda s: s['name'].lower())

        for package in results:
            name = package['name']
            version = pypi.package_releases(name)[-1]

            # Load release information
            try:
                info = pypi.release_data(name, version)
            except IndexError:
                print "Skipping %r: no versions" % name
                continue
            print "Updating: %r (%s)" % (name, version)

            # Load app or create if not existing
            try:
                app = DjangoApp.objects.get(name=name)
            except DjangoApp.DoesNotExist:
                app = DjangoApp(name=name)
                print "Created app %r" % name

            # TODO
            # Fill app with data from PyPI
            # the model fields should have the same variable name 
            for data in settings.PYPI_METADATA:
                value = info.get(data, '')
                if value is None or value.strip().lower() == "unknown":
                    value = ""
                setattr(app, data, value.strip())
            app.save()
