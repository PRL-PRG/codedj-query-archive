import os.path

from django.test import TestCase
from django.template import TemplateDoesNotExist
from django.conf import settings

class MaintenanceModeMiddlewareTestCase(TestCase):
    def setUp(self):
        # Reset config options adapted in the individual tests
        settings.MAINTENANCE_MODE = None        
        settings.TEMPLATE_DIRS = ()
    
    def test_implicitly_disabled_middleware(self):
        "Middleware should default to being disabled"        
        response = self.client.get('/')
        self.assertContains(response, text='Rendered response page', count=1, status_code=200)
    
    def test_disabled_middleware(self):
        "Explicitly disabling the MAINTENANCE_MODE should work"
        settings.MAINTENANCE_MODE = False
        
        response = self.client.get('/')
        self.assertContains(response, text='Rendered response page', count=1, status_code=200)
    
    def test_enabled_middleware_without_template(self):
        "Enabling the middleware without a proper 503 template should raise a template error"
        settings.MAINTENANCE_MODE = True
        
        self.assertRaises(TemplateDoesNotExist, self.client.get, '/')

    def test_enabled_middleware_with_template(self):
        "Enabling the middleware having a 503.html in any of the template locations should return the rendered template"
        settings.MAINTENANCE_MODE = True
        settings.TEMPLATE_DIRS = (os.path.join(os.path.dirname(os.path.abspath(__file__)), '../templates/'),)
        
        response = self.client.get('/')
        self.assertContains(response, text='Temporary unavailable', count=1, status_code=503)

    def test_middleware_with_non_staff_user(self):
        "A logged in user that is not a staff user should see the 503 message"
        settings.MAINTENANCE_MODE = True
        settings.TEMPLATE_DIRS = (os.path.join(os.path.dirname(os.path.abspath(__file__)), '../templates/'),)
        
        from django.contrib.auth.models import User
        User.objects.create_user(username='maintenance', email='maintenance@example.org', password='maintenance_pw')
        
        self.client.login(username='maintenance', password='maintenance_pw')
        
        response = self.client.get('/')
        self.assertContains(response, text='Temporary unavailable', count=1, status_code=503)

    def test_middleware_with_staff_user(self):
        "A logged in user that _is_ a staff user should be able to use the site normally"
        settings.MAINTENANCE_MODE = True
        settings.TEMPLATE_DIRS = (os.path.join(os.path.dirname(os.path.abspath(__file__)), '../templates/'),)

        from django.contrib.auth.models import User
        user = User.objects.create_user(username='maintenance', email='maintenance@example.org', password='maintenance_pw')
        user.is_staff = True
        user.save()

        self.client.login(username='maintenance', password='maintenance_pw')

        response = self.client.get('/')
        self.assertContains(response, text='Rendered response page', count=1, status_code=200)