use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Keyword {
    // System task identifiers that are treated as keyword
    Unit,
    Root,

    // Verilog 95
    And,
    Assign,
    Begin,
    Case,
    Casex,
    Casez,
    Deassign,
    Default,
    Defparam,
    Disable,
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
    If,
    Ifnone,
    Initial,
    Join,
    Module,
    Not,
    Or,
    Parameter,
    Primitive,
    Reg,
    Release,
    Repeat,
    Scalared,
    Specify,
    Specparam,
    Table,
    Task,
    Vectored,
    Wait,
    While,
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

    // SV 05
    Alias,
    Assert,
    Assume,
    Before,
    Bind,
    Bins,
    Binsof,
    Break,
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
    Interface,
    Intersect,
    JoinAny,
    JoinNone,
    Local,
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
    Return,
    Sequence,
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
            Keyword::Module => "module",
            Keyword::Interface => "interface",
            _ => {
                return write!(f, "{:?} unimp", self);
            }
        };
        write!(f, "{}", str)
    }
}
