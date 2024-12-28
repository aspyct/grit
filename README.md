# grit - GR Image Transfer

`grit` is a utility to download pictures from Ricoh GR cameras, and is also compatible with other recent Ricoh/Pentax cameras like the K3 III.

## Quickstart

First, install `grit` with cargo:

```
$ cargo install grit
```

1. Enable wifi on your camera (on GRIII: Menu > Setup > 6. Wireless Connection > Wireless Lan Settings > Action Mode)
2. Connect to the camera wifi with your computer (Wireless Lan Settings > Communication Info)
3. Run `grit` from the directory where you want to store pictures

## How does this work?

Most recent cameras have built-in wifi access point capabilities, and embed a web server.

The Ricoh GR acts as a wifi access point, and serves content on http://192.168.0.1 . Most importantly, there is http://192.168.0.1/v1/photos which will list all the pictures present on the device. From there, you can download the pictures at http://192.168.0.1/v1/photos/{folder}/{photo}. Easy!

Thanks Ricoh <3 Keep rockin'

## Why would you use this?

I find that the process of removing the SD card from the camera and plugging it into a laptop, while a simple thing, does not always fit in my day. The mobile application, while technically working, isn't the smoothest way to export the pictures either.

The end goal of this project is to have a raspberry pi listening for live cameras and backing up pictures automatically. But before I do that, I need a way to extract the pictures from the cameras.

So here's that part of the project :)

## Supported Cameras

I tested `grit` with the following cameras:

- Ricoh GR IIIx
- Ricoh GR III HDF
- Pentax K3 III Monochrome

If you have other wifi-enabled Ricoh/Pentax cameras, additional testing is welcome :)

## License?

GPLv3. No warranty, use for free, don't resell, yada yada.
