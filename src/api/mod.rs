#[cfg(all(target_os = "linux", feature = "ksni"))]
mod linux_ksni;

#[cfg(all(target_os = "linux", feature = "libappindicator"))]
mod linux_libappindicator;

// Set type depending on OS and feature
#[cfg(all(target_os = "linux", feature = "ksni"))]
pub type TrayItemImpl = linux_ksni::TrayItemLinux;

#[cfg(all(target_os = "linux", feature = "libappindicator"))]
pub type TrayItemImpl = linux_libappindicator::TrayItemLinux;
