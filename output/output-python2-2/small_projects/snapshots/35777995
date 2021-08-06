import os


def isVerbose():
    # If False, suppresses most commentary printed to stderr during test runs.
    return True


def tempDir():
    if 'TEST_TEMPDIR' in os.environ:
        testTempDir = os.environ['TEST_TEMPDIR']
        if testTempDir != '':
            return testTempDir

    return None
