import unittest

def init_blackboxtest_suite():
	"""Assembles all blackbox test (API, ...) into one suite.
	"""
	from blackbox_api_unicode import ApiUnicodeTest
	from blackbox_api_latin1 import ApiLatin1Test

	suites = []
	for testcase in (ApiUnicodeTest, ApiLatin1Test):
		suites.append(unittest.TestLoader().loadTestsFromTestCase(testcase))
	
	return unittest.TestSuite(suites)
	
def init_unittest_suite():
	"""Assembles all unittests into one suite.
	"""
	from deliciousapi import DeliciousApiUnitTest
	
	suites = []
	for testcase in (DeliciousApiUnitTest,):
		suites.append(unittest.TestLoader().loadTestsFromTestCase(testcase))
	
	return unittest.TestSuite(suites)

def main():
	"""Run all tests.
	"""
	print "Start all tests"
	
	# Load all suites here	
	api_suite = init_blackboxtest_suite()
	unit_suite = init_unittest_suite()
	
	main_suite = unittest.TestSuite([api_suite, unit_suite])
	unittest.TextTestRunner(verbosity=2).run(main_suite)

def unittest_main():
	"""Run unittests only
	"""
	print "Start unittests"
	
	unittest.TextTestRunner(verbosity=2).run(init_unittest_suite())
	
if __name__ == '__main__': 
	main()
	#unittest_main()
