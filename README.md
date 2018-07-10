apu_tt (tt)
===========

Display intake timetable Asia Pacific University with open web services API.

Features
--------
- pretty table
- fully colored
- cache response
- offline request

Building
--------
tt is written in [Rust](https://rustup.rs/).

    $ git clone https://github.com/pickfire/apu_tt
    $ cd apu_tt
    $ cargo build --release
    $ ./target/release/tt
    Mon Jul 02  1450-1550  NEW  D-07-08                 CT075-3-2-DTM-L       SLM
    Tue Jul 03  1035-1235  NEW  Tech Lab 6-09           CT075-3-2-DTM-LAB     SLM
    Tue Jul 03  1730-2030  NEW  Auditorium 5 @ Level 3  MPU3113-HE(LS)        SUH
    Tue Jul 03  1730-2030  NEW  B-07-04                 MPU3173-MLY3 (FS)     RGA
    Wed Jul 04  1240-1340  NEW  B-08-08                 CT111-3-2-COMT-L      LGR
    Thu Jul 05  0830-0930  NEW  Auditorium 2 @ Level 6  CT046-3-2-SDM-L       SVC
    Thu Jul 05  0930-1030  NEW  Auditorium 5 @ Level 3  CT042-3-2-PSMOD-L     TKK
    Fri Jul 06  1035-1235  NEW  Tech Lab 4-03           CT038-3-2-OODJ-LAB-T  LKK
    Fri Jul 06  1445-1545  NEW  Auditorium 2 @ Level 6  CT105-3-2-PDT-L       KID
    Fri Jul 06  1545-1645  NEW  Auditorium 2 @ Level 6  CT038-3-2-OODJ-L      LKK

