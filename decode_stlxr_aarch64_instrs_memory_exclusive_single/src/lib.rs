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
use execute_aarch64_instrs_memory_exclusive_single::*;
use ConstrainUnpredictable::*;
use EndOfInstruction::*;
use common::*;
pub fn decode_stlxr_aarch64_instrs_memory_exclusive_single<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rt: u8,
    Rn: u8,
    Rt2: u8,
    o0: bool,
    Rs: u8,
    L: bool,
    size: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        tagchecked: bool,
        t: i64,
        t2: i64,
        s: i64,
        ga_259850: i64,
        elsize: i64,
        gs_160064: bool,
        n: i64,
        acqrel: bool,
        memop: u32,
        gs_160073: bool,
        regsize: i64,
        rn_unknown: bool,
        gs_160061: bool,
        gs_160063: bool,
        gs_160059: bool,
        datasize: i64,
        rt_unknown: bool,
        c: u32,
        u_2118: u32,
        gs_160072: bool,
        Rt: u8,
        Rn: u8,
        Rt2: u8,
        o0: bool,
        Rs: u8,
        L: bool,
        size: u8,
    }
    let fn_state = FunctionState {
        Rt,
        Rn,
        Rt2,
        o0,
        Rs,
        L,
        size,
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
        // N s_0_31: branch s_0_30 b47 b1
        if s_0_30 {
            return block_47(state, tracer, fn_state);
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
        // D s_2_0: read-var size:u8
        let s_2_0: u8 = fn_state.size;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 2u16);
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (s_2_1.value() as i128);
        // D s_2_3: cast reint s_2_2 -> i64
        let s_2_3: i64 = (s_2_2 as i64);
        // C s_2_4: const #8s : i64
        let s_2_4: i64 = 8;
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
        // N s_2_11: branch s_2_10 b46 b3
        if s_2_10 {
            return block_46(state, tracer, fn_state);
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
        // D s_3_1: write-var ga#259850 <= s_3_0
        fn_state.ga_259850 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var ga#259850:i64
        let s_4_0: i64 = fn_state.ga_259850;
        // D s_4_1: write-var regsize <= s_4_0
        fn_state.regsize = s_4_0;
        // D s_4_2: read-var elsize:i64
        let s_4_2: i64 = fn_state.elsize;
        // D s_4_3: write-var datasize <= s_4_2
        fn_state.datasize = s_4_2;
        // C s_4_4: const #31s : i
        let s_4_4: i128 = 31;
        // D s_4_5: read-var n:i64
        let s_4_5: i64 = fn_state.n;
        // D s_4_6: cast zx s_4_5 -> i
        let s_4_6: i128 = (i128::try_from(s_4_5).unwrap());
        // D s_4_7: call neq_int(s_4_6, s_4_4)
        let s_4_7: bool = neq_int(state, tracer, s_4_6, s_4_4);
        // D s_4_8: write-var tagchecked <= s_4_7
        fn_state.tagchecked = s_4_7;
        // C s_4_9: const #0u : u8
        let s_4_9: bool = false;
        // D s_4_10: write-var rt_unknown <= s_4_9
        fn_state.rt_unknown = s_4_9;
        // C s_4_11: const #0u : u8
        let s_4_11: bool = false;
        // D s_4_12: write-var rn_unknown <= s_4_11
        fn_state.rn_unknown = s_4_11;
        // D s_4_13: read-var memop:u32
        let s_4_13: u32 = fn_state.memop;
        // C s_4_14: const #1u : u32
        let s_4_14: u32 = 1;
        // D s_4_15: cmp-eq s_4_13 s_4_14
        let s_4_15: bool = ((s_4_13) == (s_4_14));
        // N s_4_16: branch s_4_15 b7 b5
        if s_4_15 {
            return block_7(state, tracer, fn_state);
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
        // D s_6_0: read-var datasize:i64
        let s_6_0: i64 = fn_state.datasize;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: cast reint s_6_1 -> i64
        let s_6_2: i64 = (s_6_1 as i64);
        // D s_6_3: read-var elsize:i64
        let s_6_3: i64 = fn_state.elsize;
        // D s_6_4: cast zx s_6_3 -> i
        let s_6_4: i128 = (i128::try_from(s_6_3).unwrap());
        // D s_6_5: cast reint s_6_4 -> i64
        let s_6_5: i64 = (s_6_4 as i64);
        // D s_6_6: read-var regsize:i64
        let s_6_6: i64 = fn_state.regsize;
        // D s_6_7: cast zx s_6_6 -> i
        let s_6_7: i128 = (i128::try_from(s_6_6).unwrap());
        // D s_6_8: cast reint s_6_7 -> i64
        let s_6_8: i64 = (s_6_7 as i64);
        // D s_6_9: read-var acqrel:u8
        let s_6_9: bool = fn_state.acqrel;
        // D s_6_10: read-var memop:u32
        let s_6_10: u32 = fn_state.memop;
        // D s_6_11: read-var n:i64
        let s_6_11: i64 = fn_state.n;
        // C s_6_12: const #0u : u8
        let s_6_12: bool = false;
        // D s_6_13: read-var rn_unknown:u8
        let s_6_13: bool = fn_state.rn_unknown;
        // D s_6_14: read-var rt_unknown:u8
        let s_6_14: bool = fn_state.rt_unknown;
        // D s_6_15: read-var s:i64
        let s_6_15: i64 = fn_state.s;
        // D s_6_16: read-var t:i64
        let s_6_16: i64 = fn_state.t;
        // D s_6_17: read-var t2:i64
        let s_6_17: i64 = fn_state.t2;
        // D s_6_18: read-var tagchecked:u8
        let s_6_18: bool = fn_state.tagchecked;
        // D s_6_19: call execute_aarch64_instrs_memory_exclusive_single(s_6_9, s_6_2, s_6_5, s_6_10, s_6_11, s_6_12, s_6_8, s_6_13, s_6_14, s_6_15, s_6_16, s_6_17, s_6_18)
        let s_6_19: () = execute_aarch64_instrs_memory_exclusive_single(
            state,
            tracer,
            s_6_9,
            s_6_2,
            s_6_5,
            s_6_10,
            s_6_11,
            s_6_12,
            s_6_8,
            s_6_13,
            s_6_14,
            s_6_15,
            s_6_16,
            s_6_17,
            s_6_18,
        );
        // N s_6_20: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var s:i64
        let s_7_0: i64 = fn_state.s;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: read-var t:i64
        let s_7_2: i64 = fn_state.t;
        // D s_7_3: cast zx s_7_2 -> i
        let s_7_3: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_4: cmp-eq s_7_1 s_7_3
        let s_7_4: bool = ((s_7_1) == (s_7_3));
        // N s_7_5: branch s_7_4 b45 b8
        if s_7_4 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var gs#160059 <= s_8_0
        fn_state.gs_160059 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#160059:u8
        let s_9_0: bool = fn_state.gs_160059;
        // N s_9_1: branch s_9_0 b31 b10
        if s_9_0 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_10_0: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var s:i64
        let s_11_0: i64 = fn_state.s;
        // D s_11_1: cast zx s_11_0 -> i
        let s_11_1: i128 = (i128::try_from(s_11_0).unwrap());
        // D s_11_2: read-var n:i64
        let s_11_2: i64 = fn_state.n;
        // D s_11_3: cast zx s_11_2 -> i
        let s_11_3: i128 = (i128::try_from(s_11_2).unwrap());
        // D s_11_4: cmp-eq s_11_1 s_11_3
        let s_11_4: bool = ((s_11_1) == (s_11_3));
        // N s_11_5: branch s_11_4 b30 b12
        if s_11_4 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #0u : u8
        let s_12_0: bool = false;
        // D s_12_1: write-var gs#160061 <= s_12_0
        fn_state.gs_160061 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#160061:u8
        let s_13_0: bool = fn_state.gs_160061;
        // N s_13_1: branch s_13_0 b16 b14
        if s_13_0 {
            return block_16(state, tracer, fn_state);
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
        // N s_15_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #4u : u32
        let s_16_0: u32 = 4;
        // S s_16_1: call ConstrainUnpredictable(s_16_0)
        let s_16_1: u32 = ConstrainUnpredictable(state, tracer, s_16_0);
        // D s_16_2: write-var u#2118 <= s_16_1
        fn_state.u_2118 = s_16_1;
        // D s_16_3: read-var u#2118:u32
        let s_16_3: u32 = fn_state.u_2118;
        // C s_16_4: const #1u : u32
        let s_16_4: u32 = 1;
        // D s_16_5: cmp-eq s_16_3 s_16_4
        let s_16_5: bool = ((s_16_3) == (s_16_4));
        // N s_16_6: branch s_16_5 b29 b17
        if s_16_5 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var u#2118:u32
        let s_17_0: u32 = fn_state.u_2118;
        // C s_17_1: const #2u : u32
        let s_17_1: u32 = 2;
        // D s_17_2: cmp-eq s_17_0 s_17_1
        let s_17_2: bool = ((s_17_0) == (s_17_1));
        // N s_17_3: branch s_17_2 b28 b18
        if s_17_2 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var u#2118:u32
        let s_18_0: u32 = fn_state.u_2118;
        // C s_18_1: const #4u : u32
        let s_18_1: u32 = 4;
        // D s_18_2: cmp-eq s_18_0 s_18_1
        let s_18_2: bool = ((s_18_0) == (s_18_1));
        // D s_18_3: write-var gs#160063 <= s_18_2
        fn_state.gs_160063 = s_18_2;
        // N s_18_4: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#160063:u8
        let s_19_0: bool = fn_state.gs_160063;
        // D s_19_1: write-var gs#160064 <= s_19_0
        fn_state.gs_160064 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#160064:u8
        let s_20_0: bool = fn_state.gs_160064;
        // N s_20_1: assert s_20_0
        let s_20_1: () = assert!(s_20_0);
        // C s_20_2: const #1u : u32
        let s_20_2: u32 = 1;
        // D s_20_3: read-var u#2118:u32
        let s_20_3: u32 = fn_state.u_2118;
        // D s_20_4: cmp-eq s_20_2 s_20_3
        let s_20_4: bool = ((s_20_2) == (s_20_3));
        // D s_20_5: not s_20_4
        let s_20_5: bool = !s_20_4;
        // N s_20_6: branch s_20_5 b23 b21
        if s_20_5 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #1u : u8
        let s_21_0: bool = true;
        // D s_21_1: write-var rn_unknown <= s_21_0
        fn_state.rn_unknown = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_22_0: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #2u : u32
        let s_23_0: u32 = 2;
        // D s_23_1: read-var u#2118:u32
        let s_23_1: u32 = fn_state.u_2118;
        // D s_23_2: cmp-eq s_23_0 s_23_1
        let s_23_2: bool = ((s_23_0) == (s_23_1));
        // D s_23_3: not s_23_2
        let s_23_3: bool = !s_23_2;
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
        // N s_24_0: panic
        panic!("{:?}", ());
        // N s_24_1: return
        return;
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #4u : u32
        let s_25_0: u32 = 4;
        // D s_25_1: read-var u#2118:u32
        let s_25_1: u32 = fn_state.u_2118;
        // D s_25_2: cmp-eq s_25_0 s_25_1
        let s_25_2: bool = ((s_25_0) == (s_25_1));
        // D s_25_3: not s_25_2
        let s_25_3: bool = !s_25_2;
        // N s_25_4: branch s_25_3 b27 b26
        if s_25_3 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #() : ()
        let s_26_0: () = ();
        // S s_26_1: call EndOfInstruction(s_26_0)
        let s_26_1: () = EndOfInstruction(state, tracer, s_26_0);
        // N s_26_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_27_0: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #1u : u8
        let s_28_0: bool = true;
        // D s_28_1: write-var gs#160063 <= s_28_0
        fn_state.gs_160063 = s_28_0;
        // N s_28_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #1u : u8
        let s_29_0: bool = true;
        // D s_29_1: write-var gs#160064 <= s_29_0
        fn_state.gs_160064 = s_29_0;
        // N s_29_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #31s : i
        let s_30_0: i128 = 31;
        // D s_30_1: read-var n:i64
        let s_30_1: i64 = fn_state.n;
        // D s_30_2: cast zx s_30_1 -> i
        let s_30_2: i128 = (i128::try_from(s_30_1).unwrap());
        // D s_30_3: call neq_int(s_30_2, s_30_0)
        let s_30_3: bool = neq_int(state, tracer, s_30_2, s_30_0);
        // D s_30_4: write-var gs#160061 <= s_30_3
        fn_state.gs_160061 = s_30_3;
        // N s_30_5: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #5u : u32
        let s_31_0: u32 = 5;
        // S s_31_1: call ConstrainUnpredictable(s_31_0)
        let s_31_1: u32 = ConstrainUnpredictable(state, tracer, s_31_0);
        // D s_31_2: write-var c <= s_31_1
        fn_state.c = s_31_1;
        // D s_31_3: read-var c:u32
        let s_31_3: u32 = fn_state.c;
        // C s_31_4: const #1u : u32
        let s_31_4: u32 = 1;
        // D s_31_5: cmp-eq s_31_3 s_31_4
        let s_31_5: bool = ((s_31_3) == (s_31_4));
        // N s_31_6: branch s_31_5 b44 b32
        if s_31_5 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var c:u32
        let s_32_0: u32 = fn_state.c;
        // C s_32_1: const #2u : u32
        let s_32_1: u32 = 2;
        // D s_32_2: cmp-eq s_32_0 s_32_1
        let s_32_2: bool = ((s_32_0) == (s_32_1));
        // N s_32_3: branch s_32_2 b43 b33
        if s_32_2 {
            return block_43(state, tracer, fn_state);
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
        // C s_33_1: const #4u : u32
        let s_33_1: u32 = 4;
        // D s_33_2: cmp-eq s_33_0 s_33_1
        let s_33_2: bool = ((s_33_0) == (s_33_1));
        // D s_33_3: write-var gs#160072 <= s_33_2
        fn_state.gs_160072 = s_33_2;
        // N s_33_4: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var gs#160072:u8
        let s_34_0: bool = fn_state.gs_160072;
        // D s_34_1: write-var gs#160073 <= s_34_0
        fn_state.gs_160073 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var gs#160073:u8
        let s_35_0: bool = fn_state.gs_160073;
        // N s_35_1: assert s_35_0
        let s_35_1: () = assert!(s_35_0);
        // C s_35_2: const #1u : u32
        let s_35_2: u32 = 1;
        // D s_35_3: read-var c:u32
        let s_35_3: u32 = fn_state.c;
        // D s_35_4: cmp-eq s_35_2 s_35_3
        let s_35_4: bool = ((s_35_2) == (s_35_3));
        // D s_35_5: not s_35_4
        let s_35_5: bool = !s_35_4;
        // N s_35_6: branch s_35_5 b38 b36
        if s_35_5 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #1u : u8
        let s_36_0: bool = true;
        // D s_36_1: write-var rt_unknown <= s_36_0
        fn_state.rt_unknown = s_36_0;
        // N s_36_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_37_0: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #2u : u32
        let s_38_0: u32 = 2;
        // D s_38_1: read-var c:u32
        let s_38_1: u32 = fn_state.c;
        // D s_38_2: cmp-eq s_38_0 s_38_1
        let s_38_2: bool = ((s_38_0) == (s_38_1));
        // D s_38_3: not s_38_2
        let s_38_3: bool = !s_38_2;
        // N s_38_4: branch s_38_3 b40 b39
        if s_38_3 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_39_0: panic
        panic!("{:?}", ());
        // N s_39_1: return
        return;
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #4u : u32
        let s_40_0: u32 = 4;
        // D s_40_1: read-var c:u32
        let s_40_1: u32 = fn_state.c;
        // D s_40_2: cmp-eq s_40_0 s_40_1
        let s_40_2: bool = ((s_40_0) == (s_40_1));
        // D s_40_3: not s_40_2
        let s_40_3: bool = !s_40_2;
        // N s_40_4: branch s_40_3 b42 b41
        if s_40_3 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #() : ()
        let s_41_0: () = ();
        // S s_41_1: call EndOfInstruction(s_41_0)
        let s_41_1: () = EndOfInstruction(state, tracer, s_41_0);
        // N s_41_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_42_0: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #1u : u8
        let s_43_0: bool = true;
        // D s_43_1: write-var gs#160072 <= s_43_0
        fn_state.gs_160072 = s_43_0;
        // N s_43_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #1u : u8
        let s_44_0: bool = true;
        // D s_44_1: write-var gs#160073 <= s_44_0
        fn_state.gs_160073 = s_44_0;
        // N s_44_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #1u : u8
        let s_45_0: bool = true;
        // D s_45_1: write-var gs#160059 <= s_45_0
        fn_state.gs_160059 = s_45_0;
        // N s_45_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #64s : i64
        let s_46_0: i64 = 64;
        // D s_46_1: write-var ga#259850 <= s_46_0
        fn_state.ga_259850 = s_46_0;
        // N s_46_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #0u : u32
        let s_47_0: u32 = 0;
        // D s_47_1: write-var memop <= s_47_0
        fn_state.memop = s_47_0;
        // N s_47_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
