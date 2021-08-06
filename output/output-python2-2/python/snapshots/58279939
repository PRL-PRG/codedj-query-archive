#!/usr/bin/python

try:
    import subprocess
    import sys

    import dbus

    def run(cmd):
        bus = dbus.SessionBus()
        screensaver = bus.get_object("org.gnome.ScreenSaver",
                                     "/org/gnome/ScreenSaver")
        screensaver.Inhibit("rungame", "Playing a game",
                            dbus_interface="org.gnome.ScreenSaver")
        return subprocess.call(cmd)


    if __name__ == "__main__":
        sys.exit(run(sys.argv[1:]))
except KeyboardInterrupt:
    pass

