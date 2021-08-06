#!/usr/bin/env python

from google.appengine.ext import db

class ExtendedModel(db.Model):
  def key_id(self):
    return self.key().id()

class Contact(ExtendedModel):
  user = db.UserProperty(required=True)
  enabled = db.BooleanProperty(required=True, default=False)
  register_key = db.StringProperty(required=True)

class Phone(Contact):
  phone_number = db.PhoneNumberProperty(required=True)

class Email(Contact):
  email = db.EmailProperty(required=True)

class Category(ExtendedModel):
  user = db.UserProperty(required=True)
  name = db.StringProperty(required=True)

class DefaultCategory(ExtendedModel):
  user = db.UserProperty(required=True)
  name = db.StringProperty(required=True)

class TodoItem(ExtendedModel):
  user = db.UserProperty(required=True)
  ctime = db.DateTimeProperty(auto_now_add=True, required=True)
  mtime = db.DateTimeProperty(auto_now=True, required=True)
  title = db.StringProperty(required=True, multiline=True)
  body = db.StringProperty(multiline=True)
  completed = db.BooleanProperty(default=False)
  archived = db.BooleanProperty(default=False)
  category = db.ReferenceProperty(Category, collection_name="items")

def GetDefaultCategory(user):
    try:
      category = list(DefaultCategory.all().filter("user =", user).fetch(1))[0]
    except: return "Incomplete"
    return category.name

def SetDefaultCategory(user, category):
    try:
      list(DefaultCategory.all().filter("user =", user).fetch(1))[0].delete()
    except: pass
    DefaultCategory(name=category,user=user).put()
