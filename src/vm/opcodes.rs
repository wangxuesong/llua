/* OpMode */
pub const OP_MODE_ABC: u8 = 0; // iABC
pub const OP_MODE_ABX: u8 = 1; // iABx
pub const OP_MODE_ASBX: u8 = 2; // iAsBx
pub const OP_MODE_AX: u8 = 3; // iAx

/* OpArgMask */
pub const OP_ARG_N: u8 = 0; // OpArgN
pub const OP_ARG_U: u8 = 1; // OpArgU
pub const OP_ARG_R: u8 = 2; // OpArgR
pub const OP_ARG_K: u8 = 3; // OpArgK

pub const OPCODES: &'static [OpCode] = &[
    /*       B       C     mode    name    */
    opcode(OP_ARG_R, OP_ARG_N, OP_MODE_ABC, "MOVE    "), // R(A) := R(B)
    opcode(OP_ARG_K, OP_ARG_N, OP_MODE_ABX, "LOADK   "), // R(A) := Kst(Bx)
    opcode(OP_ARG_N, OP_ARG_N, OP_MODE_ABX, "LOADKX  "), // R(A) := Kst(extra arg)
    opcode(OP_ARG_U, OP_ARG_U, OP_MODE_ABC, "LOADBOOL"), // R(A) := (bool)B; if (C) pc++
    opcode(OP_ARG_U, OP_ARG_N, OP_MODE_ABC, "LOADNIL "), // R(A), R(A+1), ..., R(A+B) := nil
    opcode(OP_ARG_U, OP_ARG_N, OP_MODE_ABC, "GETUPVAL"), // R(A) := UpValue[B]
    opcode(OP_ARG_U, OP_ARG_K, OP_MODE_ABC, "GETTABUP"), // R(A) := UpValue[B][RK(C)]
    opcode(OP_ARG_R, OP_ARG_K, OP_MODE_ABC, "GETTABLE"), // R(A) := R(B)[RK(C)]
    opcode(OP_ARG_K, OP_ARG_K, OP_MODE_ABC, "SETTABUP"), // UpValue[A][RK(B)] := RK(C)
    opcode(OP_ARG_U, OP_ARG_N, OP_MODE_ABC, "SETUPVAL"), // UpValue[B] := R(A)
    opcode(OP_ARG_K, OP_ARG_K, OP_MODE_ABC, "SETTABLE"), // R(A)[RK(B)] := RK(C)
    opcode(OP_ARG_U, OP_ARG_U, OP_MODE_ABC, "NEWTABLE"), // R(A) := {} (size = B,C)
    opcode(OP_ARG_R, OP_ARG_K, OP_MODE_ABC, "SELF    "), // R(A+1) := R(B); R(A) := R(B)[RK(C)]
    opcode(OP_ARG_K, OP_ARG_K, OP_MODE_ABC, "ADD     "), // R(A) := RK(B) + RK(C)
    opcode(OP_ARG_K, OP_ARG_K, OP_MODE_ABC, "SUB     "), // R(A) := RK(B) - RK(C)
    opcode(OP_ARG_K, OP_ARG_K, OP_MODE_ABC, "MUL     "), // R(A) := RK(B) * RK(C)
    opcode(OP_ARG_K, OP_ARG_K, OP_MODE_ABC, "MOD     "), // R(A) := RK(B) % RK(C)
    opcode(OP_ARG_K, OP_ARG_K, OP_MODE_ABC, "POW     "), // R(A) := RK(B) ^ RK(C)
    opcode(OP_ARG_K, OP_ARG_K, OP_MODE_ABC, "DIV     "), // R(A) := RK(B) / RK(C)
    opcode(OP_ARG_K, OP_ARG_K, OP_MODE_ABC, "IDIV    "), // R(A) := RK(B) // RK(C)
    opcode(OP_ARG_K, OP_ARG_K, OP_MODE_ABC, "BAND    "), // R(A) := RK(B) & RK(C)
    opcode(OP_ARG_K, OP_ARG_K, OP_MODE_ABC, "BOR     "), // R(A) := RK(B) | RK(C)
    opcode(OP_ARG_K, OP_ARG_K, OP_MODE_ABC, "BXOR    "), // R(A) := RK(B) ~ RK(C)
    opcode(OP_ARG_K, OP_ARG_K, OP_MODE_ABC, "SHL     "), // R(A) := RK(B) << RK(C)
    opcode(OP_ARG_K, OP_ARG_K, OP_MODE_ABC, "SHR     "), // R(A) := RK(B) >> RK(C)
    opcode(OP_ARG_R, OP_ARG_N, OP_MODE_ABC, "UNM     "), // R(A) := -R(B)
    opcode(OP_ARG_R, OP_ARG_N, OP_MODE_ABC, "BNOT    "), // R(A) := ~R(B)
    opcode(OP_ARG_R, OP_ARG_N, OP_MODE_ABC, "NOT     "), // R(A) := not R(B)
    opcode(OP_ARG_R, OP_ARG_N, OP_MODE_ABC, "LEN     "), // R(A) := length of R(B)
    opcode(OP_ARG_R, OP_ARG_R, OP_MODE_ABC, "CONCAT  "), // R(A) := R(B).. ... ..R(C)
    opcode(OP_ARG_R, OP_ARG_N, OP_MODE_ASBX, "JMP     "), // pc+=sBx; if (A) close all upvalues >= R(A - 1)
    opcode(OP_ARG_K, OP_ARG_K, OP_MODE_ABC, "EQ      "), // if ((RK(B) == RK(C)) ~= A) then pc++
    opcode(OP_ARG_K, OP_ARG_K, OP_MODE_ABC, "LT      "), // if ((RK(B) <  RK(C)) ~= A) then pc++
    opcode(OP_ARG_K, OP_ARG_K, OP_MODE_ABC, "LE      "), // if ((RK(B) <= RK(C)) ~= A) then pc++
    opcode(OP_ARG_N, OP_ARG_U, OP_MODE_ABC, "TEST    "), // if not (R(A) <=> C) then pc++
    opcode(OP_ARG_R, OP_ARG_U, OP_MODE_ABC, "TESTSET "), // if (R(B) <=> C) then R(A) := R(B) else pc++
    opcode(OP_ARG_U, OP_ARG_U, OP_MODE_ABC, "CALL    "), // R(A), ... ,R(A+C-2) := R(A)(R(A+1), ... ,R(A+B-1))
    opcode(OP_ARG_U, OP_ARG_U, OP_MODE_ABC, "TAILCALL"), // return R(A)(R(A+1), ... ,R(A+B-1))
    opcode(OP_ARG_U, OP_ARG_N, OP_MODE_ABC, "RETURN  "), // return R(A), ... ,R(A+B-2)
    opcode(OP_ARG_R, OP_ARG_N, OP_MODE_ASBX, "FORLOOP "), // R(A)+=R(A+2); if R(A) <?= R(A+1) then { pc+=sBx; R(A+3)=R(A) }
    opcode(OP_ARG_R, OP_ARG_N, OP_MODE_ASBX, "FORPREP "), // R(A)-=R(A+2); pc+=sBx
    opcode(OP_ARG_N, OP_ARG_U, OP_MODE_ABC, "TFORCALL"),  // R(A+3), ... ,R(A+2+C) := R(A)(R(A+1), R(A+2));
    opcode(OP_ARG_R, OP_ARG_N, OP_MODE_ASBX, "TFORLOOP"), // if R(A+1) ~= nil then { R(A)=R(A+1); pc += sBx }
    opcode(OP_ARG_U, OP_ARG_U, OP_MODE_ABC, "SETLIST "),  // R(A)[(C-1)*FPF+i] := R(A+i), 1 <= i <= B
    opcode(OP_ARG_U, OP_ARG_N, OP_MODE_ABX, "CLOSURE "),  // R(A) := closure(KPROTO[Bx])
    opcode(OP_ARG_U, OP_ARG_N, OP_MODE_ABC, "VARARG  "),  // R(A), R(A+1), ..., R(A+B-2) = vararg
    opcode(OP_ARG_U, OP_ARG_U, OP_MODE_AX, "EXTRAARG"),   // extra (larger) argument for previous opcode
];

const fn opcode(bmode: u8, cmode: u8, opmode: u8, name: &'static str) -> OpCode {
    OpCode {
        bmode,
        cmode,
        opmode,
        name,
    }
}

pub struct OpCode {
    pub bmode: u8,  // B arg mode
    pub cmode: u8,  // C arg mode
    pub opmode: u8, // op mode
    pub name: &'static str,
}
