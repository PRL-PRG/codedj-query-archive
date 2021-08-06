import os, os.path, sys, tempfile

originalTempDir = tempfile.gettempdir()

# Importing test_base will add the appropriate subdirectory of the build
# directory to Python's path, so that a compiled version of pyclene can be
# imported.
sys.path.append(os.path.abspath(os.path.join(os.pardir, os.pardir)))
import test_base

from pyclene import lucene

# Even the test support system in test_base reset Python's temp dir on the
# basis of environment variable TEST_TEMPDIR to point to a ramdisk, that's not
# appropriate for this test program, so we enforce the original temp dir.
tempfile.tempdir = originalTempDir
assert tempfile.gettempdir() == originalTempDir
