# Copyright (C) Scott Walker 2007 <iswalker at gmail dot com>
#
# This program is free software; you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation; either version 2, or (at your option)
# any later version.
# 
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
# 
# You should have received a copy of the GNU General Public License
# along with this program.  If not, write to:
# 	The Free Software Foundation, Inc.,
# 	51 Franklin Street, Fifth Floor
# 	Boston, MA  02110-1301, USA.
#

from btpdwebui.config import userconf, PROGRAM_VERSION
from btpdwebui.webui import template
from btpdwebui.btpd import Btpd, TActivity, TStates, TAttrs, TErrors
from twisted.web.resource import Resource
from os import makedirs
from os.path import normpath, exists, abspath, expanduser


_btpd_inst = None
def btpd():
    """Module global btpd instance"""
    global _btpd_inst
    if _btpd_inst is None:
        _btpd_inst = Btpd()
    return _btpd_inst

def getarg(request, name):
    """Twisted args are always a list.  This function returns the first
    item in the list if the arg is present, or None if it isn't."""
    if request.args.has_key(name):
       return request.args[name][0]
    return None


class Login(Resource):

    def _valid(self, username, password):
        return (username == userconf['username'] and \
                password == userconf['password'])    

    def render_POST(self, request):
        try:
            username = getarg(request, 'username')
            password = getarg(request, 'password')
        except:
            return 'Missing parameter(s)'
        if self._valid(username, password):
            session = request.getSession()
            session.active = True
            session.username = username
            return 'OK'
        else:            
            return 'Invalid credentials'

    def render_GET(self, request):
        template.set('version', PROGRAM_VERSION)
        return template.render('login')


class Logout(Resource):

    def render_GET(self, request):
        session = request.getSession()
        session.expire()
        template.set('version', PROGRAM_VERSION)
        return template.render('login')


class InvalidSession(Exception):
    pass

class Torrents(Resource):
    
    attrs = [TAttrs.NAME, 
             TAttrs.NUM,
             TAttrs.CGOT,
             TAttrs.CSIZE, 
             TAttrs.PCOUNT,
             TAttrs.RATEDWN, 
             TAttrs.RATEUP,
             TAttrs.TOTUP,
             TAttrs.STATE]      

    def _load_session(self, request):
        """Get the current established session"""
        session = request.getSession()           
        if not (hasattr(session, 'active') and session.active):
            raise InvalidSession('invalid session')
        self._session = session

    def _format_size(self, size):        
        labels = ('B', 'KB', 'MB', 'GB')
        for i in range(len(labels)):
            if size < 1024: break
            size /= 1024.0
        fstr = i == 0 and '%d%s' or '%.02f%s'
        return fstr % (size, labels[i])

    def _format_rate(self, rate):
        return self._format_size(rate) + '/s'

    def _btpd_cb(self, error, data):
        if error == TErrors.OK:                
            state = TStates.enumstr(data[TAttrs.STATE])            
            template.set(TStates.enumstr(data[TAttrs.STATE]), '1')
            template.set('num',    data[TAttrs.NUM])
            template.set('name',   data[TAttrs.NAME])
            template.set('got',    self._format_size(data[TAttrs.CGOT]))
            template.set('size',   self._format_size(data[TAttrs.CSIZE]))
            template.set('urate',  self._format_rate(data[TAttrs.RATEUP]))
            template.set('drate',  self._format_rate(data[TAttrs.RATEDWN]))
            template.set('totup',  self._format_size(data[TAttrs.TOTUP]))
            template.set('peers', data[TAttrs.PCOUNT])
            percent = data[TAttrs.CGOT] * 100.0 / data[TAttrs.CSIZE]
            template.set('percent', '%d' % percent)
            ratio = data[TAttrs.TOTUP] * 100.0 / data[TAttrs.CSIZE]
            template.set('ratio', '%d' % ratio)
            self._torrents.append(template.render('torrents_torrent'))           
        else:
            # Not quite sure what to do here...
            # Should an error be displayed?  For now, just ignore
            # it and don't display anything in the webui.
            pass

    def _make_tab(self, tab):
        template.set('tab', tab)
        template.set('tab_%s' % tab, '1')
        template.set('version', PROGRAM_VERSION)
        template.set('frequency', userconf['update_frequency'])
        template.set('contentdir', userconf['content_directory'])
        template.set('username', self._session.username)            
        return template.render('torrents')

    def render_GET(self, request):        
        tab = getarg(request, 'tab')
        if tab not in ('new', 'all', 'active', 'inactive'): tab = 'all'        
        action = getarg(request, 'action')
        if action is None: # page request
            try:
                self._load_session(request)
            except:
                return template.render('noauth')
            return self._make_tab(tab)
        else: # ajax update request
            try:
                self._load_session(request)
                self._torrents = []
                btpd().get(TActivity.enumval(tab), 
                           Torrents.attrs, self._btpd_cb)              
            except Exception, err:
                template.set('error', str(err))
                return template.render('torrents_error')
            if len(self._torrents) == 0:
                return template.render('torrents_empty')
            return ''.join(self._torrents)            

    def _makepath(self, path):        
        path = expanduser(normpath(path))
        if not exists(path): makedirs(path)
        return normpath(abspath(path))

    def render_POST(self, request):
        action = getarg(request, 'action')           
        if action in ('start', 'stop', 'delete'):
            # ajax torrent action request
            try:
                self._load_session(request)
                num = int(getarg(request, 'num'))
                getattr(btpd(), action)(num)
            except Exception, err:
                return 'Request failed: %s' % err
            return 'OK'
        else: # torrent upload 
            try:
                self._load_session(request)
                # create the content directory if it doesn't
                # already exist, and get the absolute path.
                cdir = self._makepath(userconf['content_directory'])
                # try to add the torrent    
                torrent = getarg(request, 'torrent')
                if torrent is None:
                    raise Exception('no torrent file')
                topdir = bool(getarg(request, 'topdir'))                
                tnum = btpd().add(torrent, cdir, topdir)
                if getarg(request, 'start'): btpd().start(tnum)
                template.set('new_added', '1')
            except InvalidSession:
                return template.render('noauth')
            except Exception, err:            
                template.set('new_error', str(err))                               
            return self._make_tab('new')

