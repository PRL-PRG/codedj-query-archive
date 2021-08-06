"""Schema for BlogTut."""

from schevo.schema import *
schevo.schema.prep(locals())


class SchevoIcon(E.Entity):
    """Stores icons for Schevo database and application objects."""

    _hidden = True

    name = f.unicode()
    data = f.image()

    _key(name)

class Post(E.Entity):
    """Blog post entity"""
    title = f.unicode()
    slug = f.string()
    abstract = f.memo()
    body = f.memo(allow_empty=True, required=False)
    created_at = f.datetime()

    _index(created_at)
    _key(slug)

class Comment(E.Entity):
    """Comment on blog post entity"""
    post = f.entity('Post', CASCADE)
    author = f.unicode()
    author_email = f.unicode()
    author_site = f.unicode(required=False, allow_empty=True)
    body = f.memo()
    created_at = f.datetime()

    _index(created_at)
    _key(author, author_email, created_at, body)

class Tag(E.Entity):
    """Tag for blog post entity"""
    name = f.unicode()
    slug = f.string()

    _key(name, slug)

class PostTag(E.Entity):
    """Many-to-many relation between Post and Tag"""
    tag = f.entity('Tag')
    post = f.entity('Post')

    _key(tag, post)


# sample data
E.Post._sample = [
  (u"Hello, World", "hello-world", u"Just say hello to everyone!", DEFAULT, "2007-09-27 11:14"),
  (u"Test", "test", u"Test, please ignore it", "The body of test post", "2007-09-28 17:36"),
]

E.Comment._sample = [
  (("hello-world", ), u"Pythy", u"the.pythy@gmail.com", u"http://www.pyobject.ru", u"Firstf**k", "2007-09-27 12:42"),
  (("hello-world", ), u"DummyCommenter", u"the.pythy@gmail.com", DEFAULT, u"+1", "2007-09-28 00:15"),
  (("test", ), u"DummyCommenter", u"the.pythy@gmail.com", DEFAULT, u"+1", "2007-09-28 00:22"),
]

E.Tag._sample = [
  (u"Hello", "hello"),
  (u"Test", "test"),
]

E.PostTag._sample = [
  ((u"Hello", "hello"), ("hello-world", )),
  ((u"Test", "test"), ("hello-world", )),
  ((u"Test", "test"), ("test", )),
]
