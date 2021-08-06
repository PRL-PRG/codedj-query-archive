import rox, gtk
from rox import Menu

ADD = 1
EDIT = 2
DELETE = 3
REVERT = 4

class AccountEditor(rox.Dialog):
	last_row = 0

	def __init__(self, account):
		rox.Dialog.__init__(self)
		self.set_title(_('Account Editor'))
		self.set_size_request(300, -1)

		self.account = account

		self.table = gtk.Table(2, 4)
		self.vbox.add(self.table)

		self.add_str(_('Name: '), 'name', account.name)
		self.add_int(_('Poll Time: '), 'polltime', account.polltime)

		self.add_button(gtk.STOCK_OK, gtk.RESPONSE_OK)
		self.set_default_response(gtk.RESPONSE_OK)

		def response(self, resp):
			if resp == gtk.RESPONSE_OK:
				self.save()
			self.destroy()
		self.connect('response', response)

	def save(self):
		pass

	def add_row(self, label, widget):
		n = self.last_row
		self.table.attach(gtk.Label(label), 0, 1, n, n+1, 0, 0)
		self.table.attach(widget, 1, 2, n, n+1, gtk.EXPAND|gtk.FILL, 0)
		self.last_row += 1

	def add_int(self, label, name, value, min=1, max=65535, step=1, page=10):
		entry = gtk.SpinButton()
		entry.set_range(min, max)
		entry.set_increments(step, page)
		entry.set_value(value)
		entry.connect('value-changed', self.set_int, name)
		self.add_row(label, entry)

	def set_int(self, widget, name):
		self.account.__dict__[name] = widget.get_value()
	
	def add_str(self, label, name, value, visible=True):
		entry = gtk.Entry()
		entry.set_text(value)
		entry.set_visibility(visible)
		entry.connect('changed', self.set_str, name)
		self.add_row(label, entry)

	def set_str(self, widget, name):
		self.account.__dict__[name] = widget.get_text()

	def add_list(self, label, name, value):
		string = ','.join(value)
		entry = gtk.Entry()
		entry.set_text(string)
		entry.connect('changed', self.set_list, name)
		self.add_row(label, entry)

	def set_list(self, widget, name):
		string = widget.get_text()
		self.account.__dict__[name] = string.split(',') 

	def add_bool(self, label, name, value, text=None):
		item = gtk.CheckButton(label=text)
		item.set_active(value)
		item.connect('toggled', self.set_bool, name)
		self.add_row(label, item)

	def set_bool(self, widget, name):
		self.account.__dict__[name] = widget.get_active()

class MBOXEditor(AccountEditor):
	def __init__(self, account):
		AccountEditor.__init__(self, account)
		self.set_title(_('MBOX Account Editor'))
		self.add_str(_('Path: '), 'filename', account.filename)
		self.show_all()

class IMAPEditor(AccountEditor):
	def __init__(self, account):
		AccountEditor.__init__(self, account)
		self.set_title(_('IMAP Account Editor'))
		self.add_str(_('Server: '), 'server', account.server)
		self.add_int(_('Port: '), 'port', account.port, 1, 65535, 1, 100)
		self.add_str(_('Username: '), 'username', account.username)
		self.add_str(_('Password: '), 'password', account.password, False)
		self.add_list(_('Folders: '), 'folders', account.folders)
		self.add_bool(_('SSL: '), 'ssl', account.ssl)
		self.show_all()

class POPEditor(AccountEditor):
	def __init__(self, account):
		AccountEditor.__init__(self, account)
		self.set_title(_('POP Account Editor'))
		self.add_str(_('Server: '), 'server', account.server)
		self.add_int(_('Port: '), 'port', account.port, 1, 65535, 1, 100)
		self.add_str(_('Username: '), 'username', account.username)
		self.add_str(_('Password: '), 'password', account.password, False)
		self.add_bool(_('SSL: '), 'ssl', account.ssl)
		self.add_bool(_('APOP: '), 'apop', account.apop)
		self.show_all()


class AccountList(rox.Dialog):
	def __init__(self, accounts):
		rox.Dialog.__init__(self)
		self.set_title(_('Postal: Account Editor'))

		self.accounts = accounts

		hbox = gtk.HBox()
		self.vbox.pack_start(hbox)
		swin = gtk.ScrolledWindow()
		swin.set_size_request(-1, 200)
		hbox.pack_start(swin)

		self.accts = gtk.ListStore(str, str, object)
		if accounts != None:
			for acct in accounts:
				row = self.accts.append()
				self.accts.set(row, 0, acct.name, 1, acct.protocol, 2, acct)
			
		view = gtk.TreeView(self.accts)
		view.append_column(gtk.TreeViewColumn(_('Name'), gtk.CellRendererText(), text=0))
		view.append_column(gtk.TreeViewColumn(_('Type'), gtk.CellRendererText(), text=1))
		view.connect('row-activated', self.select_row)
		swin.add(view)
		self.view = view

		def response(self, resp):
			if resp == EDIT:
				self.edit()
			elif resp == DELETE:
				self.remove()
			elif resp == ADD:
				self.add()
			else:
				return False
			self.emit_stop_by_name('response')
			return True

		self.connect('response', response)
		self.add_button(gtk.STOCK_ADD, ADD)
		self.add_button(gtk.STOCK_DELETE, DELETE)
		self.add_button(gtk.STOCK_EDIT, EDIT)
		self.add_button(gtk.STOCK_CLOSE, gtk.RESPONSE_CANCEL)
		self.set_default_response(gtk.RESPONSE_CANCEL)

		self.vbox.show_all()
		self.action_area.show_all()
	
	def select_row(self, list, row, *stuff):
		model = list.get_model()
		iter = model.get_iter(row)
		account = model.get_value(iter, 2)
		self.edit_account(account)

	def add(self):
		#choose BOX type
		items = []
		items.append(Menu.Action(_('IMAP'), 'add_account', None, None, ('IMAP',)))
		items.append(Menu.Action(_('POP'), 'add_account', None, None, ('POP',)))
		items.append(Menu.Action(_('MBOX'), 'add_account', None, None, ('MBOX',)))
		menu = Menu.Menu('choose', items)

		menu.attach(self, self)
		menu.popup(self, None)

	def add_account(self, account_type):
		if account_type == 'IMAP':
			import imap_check
			account = imap_check.IMAPChecker()
		elif account_type == 'POP':
			import pop_check
			account = pop_check.POPChecker()
		elif account_type == 'MBOX':
			import mbox_check
			account = mbox_check.MBOXChecker()
		else:
			return

		if self.edit_account(account):
			self.accts.append([account.name, account.protocol, account])
			self.accounts.append(account)

	def remove(self):
		model, iter = self.view.get_selection().get_selected()
		path = model.get_path(iter)
		model.remove(iter)
		del self.accounts[path[0]]

	def edit(self):
		model, iter = self.view.get_selection().get_selected()
		account = model.get_value(iter, 2)
		self.edit_account(account)

	def edit_account(self, account):
		if account.protocol == 'IMAP':
			dlg = IMAPEditor(account)
		elif account.protocol == 'MBOX':
			dlg = MBOXEditor(account)
		elif account.protocol == 'POP':
			dlg = POPEditor(account)
		dlg.run()
		return True

