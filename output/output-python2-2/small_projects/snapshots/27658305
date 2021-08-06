#########################################################################
#  Defines generateWeaponResults.					#
#  Function takes 2 strings for the name of the weapon and its target.  #
#  wType refers to designate what type of weapon it is			#
#  A seed may be used to get consistent values.				#
#  Function returns a string with the weapon result description.        #
#########################################################################



import random


def generateWeaponResult(name, target, wtype, theSeed = None):
    #neutrals refers to descriptions that both energy and physical weapons should contain.
    shipPart = open("shippart.txt")
    miscResults = open('neutrals.txt')
    if wtype == 0:
        damageDescription = open("physical.txt")
    else:
        damageDescription = open("energy.txt")
    random.seed(theSeed)

    part = []
    des = []


    #Parts and damage descriptions are each placed on their own list.
    for x in shipPart:
        part.append(x.strip())
    for x in miscResults:
        des.append(x.strip())
    for x in damageDescription:
        des.append(x.strip())
        
    shipPart.close()
    damageDescription.close()
    miscResults.close()

    #Randomly determine Suffix and Prefix
    randomP = random.randint(0, len(part) -1)
    randomD = random.randint(0, len(des) -1)

    result = 'Your ' + name + ' ' + des[randomP] + ' ' + ' the ' + target + "'s " + part[randomP] 
    return result

#Testing Code
#generateWeaponResult('Machine Gun', 'Leto', 0)
#generateWeaponResult('Energy Laser'. Bobo, 1, 100)
