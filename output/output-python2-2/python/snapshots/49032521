#!/usr/bin/python
# -*- coding: utf-8 -*-

import gtk
import gtk.gdk
import gtk.glade
import gtkmozembed

ICON = "/usr/share/pixmaps/lemurae.svg"

class Lemurae:

    def __init__(self):

        xml = gtk.glade.XML("/usr/share/lemurae/lemurae.glade", "appw")
        xml.signal_autoconnect(self)

        # Widgets
        self.word = xml.get_widget("Palabra")
        self.frame = xml.get_widget("Marco")
        self.appw = xml.get_widget("appw")
        self.back_button = xml.get_widget("bAtras")
        self.next_button = xml.get_widget("bAdelante")
        self.statusbar = xml.get_widget("statusbar")
        self.web_control = gtkmozembed.MozEmbed()

        self.__set_widgets()

        self.appw.show_all()


    def __set_widgets(self):
        # Quit app
        self.appw.connect("destroy", gtk.main_quit)

        # Set buttons sensitive
        self.back_button.set_sensitive(False)
        self.next_button.set_sensitive(False)

        # Set web_control
        self.web_control.connect('net-stop', self.__on_net_stop)
        self.web_control.connect('net-start', self.__on_net_start)
        self.web_control.show_all()
        self.statusbar.push(0, "Listo.")

        # Add web_control
        self.frame.add(self.web_control)

        # App icon
        icon = gtk.gdk.pixbuf_new_from_file(ICON)
        self.appw.set_icon(icon)


    # SIGNAL HANDLERS ##############################
    def on_bBuscar_clicked(self, widget):
        url = "http://buscon.rae.es/draeI/SrvltGUIBusUsual?LEMA="
        url += self.word.get_text().lower()
        self.web_control.load_url(url)
        self.word.select_region(0, len(self.word.get_text()))
        self.appw.set_focus(self.word)


    def on_bAdelante_clicked(self, widget):
        self.web_control.go_forward()


    def on_bAtras_clicked(self, widget):
        self.web_control.go_back()


    def on_bAyuda_clicked(self, widget):
        dlg = gtk.AboutDialog()

        dlg.set_name("Lemurae")
        dlg.set_copyright("(c) 2006 Gumer Coronel Pérez")
        dlg.set_version('0.2.1')

        logo = gtk.gdk.pixbuf_new_from_file(ICON)
        dlg.set_logo(logo)
        dlg.run()
        dlg.destroy()


    def __on_net_start(self, *args):
        self.statusbar.push(0, "Buscando...")


    def __on_net_stop(self, *args):
        self.statusbar.pop(0)
        self.back_button.set_sensitive(self.web_control.can_go_back())
        self.next_button.set_sensitive(self.web_control.can_go_forward())
        self.appw.set_focus(self.word)



if __name__ == "__main__":
    lm = Lemurae()
    gtk.main()

