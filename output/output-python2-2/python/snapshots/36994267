#!/usr/bin/python

import commands

machines = []
machines.append("whip01")
machines.append("whip02")
machines.append("whip03")
machines.append("whip04")
#machines.append("whip05")
machines.append("whip06")
machines.append("whip07")
machines.append("whip08")
machines.append("whip09")
machines.append("whip10")

minload = 5.0
minmach = machines[0]
for machine in machines:
	print 'Checking machine: ',machine
	exe = "ssh " + machine + " uptime"
	ans = commands.getoutput(exe)
	cols = ans.split("average:")
	if len(cols)>1:
		inter = cols[1].split(",")
		load = float(inter[0])
		print '   LOAD: ',load
		if load < minload:
			minload = load
			minmach = machine
	else:
		print '  PROBLEM!'
print
print 'BEST MACHINE:',
print minmach
