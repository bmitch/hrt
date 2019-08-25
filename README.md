# hrt - ham radio terminal

`hrt` is a simple terminal based application aimed at providing various tools for Amateur Radio operators.

## Config

Copy the example [hrt.toml](hrt.toml) config file to the home directory on your system (i.e. `~/.hrt.toml`) and set the config values for the commands you plan to use.

## Commands

### Callsign lookup

You will need to set your account info in `~/.hrt.toml` for [qrz](https://www.qrz.com) and/or [hamqth](https://www.hamqth.com) before attempting to do callsign lookups.

```bash
hrt call C4LLS1GN
```

```
C4LLS1GN (QRZ)
  Name: Bryce Johnston
  Location: Manhattan, KS, United States
  Class: G
```

Use alternative lookup source instead of the default specified in config:

```bash
hrt call C4LLS1GN -s hamqth
```

```
c4lls1gn (HamQTH)
  Name: Bryce Johnston
  Location: Manhattan, KS, United States
```

Call command arguments will be added to allow returning additional information.

### DXCC lookup

You will need to set your account info in `~/.hrt.toml` for [qrz](https://www.qrz.com) and/or [hamqth](https://www.hamqth.com) before attempting to do DXCC lookups.


DXCC lookup by callsign
```bash
hrt dxcc C4LLS1GN
```

DXCC lookup by entity code / adif number
```bash
hrt dxcc 291
```

Use alternative lookup source instead of the default specified in config:

```bash
hrt dxcc C4LLS1GN -s hamqth
```

## Contributing

Feature requests, bug reports, and pull requests are welcome on GitHub at https://github.com/beaorn/hrt.

## License

Licensed under the [MIT License](LICENSE.md).