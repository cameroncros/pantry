# Pantry

This is a self-hosted NFC labelling system.

It is designed for use with NFC stickers such as:

- https://www.aliexpress.com/item/32814647380.html

By attaching the NFC stickers to containers,
you can use your phone to read and update the contents of the container.

# Usage

1. Run the docker-compose.yml script, and deploy the server.
2. Purchase NFC stickers, and attach to containers.
3. Use `http://[SERVER_ADDR]/write` page to write unique IDs to the NFC stickers.
    * Requires Chrome on Android
    * Alternatively, you can use (NFC tools)[https://play.google.com/store/apps/details?id=com.wakdev.wdnfc].
4. When you fill a container for the fridge, scan the container, and update the description.
5. To see a list of all your containers, use the `http://[SERVER_ADDR]/list` page.

# Contributing

I am open to contributions. Please keep the following goals in mind when suggesting changes/improvements:

* Speed - The web interface needs to load as fast as possible. Any changes that make the page load slower (excessive JS
  libraries etc) will likely not be accepted.
* Simplicity - The web interface needs to be as simple and intuitive as possible. Also, mobile first, as most people
  will use their phones to scan the NFC chips.

# Background

I originally started using QR codes, but couldn't find a way to reliably
and durably print QR codes onto the containers.
NFC tags seem to work quite a bit faster, and are much more durable,
but I am open to re-adding QR code functionality if required.
