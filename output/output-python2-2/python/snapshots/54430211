#!/usr/bin/env python

from google.appengine.ext import webapp
from google.appengine.api import users
from google.appengine.ext.webapp import util
from google.appengine.ext.webapp import template
from models import TodoItem

# TODO(jtolds): make sure each user has an email instantiation
#               make sure escaping is happening
#               use gmail design of fixed amount of labels and archiving

class Dashboard(webapp.RequestHandler):
  def get(self):
    user = users.get_current_user()
    logout_url = users.create_logout_url("/")
    todo_items = TodoItem.all().filter("user =", user)\
        .filter("archived =", False).order("completed").order("-mtime")
    archived_todo_items = TodoItem.all().filter("user =", user)\
        .filter("archived =", True).order("completed").order("-mtime")
    self.response.out.write(template.render("templates/dashboard.html",
        locals()))

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
    return self.redirect("/dashboard")

class EditTodoItem(webapp.RequestHandler):
  def update_field(self, todo_item, field, value=None):
    if value is None: value = self.request.get(field)
    if getattr(todo_item, field) == value:
      return False
    setattr(todo_item, field, value)
    return True
  def get(self, item_id):
    user = users.get_current_user()
    todo_item = TodoItem.get_by_id(int(item_id))
    self.response.out.write(template.render("templates/edit.html", locals()))
  def post(self, item_id):
    user = users.get_current_user()
    todo_item = TodoItem.get_by_id(int(item_id))
    edit_type = self.request.get("edit_type")
    changed = False
    for field in ("title", "body"):
      changed = self.update_field(todo_item, field) or changed
    for field in ("completed", "archived"):
      changed = self.update_field(todo_item, field,
          value=self.request.get(field) == "on") or changed
    if changed:
      todo_item.put()
    return self.redirect("/dashboard")

class DeleteTodoItem(webapp.RequestHandler):
  def get(self, item_id): return self.redirect("/dashboard")
  def post(self, item_id):
    TodoItem.get_by_id(int(item_id)).delete()
    return self.redirect("/dashboard")

class BatchEdit(webapp.RequestHandler):
  def get(self): return self.redirect("/dashboard")
  def post(self):
    user = users.get_current_user()
    checked_ids = set((int(x) for x in self.request.get('completed[]',
        allow_multiple=True)))
    all_ids = (int(x) for x in self.request.get("all_items").split(",")
        if x)
    for item_id in all_ids:
      item = TodoItem.get_by_id(item_id)
      new_completed = (item_id in checked_ids)
      if item.completed != new_completed:
        item.completed = new_completed
        item.put()
    return self.redirect("/dashboard")

class Redirect(webapp.RequestHandler):
  def get(self):
    return self.redirect("/")
  post = get

def main():
  app = webapp.WSGIApplication([
      ('/dashboard/*', Dashboard),
      ('/dashboard/new/*', NewTodoItem),
      ('/dashboard/edit/([a-zA-Z0-9]*)/*', EditTodoItem),
      ('/dashboard/delete/([a-zA-Z0-9]*)/*', DeleteTodoItem),
      ('/dashboard/batch_edit/*', BatchEdit),
      ('.*', Redirect),
    ], debug=False)
  util.run_wsgi_app(app)

if __name__ == '__main__':
  main()
