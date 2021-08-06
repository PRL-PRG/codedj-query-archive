#########################################################################
#  Defines generateName.						#
#  Function takes 2 strings for files with the suffix and Prefix.       #
#  A seed may be used to get consistent values.				#
#  Function returns string of the Generated Space gang name.		#
#########################################################################

import random

def generateName(p, s, theSeed = None):
	prefix = open(p)
	suffix = open(s)
	random.seed(theSeed)

	#This will determine the name Type
	#For now 0 = The <prefix> <suffix>
	#        1 = The <suffix> of <prefix>
	nameType = random.randint(0, 1)
	pre = []
	suf = []

	#Suffixes and Prefixes are each placed on their own list.
	for x in prefix:
		pre.append(x.strip())


	for x in suffix:
		suf.append(x.strip())
	
	suffix.close()
	prefix.close()
	#Randomly determine Suffix and Prefix
	randomP = random.randint(0, len(pre) -1)
	randomS = random.randint(0, len(suf) -1)

	if nameType == 0:
	   name ='The ' + pre[randomP] + ' ' + suf[randomS]
	else:
	   name = 'The ' +  suf[randomS] + ' of '+ pre[randomP]

	return name

#Testing Code
#generateName('prefix.txt', 'suffix.txt')
#generateName('prefix.txt', 'suffix.txt', 100)
