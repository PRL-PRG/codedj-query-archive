# -*- coding: utf-8 -*-
# vim: sw=4 ts=4 fenc=utf-8
# =============================================================================
# $Id: web_ui.py 26 2008-01-14 17:55:31Z s0undt3ch $
# =============================================================================
#             $URL: http://devnull.ufsoft.org/svn/TracAdsPanel/trunk/adspanel/web_ui.py $
# $LastChangedDate: 2008-01-14 17:55:31 +0000 (Mon, 14 Jan 2008) $
#             $Rev: 26 $
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
        self.log.debug(req.session)
        if req.path_info.startswith('/admin'):
            # Don't even show the ads link on admin pages
            return stream

        state = req.session.get('adspanel.state', 'shown')
        if state == 'hidden':
            state = 'show'
        elif state == 'shown':
            state = 'hide'
        add_ctxtnav(
            req,
            tag.a('%s Ads' % state.capitalize(),
                  href=req.href.adspanel(state),
                  class_="toggle_ads"
            )
        )

        if self.dont_show_ads(req):
            self.log.debug('Not displaying ads, returning stream')
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
            req.args['adspanel.state'] = 'hidden'
            return True
        if req.path_info == '/adspanel/show':
            req.args['adspanel.state'] = 'shown'
            return True
        return False

    def process_request(self, req):
        req.session['adspanel.state'] = req.args.get('adspanel.state', 'hidden')
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
        if req.session.get('adspanel.state') == 'hidden':
            return True
        elif req.session.get('adspanel.state') == 'shown':
            return False
        elif (req.authname and req.authname != 'anonymous'):
            if self.config.getbool('adspanel', 'hide_for_authenticated', False):
                return True
        return False
