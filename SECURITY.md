## Security warning
RSTransfer (`rst`) is intentionally **minimal** and **does not include** any security features.

This means:

- Anyone on the same network can sniff the file contents in plaintext, and can modify the file as they please.
- A malicious actor could send arbitrary data to the receiver.
- There is no way to validate that the file was received unmodified.

We strongly recommend that you:
- Use `rst` **only within trusted, isolated networks** (e.g., LAN, VM, Docker)
- Avoid usage across the public internet without encapsulation
- Consider tunneling through SSH or a VPN for safer communication.

