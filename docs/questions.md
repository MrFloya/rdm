# rdm - questions.md

## .Xauthority
- Generated by dm (either by `xauth` or directly)
- Passed to X
- Persisted in `%HOME/.Xauthority` to allow access for apps

## Dbus

## PAM
- rdm
    - `system-login`?
- rdm-greeter
    - Opens sessions -> "`-session optional pam_systemd.so`"
    - sddm: Cannot change password?

## Systemd
- What are the requirements?
    - `pam_systemd.so` does not seem to be enough
- How to properly validate?
    - Existence of user run directory not sufficient