import unittest


def init_blackboxtest_suite():
    """Gather all *blackbox tests* (API, ...) into one suite.

    """
    import blackbox_api
    if not (blackbox_api.usr and blackbox_api.pwd):
        print "Skipping Blackbox API tests"
        return

    from blackbox_api_unicode import ApiUnicodeTest
    from blackbox_api_latin1 import ApiLatin1Test

    suites = []
    for testcase in (blackbox_api.TestApiCalls,
            ApiUnicodeTest,
            ApiLatin1Test):
        suites.append(unittest.TestLoader().loadTestsFromTestCase(testcase))

    return unittest.TestSuite(suites)

def init_unittest_suite():
    """Gather all *unittests* into one suite.
    """
    from pydelicioustest import __testcases__ as l1
    from pydelicioustest_unicode import __testcases__ as l2
    from pydelicioustest_latin1 import __testcases__ as l3

    suites = []
    for testcase in l1 + l2 + l3:
        suites.append(unittest.TestLoader().loadTestsFromTestCase(testcase))

    return unittest.TestSuite(suites)

def main():
    """Run all tests.
    """
    print "Starting all tests"

    suites = []

    # Load all suites here
    api_suite = init_blackboxtest_suite()
    if api_suite: # user can choose to skip tests which query the server
        suites.append(api_suite)
    suites.append(init_unittest_suite())

    test(suites)

def test(suites):
    main_suite = unittest.TestSuite(suites)
    unittest.TextTestRunner(verbosity=2).run(main_suite)

def unittest_main():
    """Run unittests only
    """
    print "Starting unittests"

    unittest.TextTestRunner(verbosity=2).run(init_unittest_suite())

if __name__ == '__main__': main()
