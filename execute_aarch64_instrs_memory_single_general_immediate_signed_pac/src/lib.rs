#![no_std]
#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_doc_comments)]
#![allow(non_upper_case_globals)]
//! BOREALIS GENERATED FILE
extern crate alloc;
use SP_set::*;
use CreateAccDescGPR::*;
use X_read::*;
use ConstrainUnpredictable::*;
use AuthDA::*;
use AArch64_SetLSInstructionSyndrome::*;
use X_set::*;
use u__UNKNOWN_bits::*;
use neq_int::*;
use SP_read::*;
use Mem_read::*;
use EndOfInstruction::*;
use SPESampleGeneralPurposeLoadStore::*;
use CheckSPAlignment::*;
use AuthDB::*;
use common::*;
pub fn execute_aarch64_instrs_memory_single_general_immediate_signed_pac<T: Tracer>(
    state: &mut State,
    tracer: &T,
    memop: u32,
    n: i64,
    nontemporal: bool,
    offset: u64,
    t: i64,
    tagchecked: bool,
    use_key_a: bool,
    wback__arg: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_164002: bool,
        gs_164017: bool,
        gs_164000: bool,
        address: u64,
        accdesc: ProductType9878976b5bcce9c9,
        gs_164019: bool,
        gs_705849: Bits,
        wback: bool,
        gs_164018: bool,
        c: u32,
        wb_unknown: bool,
        memop: u32,
        n: i64,
        nontemporal: bool,
        offset: u64,
        t: i64,
        tagchecked: bool,
        use_key_a: bool,
        wback__arg: bool,
    }
    let fn_state = FunctionState {
        memop,
        n,
        nontemporal,
        offset,
        t,
        tagchecked,
        use_key_a,
        wback__arg,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var wback__arg:u8
        let s_0_0: bool = fn_state.wback__arg;
        // D s_0_1: write-var wback <= s_0_0
        fn_state.wback = s_0_0;
        // C s_0_2: const #16975u : u32
        let s_0_2: u32 = 16975;
        // D s_0_3: read-reg s_0_2:u8
        let s_0_3: u8 = {
            let value = state.read_register::<u8>(s_0_2 as isize);
            tracer.read_register(s_0_2 as isize, value);
            value
        };
        // D s_0_4: cast zx s_0_3 -> bv
        let s_0_4: Bits = Bits::new(s_0_3 as u128, 2u16);
        // C s_0_5: const #448u : u32
        let s_0_5: u32 = 448;
        // D s_0_6: read-reg s_0_5:u8
        let s_0_6: u8 = {
            let value = state.read_register::<u8>(s_0_5 as isize);
            tracer.read_register(s_0_5 as isize, value);
            value
        };
        // D s_0_7: cast zx s_0_6 -> bv
        let s_0_7: Bits = Bits::new(s_0_6 as u128, 2u16);
        // D s_0_8: cmp-ne s_0_4 s_0_7
        let s_0_8: bool = ((s_0_4) != (s_0_7));
        // C s_0_9: const #0u : u8
        let s_0_9: bool = false;
        // D s_0_10: write-var wb_unknown <= s_0_9
        fn_state.wb_unknown = s_0_9;
        // D s_0_11: read-var memop:u32
        let s_0_11: u32 = fn_state.memop;
        // D s_0_12: read-var nontemporal:u8
        let s_0_12: bool = fn_state.nontemporal;
        // D s_0_13: read-var tagchecked:u8
        let s_0_13: bool = fn_state.tagchecked;
        // D s_0_14: call CreateAccDescGPR(s_0_11, s_0_12, s_0_8, s_0_13)
        let s_0_14: ProductType9878976b5bcce9c9 = CreateAccDescGPR(
            state,
            tracer,
            s_0_11,
            s_0_12,
            s_0_8,
            s_0_13,
        );
        // D s_0_15: write-var accdesc <= s_0_14
        fn_state.accdesc = s_0_14;
        // D s_0_16: read-var wback:u8
        let s_0_16: bool = fn_state.wback;
        // N s_0_17: branch s_0_16 b52 b1
        if s_0_16 {
            return block_52(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#164000 <= s_1_0
        fn_state.gs_164000 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#164000:u8
        let s_2_0: bool = fn_state.gs_164000;
        // N s_2_1: branch s_2_0 b51 b3
        if s_2_0 {
            return block_51(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#164002 <= s_3_0
        fn_state.gs_164002 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#164002:u8
        let s_4_0: bool = fn_state.gs_164002;
        // N s_4_1: branch s_4_0 b32 b5
        if s_4_0 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #31s : i
        let s_6_0: i128 = 31;
        // D s_6_1: read-var n:i64
        let s_6_1: i64 = fn_state.n;
        // D s_6_2: cast zx s_6_1 -> i
        let s_6_2: i128 = (i128::try_from(s_6_1).unwrap());
        // D s_6_3: cmp-eq s_6_2 s_6_0
        let s_6_3: bool = ((s_6_2) == (s_6_0));
        // N s_6_4: branch s_6_3 b31 b7
        if s_6_3 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #64s : i64
        let s_7_0: i64 = 64;
        // D s_7_1: read-var n:i64
        let s_7_1: i64 = fn_state.n;
        // D s_7_2: cast zx s_7_1 -> i
        let s_7_2: i128 = (i128::try_from(s_7_1).unwrap());
        // D s_7_3: call X_read(s_7_2, s_7_0)
        let s_7_3: Bits = X_read(state, tracer, s_7_2, s_7_0);
        // D s_7_4: cast reint s_7_3 -> u64
        let s_7_4: u64 = (s_7_3.value() as u64);
        // D s_7_5: write-var address <= s_7_4
        fn_state.address = s_7_4;
        // N s_7_6: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var wback:u8
        let s_8_0: bool = fn_state.wback;
        // D s_8_1: not s_8_0
        let s_8_1: bool = !s_8_0;
        // N s_8_2: branch s_8_1 b30 b9
        if s_8_1 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_9_0: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var use_key_a:u8
        let s_10_0: bool = fn_state.use_key_a;
        // N s_10_1: branch s_10_0 b28 b11
        if s_10_0 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #64s : i64
        let s_11_0: i64 = 64;
        // C s_11_1: const #31s : i
        let s_11_1: i128 = 31;
        // S s_11_2: call X_read(s_11_1, s_11_0)
        let s_11_2: Bits = X_read(state, tracer, s_11_1, s_11_0);
        // S s_11_3: cast reint s_11_2 -> u64
        let s_11_3: u64 = (s_11_2.value() as u64);
        // D s_11_4: read-var address:u64
        let s_11_4: u64 = fn_state.address;
        // C s_11_5: const #1u : u8
        let s_11_5: bool = true;
        // D s_11_6: call AuthDB(s_11_4, s_11_3, s_11_5)
        let s_11_6: u64 = AuthDB(state, tracer, s_11_4, s_11_3, s_11_5);
        // D s_11_7: write-var address <= s_11_6
        fn_state.address = s_11_6;
        // N s_11_8: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #31s : i
        let s_13_0: i128 = 31;
        // D s_13_1: read-var n:i64
        let s_13_1: i64 = fn_state.n;
        // D s_13_2: cast zx s_13_1 -> i
        let s_13_2: i128 = (i128::try_from(s_13_1).unwrap());
        // D s_13_3: cmp-eq s_13_2 s_13_0
        let s_13_3: bool = ((s_13_2) == (s_13_0));
        // N s_13_4: branch s_13_3 b27 b14
        if s_13_3 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_14_0: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var address:u64
        let s_15_0: u64 = fn_state.address;
        // D s_15_1: cast zx s_15_0 -> bv
        let s_15_1: Bits = Bits::new(s_15_0 as u128, 64u16);
        // D s_15_2: read-var offset:u64
        let s_15_2: u64 = fn_state.offset;
        // D s_15_3: cast zx s_15_2 -> bv
        let s_15_3: Bits = Bits::new(s_15_2 as u128, 64u16);
        // D s_15_4: add s_15_1 s_15_3
        let s_15_4: Bits = (s_15_1 + s_15_3);
        // D s_15_5: cast reint s_15_4 -> u64
        let s_15_5: u64 = (s_15_4.value() as u64);
        // D s_15_6: write-var address <= s_15_5
        fn_state.address = s_15_5;
        // C s_15_7: const #8s : i
        let s_15_7: i128 = 8;
        // D s_15_8: read-var address:u64
        let s_15_8: u64 = fn_state.address;
        // D s_15_9: read-var accdesc:struct
        let s_15_9: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_15_10: call Mem_read(s_15_8, s_15_7, s_15_9)
        let s_15_10: Bits = Mem_read(state, tracer, s_15_8, s_15_7, s_15_9);
        // D s_15_11: write-var gs#705849 <= s_15_10
        fn_state.gs_705849 = s_15_10;
        // N s_15_12: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#705849:bv
        let s_16_0: Bits = fn_state.gs_705849;
        // D s_16_1: cast reint s_16_0 -> u64
        let s_16_1: u64 = (s_16_0.value() as u64);
        // C s_16_2: const #64s : i64
        let s_16_2: i64 = 64;
        // D s_16_3: read-var t:i64
        let s_16_3: i64 = fn_state.t;
        // D s_16_4: cast zx s_16_3 -> i
        let s_16_4: i128 = (i128::try_from(s_16_3).unwrap());
        // D s_16_5: cast zx s_16_1 -> bv
        let s_16_5: Bits = Bits::new(s_16_1 as u128, 64u16);
        // D s_16_6: call X_set(s_16_4, s_16_2, s_16_5)
        let s_16_6: () = X_set(state, tracer, s_16_4, s_16_2, s_16_5);
        // D s_16_7: read-var wback:u8
        let s_16_7: bool = fn_state.wback;
        // N s_16_8: branch s_16_7 b21 b17
        if s_16_7 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_17_0: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #22416u : u32
        let s_18_0: u32 = 22416;
        // D s_18_1: read-reg s_18_0:u8
        let s_18_1: bool = {
            let value = state.read_register::<bool>(s_18_0 as isize);
            tracer.read_register(s_18_0 as isize, value);
            value
        };
        // N s_18_2: branch s_18_1 b20 b19
        if s_18_1 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_19_0: return
        return;
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #() : ()
        let s_20_0: () = ();
        // S s_20_1: call SPESampleGeneralPurposeLoadStore(s_20_0)
        let s_20_1: () = SPESampleGeneralPurposeLoadStore(state, tracer, s_20_0);
        // N s_20_2: return
        return;
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var wb_unknown:u8
        let s_21_0: bool = fn_state.wb_unknown;
        // N s_21_1: branch s_21_0 b26 b22
        if s_21_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_22_0: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #31s : i
        let s_23_0: i128 = 31;
        // D s_23_1: read-var n:i64
        let s_23_1: i64 = fn_state.n;
        // D s_23_2: cast zx s_23_1 -> i
        let s_23_2: i128 = (i128::try_from(s_23_1).unwrap());
        // D s_23_3: cmp-eq s_23_2 s_23_0
        let s_23_3: bool = ((s_23_2) == (s_23_0));
        // N s_23_4: branch s_23_3 b25 b24
        if s_23_3 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #64s : i64
        let s_24_0: i64 = 64;
        // D s_24_1: read-var n:i64
        let s_24_1: i64 = fn_state.n;
        // D s_24_2: cast zx s_24_1 -> i
        let s_24_2: i128 = (i128::try_from(s_24_1).unwrap());
        // D s_24_3: read-var address:u64
        let s_24_3: u64 = fn_state.address;
        // D s_24_4: cast zx s_24_3 -> bv
        let s_24_4: Bits = Bits::new(s_24_3 as u128, 64u16);
        // D s_24_5: call X_set(s_24_2, s_24_0, s_24_4)
        let s_24_5: () = X_set(state, tracer, s_24_2, s_24_0, s_24_4);
        // N s_24_6: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var address:u64
        let s_25_0: u64 = fn_state.address;
        // D s_25_1: call SP_set(s_25_0)
        let s_25_1: () = SP_set(state, tracer, s_25_0);
        // N s_25_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #64s : i64
        let s_26_0: i64 = 64;
        // C s_26_1: cast zx s_26_0 -> i
        let s_26_1: i128 = (i128::try_from(s_26_0).unwrap());
        // S s_26_2: call __UNKNOWN_bits(s_26_1)
        let s_26_2: Bits = u__UNKNOWN_bits(state, tracer, s_26_1);
        // S s_26_3: cast reint s_26_2 -> u64
        let s_26_3: u64 = (s_26_2.value() as u64);
        // D s_26_4: write-var address <= s_26_3
        fn_state.address = s_26_3;
        // N s_26_5: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #() : ()
        let s_27_0: () = ();
        // S s_27_1: call CheckSPAlignment(s_27_0)
        let s_27_1: () = CheckSPAlignment(state, tracer, s_27_0);
        // N s_27_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #64s : i64
        let s_28_0: i64 = 64;
        // C s_28_1: const #31s : i
        let s_28_1: i128 = 31;
        // S s_28_2: call X_read(s_28_1, s_28_0)
        let s_28_2: Bits = X_read(state, tracer, s_28_1, s_28_0);
        // S s_28_3: cast reint s_28_2 -> u64
        let s_28_3: u64 = (s_28_2.value() as u64);
        // D s_28_4: read-var address:u64
        let s_28_4: u64 = fn_state.address;
        // C s_28_5: const #1u : u8
        let s_28_5: bool = true;
        // D s_28_6: call AuthDA(s_28_4, s_28_3, s_28_5)
        let s_28_6: u64 = AuthDA(state, tracer, s_28_4, s_28_3, s_28_5);
        // D s_28_7: write-var address <= s_28_6
        fn_state.address = s_28_6;
        // N s_28_8: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_29_0: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #8s : i
        let s_30_0: i128 = 8;
        // D s_30_1: read-var t:i64
        let s_30_1: i64 = fn_state.t;
        // D s_30_2: cast zx s_30_1 -> i
        let s_30_2: i128 = (i128::try_from(s_30_1).unwrap());
        // C s_30_3: const #0u : u8
        let s_30_3: bool = false;
        // C s_30_4: const #1u : u8
        let s_30_4: bool = true;
        // C s_30_5: const #0u : u8
        let s_30_5: bool = false;
        // D s_30_6: call AArch64_SetLSInstructionSyndrome(s_30_0, s_30_3, s_30_2, s_30_4, s_30_5)
        let s_30_6: () = AArch64_SetLSInstructionSyndrome(
            state,
            tracer,
            s_30_0,
            s_30_3,
            s_30_2,
            s_30_4,
            s_30_5,
        );
        // N s_30_7: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #() : ()
        let s_31_0: () = ();
        // S s_31_1: call SP_read(s_31_0)
        let s_31_1: u64 = SP_read(state, tracer, s_31_0);
        // D s_31_2: write-var address <= s_31_1
        fn_state.address = s_31_1;
        // N s_31_3: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #1u : u32
        let s_32_0: u32 = 1;
        // S s_32_1: call ConstrainUnpredictable(s_32_0)
        let s_32_1: u32 = ConstrainUnpredictable(state, tracer, s_32_0);
        // D s_32_2: write-var c <= s_32_1
        fn_state.c = s_32_1;
        // D s_32_3: read-var c:u32
        let s_32_3: u32 = fn_state.c;
        // C s_32_4: const #11u : u32
        let s_32_4: u32 = 11;
        // D s_32_5: cmp-eq s_32_3 s_32_4
        let s_32_5: bool = ((s_32_3) == (s_32_4));
        // N s_32_6: branch s_32_5 b50 b33
        if s_32_5 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var c:u32
        let s_33_0: u32 = fn_state.c;
        // C s_33_1: const #1u : u32
        let s_33_1: u32 = 1;
        // D s_33_2: cmp-eq s_33_0 s_33_1
        let s_33_2: bool = ((s_33_0) == (s_33_1));
        // N s_33_3: branch s_33_2 b49 b34
        if s_33_2 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var c:u32
        let s_34_0: u32 = fn_state.c;
        // C s_34_1: const #2u : u32
        let s_34_1: u32 = 2;
        // D s_34_2: cmp-eq s_34_0 s_34_1
        let s_34_2: bool = ((s_34_0) == (s_34_1));
        // N s_34_3: branch s_34_2 b48 b35
        if s_34_2 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var c:u32
        let s_35_0: u32 = fn_state.c;
        // C s_35_1: const #4u : u32
        let s_35_1: u32 = 4;
        // D s_35_2: cmp-eq s_35_0 s_35_1
        let s_35_2: bool = ((s_35_0) == (s_35_1));
        // D s_35_3: write-var gs#164017 <= s_35_2
        fn_state.gs_164017 = s_35_2;
        // N s_35_4: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var gs#164017:u8
        let s_36_0: bool = fn_state.gs_164017;
        // D s_36_1: write-var gs#164018 <= s_36_0
        fn_state.gs_164018 = s_36_0;
        // N s_36_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var gs#164018:u8
        let s_37_0: bool = fn_state.gs_164018;
        // D s_37_1: write-var gs#164019 <= s_37_0
        fn_state.gs_164019 = s_37_0;
        // N s_37_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var gs#164019:u8
        let s_38_0: bool = fn_state.gs_164019;
        // N s_38_1: assert s_38_0
        let s_38_1: () = assert!(s_38_0);
        // C s_38_2: const #11u : u32
        let s_38_2: u32 = 11;
        // D s_38_3: read-var c:u32
        let s_38_3: u32 = fn_state.c;
        // D s_38_4: cmp-eq s_38_2 s_38_3
        let s_38_4: bool = ((s_38_2) == (s_38_3));
        // D s_38_5: not s_38_4
        let s_38_5: bool = !s_38_4;
        // N s_38_6: branch s_38_5 b41 b39
        if s_38_5 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #0u : u8
        let s_39_0: bool = false;
        // D s_39_1: write-var wback <= s_39_0
        fn_state.wback = s_39_0;
        // N s_39_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_40_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #1u : u32
        let s_41_0: u32 = 1;
        // D s_41_1: read-var c:u32
        let s_41_1: u32 = fn_state.c;
        // D s_41_2: cmp-eq s_41_0 s_41_1
        let s_41_2: bool = ((s_41_0) == (s_41_1));
        // D s_41_3: not s_41_2
        let s_41_3: bool = !s_41_2;
        // N s_41_4: branch s_41_3 b43 b42
        if s_41_3 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_42(state, tracer, fn_state);
        };
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #1u : u8
        let s_42_0: bool = true;
        // D s_42_1: write-var wb_unknown <= s_42_0
        fn_state.wb_unknown = s_42_0;
        // N s_42_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #2u : u32
        let s_43_0: u32 = 2;
        // D s_43_1: read-var c:u32
        let s_43_1: u32 = fn_state.c;
        // D s_43_2: cmp-eq s_43_0 s_43_1
        let s_43_2: bool = ((s_43_0) == (s_43_1));
        // D s_43_3: not s_43_2
        let s_43_3: bool = !s_43_2;
        // N s_43_4: branch s_43_3 b45 b44
        if s_43_3 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_44(state, tracer, fn_state);
        };
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_44_0: panic
        panic!("{:?}", ());
        // N s_44_1: return
        return;
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #4u : u32
        let s_45_0: u32 = 4;
        // D s_45_1: read-var c:u32
        let s_45_1: u32 = fn_state.c;
        // D s_45_2: cmp-eq s_45_0 s_45_1
        let s_45_2: bool = ((s_45_0) == (s_45_1));
        // D s_45_3: not s_45_2
        let s_45_3: bool = !s_45_2;
        // N s_45_4: branch s_45_3 b47 b46
        if s_45_3 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_46(state, tracer, fn_state);
        };
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #() : ()
        let s_46_0: () = ();
        // S s_46_1: call EndOfInstruction(s_46_0)
        let s_46_1: () = EndOfInstruction(state, tracer, s_46_0);
        // N s_46_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_47_0: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #1u : u8
        let s_48_0: bool = true;
        // D s_48_1: write-var gs#164017 <= s_48_0
        fn_state.gs_164017 = s_48_0;
        // N s_48_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #1u : u8
        let s_49_0: bool = true;
        // D s_49_1: write-var gs#164018 <= s_49_0
        fn_state.gs_164018 = s_49_0;
        // N s_49_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #1u : u8
        let s_50_0: bool = true;
        // D s_50_1: write-var gs#164019 <= s_50_0
        fn_state.gs_164019 = s_50_0;
        // N s_50_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #31s : i
        let s_51_0: i128 = 31;
        // D s_51_1: read-var n:i64
        let s_51_1: i64 = fn_state.n;
        // D s_51_2: cast zx s_51_1 -> i
        let s_51_2: i128 = (i128::try_from(s_51_1).unwrap());
        // D s_51_3: call neq_int(s_51_2, s_51_0)
        let s_51_3: bool = neq_int(state, tracer, s_51_2, s_51_0);
        // D s_51_4: write-var gs#164002 <= s_51_3
        fn_state.gs_164002 = s_51_3;
        // N s_51_5: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var n:i64
        let s_52_0: i64 = fn_state.n;
        // D s_52_1: cast zx s_52_0 -> i
        let s_52_1: i128 = (i128::try_from(s_52_0).unwrap());
        // D s_52_2: read-var t:i64
        let s_52_2: i64 = fn_state.t;
        // D s_52_3: cast zx s_52_2 -> i
        let s_52_3: i128 = (i128::try_from(s_52_2).unwrap());
        // D s_52_4: cmp-eq s_52_1 s_52_3
        let s_52_4: bool = ((s_52_1) == (s_52_3));
        // D s_52_5: write-var gs#164000 <= s_52_4
        fn_state.gs_164000 = s_52_4;
        // N s_52_6: jump b2
        return block_2(state, tracer, fn_state);
    }
}
