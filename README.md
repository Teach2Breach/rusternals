# rusternals
A rust code notebook for working with windows internals.

## Projects

Each project demonstrates the usage of a specific Windows Native API function:

1. [RtlQueryEnvironmentVariable_U](RtlQueryEnvironmentVariable_U/) - Query environment variables using the Windows Native API
   - Documentation: [RtlQueryEnvironmentVariable_U](https://undocumented-ntinternals.github.io/UserMode/Undocumented%20Functions/Executable%20Images/Environment/RtlQueryEnvironmentVariable_U.html)
   - Function: `RtlQueryEnvironmentVariable_U` - Retrieves the value of an environment variable

2. [LdrQueryProcessModuleInformation](LdrQueryProcessModuleInformation/) - Query process module information
   - Documentation: [LdrQueryProcessModuleInformation](https://undocumented-ntinternals.github.io/UserMode/Undocumented%20Functions/Executable%20Images/LdrQueryProcessModuleInformation.html)
   - Function: `LdrQueryProcessModuleInformation` - Retrieves information about loaded modules in a process

3. [NtSetTimer](NtSetTimer/) - Set a timer using the Windows Native API
   - Documentation: [NtSetTimer](https://undocumented-ntinternals.github.io/UserMode/Undocumented%20Functions/NT%20Objects/Timer/NtSetTimer.html)
   - Function: `NtSetTimer` - Creates or sets a timer object

4. [NtDebugActiveProcess](NtDebugActiveProcess/) - Debug a process using the Windows Native API
   - Documentation: [NtDebugActiveProcess](http://undocumented.ntinternals.net/UserMode/Undocumented%20Functions/NT%20Objects/DebugObject/NtDebugActiveProcess.html)
   - Function: `NtDebugActiveProcess` - Attaches a debug object to a process for debugging
   - Requires: SeDebugPrivilege to be enabled in the calling process token

Each project includes:
- Example usage
- Documentation
- Build and run instructions
- Sample output
