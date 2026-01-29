
# Rust Attendance & Employee Management System
A desktop application built with Rust that utilizes the egui framework for its interface and SQLite for data persistence. This system features a unique geolocation-based attendance tracker that ensures employees are physically present within a designated "work zone" to log their hours.

# Key Features
Geofenced Attendance: * Automatically fetches the user's IP-based location.

Verifies if the user is within a predefined Square (latitude/longitude boundaries).
Logs "Work Time" in seconds only when the user is inside the geofence.

Attendance States: Tracks three distinct statuses:
Fully Attended: Met the daily required time.
Partially Attended: Was present but didn't meet the full requirement.
Absence: Did not show up during the shift.
Employee Profiles: View detailed user information including Department, Role, Age, and historical attendance stats.
Auth System: Secure Login and Registration integrated with a local SQLite database.
Modern Desktop UI: Built using eframe and egui for a high-performance, immediate-mode graphical interface.


Language: Rust
UI Framework: eframe / egui
Database: rusqlite (SQLite)
Time Handling: chrono
Networking: reqwest & serde (for location API calls)
Async Runtime: tokio
