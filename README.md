# syscalls-rs
This library is an attempt to remove the libc dependency from rust with `x86_64-unknown-linux-bare` (and the same for the rest of tier 1 platforms)



# Open Questions
1. How much should we aim to make the syscalls "safe"?(i.e. not `unsafe fn`)
2. Should we provide syscalls only or library functions too? (only 2, or also 3?)
3. If not then where should (3) be defined/implemented?
4. How much should they be compatible with glibc? (i.e. threads?)
5. Should we provide enums for the flags? or should the flag stay ints?
6. Minimum Supported kernel version. how?. which?.
7. Should we have our own types? (i.e. FileDescriptor type) traits?.
8. fnctl: according to the man page `However, these details can be ignored by applications using glibc,whose fcntl() wrapper function transparently employs the more recentsystem call where it is available.`
looking at glibc code it does some work and then calls `fcntl64`. should we just only implement `fcntl64`? should we wrap that?
9. glibc uses `SYSCALL_CANCEL` which calls `LIBC_CANCEL_ASYNC` incase of multithreading. should we look into that? what are the implications of this?
10. Should we split some syscalls into multiple functions? i.e. we could split `fnctl` into a function where the third argument is(`*mut flock`) and one where it's an int(and one without a third argument).


## List of syscalls used in rust/src/libstd:
### Kernel Calls(2)
 - [ ] exit
 - [ ] ftruncate
 - [ ] pread
 - [ ] pwrite
 - [ ] gettimeofday
 - [ ] connect
 - [ ] getsockname
 - [ ] getpeername
 - [ ] bind
 - [ ] listen
 - [ ] recvfrom
 - [ ] sendto
 - [x] read
 - [ ] readv
 - [ ] pread64
 - [x] write
 - [ ] pwrite64
 - [ ] pwrite
 - [ ] fcntl
 - [ ] ioctl
 - [ ] close
 - [ ] stat64
 - [ ] fstat64
 - [ ] lstat64
 - [ ] ftruncate64
 - [ ] lseek64
 - [ ] dirent64
 - [ ] open64
 - [ ] fstatat64
 - [ ] stat
 - [ ] fstat
 - [ ] fstatat
 - [ ] lstat
 - [ ] dirent
 - [ ] open
 - [ ] fdatasync
 - [ ] fsync
 - [ ] fchmod
 - [ ] mkdir
 - [ ] unlink
 - [ ] rename
 - [ ] chmod
 - [ ] rmdir
 - [ ] readlink
 - [ ] symlink
 - [ ] link
 - [ ] syscall
 - [ ] signal
 - [ ] socket
 - [ ] socketpair
 - [ ] poll
 - [ ] accept
 - [ ] shutdown
 - [ ] chdir
 - [ ] getuid
 - [ ] getpid
 - [ ] getppid
 - [ ] pipe2
 - [ ] pipe
 - [ ] kill
 - [ ] fork
 - [ ] _exit
 - [ ] dup2
 - [ ] setgid
 - [ ] setgroups
 - [ ] setuid
 - [ ] waitpid
 - [ ] WIFEXITED
 - [ ] WEXITSTATUS
 - [ ] WTERMSIG
 - [ ] getrandom
 - [ ] sigaltstack
 - [ ] sigaction
 - [ ] mmap
 - [ ] munmap
 - [ ] sched_yield
 - [ ] prctl
 - [ ] nanosleep
 - [ ] mprotect
 - [ ] accept4


### glibc implemented(3)
 - [ ] strlen
 - [ ] unsetenv
 - [ ] setenv
 - [ ] getenv
 - [ ] environ(7)?
 - [ ] strerror_r
 - [ ] abort
 - [ ] malloc
 - [ ] calloc
 - [ ] free
 - [ ] realloc
 - [ ] memalign
 - [ ] posix_memalign
 - [ ] clock_gettime
 - [ ] dirfd
 - [ ] readdir
 - [ ] closedir
 - [ ] opendir
 - [ ] realpath
 - [ ] memchr
 - [ ] gai_strerror
 - [ ] res_init
 - [ ] getcwd
 - [ ] sysconf
 - [ ] getpwuid_r
 - [ ] gnu_get_libc_version
 - [ ] sigemptyset
 - [ ] sigaddset
 - [ ] execvp
 - [ ] posix_spawn_file_actions_destroy
 - [ ] posix_spawnattr_destroy
 - [ ] posix_spawnattr_init
 - [ ] posix_spawn_file_actions_init
 - [ ] posix_spawn_file_actions_adddup2
 - [ ] posix_spawnattr_setsigmask
 - [ ] posix_spawnattr_setsigdefault
 - [ ] posix_spawnattr_setflags
 - [ ] posix_spawnp
 - [ ] dlsym
 
 #### pthreads
  - [ ] pthread_condattr_init
  - [ ] pthread_condattr_setclock
  - [ ] pthread_cond_init
  - [ ] pthread_condattr_destroy
  - [ ] pthread_cond_signal
  - [ ] pthread_cond_broadcast
  - [ ] pthread_cond_wait
  - [ ] pthread_cond_timedwait
  - [ ] pthread_cond_destroy
  - [ ] pthread_mutexattr_init
  - [ ] pthread_mutexattr_settype
  - [ ] pthread_mutex_init
  - [ ] pthread_mutexattr_destroy
  - [ ] pthread_mutex_lock
  - [ ] pthread_mutex_unlock
  - [ ] pthread_mutex_trylock
  - [ ] pthread_mutex_destroy
  - [ ] pthread_sigmask
  - [ ] pthread_rwlock_rdlock
  - [ ] pthread_rwlock_tryrdlock
  - [ ] pthread_rwlock_wrlock
  - [ ] pthread_rwlock_trywrlock
  - [ ] pthread_rwlock_unlock
  - [ ] pthread_rwlock_destroy
  - [ ] pthread_key_create
  - [ ] pthread_setspecific
  - [ ] pthread_getspecific
  - [ ] pthread_key_delete
  - [ ] pthread_attr_setstacksize
  - [ ] pthread_attr_init
  - [ ] pthread_create
  - [ ] pthread_attr_destroy
  - [ ] pthread_set_name_np
  - [ ] pthread_setname_np
  - [ ] pthread_self
  - [ ] pthread_join
  - [ ] pthread_detach
  - [ ] pthread_getattr_np
  - [ ] pthread_attr_getguardsize
  - [ ] pthread_attr_getstack
  
  
 ### Other
  - [ ] sysctl