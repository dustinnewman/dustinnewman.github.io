---
title: "HFS+ to ext4"
date: 2023-11-13T22:34:17-08:00
draft: false
---

This is a straight-forward guide to copy a MacOS HFS+ filesystem partition to a Linux ext4 partition. When I looked online, I saw constant warnings and cautionary tales about how this is an inherently dangerous operation and you should never perform it. As I had a 5 TB hard drive sitting right in front me with only 1.2 TB used, the thought of purchasing a new hard drive because some risk is involved was ludicrous. After all, we are technologists. Transferring files from one file system to another should not raise so much handwringing.

That said, here is your one and only warning if you decide to follow this guide. Any time you work with important data (and I assume any data you follow this guide for is important), you should have a backup. Even in `$CURRENT_YEAR`, there is always a risk with data corruption or some hardware failure which prevents successful data transfer. If you do not make a backup, you have to be okay with total data loss. If you have any further objections than this, I assume you then know enough to do it a better and safer way. Note that you must have at least as much free space on your drive as you do used space e.g. if you are using 1 TB of data, you must have 1 TB free as well. With that out of the way, let's get started.

## On your Mac

First, plug in the hard drive to your Mac. You should see your disk show up in the output of the `diskutil list` command.

```sh
diskutil list

/dev/disk0 (internal, physical):
   #:                       TYPE NAME                    SIZE       IDENTIFIER
...
/dev/disk6 (external, physical):
   #:                       TYPE NAME                    SIZE       IDENTIFIER
   0:      GUID_partition_scheme                        *5.0 TB     disk6
   1:                        EFI EFI                     209.7 MB   disk6s1
   2:                  Apple_HFS SEAGATE                 5.0 TB     disk6s2
```

In my case, I know I have a 5 TB hard drive, so it is listed as `disk6`. We want to specifically deal with the `disk6s2` partition. However, your disk will of course be different from mine. To avoid confusion, set the following shell variable, replacing the value with your disk identifier:

```sh
export DISK_ID=<YOUR DISK PARTITION>
# e.g. export DISK_ID=disk6s2
```

In my case, the entire 5 TB was allocated for the HFS+ file system. This is not going to work because I need two equal portions to transfer over the files: the existing one for HFS, and another one greater than or equal in size for ext4. So we have to resize the existing partition. If you have more than half your hard drive already used, **do not do this**. **You will lose your data.**

For disk utility to allow us to resize, we have to make sure journaling is enabled.

```sh
diskutil enableJournal $DISK_ID
```

And now we can resize. In my case, I only had 1.2 TB occupied, so I will resize to 2 TB, making sure that we have enough space for the ext4 partition while not resulting in data loss of the existing HFS+ data.

```sh
diskutil resizeVolume disk6s2 2T

/dev/disk6 (external, physical):
   #:                       TYPE NAME                    SIZE       IDENTIFIER
   0:      GUID_partition_scheme                        *5.0 TB     disk6
   1:                        EFI EFI                     209.7 MB   disk6s1
   2:                  Apple_HFS SEAGATE                 2.0 TB     disk6s2
                    (free space)                         3.0 TB     -
```

Now we need to disable journaling.

```sh
diskutil disableJournal disk6s2
```

And we can safely eject the drive. Note that we eject the entire disk, not the partition. Please replace the following with your actual disk.

```sh
diskutil eject <YOUR DISK>
# e.g. diskutil eject disk6
```

It is now time to unplug the disk from your Mac and plug it into your Linux device.

## On Linux

As always, make sure your system is up to date.

```sh
sudo apt-get update
```

We need the `hfsprogs` program to work with the HFS file system on Linux systems. As its [package page](https://packages.ubuntu.com/jammy/hfsprogs) states, `hfsprogs` is "mkfs and fsck for HFS and HFS+ file systems".

```sh
sudo apt-get install hfsprogs
```

You now need to get the disk information for your drive.

```sh
sudo fdisk -l
Device      Start        End    Sectors  Size Type
/dev/sda1      34     409633     409600  200M EFI System
/dev/sda2  411648 9767541133 9767129486  2T   HFS / HFS+
```

For this portion, I even more highly recommend using shell variables to avoid mistakes since we are actively working with two different filesystems here, whereas before we only had one.

```sh
export HFS_DISK_ID=/dev/sda2
```

We check the integrity of the HFS+ filesystem with the `fsck` from `hfsprogs` (`fsck.hfsplus`) and then unmount it, since it is never a good idea to operate on a mounted filesystem.

```sh
sudo fsck.hfsplus $HFS_DISK_ID
sudo umount $HFS_DISK_ID
```

Now time to finally create the new ext4 partition! We do this with the `fdisk` command. `fdisk` is used to manage disk partitions on Linux systems. It will open up a set of prompts requiring your input. I will list what to enter for each prompt.

```sh
sudo fdisk /dev/sda
```

- Command should be `n` for new partition
- Partition number should be next after HFS partition, in my case, 3
- Start can be default to start right after existing HFS partition
- Size depends on your specifics. For me, it should be `+2T` for 2 TB
- `w` to write the command to disk

Woohoo! Our partition is created. Store it in an environment variable.

```sh
export EXT_DISK_ID=sda3
```

And now we need to actually make this partition use the ext4 filesystem.

```sh
sudo mkfs.ext4 $EXT_DISK_ID
```

In order to do anything with these filesystems, we need to mount them. The mount directory paths can be anything you want, but for simplicity I'd recommend just following these unless you have a good reason. These are where the environment variables really come in handy to avoid mixups.

```sh
sudo mkdir /mnt/hfsplus
sudo mkdir /mnt/ext4
sudo mount -t hfsplus -o force,rw $HFS_DISK_ID /mnt/hfsplus
sudo mount $EXT_DISK_ID /mnt/ext4
```

Now the actual transfer. For this, I used `rsync`. Note that for my 1.2 TB, this transfer took 20 hours. The extended length of this transfer also increases the likelihood of hardware failure, network failure (which happened to me twice during this process), or power supply issues. Please take care to stabilize your system as much as possible.

```sh
sudo rsync -av /mnt/hfsplus/ /mnt/ext4/
```
