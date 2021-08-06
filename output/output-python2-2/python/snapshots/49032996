# -*- coding: utf-8 -*-
#
# «steps» - Steps management library
#
# Copyright (C) 2005 Junta de Andalucía
# Copyright (C) 2005, 2006 Canonical Ltd.
#
# Authors:
#
# - Jesús Espino <jespino@emergya.info>
#
# This file is part of Ubiquity.
#
# Ubiquity is free software; you can redistribute it and/or modify it under
# the terms of the GNU General Public License as published by the Free
# Software Foundation; either version 2 of the License, or at your option)
# any later version.
#
# Ubiquity is distributed in the hope that it will be useful, but WITHOUT
# ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
# FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for
# more details.
#
# You should have received a copy of the GNU General Public License along
# with Ubiquity; if not, write to the Free Software Foundation, Inc., 51
# Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
##################################################################################

import debconf
import os
try:
    from debconf import DebconfCommunicator
except ImportError:
    from ubiquity.debconfcommunicator import DebconfCommunicator

DEFAULT_PRESEED_FILE="/ubiquity.preseed"

class Steps:
    def __init__(self):
	db = DebconfCommunicator("ubiquity")

	# Run all debconf commands in preseed file
        if os.path.exists(DEFAULT_PRESEED_FILE):
	    preseed_file = os.file(DEFAULT_PRESEED_FILE)
	    for line in preseed_file:
		db.command(line)
	
	self.BREADCRUMB_STEPS_PRESEED = {
	    "stepLanguage": db.exist("ubiquity/stepLanguage")=="true" and db.get("ubiquity/stepLanguage")=="true",        
	    "stepLocation": db.exist("ubiquity/stepLocation")=="true" and db.get("ubiquity/stepLocation")=="true",        
	    "stepKeyboardConf": db.exist("ubiquity/stepKeyboardConf")=="true" and db.get("ubiquity/stepKeyboardConf")=="true",
	    "stepUserInfo": db.exist("ubiquity/stepUserInfo")=="true" and db.get("ubiquity/stepUserInfo")=="true",        
	    "stepPartDisk": db.exist("ubiquity/stepPartDisk")=="true" and db.get("ubiquity/stepPartDisk")=="true",        
	    "stepPartAuto": db.exist("ubiquity/stepPartAuto")=="true" and db.get("ubiquity/stepPartAuto")=="true",        
	    "stepPartAdvanced": db.exist("ubiquity/stepPartAdvanced")=="true" and db.get("ubiquity/stepPartAdvanced")=="true",
	    "stepPartMountpoints": db.exist("ubiquity/stepMountpoints")=="true" and db.get("ubiquity/stepMountpoints")=="true",  
	    "stepReady": db.exist("ubiquity/stepReady")=="true" and db.get("ubiquity/stepReady")=="true"               
	}
	db.shutdown()
	self.BREADCRUMB_STEPS = {
	    "stepLanguage": 1,
	    "stepLocation": 2,
	    "stepKeyboardConf": 3,
	    "stepUserInfo": 4,
	    "stepPartDisk": 5,
	    "stepPartAuto": 5,
	    "stepPartAdvanced": 5,
	    "stepPartMountpoints": 5,
	    "stepReady": 6
	}

    def step_preseeded(self,step):
	return self.BREADCRUMB_STEPS_PRESEED[step]

    def add_step(self,position,stepname):
        for step in [key for key in self.BREADCRUMB_STEPS.keys() if self.BREADCRUMB_STEPS[key]>=position]:
            BREADCRUMB_STEPS[step] += 1
        BREADCRUMB_STEPS[stepname] = position

    def get_total_steps(self):
	posible_values = []
	preseeded_steps = 0
	for step in self.BREADCRUMB_STEPS.keys():
	    if not self.step_preseeded(step) and self.BREADCRUMB_STEPS[step] not in posible_values:
		posible_values.append(self.BREADCRUMB_STEPS[step])

	total_steps = len(posible_values)
	
	return total_steps

    def get_curstep(self,stepname):
        if stepname in self.BREADCRUMB_STEPS.keys():
            curstep = self.BREADCRUMB_STEPS[stepname]
	previous_preseeded_steps = [step for step in self.BREADCRUMB_STEPS.keys() if self.BREADCRUMB_STEPS[step]<curstep and self.step_preseeded(step)]
	curstep -=len(previous_preseeded_steps)
	return str(curstep)

    def get_steps_list(self):
	posible_steps = {}
	for step in self.BREADCRUMB_STEPS.keys():
	    if not self.step_preseeded(step):
	        posible_steps[step] = self.BREADCRUMB_STEPS[step]

	steps = []
	while len(posible_steps)>0:
	    first = self.get_first_step(posible_steps)
	    steps.append(first)
	    posible_steps.pop(first)
        return steps

    def get_first_step(self,list_dict=[]):
	if len(list_dict)==0:
	    list_dict = self.BREADCRUMB_STEPS
	posible_steps = [step for step in list_dict.keys() if not self.step_preseeded(step)]
	min_steps = []
	for step in posible_steps:
	    if len(min_steps)==0 or list_dict[step]<list_dict[min_steps[0]]:
		min_steps = [step]
	    elif list_dict[step]==min_steps:
		min_steps.append(step)

	while len(min_steps)>1:
	    if min_steps[-1]=="setpPartMountpoints":
		min_steps.remove("stepPartMounpoints")
	    elif min_steps[-1]=="stepPartAdvanced":
		min_steps.remove("stepPartAdvanced")
	    elif min_steps[-1]=="stepPartAuto":
		min_steps.remove("stepPartAuto")
	    elif min_steps[-1]=="stepPartDisk":
		min_steps.remove("stepPartDisk")
	return min_steps[0]
