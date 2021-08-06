
import subprocess
import sys
import os


def main():
    # /proc/XXXX/stat contains gnome-settings-
    p = subprocess.Popen(["pgrep", "gnome-settings-"], stdout=subprocess.PIPE)
    try:
        gnome_settings_daemon_pid = int(p.stdout.read())
    except:
        sys.exit("Couldn't find gnome-settings-daemon")
    p.wait()
    environ_filepath = os.path.join("/proc", str(gnome_settings_daemon_pid),
                                    "environ")
    environ = open(environ_filepath).read().split("\0")
    for entry in environ:
        if entry.startswith("SSH_AUTH_SOCK="):
            sys.stdout.write("%s\n" % entry[len("SSH_AUTH_SOCK="):])
            break


if __name__ == "__main__":
    main()

