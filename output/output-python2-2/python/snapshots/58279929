#!/usr/bin/python

import os
import sys
import time


def slurp(*path):
    p = os.path.join(*path)
    if os.path.exists(p):
        return open(p).read().rstrip()
    else:
        return None


def get_sg_device(vendor, model, buspath="/sys/bus/scsi/devices"):
    for device in os.listdir(buspath):
        syspath = os.path.join(buspath, device)
        if slurp(syspath, "vendor") == vendor and \
                slurp(syspath, "model") == model:
            return \
                os.path.join("/dev",
                    os.path.basename(
                        os.readlink(os.path.join(syspath, "generic"))))


def probe(host_driver, scsi_id,
          classpath="/sys/class/scsi_host",
          proc_scsi="/proc/scsi/scsi"):
    for host in os.listdir(classpath):
        hostpath = os.path.join(classpath, host)
        if host.startswith("host") and \
                slurp(hostpath, "proc_name") == host_driver:
            open(proc_scsi, "w").write(
                "scsi add-single-device %s %d %s %d\n" % 
                    (host[len("host"):], 0, scsi_id, 0))
    

import shutil
import tempfile
import unittest

class Tests(unittest.TestCase):

    def setUp(self):
        self.tempdir = tempfile.mkdtemp()
   
    def tearDown(self):
        shutil.rmtree(self.tempdir)

    def test_get_sg_device(self):
        devicepath = os.path.join(self.tempdir, "a_device")
        os.mkdir(devicepath)
        os.mkdir(os.path.join(self.tempdir, "somethingelse"))
        open(os.path.join(devicepath, "vendor"), "w").write("a vendor\n")
        open(os.path.join(devicepath, "model"), "w").write("a model\n")
        os.symlink("/a/path/sg0", os.path.join(devicepath, "generic"))
        
        self.assertEqual(get_sg_device("a vendor", "a model", self.tempdir),
                         "/dev/sg0")
        
    def test_probe(self):
        hostpath = os.path.join(self.tempdir, "host0")
        os.mkdir(hostpath)
        open(os.path.join(hostpath, "proc_name"), "w").write("driver\n")
        
        proc_scsi = os.path.join(self.tempdir, "scsi")
        probe("driver", "2", self.tempdir, proc_scsi)
        self.assertEqual(open(proc_scsi).read(),
                         "scsi add-single-device 0 0 2 0\n")


def main(args):
    host_driver, scsi_id, vendor, model = args
    for i in range(2):
        sg_device = get_sg_device(vendor, model)
        if sg_device is not None:
            print sg_device
            break
        else:
            probe(host_driver, scsi_id)
            time.sleep(1)


if __name__ == "__main__":
    main(sys.argv[1:])

