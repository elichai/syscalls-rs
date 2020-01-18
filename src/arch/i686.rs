#![allow(dead_code)]

#[inline]
#[rustfmt::skip]
pub unsafe fn syscall0(n: isize) -> isize {
    let mut ret: usize;
    asm! {"int $$0x80": "={eax}"(ret) : "{eax}"(n) : "cc",  "memory" : "volatile"};
    ret as isize
}

#[inline]
#[rustfmt::skip]
pub unsafe fn syscall1(n: isize, a1: isize) -> isize {
    let mut ret: usize;
    asm! {"int $$0x80": "={eax}"(ret) : "{eax}"(n), "{ebx}"(a1) : "cc",  "memory" : "volatile"};
    ret as isize
}

#[inline]
#[rustfmt::skip]
pub unsafe fn syscall2(n: isize, a1: isize, a2: isize) -> isize {
    let mut ret: usize;
    asm! {"int $$0x80": "={eax}"(ret) : "{eax}"(n), "{ebx}"(a1) , "{ecx}"(a2)
          : "cc",  "memory" : "volatile"};
    ret as isize
}

#[inline]
#[rustfmt::skip]
pub unsafe fn syscall3(n: isize, a1: isize, a2: isize, a3: isize) -> isize {
    let mut ret: usize;
    asm! {"int $$0x80": "={eax}"(ret) : "{eax}"(n), "{ebx}"(a1) , "{ecx}"(a2),
          "{edx}"(a3) : "cc",  "memory" : "volatile"};
    ret as isize
}

#[inline]
#[rustfmt::skip]
pub unsafe fn syscall4(n: isize, a1: isize, a2: isize, a3: isize, a4: isize) -> isize {
    let mut ret: usize;
    asm! {"int $$0x80": "={eax}"(ret) : "{eax}"(n), "{ebx}"(a1) , "{ecx}"(a2),
          "{edx}"(a3), "{esi}"(a4) : "cc",  "memory" : "volatile"};
    ret as isize
}

#[inline]
#[rustfmt::skip]
pub unsafe fn syscall5(n: isize, a1: isize, a2: isize, a3: isize, a4: isize, a5: isize) -> isize {
    let mut ret: usize;
    asm! {"int $$0x80": "={eax}"(ret) : "{eax}"(n), "{ebx}"(a1) , "{ecx}"(a2),
          "{edx}"(a3), "{esi}"(a4), "{edi}"(a5) : "cc",  "memory" : "volatile"};
    ret as isize
}

#[inline]
#[rustfmt::skip]
pub unsafe fn syscall6(n: isize, a1: isize, a2: isize, a3: isize, a4: isize, a5: isize, a6: isize) -> isize {
    let mut ret: usize;
    asm! {"int $$0x80": "={eax}"(ret) : "{eax}"(n), "{ebx}"(a1) , "{ecx}"(a2),
          "{edx}"(a3), "{esi}"(a4), "{edi}"(a5), "{ebp}"(a6) : "cc",  "memory" : "volatile"};
    ret as isize
}

#[allow(dead_code)]
pub enum Syscalls {
    RestartSyscall = 0,
    Exit = 1,
    Fork = 2,
    Read = 3,
    Write = 4,
    Open = 5,
    Close = 6,
    Waitpid = 7,
    Creat = 8,
    Link = 9,
    Unlink = 10,
    Execve = 11,
    Chdir = 12,
    Time = 13,
    Mknod = 14,
    Chmod = 15,
    Lchown = 16,
    Break = 17,
    Oldstat = 18,
    Lseek = 19,
    Getpid = 20,
    Mount = 21,
    Umount = 22,
    Setuid = 23,
    Getuid = 24,
    Stime = 25,
    Ptrace = 26,
    Alarm = 27,
    Oldfstat = 28,
    Pause = 29,
    Utime = 30,
    Stty = 31,
    Gtty = 32,
    Access = 33,
    Nice = 34,
    Ftime = 35,
    Sync = 36,
    Kill = 37,
    Rename = 38,
    Mkdir = 39,
    Rmdir = 40,
    Dup = 41,
    Pipe = 42,
    Times = 43,
    Prof = 44,
    Brk = 45,
    Setgid = 46,
    Getgid = 47,
    Signal = 48,
    Geteuid = 49,
    Getegid = 50,
    Acct = 51,
    Umount2 = 52,
    Lock = 53,
    Ioctl = 54,
    Fcntl = 55,
    Mpx = 56,
    Setpgid = 57,
    Ulimit = 58,
    Oldolduname = 59,
    Umask = 60,
    Chroot = 61,
    Ustat = 62,
    Dup2 = 63,
    Getppid = 64,
    Getpgrp = 65,
    Setsid = 66,
    Sigaction = 67,
    Sgetmask = 68,
    Ssetmask = 69,
    Setreuid = 70,
    Setregid = 71,
    Sigsuspend = 72,
    Sigpending = 73,
    Sethostname = 74,
    Setrlimit = 75,
    Getrlimit = 76,
    Getrusage = 77,
    Gettimeofday = 78,
    Settimeofday = 79,
    Getgroups = 80,
    Setgroups = 81,
    Select = 82,
    Symlink = 83,
    Oldlstat = 84,
    Readlink = 85,
    Uselib = 86,
    Swapon = 87,
    Reboot = 88,
    Readdir = 89,
    Mmap = 90,
    Munmap = 91,
    Truncate = 92,
    Ftruncate = 93,
    Fchmod = 94,
    Fchown = 95,
    Getpriority = 96,
    Setpriority = 97,
    Profil = 98,
    Statfs = 99,
    Fstatfs = 100,
    Ioperm = 101,
    Socketcall = 102,
    Syslog = 103,
    Setitimer = 104,
    Getitimer = 105,
    Stat = 106,
    Lstat = 107,
    Fstat = 108,
    Olduname = 109,
    Iopl = 110,
    Vhangup = 111,
    Idle = 112,
    Vm86old = 113,
    Wait4 = 114,
    Swapoff = 115,
    Sysinfo = 116,
    Ipc = 117,
    Fsync = 118,
    Sigreturn = 119,
    Clone = 120,
    Setdomainname = 121,
    Uname = 122,
    ModifyLdt = 123,
    Adjtimex = 124,
    Mprotect = 125,
    Sigprocmask = 126,
    CreateModule = 127,
    InitModule = 128,
    DeleteModule = 129,
    GetKernelSyms = 130,
    Quotactl = 131,
    Getpgid = 132,
    Fchdir = 133,
    Bdflush = 134,
    Sysfs = 135,
    Personality = 136,
    AfsSyscall = 137,
    Setfsuid = 138,
    Setfsgid = 139,
    Llseek = 140,
    Getdents = 141,
    Newselect = 142,
    Flock = 143,
    Msync = 144,
    Readv = 145,
    Writev = 146,
    Getsid = 147,
    Fdatasync = 148,
    Sysctl = 149,
    Mlock = 150,
    Munlock = 151,
    Mlockall = 152,
    Munlockall = 153,
    SchedSetparam = 154,
    SchedGetparam = 155,
    SchedSetscheduler = 156,
    SchedGetscheduler = 157,
    SchedYield = 158,
    SchedGetPriorityMax = 159,
    SchedGetPriorityMin = 160,
    SchedRrGetInterval = 161,
    Nanosleep = 162,
    Mremap = 163,
    Setresuid = 164,
    Getresuid = 165,
    Vm86 = 166,
    QueryModule = 167,
    Poll = 168,
    Nfsservctl = 169,
    Setresgid = 170,
    Getresgid = 171,
    Prctl = 172,
    RtSigreturn = 173,
    RtSigaction = 174,
    RtSigprocmask = 175,
    RtSigpending = 176,
    RtSigtimedwait = 177,
    RtSigqueueinfo = 178,
    RtSigsuspend = 179,
    Pread64 = 180,
    Pwrite64 = 181,
    Chown = 182,
    Getcwd = 183,
    Capget = 184,
    Capset = 185,
    Sigaltstack = 186,
    Sendfile = 187,
    Getpmsg = 188,
    Putpmsg = 189,
    Vfork = 190,
    Ugetrlimit = 191,
    Mmap2 = 192,
    Truncate64 = 193,
    Ftruncate64 = 194,
    Stat64 = 195,
    Lstat64 = 196,
    Fstat64 = 197,
    Lchown32 = 198,
    Getuid32 = 199,
    Getgid32 = 200,
    Geteuid32 = 201,
    Getegid32 = 202,
    Setreuid32 = 203,
    Setregid32 = 204,
    Getgroups32 = 205,
    Setgroups32 = 206,
    Fchown32 = 207,
    Setresuid32 = 208,
    Getresuid32 = 209,
    Setresgid32 = 210,
    Getresgid32 = 211,
    Chown32 = 212,
    Setuid32 = 213,
    Setgid32 = 214,
    Setfsuid32 = 215,
    Setfsgid32 = 216,
    PivotRoot = 217,
    Mincore = 218,
    Madvise = 219,
    Getdents64 = 220,
    Fcntl64 = 221,
    Gettid = 224,
    Readahead = 225,
    Setxattr = 226,
    Lsetxattr = 227,
    Fsetxattr = 228,
    Getxattr = 229,
    Lgetxattr = 230,
    Fgetxattr = 231,
    Listxattr = 232,
    Llistxattr = 233,
    Flistxattr = 234,
    Removexattr = 235,
    Lremovexattr = 236,
    Fremovexattr = 237,
    Tkill = 238,
    Sendfile64 = 239,
    Futex = 240,
    SchedSetaffinity = 241,
    SchedGetaffinity = 242,
    SetThreadArea = 243,
    GetThreadArea = 244,
    IoSetup = 245,
    IoDestroy = 246,
    IoGetevents = 247,
    IoSubmit = 248,
    IoCancel = 249,
    Fadvise64 = 250,
    ExitGroup = 252,
    LookupDcookie = 253,
    EpollCreate = 254,
    EpollCtl = 255,
    EpollWait = 256,
    RemapFilePages = 257,
    SetTidAddress = 258,
    TimerCreate = 259,
    TimerSettime = 260,
    TimerGettime = 261,
    TimerGetoverrun = 262,
    TimerDelete = 263,
    ClockSettime = 264,
    ClockGettime = 265,
    ClockGetres = 266,
    ClockNanosleep = 267,
    Statfs64 = 268,
    Fstatfs64 = 269,
    Tgkill = 270,
    Utimes = 271,
    Fadvise6464 = 272,
    Vserver = 273,
    Mbind = 274,
    GetMempolicy = 275,
    SetMempolicy = 276,
    MqOpen = 277,
    MqUnlink = 278,
    MqTimedsend = 279,
    MqTimedreceive = 280,
    MqNotify = 281,
    MqGetsetattr = 282,
    KexecLoad = 283,
    Waitid = 284,
    AddKey = 286,
    RequestKey = 287,
    Keyctl = 288,
    IoprioSet = 289,
    IoprioGet = 290,
    InotifyInit = 291,
    InotifyAddWatch = 292,
    InotifyRmWatch = 293,
    MigratePages = 294,
    Openat = 295,
    Mkdirat = 296,
    Mknodat = 297,
    Fchownat = 298,
    Futimesat = 299,
    Fstatat64 = 300,
    Unlinkat = 301,
    Renameat = 302,
    Linkat = 303,
    Symlinkat = 304,
    Readlinkat = 305,
    Fchmodat = 306,
    Faccessat = 307,
    Pselect6 = 308,
    Ppoll = 309,
    Unshare = 310,
    SetRobustList = 311,
    GetRobustList = 312,
    Splice = 313,
    SyncFileRange = 314,
    Tee = 315,
    Vmsplice = 316,
    MovePages = 317,
    Getcpu = 318,
    EpollPwait = 319,
    Utimensat = 320,
    Signalfd = 321,
    TimerfdCreate = 322,
    Eventfd = 323,
    Fallocate = 324,
    TimerfdSettime = 325,
    TimerfdGettime = 326,
    Signalfd4 = 327,
    Eventfd2 = 328,
    EpollCreate1 = 329,
    Dup3 = 330,
    Pipe2 = 331,
    InotifyInit1 = 332,
    Preadv = 333,
    Pwritev = 334,
    RtTgsigqueueinfo = 335,
    PerfEventOpen = 336,
    Recvmmsg = 337,
    FanotifyInit = 338,
    FanotifyMark = 339,
    Prlimit64 = 340,
    NameToHandleAt = 341,
    OpenByHandleAt = 342,
    ClockAdjtime = 343,
    Syncfs = 344,
    Sendmmsg = 345,
    Setns = 346,
    ProcessVmReadv = 347,
    ProcessVmWritev = 348,
    Kcmp = 349,
    FinitModule = 350,
    SchedSetattr = 351,
    SchedGetattr = 352,
    Renameat2 = 353,
    Seccomp = 354,
    Getrandom = 355,
    MemfdCreate = 356,
    Bpf = 357,
    Execveat = 358,
    Socket = 359,
    Socketpair = 360,
    Bind = 361,
    Connect = 362,
    Listen = 363,
    Accept4 = 364,
    Getsockopt = 365,
    Setsockopt = 366,
    Getsockname = 367,
    Getpeername = 368,
    Sendto = 369,
    Sendmsg = 370,
    Recvfrom = 371,
    Recvmsg = 372,
    Shutdown = 373,
    Userfaultfd = 374,
    Membarrier = 375,
    Mlock2 = 376,
    CopyFileRange = 377,
    Preadv2 = 378,
    Pwritev2 = 379,
    PkeyMprotect = 380,
    PkeyAlloc = 381,
    PkeyFree = 382,
    Statx = 383,
    ArchPrctl = 384,
    IoPgetevents = 385,
    Rseq = 386,
    Semget = 393,
    Semctl = 394,
    Shmget = 395,
    Shmctl = 396,
    Shmat = 397,
    Shmdt = 398,
    Msgget = 399,
    Msgsnd = 400,
    Msgrcv = 401,
    Msgctl = 402,
    ClockGettime64 = 403,
    ClockSettime64 = 404,
    ClockAdjtime64 = 405,
    ClockGetresTime64 = 406,
    ClockNanosleepTime64 = 407,
    TimerGettime64 = 408,
    TimerSettime64 = 409,
    TimerfdGettime64 = 410,
    TimerfdSettime64 = 411,
    UtimensatTime64 = 412,
    Pselect6Time64 = 413,
    PpollTime64 = 414,
    IoPgeteventsTime64 = 416,
    RecvmmsgTime64 = 417,
    MqTimedsendTime64 = 418,
    MqTimedreceiveTime64 = 419,
    SemtimedopTime64 = 420,
    RtSigtimedwaitTime64 = 421,
    FutexTime64 = 422,
    SchedRrGetIntervalTime64 = 423,
    PidfdSendSignal = 424,
    IoUringSetup = 425,
    IoUringEnter = 426,
    IoUringRegister = 427,
    OpenTree = 428,
    MoveMount = 429,
    Fsopen = 430,
    Fsconfig = 431,
    Fsmount = 432,
    Fspick = 433,
    PidfdOpen = 434,
    Clone3 = 435,
}