# Add Sudo Backdoor User (Linux)

Creates a new Linux user with sudo group membership as a persistent backdoor account.

- **Requirements:**
  - Admin/sudo access on the target
  - Replace `BACKDOOR_USERNAME` and `BACKDOOR_PASSWORD` with desired credentials
- **Note:** Works on Debian/Ubuntu. For RHEL/CentOS replace `sudo` group with `wheel`.
- **Access:** SSH as `BACKDOOR_USERNAME` or `su BACKDOOR_USERNAME` from a shell
