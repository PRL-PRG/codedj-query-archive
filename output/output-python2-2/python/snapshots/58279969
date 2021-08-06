#!/usr/bin/python
# Copyright 2007 Tristan Hill <stan@saticed.me.uk>
# This program is free software; you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation; either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <http://www.gnu.org/licenses/>.


"""
To have a fully functional chroot you want /proc etc mounted -
create /etc/fstab in the chroot with:
proc /proc proc defaults 0 0
sys /sys sysfs defaults 0 0
devpts /dev/pts devpts defaults 0 0
#/dev /dev none bind 0 0
"""



import distutils.dir_util
import grp
import optparse
import os
import pickle
import pwd
import shutil
import subprocess
import sys
import traceback
import logging


def my_expanduser(path, home=None):
    """
    like os.expanduser but allows specifying a custom home directory
    """
    oldhome = None
    if home:
        if "HOME" in os.environ:
            oldhome = os.environ["HOME"]
        os.environ["HOME"] = home
    newpath = os.path.expanduser(path)
    if oldhome:
        os.environ["HOME"] = oldhome
    elif home:
        del os.environ["HOME"]
    return newpath


def call(cmd):
    logging.info("call:%s", " ".join(cmd))
    return subprocess.check_call(cmd)


def get_mountpoints(deleted_only=False):
    mounts = []
    for line in file("/proc/mounts"):
        mountpoint = line.split()[1]
        deleted_marker = r"\040(deleted)"
        if mountpoint.endswith(deleted_marker):
            mountpoint = mountpoint[:-len(deleted_marker)]
            if deleted_only:
                mounts.append(mountpoint)
        if not deleted_only:
            mounts.append(mountpoint)
    # sort so parents come first
    mounts.sort()
    return mounts


def chroot_unmount(root, deleted_only=False):
    for dirname in get_mountpoints(deleted_only)[::-1]:
        if dirname.startswith(root):
            call(["umount", "-n", dirname])


class Chroot(object):

    def __init__(self, root, user):
        if not os.path.isabs(root):
            raise RuntimeError("root needs to be absolute path")
        self.root = root.rstrip(os.sep)
        if not os.path.isdir(self.root):
            raise RuntimeError("root doesn't exist")
        self.user = user
        self._mount_all()

    def _mount_all(self):
        for line in file(os.path.join(self.root, "etc", "fstab")):
            fields = line.split()
            if len(fields) == 0 or fields[0].startswith("#"):
                continue
            elif len(fields) >= 4:
                filesystem, mountpoint, type, options = fields[:4]
                mount_args = ["-o", options, "-t", type, filesystem]
                self._mount_once(mount_args, mountpoint)

    def _mount_once(self, args, mountpoint):
        mountpoint = self.root + mountpoint
        if mountpoint not in get_mountpoints():
            distutils.dir_util.mkpath(mountpoint)            
            call(["mount", "-n"] + args + [mountpoint])

    def _login(self):
        def getgroups(user):
            groups = []
            for g in grp.getgrall():
                if user in g.gr_mem:
                    groups.append(g.gr_gid)
            return groups
        os.chroot(self.root)
        try:
            pwent = pwd.getpwnam(self.user)
        except KeyError:
            sys.stderr.write("no such user \"%s\" in chroot\n" % self.user)
            sys.exit(1)
        os.setgroups(getgroups(self.user))
        os.setgid(pwent.pw_gid)
        os.setuid(pwent.pw_uid)
        env = {}
        env["USER"] = self.user
        env["LOGNAME"] = self.user
        env["HOME"] = pwent.pw_dir
        env["SHELL"] = pwent.pw_shell
        os.environ = env
        os.chdir(pwent.pw_dir)

    def login(self, extra_env=None):
        chroot_unmount(self.root, deleted_only=True)
        self._login()
        if extra_env:
            os.environ.update(extra_env)
        shell = os.environ["SHELL"]
        if shell == "":
            raise RuntimeError("no shell set")
        os.execve(shell, ["-" + os.path.basename(shell)], os.environ)

    def _get_chroot_pwent(self):
        read_end, write_end = os.pipe()
        if os.fork() == 0:
            os.close(read_end)
            os.chroot(self.root)
            f = os.fdopen(write_end, "wb")
            p = pickle.dump(pwd.getpwnam(self.user), f)
            f.close()
            sys.exit(0)
        else:
            os.close(write_end)
            f = os.fdopen(read_end, "rb")
            pwent = pickle.load(f)
            os.wait()
            f.close()
            return pwent

    def run(self, func, args=(), env=None):
        return_read_end, return_write_end = os.pipe()
        exception_read_end, exception_write_end = os.pipe()
        if os.fork() == 0:
            os.close(return_read_end)
            os.close(exception_read_end)
            self._login()
            if env:
                os.environ.update(env)
            return_file = os.fdopen(return_write_end, "wb")
            exception_file = os.fdopen(exception_write_end, "wb")
            try:
                try:
                    ret = func(*args)
                    pickle.dump(ret, return_file)
                    pickle.dump((None, None), exception_file)
                except Exception, e:
                    # can't pickle traceback object
                    pickle.dump((e, traceback.format_exc()), exception_file)
            finally:
                return_file.close()
                exception_file.close()
            os._exit(0)
        else:
            os.close(return_write_end)
            os.close(exception_write_end)     
            return_file = os.fdopen(return_read_end, "rb")
            exception_file = os.fdopen(exception_read_end, "rb")
            try:
                try:
                    ret = pickle.load(return_file)
                except EOFError:
                    e, tb = pickle.load(exception_file)
                    sys.stderr.write("%s\n" % tb)
                    raise e
                os.wait()
            finally:
                return_file.close()
                exception_file.close()
            return ret

    def copyfiles(self, filenames):
        new_pwent = self._get_chroot_pwent()
        for filename in filenames:
            src = my_expanduser(filename)
            if os.path.exists(src):
                dst = my_expanduser(filename, home=new_pwent.pw_dir)
                dst = os.path.join(self.root, dst[1:])
                shutil.copyfile(src, dst)
                os.chown(dst, new_pwent.pw_uid, new_pwent.pw_gid)

    def mountbind(self, dirs):
        for dirname in dirs:
            self._mount_once(["--bind", dirname], dirname)

    def xauth(self, display, xauthority):
        if display.startswith("localhost:"):
            display = display[len("localhost"):]
        p = subprocess.Popen(["xauth", "-f", xauthority, "extract", "-", 
                              display], stdout=subprocess.PIPE)
        if os.fork() == 0:
            self._login()
            try:
                os.environ["PATH"] = "/usr/local/sbin:/usr/local/bin:" \
                    "/usr/sbin:/usr/bin:/sbin:/bin:/usr/bin/X11"
                subprocess.call(["xauth", "merge", "-"], stdin=p.stdout,
                                env=os.environ)
            except OSError:
                sys.stderr.write("xauth not found\n")
            sys.exit(0)
        else:
            p.stdout.close()
            p.wait()
            os.wait()


def main():
    argv = sys.argv
    if "DISPLAY" in os.environ and "--display" not in argv:
        argv.extend(["--display", os.environ["DISPLAY"]])
    if "--xauthority" not in argv:
        if "XAUTHORITY" in os.environ:
            xauthority = os.environ["XAUTHORITY"]
        else:
            xauthority = os.path.expanduser("~/.Xauthority")
        argv.extend(["--xauthority", xauthority])
    if "SSH_AUTH_SOCK" in os.environ and "--ssh-auth-sock" not in argv:
        argv.extend(["--ssh-auth-sock", os.environ["SSH_AUTH_SOCK"]])
    
    parser = optparse.OptionParser(usage="%prog [options] newroot [user]")
    parser.add_option("-u", "--unmount", dest="unmount", action="store_true",
                      help="unmount all mountpoints in chroot")
    parser.add_option("-v", dest="verbose", action="store_true",
                      help="be verbose")
    parser.add_option("--remove", dest="remove", action="store_true",
                      help="remove chroot")
    parser.add_option("-r", "--gain-root-command", dest="gain_root_command",
                      default="sudo", help="command to run to gain root")
    parser.add_option("--ssh-auth-sock", dest="ssh_auth_sock")
    parser.add_option("--display", dest="display")
    parser.add_option("--xauthority", dest="xauthority")
    options, args = parser.parse_args(argv[1:])
    root = os.path.abspath(os.path.expanduser(args[0]))
    if len(args) > 1:
        user = args[1]
    else:
        user = None
    
    if os.geteuid() == 0:
        if options.unmount:
            chroot_unmount(root)
            sys.exit(0)
        if options.remove:
            chroot_unmount(root)
            distutils.dir_util.remove_tree(root)
            sys.exit(0)
        if options.verbose:
            logging.basicConfig(level=logging.INFO)
        c = Chroot(root, user)
        env = {"TERM": os.environ["TERM"],
               "PATH": "/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:"
                       "/sbin:/bin:/usr/games"}
        mount_binds = []
        if options.display:
            mount_binds.append("/tmp/.X11-unix")
            c.xauth(options.display, options.xauthority)
            env["DISPLAY"] = options.display
        if options.ssh_auth_sock:
            mount_binds.append(os.path.dirname(options.ssh_auth_sock))
            env["SSH_AUTH_SOCK"] = options.ssh_auth_sock
        c.mountbind(mount_binds)
        c.login(env)
    else:
        if user is None:
            argv.append(pwd.getpwuid(os.getuid()).pw_name)
        os.execvp(options.gain_root_command,
                  [options.gain_root_command, sys.executable] + argv)    


if __name__ == "__main__":
    main()

