#!/usr/bin/env python

from google.appengine.ext import webapp
from google.appengine.api import users
from google.appengine.ext.webapp import util
from google.appengine.ext.webapp import template
from models import TodoItem

# TODO(jtolds): make sure each user has an email instantiation
#               make sure escaping is happening
#               use gmail design of fixed amount of labels and archiving

def GetTodoItems(user, category):
    if category not in ("Inbox", "Completed"):
        return [], False
    todo_items = TodoItem.all().filter("user =", user)\
            .filter("archived =", False)
    if category == "Inbox":
        todo_items = todo_items.filter("completed =", False)
    elif category == "Completed":
        todo_items = todo_items.filter("completed =", True)
    todo_items = list(todo_items.order("completed").order("-mtime"))
    return todo_items, len(todo_items) != 0

class Dashboard(webapp.RequestHandler):
  def post(self): return self.redirect("/dashboard")
  def get(self):
    user = users.get_current_user()
    logout_url = users.create_logout_url("/")
    category = "Inbox"
    todo_items, have_todo_items = GetTodoItems(user, category)
    self.response.out.write(template.render("templates/dashboard.html",
        locals()))

class ItemList(webapp.RequestHandler):
    def get(self): return self.redirect("/dashboard")
    def post(self):
        user = users.get_current_user()
        logout_url = users.create_logout_url("/")
        todo_items, have_todo_items = GetTodoItems(user,
                self.request.get('category'))
        self.response.out.write(template.render("templates/todo_items.html",
                locals()))

class ItemDetail(webapp.RequestHandler):
    def post(self, todo_item): return self.redirect("/dashboard")
    def get(self, todo_item):
        user = users.get_current_user()
        logout_url = users.create_logout_url("/")
        todo_item = TodoItem.get_by_id(int(todo_item))
        self.response.out.write(template.render(
                "templates/todo_item_detail.html", locals()))

class CategoryMenu(webapp.RequestHandler):
    def post(self): return self.redirect("/dashboard")
    def get(self):
        category = self.request.get("category")
        self.response.out.write(template.render(
                "templates/menu_bar.html", locals()))

class NewTodoItem(webapp.RequestHandler):
  def get(self): return self.redirect("/dashboard")
  def post(self):
    user = users.get_current_user()
    todo_item = TodoItem(
        user=user,
        title=self.request.get('title'),
        body=self.request.get('body'),
      )
    todo_item.put()
    return self.response.out.write("OK")

class EditTodoItem(webapp.RequestHandler):
  def get(self, item_id): return self.redirect("/dashboard")
  def update_field(self, todo_item, field, value=None):
    if value is None: value = self.request.get(field)
    if getattr(todo_item, field) == value:
      return False
    setattr(todo_item, field, value)
    return True
  def post(self, item_id):
    user = users.get_current_user()
    todo_item = TodoItem.get_by_id(int(item_id))
    changed = False
    for field in ("title", "body"):
      changed = self.update_field(todo_item, field) or changed
    for field in ("completed", "archived"):
      changed = self.update_field(todo_item, field,
          value=self.request.get(field) == "on") or changed
    if changed:
      todo_item.put()
    return self.response.out.write("OK");

class DeleteTodoItem(webapp.RequestHandler):
  def get(self, item_id): return self.redirect("/dashboard")
  def post(self, item_id):
    TodoItem.get_by_id(int(item_id)).delete()
    return self.redirect("/dashboard")

class BatchEdit(webapp.RequestHandler):
  def get(self): return self.redirect("/dashboard")
  def post(self):
    user = users.get_current_user()
    items = set((TodoItem.get_by_id(int(x))
            for x in self.request.get('items').split(',')))
    for item in items:
        if self.request.get('action') == "complete":
            item.completed = True
        elif self.request.get('action') == "incomplete":
            item.completed = False
        elif self.request.get('action') == "archive":
            item.archived = True
        else:
            continue
        item.put()
    return self.response.out.write("OK")

class Redirect(webapp.RequestHandler):
  def get(self):
    return self.redirect("/")
  post = get

def main():
  app = webapp.WSGIApplication([
      ('/dashboard/*', Dashboard),
      ('/dashboard/itemlist/*', ItemList),
      ('/dashboard/detail/([a-zA-Z0-9]*)/*', ItemDetail),
      ('/dashboard/new/*', NewTodoItem),
      ('/dashboard/edit/([a-zA-Z0-9]*)/*', EditTodoItem),
#      ('/dashboard/delete/([a-zA-Z0-9]*)/*', DeleteTodoItem),
      ('/dashboard/batch_edit/*', BatchEdit),
      ('/dashboard/menu/*', CategoryMenu),
      ('.*', Redirect),
    ], debug=False)
  util.run_wsgi_app(app)

if __name__ == '__main__':
  main()
