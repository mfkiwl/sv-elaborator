use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Keyword {
    // System task identifiers that are treated as keyword
    Unit,
    Root,

    // Verilog 95
    Always,
    And,
    Assign,
    Begin,
    Buf,
    Bufif0,
    Bufif1,
    Case,
    Casex,
    Casez,
    Cmos,
    Deassign,
    Default,
    Defparam,
    Disable,
    Edge,
    Else,
    End,
    Endcase,
    Endmodule,
    Endfunction,
    Endprimitive,
    Endspecify,
    Endtable,
    Endtask,
    Event,
    For,
    Force,
    Forever,
    Fork,
    Function,
    Highz0,
    Highz1,
    If,
    Ifnone,
    Initial,
    Inout,
    Input,
    Integer,
    Join,
    Large,
    Medium,
    Module,
    Nand,
    Negedge,
    Nmos,
    Nor,
    Not,
    Notif0,
    Notif1,
    Or,
    Output,
    Parameter,
    Pmos,
    Posedge,
    Primitive,
    Pull0,
    Pull1,
    Pullup,
    Pulldown,
    Rcmos,
    Real,
    Reg,
    Release,
    Repeat,
    Rnmos,
    Rpmos,
    Rtran,
    Rtranif0,
    Rtranif1,
    Scalared,
    Small,
    Specify,
    Specparam,
    Strong0,
    Strong1,
    Supply0,
    Supply1,
    Table,
    Task,
    Time,
    Tran,
    Tranif0,
    Tranif1,
    Tri,
    Tri0,
    Tri1,
    Triand,
    Trior,
    Trireg,
    Vectored,
    Wait,
    Wand,
    Weak0,
    Weak1,
    While,
    Wire,
    Wor,
    Xnor,
    Xor,

    // Verilog 01-noconfig
    Automatic,
    Endgenerate,
    Generate,
    Genvar,
    Localparam,
    Noshowcancelled,
    PulsestyleOndetect,
    PulsestyleOnevent,
    Showcancelled,
    Signed,
    Unsigned,

    // Verilog 01
    Cell,
    Config,
    Design,
    Endconfig,
    Incdir,
    Include,
    Instance,
    Liblist,
    Library,
    Use,

    // Verilog 05
    Uwire,

    // SV 05
    Alias,
    AlwaysComb,
    AlwaysFf,
    AlwaysLatch,
    Assert,
    Assume,
    Before,
    Bind,
    Bins,
    Binsof,
    Bit,
    Break,
    Byte,
    Chandle,
    Class,
    Clocking,
    Const,
    Constraint,
    Context,
    Continue,
    Cover,
    Covergroup,
    Coverpoint,
    Cross,
    Dist,
    Do,
    Endclass,
    Endclocking,
    Endgroup,
    Endinterface,
    Endpackage,
    Endprogram,
    Endproperty,
    Endsequence,
    Enum,
    Expect,
    Export,
    Extends,
    Extern,
    Final,
    FirstMatch,
    Foreach,
    Forkjoin,
    Iff,
    IgnoreBins,
    IllegalBins,
    Import,
    Inside,
    Int,
    Interface,
    Intersect,
    JoinAny,
    JoinNone,
    Local,
    Logic,
    Longint,
    Matches,
    Modport,
    New,
    Null,
    Package,
    Packed,
    Priority,
    Program,
    Property,
    Protected,
    Pure,
    Rand,
    Randc,
    Randcase,
    Randsequence,
    Ref,
    Return,
    Sequence,
    Shortint,
    Shortreal,
    Solve,
    Static,
    String,
    Struct,
    Super,
    Tagged,
    This,
    Throughout,
    Timeprecision,
    Timeunit,
    Type,
    Typedef,
    Union,
    Unique,
    Var,
    Virtual,
    Void,
    WaitOrder,
    Wildcard,
    With,
    Within,

    // SV 09
    AcceptOn,
    Checker,
    Endchecker,
    Eventually,
    Global,
    Implies,
    Let,
    Nexttime,
    RejectOn,
    Restrict,
    SAlways,
    SEventually,
    SNexttime,
    SUntil,
    SUntilWith,
    Strong,
    SyncAcceptOn,
    SyncRejectOn,
    Unique0,
    Until,
    UntilWith,
    Untyped,
    Weak,

    // SV 12
    Implements,
    Interconnect,
    Nettype,
    Soft,

    // SV 17
    // No new keywords
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = match self {
            Keyword::Logic => "logic",
            _ => {
                return write!(f, "{:?} unimp", self);
            }
        };
        write!(f, "{}", str)
    }
}
