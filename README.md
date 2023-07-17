# Log Monitoring with Rust

This is a Rust program that monitors a log file and extracts the `remote_ip` from log lines that match certain criteria. It writes the extracted `remote_ip` values to a result file, ensuring that each `remote_ip` is recorded only once.

## Features

- Monitors a log file for changes.
- Extracts `remote_ip` from log lines that contain specific criteria.
- Writes the extracted `remote_ip` values to a result file.
- Ensures that each `remote_ip` is recorded only once.

## Requirements

- Rust (stable)
- Cargo

## Usage

1. Clone the repository:

   ```shell
   git clone https://github.com/oarinv/readlog.git
2. Navigate to the project directory:
   ```shell
   cd log-monitoring-rust
3. Modify the log_file_path and result_file_path variables in the main function of src/main.rs to specify the paths of your log file and result file.

4. Build and run the program:
   ```shell
   cargo run
The program will start monitoring the log file and write the extracted remote_ip values to the result file.

5. To stop the program, press Ctrl + C.

## Customization
- You can modify the criteria for matching log lines by editing the conditions in the if statement in the main function of src/main.rs.
- Additional processing logic can be added after writing each remote_ip value to the result file.
## License
This project is licensed under the MIT License.


