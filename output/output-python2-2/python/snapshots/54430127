#!/usr/bin/env python

import re
from google.appengine.ext import webapp
from google.appengine.api import users
from google.appengine.ext.webapp import util
from google.appengine.ext.webapp import template
from models import TodoItem, Category, GetDefaultCategory, SetDefaultCategory

# TODO(jtolds): make sure escaping is happening
#               use gmail design of fixed amount of labels and archiving

NON_SAFE_CHARACTERS = re.compile("[^a-zA-Z0-9_]")

def GetTodoItems(user, category):
    if category == "Incomplete":
        todo_items = TodoItem.all().filter("user =", user)\
                .filter("archived =", False).filter("completed =", False)
    elif category == "Completed":
        todo_items = TodoItem.all().filter("user =", user)\
                .filter("archived =", False).filter("completed =", True)
    else:
        category_obj = Category.all().filter("user =", user).filter("name =",
                category).fetch(1)
        if(len(category_obj) != 1): return [], False
        todo_items = category_obj[0].items.filter("archived =", False)\
                .filter("completed =", False)
    todo_items = list(todo_items.order("-mtime"))
    return todo_items, len(todo_items) != 0

def GetCategories(user):
    return list(Category.all().filter("user =", user))

class Dashboard(webapp.RequestHandler):
  def post(self): return self.redirect("/dashboard")
  def get(self):
    user = users.get_current_user()
    logout_url = users.create_logout_url("/")
    category = GetDefaultCategory(user)
    todo_items, have_todo_items = GetTodoItems(user, category)
    categories = GetCategories(user)
    self.response.out.write(template.render("templates/dashboard.html",
        locals()))

class ItemList(webapp.RequestHandler):
    def get(self): return self.redirect("/dashboard")
    def post(self):
        user = users.get_current_user()
        logout_url = users.create_logout_url("/")
        todo_items, have_todo_items = GetTodoItems(user,
                self.request.get('category'))
        category = self.request.get('category')
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
        user = users.get_current_user()
        category = self.request.get("category")
        categories = GetCategories(user)
        self.response.out.write(template.render(
                "templates/menu_bar.html", locals()))

class AddCategory(webapp.RequestHandler):
    def get(self): return self.redirect("/dashboard")
    def post(self):
        user = users.get_current_user()
        category = self.request.get('name')
        if (category in ("Incomplete", "Completed") or
                NON_SAFE_CHARACTERS.search(category)):
            return self.error(500)
        new_category = Category(name=category, user=user)
        new_category.put()
        del new_category
        categories = GetCategories(user)
        self.response.out.write(template.render(
                "templates/category_list.html", locals()))

class RemoveCategory(webapp.RequestHandler):
    def get(self): return self.redirect("/dashboard")
    def post(self):
        user = users.get_current_user()
        category = self.request.get('name')
        default_category = GetDefaultCategory(user)
        cat_objs = Category.all().filter("user =", user).filter("name =",
                category)
        for cat_obj in cat_objs:
            if cat_obj.name == default_category:
                SetDefaultCategory(user, "Incomplete")
                default_category = "Incomplete"
            for item in cat_obj.items:
                item.category = None
                item.put()
            cat_obj.delete()
        categories = GetCategories(user)
        self.response.out.write(template.render(
                "templates/category_list.html", {"user": user,
                "categories": categories}))

class NewTodoItem(webapp.RequestHandler):
  def get(self): return self.redirect("/dashboard")
  def post(self):
    user = users.get_current_user()
    try:
        cat_obj = list(Category.all().filter("user =", user).filter("name =",
                self.request.get('category')).fetch(1))[0]
    except: cat_obj = None

    todo_item = TodoItem(
        user=user,
        title=self.request.get('title'),
        body=self.request.get('body'),
        category=cat_obj
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
    if self.request.get('action') == "category":
        cat_obj = list(Category.all().filter("user =", user).filter("name =",
                self.request.get('variable')).fetch(1))
        if len(cat_obj) != 1:
            cat_obj = None
        else:
            cat_obj = cat_obj[0]
    for item in items:
        if self.request.get('action') == "complete":
            item.completed = True
        elif self.request.get('action') == "incomplete":
            item.completed = False
        elif self.request.get('action') == "archive":
            item.archived = True
        elif self.request.get('action') == "category":
            item.category = cat_obj
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
      ('/dashboard/add_category/*', AddCategory),
      ('/dashboard/remove_category/*', RemoveCategory),
      ('.*', Redirect),
    ], debug=False)
  util.run_wsgi_app(app)

if __name__ == '__main__':
  main()
