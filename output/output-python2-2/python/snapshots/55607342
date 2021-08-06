# -*- coding: utf-8 -*-
# vim: sw=4 ts=4 fenc=utf-8 et
# ==============================================================================
# Copyright © 2008 UfSoft.org - Pedro Algarvio <ufs@ufsoft.org>
#
# Please view LICENSE for additional licensing information.
# ==============================================================================

from trac.core import Component, implements
from trac.admin import IAdminPanelProvider
from trac.config import Option,  _TRUE_VALUES
from trac.util.text import unicode_unquote
from trac.web.chrome import add_stylesheet

class GoogleAdsAdmin(Component):
    config = env = log = None # make pylint happy
    implements(IAdminPanelProvider)
    options = {}

    # IAdminPanelProvider methods
    def get_admin_panels(self, req):
        if req.perm.has_permission('TRAC_ADMIN'):
            yield ('google', 'Google', 'ads', 'Ads')

    def render_admin_panel(self, req, cat, page, path_info):
        add_stylesheet(req, 'googlesads/googleads.css')
        self.log.debug('Saving Google Ads Options')
        if req.method == 'POST':
            self.config.set('google.ads', 'hide_for_authenticated',
                            req.args.get('hide_for_authenticated') in
                            _TRUE_VALUES)
            self.config.save()
            code = req.args.get('ads_html')
            db = self.env.get_db_cnx()
            cursor = db.cursor()
            cursor.execute('SELECT value FROM system WHERE name=%s',
                           ('google.ads_html',))
            if cursor.fetchone():
                self.log.debug('Updating Ads HTML Code')
                cursor.execute('UPDATE system SET value=%s WHERE name=%s',
                               (code, 'google.ads_html'))
            else:
                self.log.debug('Inserting Ads HTML Code')
                cursor.execute('INSERT INTO system (name,value) VALUES (%s,%s)',
                               ('google.ads_html', code))
            db.commit()

            req.redirect(req.href.admin(cat, page))
        self._update_config()
        return 'google_ads_admin.html', {'ads_options': self.options}

    # Internal methods
    def _update_config(self):
        for option in [option for option in Option.registry.values()
                       if option.section == 'google.ads']:
            if option.name == 'hide_for_authenticated':
                option.value = self.config.getbool('google.ads', option.name,
                                                   True)
            elif option.name == 'ads_html':
                # Still get the Option to get __doc__ from it
                db = self.env.get_db_cnx()
                cursor = db.cursor()
                cursor.execute('SELECT value FROM system WHERE name=%s',
                               ('google.ads_html',))
                code = cursor.fetchone()
                if code:
                    code = unicode_unquote(code[0])
                option.value = code or  ''
            else:
                option.value = self.config.get('google.ads', option.name,
                                               option.default)
            self.options[option.name] = option
