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
use neq_int::*;
use ConstrainUnpredictable::*;
use EndOfInstruction::*;
use execute_aarch64_instrs_memory_exclusive_pair::*;
use common::*;
pub fn decode_stxp_aarch64_instrs_memory_exclusive_pair<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rt: u8,
    Rn: u8,
    Rt2: u8,
    o0: bool,
    Rs: u8,
    L: bool,
    sz: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        t: i64,
        t2: i64,
        gs_159662: bool,
        gs_159660: bool,
        gs_159655: bool,
        elsize: i64,
        gs_159680: bool,
        n: i64,
        memop: u32,
        rn_unknown: bool,
        gs_159681: bool,
        c: u32,
        gs_159671: bool,
        tagchecked: bool,
        gs_159658: bool,
        gs_159663: bool,
        ga_259568: i64,
        s: i64,
        acqrel: bool,
        gs_159654: bool,
        u_2111: u32,
        regsize: i64,
        datasize: i64,
        rt_unknown: bool,
        u_2110: u32,
        gs_159672: bool,
        Rt: u8,
        Rn: u8,
        Rt2: u8,
        o0: bool,
        Rs: u8,
        L: bool,
        sz: bool,
    }
    let fn_state = FunctionState {
        Rt,
        Rn,
        Rt2,
        o0,
        Rs,
        L,
        sz,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var Rn:u8
        let s_0_0: u8 = fn_state.Rn;
        // D s_0_1: cast zx s_0_0 -> bv
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 5u16);
        // D s_0_2: cast zx s_0_1 -> i
        let s_0_2: i128 = (s_0_1.value() as i128);
        // D s_0_3: cast reint s_0_2 -> i64
        let s_0_3: i64 = (s_0_2 as i64);
        // D s_0_4: write-var n <= s_0_3
        fn_state.n = s_0_3;
        // D s_0_5: read-var Rt:u8
        let s_0_5: u8 = fn_state.Rt;
        // D s_0_6: cast zx s_0_5 -> bv
        let s_0_6: Bits = Bits::new(s_0_5 as u128, 5u16);
        // D s_0_7: cast zx s_0_6 -> i
        let s_0_7: i128 = (s_0_6.value() as i128);
        // D s_0_8: cast reint s_0_7 -> i64
        let s_0_8: i64 = (s_0_7 as i64);
        // D s_0_9: write-var t <= s_0_8
        fn_state.t = s_0_8;
        // D s_0_10: read-var Rt2:u8
        let s_0_10: u8 = fn_state.Rt2;
        // D s_0_11: cast zx s_0_10 -> bv
        let s_0_11: Bits = Bits::new(s_0_10 as u128, 5u16);
        // D s_0_12: cast zx s_0_11 -> i
        let s_0_12: i128 = (s_0_11.value() as i128);
        // D s_0_13: cast reint s_0_12 -> i64
        let s_0_13: i64 = (s_0_12 as i64);
        // D s_0_14: write-var t2 <= s_0_13
        fn_state.t2 = s_0_13;
        // D s_0_15: read-var Rs:u8
        let s_0_15: u8 = fn_state.Rs;
        // D s_0_16: cast zx s_0_15 -> bv
        let s_0_16: Bits = Bits::new(s_0_15 as u128, 5u16);
        // D s_0_17: cast zx s_0_16 -> i
        let s_0_17: i128 = (s_0_16.value() as i128);
        // D s_0_18: cast reint s_0_17 -> i64
        let s_0_18: i64 = (s_0_17 as i64);
        // D s_0_19: write-var s <= s_0_18
        fn_state.s = s_0_18;
        // D s_0_20: read-var o0:u8
        let s_0_20: bool = fn_state.o0;
        // D s_0_21: cast zx s_0_20 -> bv
        let s_0_21: Bits = Bits::new(s_0_20 as u128, 1u16);
        // C s_0_22: const #1u : u8
        let s_0_22: bool = true;
        // C s_0_23: cast zx s_0_22 -> bv
        let s_0_23: Bits = Bits::new(s_0_22 as u128, 1u16);
        // D s_0_24: cmp-eq s_0_21 s_0_23
        let s_0_24: bool = ((s_0_21) == (s_0_23));
        // D s_0_25: write-var acqrel <= s_0_24
        fn_state.acqrel = s_0_24;
        // D s_0_26: read-var L:u8
        let s_0_26: bool = fn_state.L;
        // D s_0_27: cast zx s_0_26 -> bv
        let s_0_27: Bits = Bits::new(s_0_26 as u128, 1u16);
        // C s_0_28: const #1u : u8
        let s_0_28: bool = true;
        // C s_0_29: cast zx s_0_28 -> bv
        let s_0_29: Bits = Bits::new(s_0_28 as u128, 1u16);
        // D s_0_30: cmp-eq s_0_27 s_0_29
        let s_0_30: bool = ((s_0_27) == (s_0_29));
        // N s_0_31: branch s_0_30 b69 b1
        if s_0_30 {
            return block_69(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #1u : u32
        let s_1_0: u32 = 1;
        // D s_1_1: write-var memop <= s_1_0
        fn_state.memop = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var sz:u8
        let s_2_0: bool = fn_state.sz;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 1u16);
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (s_2_1.value() as i128);
        // D s_2_3: cast reint s_2_2 -> i64
        let s_2_3: i64 = (s_2_2 as i64);
        // C s_2_4: const #32s : i64
        let s_2_4: i64 = 32;
        // D s_2_5: lsl s_2_4 s_2_3
        let s_2_5: i64 = s_2_4 << s_2_3;
        // D s_2_6: write-var elsize <= s_2_5
        fn_state.elsize = s_2_5;
        // C s_2_7: const #64s : i
        let s_2_7: i128 = 64;
        // D s_2_8: read-var elsize:i64
        let s_2_8: i64 = fn_state.elsize;
        // D s_2_9: cast zx s_2_8 -> i
        let s_2_9: i128 = (i128::try_from(s_2_8).unwrap());
        // D s_2_10: cmp-eq s_2_9 s_2_7
        let s_2_10: bool = ((s_2_9) == (s_2_7));
        // N s_2_11: branch s_2_10 b68 b3
        if s_2_10 {
            return block_68(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #32s : i64
        let s_3_0: i64 = 32;
        // D s_3_1: write-var ga#259568 <= s_3_0
        fn_state.ga_259568 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var ga#259568:i64
        let s_4_0: i64 = fn_state.ga_259568;
        // D s_4_1: write-var regsize <= s_4_0
        fn_state.regsize = s_4_0;
        // C s_4_2: const #2s : i
        let s_4_2: i128 = 2;
        // D s_4_3: read-var elsize:i64
        let s_4_3: i64 = fn_state.elsize;
        // D s_4_4: cast zx s_4_3 -> i
        let s_4_4: i128 = (i128::try_from(s_4_3).unwrap());
        // D s_4_5: mul s_4_4 s_4_2
        let s_4_5: i128 = ((s_4_4) * (s_4_2));
        // D s_4_6: cast reint s_4_5 -> i64
        let s_4_6: i64 = (s_4_5 as i64);
        // D s_4_7: write-var datasize <= s_4_6
        fn_state.datasize = s_4_6;
        // C s_4_8: const #31s : i
        let s_4_8: i128 = 31;
        // D s_4_9: read-var n:i64
        let s_4_9: i64 = fn_state.n;
        // D s_4_10: cast zx s_4_9 -> i
        let s_4_10: i128 = (i128::try_from(s_4_9).unwrap());
        // D s_4_11: call neq_int(s_4_10, s_4_8)
        let s_4_11: bool = neq_int(state, tracer, s_4_10, s_4_8);
        // D s_4_12: write-var tagchecked <= s_4_11
        fn_state.tagchecked = s_4_11;
        // C s_4_13: const #0u : u8
        let s_4_13: bool = false;
        // D s_4_14: write-var rt_unknown <= s_4_13
        fn_state.rt_unknown = s_4_13;
        // C s_4_15: const #0u : u8
        let s_4_15: bool = false;
        // D s_4_16: write-var rn_unknown <= s_4_15
        fn_state.rn_unknown = s_4_15;
        // D s_4_17: read-var memop:u32
        let s_4_17: u32 = fn_state.memop;
        // C s_4_18: const #0u : u32
        let s_4_18: u32 = 0;
        // D s_4_19: cmp-eq s_4_17 s_4_18
        let s_4_19: bool = ((s_4_17) == (s_4_18));
        // N s_4_20: branch s_4_19 b67 b5
        if s_4_19 {
            return block_67(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#159654 <= s_5_0
        fn_state.gs_159654 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#159654:u8
        let s_6_0: bool = fn_state.gs_159654;
        // N s_6_1: branch s_6_0 b66 b7
        if s_6_0 {
            return block_66(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var gs#159655 <= s_7_0
        fn_state.gs_159655 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#159655:u8
        let s_8_0: bool = fn_state.gs_159655;
        // N s_8_1: branch s_8_0 b52 b9
        if s_8_0 {
            return block_52(state, tracer, fn_state);
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
        // D s_10_0: read-var memop:u32
        let s_10_0: u32 = fn_state.memop;
        // C s_10_1: const #1u : u32
        let s_10_1: u32 = 1;
        // D s_10_2: cmp-eq s_10_0 s_10_1
        let s_10_2: bool = ((s_10_0) == (s_10_1));
        // N s_10_3: branch s_10_2 b13 b11
        if s_10_2 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var datasize:i64
        let s_12_0: i64 = fn_state.datasize;
        // D s_12_1: cast zx s_12_0 -> i
        let s_12_1: i128 = (i128::try_from(s_12_0).unwrap());
        // D s_12_2: cast reint s_12_1 -> i64
        let s_12_2: i64 = (s_12_1 as i64);
        // D s_12_3: read-var elsize:i64
        let s_12_3: i64 = fn_state.elsize;
        // D s_12_4: cast zx s_12_3 -> i
        let s_12_4: i128 = (i128::try_from(s_12_3).unwrap());
        // D s_12_5: cast reint s_12_4 -> i64
        let s_12_5: i64 = (s_12_4 as i64);
        // D s_12_6: read-var regsize:i64
        let s_12_6: i64 = fn_state.regsize;
        // D s_12_7: cast zx s_12_6 -> i
        let s_12_7: i128 = (i128::try_from(s_12_6).unwrap());
        // D s_12_8: cast reint s_12_7 -> i64
        let s_12_8: i64 = (s_12_7 as i64);
        // D s_12_9: read-var acqrel:u8
        let s_12_9: bool = fn_state.acqrel;
        // D s_12_10: read-var memop:u32
        let s_12_10: u32 = fn_state.memop;
        // D s_12_11: read-var n:i64
        let s_12_11: i64 = fn_state.n;
        // C s_12_12: const #1u : u8
        let s_12_12: bool = true;
        // D s_12_13: read-var rn_unknown:u8
        let s_12_13: bool = fn_state.rn_unknown;
        // D s_12_14: read-var rt_unknown:u8
        let s_12_14: bool = fn_state.rt_unknown;
        // D s_12_15: read-var s:i64
        let s_12_15: i64 = fn_state.s;
        // D s_12_16: read-var t:i64
        let s_12_16: i64 = fn_state.t;
        // D s_12_17: read-var t2:i64
        let s_12_17: i64 = fn_state.t2;
        // D s_12_18: read-var tagchecked:u8
        let s_12_18: bool = fn_state.tagchecked;
        // D s_12_19: call execute_aarch64_instrs_memory_exclusive_pair(s_12_9, s_12_2, s_12_5, s_12_10, s_12_11, s_12_12, s_12_8, s_12_13, s_12_14, s_12_15, s_12_16, s_12_17, s_12_18)
        let s_12_19: () = execute_aarch64_instrs_memory_exclusive_pair(
            state,
            tracer,
            s_12_9,
            s_12_2,
            s_12_5,
            s_12_10,
            s_12_11,
            s_12_12,
            s_12_8,
            s_12_13,
            s_12_14,
            s_12_15,
            s_12_16,
            s_12_17,
            s_12_18,
        );
        // N s_12_20: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var s:i64
        let s_13_0: i64 = fn_state.s;
        // D s_13_1: cast zx s_13_0 -> i
        let s_13_1: i128 = (i128::try_from(s_13_0).unwrap());
        // D s_13_2: read-var t:i64
        let s_13_2: i64 = fn_state.t;
        // D s_13_3: cast zx s_13_2 -> i
        let s_13_3: i128 = (i128::try_from(s_13_2).unwrap());
        // D s_13_4: cmp-eq s_13_1 s_13_3
        let s_13_4: bool = ((s_13_1) == (s_13_3));
        // N s_13_5: branch s_13_4 b51 b14
        if s_13_4 {
            return block_51(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var s:i64
        let s_14_0: i64 = fn_state.s;
        // D s_14_1: cast zx s_14_0 -> i
        let s_14_1: i128 = (i128::try_from(s_14_0).unwrap());
        // D s_14_2: read-var t2:i64
        let s_14_2: i64 = fn_state.t2;
        // D s_14_3: cast zx s_14_2 -> i
        let s_14_3: i128 = (i128::try_from(s_14_2).unwrap());
        // D s_14_4: cmp-eq s_14_1 s_14_3
        let s_14_4: bool = ((s_14_1) == (s_14_3));
        // D s_14_5: write-var gs#159658 <= s_14_4
        fn_state.gs_159658 = s_14_4;
        // N s_14_6: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#159658:u8
        let s_15_0: bool = fn_state.gs_159658;
        // N s_15_1: branch s_15_0 b37 b16
        if s_15_0 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_16_0: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var s:i64
        let s_17_0: i64 = fn_state.s;
        // D s_17_1: cast zx s_17_0 -> i
        let s_17_1: i128 = (i128::try_from(s_17_0).unwrap());
        // D s_17_2: read-var n:i64
        let s_17_2: i64 = fn_state.n;
        // D s_17_3: cast zx s_17_2 -> i
        let s_17_3: i128 = (i128::try_from(s_17_2).unwrap());
        // D s_17_4: cmp-eq s_17_1 s_17_3
        let s_17_4: bool = ((s_17_1) == (s_17_3));
        // N s_17_5: branch s_17_4 b36 b18
        if s_17_4 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #0u : u8
        let s_18_0: bool = false;
        // D s_18_1: write-var gs#159660 <= s_18_0
        fn_state.gs_159660 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#159660:u8
        let s_19_0: bool = fn_state.gs_159660;
        // N s_19_1: branch s_19_0 b22 b20
        if s_19_0 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_20_0: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_21_0: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #4u : u32
        let s_22_0: u32 = 4;
        // S s_22_1: call ConstrainUnpredictable(s_22_0)
        let s_22_1: u32 = ConstrainUnpredictable(state, tracer, s_22_0);
        // D s_22_2: write-var u#2111 <= s_22_1
        fn_state.u_2111 = s_22_1;
        // D s_22_3: read-var u#2111:u32
        let s_22_3: u32 = fn_state.u_2111;
        // C s_22_4: const #1u : u32
        let s_22_4: u32 = 1;
        // D s_22_5: cmp-eq s_22_3 s_22_4
        let s_22_5: bool = ((s_22_3) == (s_22_4));
        // N s_22_6: branch s_22_5 b35 b23
        if s_22_5 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var u#2111:u32
        let s_23_0: u32 = fn_state.u_2111;
        // C s_23_1: const #2u : u32
        let s_23_1: u32 = 2;
        // D s_23_2: cmp-eq s_23_0 s_23_1
        let s_23_2: bool = ((s_23_0) == (s_23_1));
        // N s_23_3: branch s_23_2 b34 b24
        if s_23_2 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var u#2111:u32
        let s_24_0: u32 = fn_state.u_2111;
        // C s_24_1: const #4u : u32
        let s_24_1: u32 = 4;
        // D s_24_2: cmp-eq s_24_0 s_24_1
        let s_24_2: bool = ((s_24_0) == (s_24_1));
        // D s_24_3: write-var gs#159662 <= s_24_2
        fn_state.gs_159662 = s_24_2;
        // N s_24_4: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var gs#159662:u8
        let s_25_0: bool = fn_state.gs_159662;
        // D s_25_1: write-var gs#159663 <= s_25_0
        fn_state.gs_159663 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#159663:u8
        let s_26_0: bool = fn_state.gs_159663;
        // N s_26_1: assert s_26_0
        let s_26_1: () = assert!(s_26_0);
        // C s_26_2: const #1u : u32
        let s_26_2: u32 = 1;
        // D s_26_3: read-var u#2111:u32
        let s_26_3: u32 = fn_state.u_2111;
        // D s_26_4: cmp-eq s_26_2 s_26_3
        let s_26_4: bool = ((s_26_2) == (s_26_3));
        // D s_26_5: not s_26_4
        let s_26_5: bool = !s_26_4;
        // N s_26_6: branch s_26_5 b29 b27
        if s_26_5 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #1u : u8
        let s_27_0: bool = true;
        // D s_27_1: write-var rn_unknown <= s_27_0
        fn_state.rn_unknown = s_27_0;
        // N s_27_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_28_0: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #2u : u32
        let s_29_0: u32 = 2;
        // D s_29_1: read-var u#2111:u32
        let s_29_1: u32 = fn_state.u_2111;
        // D s_29_2: cmp-eq s_29_0 s_29_1
        let s_29_2: bool = ((s_29_0) == (s_29_1));
        // D s_29_3: not s_29_2
        let s_29_3: bool = !s_29_2;
        // N s_29_4: branch s_29_3 b31 b30
        if s_29_3 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_30_0: panic
        panic!("{:?}", ());
        // N s_30_1: return
        return;
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #4u : u32
        let s_31_0: u32 = 4;
        // D s_31_1: read-var u#2111:u32
        let s_31_1: u32 = fn_state.u_2111;
        // D s_31_2: cmp-eq s_31_0 s_31_1
        let s_31_2: bool = ((s_31_0) == (s_31_1));
        // D s_31_3: not s_31_2
        let s_31_3: bool = !s_31_2;
        // N s_31_4: branch s_31_3 b33 b32
        if s_31_3 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #() : ()
        let s_32_0: () = ();
        // S s_32_1: call EndOfInstruction(s_32_0)
        let s_32_1: () = EndOfInstruction(state, tracer, s_32_0);
        // N s_32_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_33_0: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #1u : u8
        let s_34_0: bool = true;
        // D s_34_1: write-var gs#159662 <= s_34_0
        fn_state.gs_159662 = s_34_0;
        // N s_34_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #1u : u8
        let s_35_0: bool = true;
        // D s_35_1: write-var gs#159663 <= s_35_0
        fn_state.gs_159663 = s_35_0;
        // N s_35_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #31s : i
        let s_36_0: i128 = 31;
        // D s_36_1: read-var n:i64
        let s_36_1: i64 = fn_state.n;
        // D s_36_2: cast zx s_36_1 -> i
        let s_36_2: i128 = (i128::try_from(s_36_1).unwrap());
        // D s_36_3: call neq_int(s_36_2, s_36_0)
        let s_36_3: bool = neq_int(state, tracer, s_36_2, s_36_0);
        // D s_36_4: write-var gs#159660 <= s_36_3
        fn_state.gs_159660 = s_36_3;
        // N s_36_5: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #5u : u32
        let s_37_0: u32 = 5;
        // S s_37_1: call ConstrainUnpredictable(s_37_0)
        let s_37_1: u32 = ConstrainUnpredictable(state, tracer, s_37_0);
        // D s_37_2: write-var u#2110 <= s_37_1
        fn_state.u_2110 = s_37_1;
        // D s_37_3: read-var u#2110:u32
        let s_37_3: u32 = fn_state.u_2110;
        // C s_37_4: const #1u : u32
        let s_37_4: u32 = 1;
        // D s_37_5: cmp-eq s_37_3 s_37_4
        let s_37_5: bool = ((s_37_3) == (s_37_4));
        // N s_37_6: branch s_37_5 b50 b38
        if s_37_5 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var u#2110:u32
        let s_38_0: u32 = fn_state.u_2110;
        // C s_38_1: const #2u : u32
        let s_38_1: u32 = 2;
        // D s_38_2: cmp-eq s_38_0 s_38_1
        let s_38_2: bool = ((s_38_0) == (s_38_1));
        // N s_38_3: branch s_38_2 b49 b39
        if s_38_2 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var u#2110:u32
        let s_39_0: u32 = fn_state.u_2110;
        // C s_39_1: const #4u : u32
        let s_39_1: u32 = 4;
        // D s_39_2: cmp-eq s_39_0 s_39_1
        let s_39_2: bool = ((s_39_0) == (s_39_1));
        // D s_39_3: write-var gs#159671 <= s_39_2
        fn_state.gs_159671 = s_39_2;
        // N s_39_4: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var gs#159671:u8
        let s_40_0: bool = fn_state.gs_159671;
        // D s_40_1: write-var gs#159672 <= s_40_0
        fn_state.gs_159672 = s_40_0;
        // N s_40_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var gs#159672:u8
        let s_41_0: bool = fn_state.gs_159672;
        // N s_41_1: assert s_41_0
        let s_41_1: () = assert!(s_41_0);
        // C s_41_2: const #1u : u32
        let s_41_2: u32 = 1;
        // D s_41_3: read-var u#2110:u32
        let s_41_3: u32 = fn_state.u_2110;
        // D s_41_4: cmp-eq s_41_2 s_41_3
        let s_41_4: bool = ((s_41_2) == (s_41_3));
        // D s_41_5: not s_41_4
        let s_41_5: bool = !s_41_4;
        // N s_41_6: branch s_41_5 b44 b42
        if s_41_5 {
            return block_44(state, tracer, fn_state);
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
        // D s_42_1: write-var rt_unknown <= s_42_0
        fn_state.rt_unknown = s_42_0;
        // N s_42_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_43_0: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #2u : u32
        let s_44_0: u32 = 2;
        // D s_44_1: read-var u#2110:u32
        let s_44_1: u32 = fn_state.u_2110;
        // D s_44_2: cmp-eq s_44_0 s_44_1
        let s_44_2: bool = ((s_44_0) == (s_44_1));
        // D s_44_3: not s_44_2
        let s_44_3: bool = !s_44_2;
        // N s_44_4: branch s_44_3 b46 b45
        if s_44_3 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_45_0: panic
        panic!("{:?}", ());
        // N s_45_1: return
        return;
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #4u : u32
        let s_46_0: u32 = 4;
        // D s_46_1: read-var u#2110:u32
        let s_46_1: u32 = fn_state.u_2110;
        // D s_46_2: cmp-eq s_46_0 s_46_1
        let s_46_2: bool = ((s_46_0) == (s_46_1));
        // D s_46_3: not s_46_2
        let s_46_3: bool = !s_46_2;
        // N s_46_4: branch s_46_3 b48 b47
        if s_46_3 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #() : ()
        let s_47_0: () = ();
        // S s_47_1: call EndOfInstruction(s_47_0)
        let s_47_1: () = EndOfInstruction(state, tracer, s_47_0);
        // N s_47_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_48_0: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #1u : u8
        let s_49_0: bool = true;
        // D s_49_1: write-var gs#159671 <= s_49_0
        fn_state.gs_159671 = s_49_0;
        // N s_49_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #1u : u8
        let s_50_0: bool = true;
        // D s_50_1: write-var gs#159672 <= s_50_0
        fn_state.gs_159672 = s_50_0;
        // N s_50_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #1u : u8
        let s_51_0: bool = true;
        // D s_51_1: write-var gs#159658 <= s_51_0
        fn_state.gs_159658 = s_51_0;
        // N s_51_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #3u : u32
        let s_52_0: u32 = 3;
        // S s_52_1: call ConstrainUnpredictable(s_52_0)
        let s_52_1: u32 = ConstrainUnpredictable(state, tracer, s_52_0);
        // D s_52_2: write-var c <= s_52_1
        fn_state.c = s_52_1;
        // D s_52_3: read-var c:u32
        let s_52_3: u32 = fn_state.c;
        // C s_52_4: const #1u : u32
        let s_52_4: u32 = 1;
        // D s_52_5: cmp-eq s_52_3 s_52_4
        let s_52_5: bool = ((s_52_3) == (s_52_4));
        // N s_52_6: branch s_52_5 b65 b53
        if s_52_5 {
            return block_65(state, tracer, fn_state);
        } else {
            return block_53(state, tracer, fn_state);
        };
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_53_0: read-var c:u32
        let s_53_0: u32 = fn_state.c;
        // C s_53_1: const #2u : u32
        let s_53_1: u32 = 2;
        // D s_53_2: cmp-eq s_53_0 s_53_1
        let s_53_2: bool = ((s_53_0) == (s_53_1));
        // N s_53_3: branch s_53_2 b64 b54
        if s_53_2 {
            return block_64(state, tracer, fn_state);
        } else {
            return block_54(state, tracer, fn_state);
        };
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var c:u32
        let s_54_0: u32 = fn_state.c;
        // C s_54_1: const #4u : u32
        let s_54_1: u32 = 4;
        // D s_54_2: cmp-eq s_54_0 s_54_1
        let s_54_2: bool = ((s_54_0) == (s_54_1));
        // D s_54_3: write-var gs#159680 <= s_54_2
        fn_state.gs_159680 = s_54_2;
        // N s_54_4: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_55_0: read-var gs#159680:u8
        let s_55_0: bool = fn_state.gs_159680;
        // D s_55_1: write-var gs#159681 <= s_55_0
        fn_state.gs_159681 = s_55_0;
        // N s_55_2: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var gs#159681:u8
        let s_56_0: bool = fn_state.gs_159681;
        // N s_56_1: assert s_56_0
        let s_56_1: () = assert!(s_56_0);
        // C s_56_2: const #1u : u32
        let s_56_2: u32 = 1;
        // D s_56_3: read-var c:u32
        let s_56_3: u32 = fn_state.c;
        // D s_56_4: cmp-eq s_56_2 s_56_3
        let s_56_4: bool = ((s_56_2) == (s_56_3));
        // D s_56_5: not s_56_4
        let s_56_5: bool = !s_56_4;
        // N s_56_6: branch s_56_5 b59 b57
        if s_56_5 {
            return block_59(state, tracer, fn_state);
        } else {
            return block_57(state, tracer, fn_state);
        };
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #1u : u8
        let s_57_0: bool = true;
        // D s_57_1: write-var rt_unknown <= s_57_0
        fn_state.rt_unknown = s_57_0;
        // N s_57_2: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_58_0: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #2u : u32
        let s_59_0: u32 = 2;
        // D s_59_1: read-var c:u32
        let s_59_1: u32 = fn_state.c;
        // D s_59_2: cmp-eq s_59_0 s_59_1
        let s_59_2: bool = ((s_59_0) == (s_59_1));
        // D s_59_3: not s_59_2
        let s_59_3: bool = !s_59_2;
        // N s_59_4: branch s_59_3 b61 b60
        if s_59_3 {
            return block_61(state, tracer, fn_state);
        } else {
            return block_60(state, tracer, fn_state);
        };
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_60_0: panic
        panic!("{:?}", ());
        // N s_60_1: return
        return;
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #4u : u32
        let s_61_0: u32 = 4;
        // D s_61_1: read-var c:u32
        let s_61_1: u32 = fn_state.c;
        // D s_61_2: cmp-eq s_61_0 s_61_1
        let s_61_2: bool = ((s_61_0) == (s_61_1));
        // D s_61_3: not s_61_2
        let s_61_3: bool = !s_61_2;
        // N s_61_4: branch s_61_3 b63 b62
        if s_61_3 {
            return block_63(state, tracer, fn_state);
        } else {
            return block_62(state, tracer, fn_state);
        };
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #() : ()
        let s_62_0: () = ();
        // S s_62_1: call EndOfInstruction(s_62_0)
        let s_62_1: () = EndOfInstruction(state, tracer, s_62_0);
        // N s_62_2: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_63_0: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #1u : u8
        let s_64_0: bool = true;
        // D s_64_1: write-var gs#159680 <= s_64_0
        fn_state.gs_159680 = s_64_0;
        // N s_64_2: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #1u : u8
        let s_65_0: bool = true;
        // D s_65_1: write-var gs#159681 <= s_65_0
        fn_state.gs_159681 = s_65_0;
        // N s_65_2: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_66_0: read-var t:i64
        let s_66_0: i64 = fn_state.t;
        // D s_66_1: cast zx s_66_0 -> i
        let s_66_1: i128 = (i128::try_from(s_66_0).unwrap());
        // D s_66_2: read-var t2:i64
        let s_66_2: i64 = fn_state.t2;
        // D s_66_3: cast zx s_66_2 -> i
        let s_66_3: i128 = (i128::try_from(s_66_2).unwrap());
        // D s_66_4: cmp-eq s_66_1 s_66_3
        let s_66_4: bool = ((s_66_1) == (s_66_3));
        // D s_66_5: write-var gs#159655 <= s_66_4
        fn_state.gs_159655 = s_66_4;
        // N s_66_6: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #1u : u8
        let s_67_0: bool = true;
        // D s_67_1: write-var gs#159654 <= s_67_0
        fn_state.gs_159654 = s_67_0;
        // N s_67_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #64s : i64
        let s_68_0: i64 = 64;
        // D s_68_1: write-var ga#259568 <= s_68_0
        fn_state.ga_259568 = s_68_0;
        // N s_68_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #0u : u32
        let s_69_0: u32 = 0;
        // D s_69_1: write-var memop <= s_69_0
        fn_state.memop = s_69_0;
        // N s_69_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
