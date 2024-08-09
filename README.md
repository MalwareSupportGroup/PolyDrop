# BYOSI

## - Bring-Your-Own-Script-Interpreter

### - Leveraging the abuse of trusted applications, one is able to deliver a compatible script interpreter for a Windows, Mac, or Linux system as well as malicious source code in the form of the specific script interpreter of choice. Once both the malicious source code and the trusted script interpeter are safely written to the target system, one could simply execute said source code via the trusted script interpreter. 

# PolyDrop

## - Leverages thirteen scripting languages to perform the above attack.

The following langues are wholly ignored by AV vendors including MS-Defender:
- tcl
- php
- crystal
- julia
- golang
- dart
- dlang
- vlang
- nodejs
- bun
- python
- fsharp
- deno

All of these languages were allowed to completely execute, and establish a reverse shell by MS-Defender. We assume the list is even longer, given that languages such as PHP are considered "dead" languages.


## - Currently undetectable by most mainstream Endpoint-Detection & Response vendors.

The total number of vendors that are unable to scan or process just PHP file types is 14, and they are listed below:

- Alibaba
- Avast-Mobile
- BitDefenderFalx
- Cylance
- DeepInstinct
- Elastic
- McAfee Scanner
- Palo Alto Networks
- SecureAge
- SentinelOne (Static ML)
- Symantec Mobile Insight
- Trapmine
- Trustlook
- Webroot


And the total number of vendors that are unable to accurately identify malicious PHP scripts is 54, and they are listed below:

- Acronis (Static ML)
- AhnLab-V3
- ALYac
- Antiy-AVL
- Arcabit
- Avira (no cloud)
- Baidu
- BitDefender
- BitDefenderTheta
- ClamAV
- CMC
- CrowdStrike Falcon
- Cybereason
- Cynet
- DrWeb
- Emsisoft
- eScan
- ESET-NOD32
- Fortinet
- GData
- Gridinsoft (no cloud)
- Jiangmin
- K7AntiVirus
- K7GW
- Kaspersky
- Lionic
- Malwarebytes
- MAX
- MaxSecure
- NANO-Antivirus
- Panda
- QuickHeal
- Sangfor Engine Zero
- Skyhigh (SWG)
- Sophos
- SUPERAntiSpyware
- Symantec
- TACHYON
- TEHTRIS
- Tencent
- Trellix (ENS)
- Trellix (HX)
- TrendMicro
- TrendMicro-HouseCall
- Varist
- VBA32
- VIPRE
- VirIT
- ViRobot
- WithSecure
- Xcitium
- Yandex
- Zillya
- ZoneAlarm by Check Point
- Zoner

With this in mind, and the absolute shortcomings on identifying PHP based malware we came up with the theory that the 13 identified languages are also an oversight by these vendors, including CrowdStrike, Sentinel1, Palo Alto, Fortinet, etc.
We have been able to identify that at the very least Defender considers these obviously malicious payloads as plaintext.

## Disclaimer

We as the maintainers, are in no way responsible for the misuse or abuse of this product. This was published for legitimate penetration testing/red teaming purposes, and for educational value. Know the applicable laws in your country of residence before using this script, and do not break the law whilst using this. Thank you and have a nice day.

## EDIT

In case you are seeing all of the default declarations, and wondering wtf guys. There is a reason; this was built to be more moduler for later versions. For now, enjoy the tool and feel free to post issues. They'll be addressed as quickly as possible. 
