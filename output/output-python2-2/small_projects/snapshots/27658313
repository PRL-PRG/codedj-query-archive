#########################################################################
#  Defines generateWeapon.					        #
#  A seed may be used to get consistent values.				#
#  Function returns a string with the generated name.			#
#########################################################################

import random

def generateWeapon(theSeed = None):
	manufacturers = open("manufacturers.txt")
	design = open("design.txt")
	weaponType = open("weapon.txt")
	versionType = open("version.txt")
	versionNum = open("versionnum.txt")
	random.seed(theSeed)

	manu = []
	des = []
	wType = []
	verType = []
	verNum = []

	#Each Atrribute is placed on their own list.
	for x in manufacturers:
		manu.append(x.strip())
	for x in design:
		des.append(x.strip())
	for x in weaponType:
		wType.append(x.strip())
	for x in versionType :
		verType.append(x.strip())
	for x in versionNum:
		verNum.append(x.strip())    
    
    	design.close()
    	weaponType.close()
    	versionType.close()
    	versionNum.close()
    	manufacturers.close()

	#Randomly determine Suffix and Prefix
	randomM = random.randint(0, len(manu) -1)
	randomD = random.randint(0, len(des) -1)
	randomW = random.randint(0, len(wType) -1)
	randomVT= random.randint(0, len(verType) -1)
	randomVN = random.randint(0, len(verNum) -1)


	name = manu[randomM] + ' ' + des[randomD] + ' ' +  wType[randomW] + ' ' + verType[randomVT] + ' ' + verNum[randomVN]
    	return name


#	print manu[randomM], des[randomD], wType[randomW], verType[randomVT], verNum[randomVN]

#Testing Code
#generateWeapon()
#generateWeapon(100)
