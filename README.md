# netcounter
Even Linux distros have grown data hungry. Unless you build from the source, the images you download and install could very easily 
come preconfigured to hide some network connections from you (i.e. rootkited).

Netcounter is a dumb script to test this hypothesis.

Install the target OS in a VM (e.g. Virtualbox) and run the script in both the VM and the Host. 

Do your daily or not-so-daily stuff in the VM, and then compare the IP addresses the VM claimed it was connected to, with the connections that actually came from it.

Don't forget to adjust the VM's version to skip the "grep" step.
