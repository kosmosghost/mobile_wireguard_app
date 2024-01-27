#!/bin/bash

echo "DO NOT use as root!"

mkdir /usr/share/icons/kg

cp mobile_wireguard_app.svg /usr/share/icons/kg/

cp org.kg.mobile_wireguard_app.desktop /usr/share/applications/

cp target/release/mobile_wireguard_app /bin
