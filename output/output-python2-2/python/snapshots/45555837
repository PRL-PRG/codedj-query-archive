from random import choice
from paste.script.templates import Template

class DjangoTemplate(Template):
    _template_dir = 'template'
    summary = 'Django application template'

    def pre(self, command, output_dir, vars):
        """
        Pre-action - make secret string for Django settings
        """
        # make as django do
        secret = ''.join([choice('abcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*(-_=+)') for _ in xrange(50)])
        vars['secret'] = secret
