#!/usr/bin/python

from sys import argv

pdbfile_in = argv[1]
new_core_sequence_A = argv[2]
new_core_sequence_B = argv[3]

assert( len( new_core_sequence_A)  == 4 )
assert( len( new_core_sequence_B)  == 4 )
new_sequence = [new_core_sequence_A, new_core_sequence_B]

longer_names={
              'A':'B3A' , 'R':'B3R' , 'N':'B3N' , 'D':'B3D' ,
              'C':'B3C' , 'E':'B3E' , 'Q':'B3Q' , 'G':'B3G' ,
              'H':'B3H' , 'I':'B3I' , 'L':'B3L' , 'K':'B3K' ,
              'M':'B3M' , 'F':'B3F' , 'P':'B3P' , 'S':'B3S' ,
              'T':'B3T' , 'W':'B3W' , 'Y':'B3Y' , 'V':'B3V' ,
              'O':'B3O' , '5':'CP1' , '6':'CP2' ,
              '1':'XI1' , '2':'XI2'
              }

chain_A_or_B = [0,1,1,0,0,1,1,0]

lines = open(pdbfile_in).readlines()

chain_res_prev = ""

helix_period = 3
offset = 2

count = 0
chain = -1

OK_atoms = [' C  ',' CA ',' CM ',' N  ',' O  ',' CB ',' CG ']

newlines = []
for line in lines:
  if line[ :4 ] != 'ATOM': continue

  chain_res = line[21:26]
  if chain_res != chain_res_prev:
      if ( count % 12 == 0 ):
          chain += 1
          count = 0
      count += 1

  chain_res_prev = chain_res

  if ( count % helix_period == offset ):

    if ( chain < len( chain_A_or_B ) ):
      which_chain = chain_A_or_B[ chain ]
      register = count/helix_period

      aa = new_sequence[which_chain][register]
      newres = longer_names[ aa ]

      line = line[0:17] + newres + line[20:]

      atomname = line[12:16]
      if not (atomname in OK_atoms): continue

  newlines.append( line )
  print line[:-1]

