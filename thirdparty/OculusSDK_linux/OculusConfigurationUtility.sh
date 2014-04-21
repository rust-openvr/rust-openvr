#!/bin/bash

#############################################################################
#
# Filename    : OculusConfigurationUtility.sh
# Content     : Linux file for starting the OculusConfigurationUtility app 
#               from the SDK package root.  It will determine the 
#               architechture of the system it is running on, and start the 
#               approprite executable
# Created     : 2013
# Authors     : Simon Hallam
#
# Instruction : Ensure that the OculusConfigurationUtility.sh has execute 
#               permissions.  Navigate to a command shell, enter:
#
#               ./OculusConfigurationUtility.sh
#
# Copyright   :   Copyright 2013 Oculus VR, Inc. All Rights reserved.
#
# Licensed under the Oculus VR SDK License Version 2.0 (the "License"); 
# you may not use the Oculus VR SDK except in compliance with the License, 
# which is provided at the time of installation or download, or which 
# otherwise accompanies this software in either electronic or hard copy form.
#
# You may obtain a copy of the License at
#
# http://www.oculusvr.com/licenses/LICENSE-2.0 
#
# Unless required by applicable law or agreed to in writing, the Oculus VR SDK 
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#
############################################################################

MACHINE_TYPE=`uname -m`
if [ ${MACHINE_TYPE} == 'x86_64' ]; then
  ./Tools/OculusConfigUtil/OculusConfigUtil_x86_64
elif [ ${MACHINE_TYPE} == 'i686' ]; then
  ./Tools/OculusConfigUtil/OculusConfigUtil_i386
else
  echo "The Oculus Configuration Utility does not currently support this platform."
fi

