# -*- coding: utf-8 -*-
# vim: sw=4 ts=4 fenc=utf-8 et
# ==============================================================================
# Copyright © 2008 UfSoft.org - Pedro Algarvio <ufs@ufsoft.org>
#
# Please view LICENSE for additional licensing information.
# ==============================================================================

__version__     = '0.2.0'
__author__      = 'Pedro Algarvio'
__email__       = 'ufs@ufsoft.org'
__package__     = 'TracGoogleAds'
__license__     = 'BSD'
__url__         = 'http://google.ufsoft.org'
__summary__     = 'Trac plugin designed to display ads on your Trac environment'


import pkg_resources
from trac.config import Option, BoolOption
from trac.core import Component, implements
from trac.env import IEnvironmentSetupParticipant
from trac.web.chrome import ITemplateProvider

# ==============================================================================
# Google Ads Options
# ==============================================================================
class GoogleAdsOptions(Component):
    hide_for_authenticated = BoolOption(
        'google.ads', 'hide_for_authenticated', True,
        """Should the ads be hidden for authenticated users."""
    )
    ads_html = Option(
        'google.ads', 'ads_html', None,
        """The HTML code which displays the ads.

        NOTE: You are responsible for the HTML code you add. The author of this
              plugin won't be held responsible for the breakage of the Google
              Policy."""
    )
    ads_div_id = Option('google.ads', 'ads_div_id', 'main',
        """The div ID where ads should be placed.

        If left at default value "main", a table with two columns will be
        created where regular output apears on the left column and the ads on
        the right column."""
    )

# ==============================================================================
# Google Ads Resources
# ==============================================================================
class GoogleAdsResources(Component):
    implements(ITemplateProvider)
    # ITemplateProvider methods
    def get_htdocs_dirs(self):
        """Return the absolute path of a directory containing additional
        static resources (such as images, style sheets, etc).
        """
        yield 'googlesads', pkg_resources.resource_filename(__name__, 'htdocs')

    def get_templates_dirs(self):
        """Return the absolute path of the directory containing the provided
        Genshi templates.
        """
        yield pkg_resources.resource_filename(__name__, 'templates')

# ==============================================================================
# Upgrade Code
# ==============================================================================
class GoogleComponentSetup(Component):
    env = config = log = None # make pylink happy
    implements(IEnvironmentSetupParticipant)

    def environment_created(self):
        "Nothing to do when an environment is created"""

    def environment_needs_upgrade(self, db):
        cursor = db.cursor()
        cursor.execute('SELECT value FROM system WHERE name=%s',
                       ('adspanel.code',))
        if cursor.fetchone():
            self.log.debug('Found old AdsPanel code in database')
            return True
        self.log.debug('Did not find old AdsPanel code in database')
        return False

    def upgrade_environment(self, db):
        cursor = db.cursor()
        cursor.execute('SELECT value FROM system WHERE name=%s',
                       ('adspanel.code',))
        code = cursor.fetchone()
        self.log.debug('Upgrading Ads HTML Code from old AdsPanel to new one')
        cursor.execute('INSERT INTO system (name,value) VALUES (%s,%s)',
                       ('google.ads_html', code[0]))
        cursor.execute('DELETE from system where name=%s', ('adspanel.code',))
        db.commit()
        self.log.debug("Upgrading configuration from old AdsPanel to new one")
        for option, value in self.config.options('adspanel'):
            if self.config.has_option('google.ads', option):
                self.config.set('google.ads', option, value)
            self.config.remove('adspanel', option)
        self.config.save()
