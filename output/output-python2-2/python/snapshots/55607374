# -*- coding: utf-8 -*-
# vim: sw=4 ts=4 fenc=utf-8
# =============================================================================
# $Id: web_ui.py 6 2008-01-14 13:06:00Z s0undt3ch $
# =============================================================================
#             $URL: http://devnull.ufsoft.org/svn/TracAdsPanel/trunk/adspanel/web_ui.py $
# $LastChangedDate: 2008-01-14 13:06:00 +0000 (Mon, 14 Jan 2008) $
#             $Rev: 6 $
#   $LastChangedBy: s0undt3ch $
# =============================================================================
# Copyright (C) 2008 UfSoft.org - Pedro Algarvio <ufs@ufsoft.org>
#
# Please view LICENSE for additional licensing information.
# =============================================================================

from trac.core import *
from trac.config import Option
from trac.web.api import ITemplateStreamFilter
from trac.web.chrome import add_ctxtnav
from trac.web import HTTPNotFound, IRequestHandler
from trac.util.text import unicode_unquote
from genshi.builder import tag
from genshi.core import Markup
from genshi.filters.transform import Transformer, StreamBuffer
from pkg_resources import resource_filename

class AdsPanel(Component):
    config=None
    implements(ITemplateStreamFilter, IRequestHandler)

    # ITemplateStreamFilter method
    def filter_stream(self, req, method, filename, stream, data):
        if not req.path_info.startswith('/admin'):
            # Don't show ads on admin pages
            add_ctxtnav(
                req,
                tag.a('%s Ads' % \
                      req.session.get('adspanel.state', 'hide').capitalize(),
                      href=req.href.adspanel(req.session.get('adspanel.state',
                                                             'hide')),
                      class_="toggle_ads"
                )
            )
        if self.dont_show_ads(req):
            return stream
        jscode = """\
$(document).ready(function() {
    $('a.toggle_ads').show();
    $('a.toggle_ads').attr('href', '#');
    $('a.toggle_ads').bind('click', function() {
        var state = $('#ads_panel').is(':hidden') ? 'show' : 'hide';
        var name = $('#ads_panel').is(':hidden') ? 'Hide Ads' : 'Show Ads';
        $(this).html(name);
        $('#ads_panel').animate({opacity: state}, 200);
        $.get('%s/'+state);
    });
});""" % req.href.adspanel()
        req.href.adspanel(req.session.get('adspanel.state', 'hide'))
        code = self.config.get('adspanel', 'ads_code')
        cursor = self.env.get_db_cnx().cursor()
        cursor.execute('SELECT value FROM system WHERE name=%s',
                           ('adspanel.code',))
        code = cursor.fetchone()
        if code:
            code = unicode_unquote(code[0])
        else:
            code = ''
        streambuffer = StreamBuffer()
        return stream | Transformer('//div[@id="main"]/* | '
                                    '//div[@id="main"]/text()') \
            .cut(streambuffer).end() \
            .select('//div[@id="main"]').prepend(tag.table(tag.tr(
                tag.td(streambuffer, width="100%",
                       style="vertical-align: top;") +
                tag.td(Markup(code),
                       id="ads_panel", style="vertical-align: top;")
            ), width='100%')+ tag.script(jscode, type="text/javascript")
        )

    # IRequestHandler methods
    def match_request(self, req):
        if req.path_info == '/adspanel/hide':
            req.args['adspanel.state'] = 'show'
            return True
        if req.path_info == '/adspanel/show':
            req.args['adspanel.state'] = 'hide'
            return True
        return False

    def process_request(self, req):
#        print 'HEADERS:', req.get_header('X-Requested-With') # == 'XMLHttpRequest'
#        print 'HEADERS:', req.get_header('Referer')
        req.session['adspanel.state'] = req.args.get('adspanel.state', 'hide')
        req.session.save()
        if req.get_header('X-Requested-With') == 'XMLHttpRequest':
            # This is an ajax request
            req.send_response(code=200)
            req.end_headers()
        else:
            # This is a normal request, redirect user to last page
            referer = req.get_header('Referer')
            if referer and not referer.startswith(req.base_url):
                referer = None
            req.redirect(referer or self.env.abs_href())


    # Internal methods

    def dont_show_ads(self, req):
        if req.path_info.startswith('/admin'):
            # Don't show on Admin Pages
            return True
        if req.session.get('adspanel.state', 'hide') == 'show':
            return True
        elif (req.authname and req.authname != 'anonymous'):
            if self.config.getbool('adspanel', 'hide_for_authenticated', False):
                return True
        return False
