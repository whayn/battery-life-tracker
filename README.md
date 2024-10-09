# Battery Health Tracker

This application tracks battery health and logs the data. Follow these steps to build the project and set up the scheduled task in Task Scheduler.

> [!IMPORTANT]
> This is a windows only project!

> [!NOTE]
> This project is a work in progress and is not yet complete. The instructions provided are for building the project and setting up the scheduled task in Task Scheduler.

## Setting Up

Follow these steps to manually download and set up the Battery Health Tracker task in Task Scheduler with the specified settings.

### Download Instructions

1. Download the executable from the [releases page](https://github.com/whayn/battery-life-tracker/releases/latest)
2. Extract the contents of the zip file to a directory of your choice.
3. Note the path to the executable, e.g., `C:\Users\user\AppData\Local\Battery Life Tracker\battery-health-tracker.exe`.
4. Note the path to the working directory, e.g., `C:\Users\user\AppData\Local\Battery Life Tracker`.
5. Proceed to the [Step-by-Step Instructions](#step-by-step-instructions) to set up the scheduled task in Task Scheduler.

### Step-by-Step Instructions

1. Press `Win + R`, type `taskschd.msc`, and press `Enter`.

2. In the right-hand Actions pane, click on `Create Task...`.

3. **General Tab**:

   - **Name**: Enter `Battery Health Tracker`.
   - **Description**: (Optional) Enter a description for the task.
   - **Security options**:
     - Select `Run whether user is logged on or not`.
     - Check `Run with highest privileges`.
   - **Configure for**: Select `Windows 10`.

4. **Triggers Tab**:

   - Click `New...` to create a new trigger.
   - **Begin the task**: Select `On a schedule`.
   - **Settings**: Select `Daily`.
   - **Start**: Set the start date and time (e.g., `10:30 AM`).
   - **Advanced settings**:
     - Check `Repeat task every` and set it to `30 minutes`.
     - Set `for a duration of` to `1 hour`.
     - Ensure `Enabled` is checked.
   - Click `OK`.

5. **Actions Tab**:

   - Click `New...` to create a new action.
   - **Action**: Select `Start a program`.
   - **Program/script**: Enter the path to your executable, e.g., `C:\Users\user\AppData\Local\Battery Life Tracker\battery-health-tracker.exe`.
   - **Start in (optional)**: Enter the working directory, e.g., `C:\Users\user\AppData\Local\Battery Life Tracker`.
   - Click `OK`.

6. **Conditions Tab**:

   - Uncheck `Start the task only if the computer is on AC power`.
   - Ensure other options are set as needed.

7. **Settings Tab**:

   - Check `Allow task to be run on demand`.
   - Check `Run task as soon as possible after a scheduled start is missed`.
   - Check `If the task fails, restart every` and set it to `30 minutes`.
   - Set `Attempt to restart up to` to `3 times`.
   - Check `Stop the task if it runs longer than` and set it to `1 hour`.
   - Ensure other options are set as needed.

8. Click `OK` to create the task (If prompted, enter your user credentials to save the task.)

## Building Instructions

1. **Clone the Repository**:

   - Open a terminal and clone the repository:
     ```sh
     git clone https://github.com/yourusername/battery-health-tracker.git
     cd battery-health-tracker
     ```

2. **Install Rust**:

   - If you don't have Rust installed, you can install it from [rust-lang.org](https://www.rust-lang.org/):
     ```sh
     curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
     ```

3. **Build the Project**:

   - Build the project in release mode:
     ```sh
     cargo build --release
     ```

4. **Locate the Executable**:
   - After building, the executable will be located in the `target/release` directory:
     ```sh
     target/release/battery-health-tracker.exe
     ```
