from py.__.documentation.confrest import *
from py.xml import html

class HPage(Page):
    def fill(self):
        super(HPage, self).fill()
        self.menubar[:] = html.ul(
            html.li(html.a("home", href="index.html", class_="menu")),
            html.li(html.a("install", href="install.html", class_="menu")),
            html.li(html.a("usage", href="usage.html", class_="menu")),
            id="menubar",
        )

class Project(Project):
    title = "hype"
    stylesheet = 'style.css'
    encoding = 'UTF-8'
    prefix_title = "hype"
    logo = html.div(class_="logo")
    Page = HPage
