''' mod_python site builder

Author: Justin White
Created: 2006-05-08
'''

import os, string, time
from mod_python import apache, util, Cookie

journal = apache.import_module('journal')
#pictures = apache.import_module('pictures')

config_dir = "/home/www/config/"
content_dir = os.path.join(os.path.dirname(__file__), 'content')
menu_file = os.path.join(content_dir, 'menu.htf')
footer_file = os.path.join(content_dir, 'footer.htf')
datestamp = '%A %Y-%m-%d %H:%M:%S %Z'

def handler(req):
	w = req.write
	"""MS Internet Explorer doesn't understand application/xhtml+xml.
	If the request came from MSIE and lie to it, using text/html instead"""
	agent = req.headers_in["User-Agent"]
	#if "MSIE" in agent:
	if True:
		req.content_type = "text/html; charset=utf-8"
		#w("User-Agent is IE: %s" % agent)
	else:
		req.content_type = "application/xhtml+xml; charset=utf-8"
		#w("User-Agent is not IE: %s" % agent)

	form = util.FieldStorage(req)
	page = form.getfirst('p', 'journal')

	user_cookie = session_cookie = None
	cookies = Cookie.get_cookies(req)
	if 'user' in cookies: user_cookie = cookies['user']
	if 'session' in cookies: session_cookie = cookies['session']

	if page == 'forget':
		user_cookie = Cookie.Cookie('user', '')
		user_cookie.expires = time.time() - 60
		Cookie.add_cookie(req, user_cookie)
		user_cookie = None
		page = 'journal'
	elif page == 'signout':
		session_cookie = Cookie.Cookie('session', '')
		session_cookie.expires = time.time() - 60
		Cookie.add_cookie(req, session_cookie)
		session_cookie = None
		page = 'journal'
	elif page == 'login':
		thisTime = time.time()
		user_name = form.getfirst('user', '')
		givenPassword = form.getfirst('password', '')
		if user_name:
			user_cookie = Cookie.Cookie('user', 'justin')
			user_cookie.expires = thisTime + 86400
			Cookie.add_cookie(req, user_cookie)
		# find password in .tpasswd file
#		pwf = file(os.path.join(config_dir, 'mainpasswd'))
#		for line in pwf:
#			foundUser, foundPassword = line.split(':')
#			if foundUser == user_name: break
		if givenPassword == 'pass':
			session_cookie = Cookie.Cookie('session', thisTime)
			session_cookie.expires = thisTime + 86400
			Cookie.add_cookie(req, session_cookie)
		# don't write anything to req until here, so all cookie data gets added
		#w(user_name + ":" + str(givenPassword))
		#w(foundUser + ":" + str(foundPassword))
		page = 'journal'
	elif page == 'update':
		os.system('cd ' + content_dir +' && ./lister.rb *.list')
		page = 'home'


	# xhtml header
	w('<?xml version="1.0" encoding="UTF-8"?>\n')
	w('<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.1//EN" "http://www.w3.org/TR/xhtml11/DTD/xhtml11.dtd">\n')
	w('<html xmlns="http://www.w3.org/1999/xhtml" xml:lang="en">\n')
	w('<head>\n')
	w('<title>tinfoil - ' + page.capitalize() + '</title>\n')
	w('<link rel="stylesheet" type="text/css" href="/yui/2.4.1/build/reset/reset.css"/>')
	w('<link rel="stylesheet" type="text/css" href="/yui/2.4.1/build/fonts/fonts.css"/>')
	w('<link rel="stylesheet" type="text/css" href="/yui/2.4.1/build/base/base.css"/>')
	w('<link rel="shortcut icon" href="/favicon.ico" type="image/x-icon" />\n')
	w('<link rel="icon" href="/favicon.ico" type="image/x-icon" />\n')
	w('<link rel="stylesheet" href="styles/main.css" type="text/css" title="default" />\n')
	if page in ['journal', 'pictures']:
		w('<link rel="stylesheet" href="styles/' + page + '.css" type="text/css" title="default" />\n')
	w('<style type="text/css">\n')
	#if page == 'journal':
		#w('#page_title a:link, #page_title a:visited')
	#else:
	w('#%s a:link' % page)
	w(""" {
	color: #36c;
	border-color: white;
	background-color: white;
}""")
	w('#%s a:visited' % page)
	w(""" {
	color: #F96;
	border-color: white;
	background-color: white;
}
""")
	w('#%s a:hover' % page)
	w(""" {
	color: white;
	border-color: #6c3;
	background-color: #6c3;
}
""")
	w('</style>\n')
	w('</head>\n')
	w('<body>\n')

	w('<div id="header-relative">\n')
	w('<h1 id="journal"><a href="./">tinfoil</a></h1>\n')

	# site menu
	w('<div id="menu">\n')
	menu = open(menu_file, 'r')
	for line in menu:
		w(line)
	w('</div>\n')
	w('</div>\n')
	w('<div id="content">\n')


	#check for login cooke
	w('<div id="login">\n')
	user_name = ''
	if user_cookie:
		user_name = user_cookie.value
	if session_cookie:
		w('<p>\n')
		w('Welcome <strong>%s</strong>. (<a href="?p=signout">Signout</a>)\n' % (user_cookie.value))
		w('</p>\n')
	else:
		w('<form action="/" method="get">\n')
		w('<input type="text" name="user" value="%s" />\n' % (user_name))
		w('<input type="password" name="password" value="" />\n')
		w('<input type="submit" name="p" value="login" />\n')
		if user_name:
			w('<a href="?p=forget">forget me</a>')
			#w('<input type="submit" name="p" value="forget" />\n')
		w('</form>\n')
#		w('<p>\n')
#		w('</p>\n')
	w('</div>\n')

	# page content
	if page == 'journal':
		j = journal.Journal(req, form, user_cookie, session_cookie)
		j.dispatch()
	elif page == 'pictures':
		pictures.dispatch(req, form)
	else:
		page_file = os.path.join(content_dir, page + '.htf')
		w('<div id="content_title">\n')
		w('<h2>' + page.capitalize() + '</h2>\n')
		w('</div>\n')

		try:
			content = file(page_file, 'r')
		except IOError:
			page = 'home'
			page_file = os.path.join(content_dir, page + '.htf')
			content = open(page_file, 'r')
		for line in content:
			w(line)

		# modification time
		w('<div id="mtime">\n')
		w('<p>\n')
		w('Last updated ')
		w(time.strftime(datestamp, time.localtime(os.stat(page_file)[8])))
		w('\n</p>\n')
		w('</div>\n')

		w('</div>\n')

	# footer
	w('<div id="footer-relative">\n')
	footer = file(footer_file, 'r')
	for line in footer:
		w(line)
	w('</div>\n')

	w('</body>\n')
	w('</html>\n')

	return apache.OK
