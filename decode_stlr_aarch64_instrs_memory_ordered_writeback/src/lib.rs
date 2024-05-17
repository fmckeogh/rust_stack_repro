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
use execute_aarch64_instrs_memory_ordered::*;
use common::*;
pub fn decode_stlr_aarch64_instrs_memory_ordered_writeback<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rt: u8,
    Rn: u8,
    size: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_159338: bool,
        t: i64,
        ga_259340: i64,
        n: i64,
        regsize: i64,
        offset: i64,
        gs_159340: bool,
        datasize: i64,
        rt_unknown: bool,
        gs_159341: bool,
        c: u32,
        gs_159342: bool,
        Rt: u8,
        Rn: u8,
        size: u8,
    }
    let fn_state = FunctionState {
        Rt,
        Rn,
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
        // D s_0_10: read-var size:u8
        let s_0_10: u8 = fn_state.size;
        // D s_0_11: cast zx s_0_10 -> bv
        let s_0_11: Bits = Bits::new(s_0_10 as u128, 2u16);
        // C s_0_12: const #3u : u8
        let s_0_12: u8 = 3;
        // C s_0_13: cast zx s_0_12 -> bv
        let s_0_13: Bits = Bits::new(s_0_12 as u128, 2u16);
        // D s_0_14: cmp-eq s_0_11 s_0_13
        let s_0_14: bool = ((s_0_11) == (s_0_13));
        // N s_0_15: branch s_0_14 b27 b1
        if s_0_14 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #32s : i64
        let s_1_0: i64 = 32;
        // D s_1_1: write-var ga#259340 <= s_1_0
        fn_state.ga_259340 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var ga#259340:i64
        let s_2_0: i64 = fn_state.ga_259340;
        // D s_2_1: write-var regsize <= s_2_0
        fn_state.regsize = s_2_0;
        // D s_2_2: read-var size:u8
        let s_2_2: u8 = fn_state.size;
        // D s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 2u16);
        // D s_2_4: cast zx s_2_3 -> i
        let s_2_4: i128 = (s_2_3.value() as i128);
        // D s_2_5: cast reint s_2_4 -> i64
        let s_2_5: i64 = (s_2_4 as i64);
        // C s_2_6: const #8s : i64
        let s_2_6: i64 = 8;
        // D s_2_7: lsl s_2_6 s_2_5
        let s_2_7: i64 = s_2_6 << s_2_5;
        // D s_2_8: write-var datasize <= s_2_7
        fn_state.datasize = s_2_7;
        // D s_2_9: read-var size:u8
        let s_2_9: u8 = fn_state.size;
        // D s_2_10: cast zx s_2_9 -> bv
        let s_2_10: Bits = Bits::new(s_2_9 as u128, 2u16);
        // D s_2_11: cast zx s_2_10 -> i
        let s_2_11: i128 = (s_2_10.value() as i128);
        // D s_2_12: cast reint s_2_11 -> i64
        let s_2_12: i64 = (s_2_11 as i64);
        // C s_2_13: const #1s : i64
        let s_2_13: i64 = 1;
        // D s_2_14: lsl s_2_13 s_2_12
        let s_2_14: i64 = s_2_13 << s_2_12;
        // C s_2_15: const #-1s : i
        let s_2_15: i128 = -1;
        // D s_2_16: cast zx s_2_14 -> i
        let s_2_16: i128 = (i128::try_from(s_2_14).unwrap());
        // D s_2_17: mul s_2_15 s_2_16
        let s_2_17: i128 = ((s_2_15) * (s_2_16));
        // D s_2_18: cast reint s_2_17 -> i64
        let s_2_18: i64 = (s_2_17 as i64);
        // D s_2_19: write-var offset <= s_2_18
        fn_state.offset = s_2_18;
        // C s_2_20: const #0u : u8
        let s_2_20: bool = false;
        // D s_2_21: write-var rt_unknown <= s_2_20
        fn_state.rt_unknown = s_2_20;
        // D s_2_22: read-var n:i64
        let s_2_22: i64 = fn_state.n;
        // D s_2_23: cast zx s_2_22 -> i
        let s_2_23: i128 = (i128::try_from(s_2_22).unwrap());
        // D s_2_24: read-var t:i64
        let s_2_24: i64 = fn_state.t;
        // D s_2_25: cast zx s_2_24 -> i
        let s_2_25: i128 = (i128::try_from(s_2_24).unwrap());
        // D s_2_26: cmp-eq s_2_23 s_2_25
        let s_2_26: bool = ((s_2_23) == (s_2_25));
        // N s_2_27: branch s_2_26 b26 b3
        if s_2_26 {
            return block_26(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#159338 <= s_3_0
        fn_state.gs_159338 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#159338:u8
        let s_4_0: bool = fn_state.gs_159338;
        // N s_4_1: branch s_4_0 b7 b5
        if s_4_0 {
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
        // D s_6_3: read-var regsize:i64
        let s_6_3: i64 = fn_state.regsize;
        // D s_6_4: cast zx s_6_3 -> i
        let s_6_4: i128 = (i128::try_from(s_6_3).unwrap());
        // D s_6_5: cast reint s_6_4 -> i64
        let s_6_5: i64 = (s_6_4 as i64);
        // D s_6_6: read-var offset:i64
        let s_6_6: i64 = fn_state.offset;
        // D s_6_7: cast zx s_6_6 -> i
        let s_6_7: i128 = (i128::try_from(s_6_6).unwrap());
        // C s_6_8: const #0u : u8
        let s_6_8: bool = false;
        // C s_6_9: const #1u : u32
        let s_6_9: u32 = 1;
        // D s_6_10: read-var n:i64
        let s_6_10: i64 = fn_state.n;
        // D s_6_11: read-var rt_unknown:u8
        let s_6_11: bool = fn_state.rt_unknown;
        // D s_6_12: read-var t:i64
        let s_6_12: i64 = fn_state.t;
        // C s_6_13: const #1u : u8
        let s_6_13: bool = true;
        // C s_6_14: const #1u : u8
        let s_6_14: bool = true;
        // D s_6_15: call execute_aarch64_instrs_memory_ordered(s_6_2, s_6_8, s_6_9, s_6_10, s_6_7, s_6_5, s_6_11, s_6_12, s_6_13, s_6_14)
        let s_6_15: () = execute_aarch64_instrs_memory_ordered(
            state,
            tracer,
            s_6_2,
            s_6_8,
            s_6_9,
            s_6_10,
            s_6_7,
            s_6_5,
            s_6_11,
            s_6_12,
            s_6_13,
            s_6_14,
        );
        // N s_6_16: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #2u : u32
        let s_7_0: u32 = 2;
        // S s_7_1: call ConstrainUnpredictable(s_7_0)
        let s_7_1: u32 = ConstrainUnpredictable(state, tracer, s_7_0);
        // D s_7_2: write-var c <= s_7_1
        fn_state.c = s_7_1;
        // D s_7_3: read-var c:u32
        let s_7_3: u32 = fn_state.c;
        // C s_7_4: const #0u : u32
        let s_7_4: u32 = 0;
        // D s_7_5: cmp-eq s_7_3 s_7_4
        let s_7_5: bool = ((s_7_3) == (s_7_4));
        // N s_7_6: branch s_7_5 b25 b8
        if s_7_5 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var c:u32
        let s_8_0: u32 = fn_state.c;
        // C s_8_1: const #1u : u32
        let s_8_1: u32 = 1;
        // D s_8_2: cmp-eq s_8_0 s_8_1
        let s_8_2: bool = ((s_8_0) == (s_8_1));
        // N s_8_3: branch s_8_2 b24 b9
        if s_8_2 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var c:u32
        let s_9_0: u32 = fn_state.c;
        // C s_9_1: const #2u : u32
        let s_9_1: u32 = 2;
        // D s_9_2: cmp-eq s_9_0 s_9_1
        let s_9_2: bool = ((s_9_0) == (s_9_1));
        // N s_9_3: branch s_9_2 b23 b10
        if s_9_2 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var c:u32
        let s_10_0: u32 = fn_state.c;
        // C s_10_1: const #4u : u32
        let s_10_1: u32 = 4;
        // D s_10_2: cmp-eq s_10_0 s_10_1
        let s_10_2: bool = ((s_10_0) == (s_10_1));
        // D s_10_3: write-var gs#159340 <= s_10_2
        fn_state.gs_159340 = s_10_2;
        // N s_10_4: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#159340:u8
        let s_11_0: bool = fn_state.gs_159340;
        // D s_11_1: write-var gs#159341 <= s_11_0
        fn_state.gs_159341 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#159341:u8
        let s_12_0: bool = fn_state.gs_159341;
        // D s_12_1: write-var gs#159342 <= s_12_0
        fn_state.gs_159342 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#159342:u8
        let s_13_0: bool = fn_state.gs_159342;
        // N s_13_1: assert s_13_0
        let s_13_1: () = assert!(s_13_0);
        // C s_13_2: const #0u : u32
        let s_13_2: u32 = 0;
        // D s_13_3: read-var c:u32
        let s_13_3: u32 = fn_state.c;
        // D s_13_4: cmp-eq s_13_2 s_13_3
        let s_13_4: bool = ((s_13_2) == (s_13_3));
        // D s_13_5: not s_13_4
        let s_13_5: bool = !s_13_4;
        // N s_13_6: branch s_13_5 b16 b14
        if s_13_5 {
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
        // C s_14_0: const #0u : u8
        let s_14_0: bool = false;
        // D s_14_1: write-var rt_unknown <= s_14_0
        fn_state.rt_unknown = s_14_0;
        // N s_14_2: jump b15
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
        // C s_16_0: const #1u : u32
        let s_16_0: u32 = 1;
        // D s_16_1: read-var c:u32
        let s_16_1: u32 = fn_state.c;
        // D s_16_2: cmp-eq s_16_0 s_16_1
        let s_16_2: bool = ((s_16_0) == (s_16_1));
        // D s_16_3: not s_16_2
        let s_16_3: bool = !s_16_2;
        // N s_16_4: branch s_16_3 b18 b17
        if s_16_3 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #1u : u8
        let s_17_0: bool = true;
        // D s_17_1: write-var rt_unknown <= s_17_0
        fn_state.rt_unknown = s_17_0;
        // N s_17_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #2u : u32
        let s_18_0: u32 = 2;
        // D s_18_1: read-var c:u32
        let s_18_1: u32 = fn_state.c;
        // D s_18_2: cmp-eq s_18_0 s_18_1
        let s_18_2: bool = ((s_18_0) == (s_18_1));
        // D s_18_3: not s_18_2
        let s_18_3: bool = !s_18_2;
        // N s_18_4: branch s_18_3 b20 b19
        if s_18_3 {
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
        // N s_19_0: panic
        panic!("{:?}", ());
        // N s_19_1: return
        return;
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #4u : u32
        let s_20_0: u32 = 4;
        // D s_20_1: read-var c:u32
        let s_20_1: u32 = fn_state.c;
        // D s_20_2: cmp-eq s_20_0 s_20_1
        let s_20_2: bool = ((s_20_0) == (s_20_1));
        // D s_20_3: not s_20_2
        let s_20_3: bool = !s_20_2;
        // N s_20_4: branch s_20_3 b22 b21
        if s_20_3 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #() : ()
        let s_21_0: () = ();
        // S s_21_1: call EndOfInstruction(s_21_0)
        let s_21_1: () = EndOfInstruction(state, tracer, s_21_0);
        // N s_21_2: jump b15
        return block_15(state, tracer, fn_state);
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
        // C s_23_0: const #1u : u8
        let s_23_0: bool = true;
        // D s_23_1: write-var gs#159340 <= s_23_0
        fn_state.gs_159340 = s_23_0;
        // N s_23_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #1u : u8
        let s_24_0: bool = true;
        // D s_24_1: write-var gs#159341 <= s_24_0
        fn_state.gs_159341 = s_24_0;
        // N s_24_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #1u : u8
        let s_25_0: bool = true;
        // D s_25_1: write-var gs#159342 <= s_25_0
        fn_state.gs_159342 = s_25_0;
        // N s_25_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #31s : i
        let s_26_0: i128 = 31;
        // D s_26_1: read-var n:i64
        let s_26_1: i64 = fn_state.n;
        // D s_26_2: cast zx s_26_1 -> i
        let s_26_2: i128 = (i128::try_from(s_26_1).unwrap());
        // D s_26_3: call neq_int(s_26_2, s_26_0)
        let s_26_3: bool = neq_int(state, tracer, s_26_2, s_26_0);
        // D s_26_4: write-var gs#159338 <= s_26_3
        fn_state.gs_159338 = s_26_3;
        // N s_26_5: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #64s : i64
        let s_27_0: i64 = 64;
        // D s_27_1: write-var ga#259340 <= s_27_0
        fn_state.ga_259340 = s_27_0;
        // N s_27_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
