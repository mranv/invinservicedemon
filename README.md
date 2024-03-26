# Invinservicedemon

## Description
The Invinservicedemon repository contains a Rust project for a service helper tool that checks the installation status of various services such as osquery, Wazuh, and ClamAV. It provides a struct called `ServiceHelper` with methods to gather information about installed services and their status.

## What It Does
The Invinservicedemon tool performs the following tasks:
- Checks the installation status of osquery, Wazuh, and ClamAV services.
- Generates JSON data representing the status of these services.
- Useful for applications needing to display service status information.

## Usage
To use this project, clone the repository and run the Rust program. You can modify the `ServiceHelper` struct or its methods to suit your specific requirements for checking service installations.

## Installation
1. Clone the repository:
   ```
   git clone https://github.com/mranv/invinservicedemon.git
   ```
2. Navigate to the project directory:
   ```
   cd invinservicedemon
   ```
3. Run the Rust program:
   ```
   cargo run
   ```

## Service Status
The `ServiceHelper` struct checks the installation status of the following services:
- osquery: Checks for installation in `/usr/bin` directory.
- Wazuh: Checks for installation in `/var/ossec/bin` directory.
- ClamAV: Uses the 'which' command to check for installation.

## Contributing
Contributions are welcome! Feel free to fork the repository, make changes, and submit a pull request.

For more information, visit the [GitHub repository](https://github.com/mranv/invinservicedemon).
