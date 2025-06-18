#![doc = include_str!("zz-strategy.md")]

/// Strategy for linux and similar. [FileSystem Hierarchy Standard](https://refspecs.linuxfoundation.org/FHS_3.0/fhs-3.0.pdf).
pub mod fhs;

/// Strategy for linux and similar for retrieving directories using
/// [XDG Base Directories](https://specifications.freedesktop.org/basedir-spec/latest/).
pub mod xdg;

/// [Unix-style project directories](https://unix.stackexchange.com/questions/21778/whats-so-special-about-directories-whose-names-begin-with-a-dot) containing everything. Mainly used for local in [`crate::Scoped`].
pub mod unix;

/// Strategy for windows. Using [Known Folder API](https://docs.microsoft.com/en-us/windows/win32/api/shlobj_core/nf-shlobj_core-shgetknownfolderpath) and [Windows known directories](https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_Profile)
pub mod windows;

// TODO: https://man.freebsd.org/cgi/man.cgi?query=hier&apropos=0&sektion=0&manpath=FreeBSD+8.2-RELEASE&format=html
