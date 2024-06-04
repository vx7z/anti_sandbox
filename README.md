# 🛡️ Anti-Sandbox Detection

Welcome to the **Anti-Sandbox Detection** project! This Rust-based tool is designed to help you identify whether your current environment is a virtualized or sandboxed environment. This can be particularly useful for security professionals and developers who want to ensure their applications are running in a genuine environment.

## ✨ Features

This project includes several sophisticated checks to detect virtualized or sandboxed environments:

1. **🔍 CPU Verification**:
   - Checks the number of processors in the system. If fewer than 2 processors are detected, the environment is flagged as potentially virtualized.

2. **💾 RAM Verification**:
   - Checks the total physical memory. If the memory is less than or equal to 2 GB, the environment is flagged as potentially virtualized.

3. **🔌 USB Devices Verification**:
   - Checks the number of USB storage devices connected to the system. If fewer than 2 USB devices are found, the environment is flagged as potentially virtualized.

4. **📊 Processes Verification**:
   - Checks the number of running processes. If the number of processes is less than or equal to 50, the environment is flagged as potentially sandboxed.

## 📦 Dependencies

This project leverages the following dependencies to perform its checks:
- `sysinfo`: For retrieving comprehensive system information such as the number of processes.
- `windows`: For accessing Windows-specific APIs to retrieve system and registry information.

## 🚀 Getting Started

To get started with this project, ensure you have Rust installed on your system. Then, follow these steps:

```sh
# Clone the repository
git clone https://github.com/yourusername/anti_sandbox.git

# Navigate to the project directory
cd anti_sandbox

# Build and run the project
cargo run
```

## 🖥️ Example Output

When you run the project, it will print messages indicating whether the environment is potentially virtualized or sandboxed based on the checks performed. For example:

```
[*] Possibly a virtualised environment
[*] Possibly a sandbox environment
```

## 🤝 Contributing

We welcome contributions from the community! If you have any improvements or new features to suggest, please open an issue or submit a pull request. Let's make this project even better together!

## 📜 License

This project is licensed under the MIT License. For more details, see the [LICENSE](LICENSE) file.

---

Feel free to customize this README further to better fit your project's specifics and your personal preferences. Happy coding! 🎉

