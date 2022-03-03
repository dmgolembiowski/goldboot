## goldboot

`goldboot` simplifies building and deploying golden images to server or desktop
environments. This tool is unfinshed and should be used for testing only!

### CLI

The goldboot CLI builds and deploys golden images.

### Registry

The goldboot registry stores images similar to a docker registry.

### Boot environment

### Getting Started

First, create a directory which can later be added to version control:
```sh
mkdir WindowsMachine
cd WindowsMachine
```

Initialize the directory and choose a base profile to start with:
```sh
goldboot init Windows10
```

This will create `goldboot.json` which contains configuration options that might
need to be tweaked. For a Windows install, you'll need to supply your own install media
from Microsoft:

```json
"iso_url": "Win10_1803_English_x64.iso",
"iso_checksum": "sha1:08fbb24627fa768f869c09f44c5d6c1e53a57a6f"
```

Next, add some scripts to provision the install:

```sh
# Example provisioner script
echo 'Set-ItemProperty HKLM:\SYSTEM\CurrentControlSet\Control\Power\ -name HibernateEnabled -value 0' >disable_hibernate.ps1
```

And add it to the goldboot config in the order they should be executed:
```json
"provisioners": [
	{
		"type": "shell",
		"script": "disable_hibernate.ps1"
	}
]
```

Now, build the image:
```sh
goldboot build
```

And finally deploy it to a physical disk:
```sh
# THIS WILL OVERWRITE /dev/sdb! TAKE A BACKUP FIRST!
goldboot write WindowsMachine /dev/sdb
```