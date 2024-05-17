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
use execute_aarch64_instrs_memory_ordered::*;
use common::*;
pub fn decode_stlrb_aarch64_instrs_memory_ordered<T: Tracer>(
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
        t: i64,
        elsize: i64,
        n: i64,
        memop: u32,
        ga_259362: i64,
        limitedordered: bool,
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
        // D s_0_10: read-var o0:u8
        let s_0_10: bool = fn_state.o0;
        // D s_0_11: cast zx s_0_10 -> bv
        let s_0_11: Bits = Bits::new(s_0_10 as u128, 1u16);
        // C s_0_12: const #0u : u8
        let s_0_12: bool = false;
        // C s_0_13: cast zx s_0_12 -> bv
        let s_0_13: Bits = Bits::new(s_0_12 as u128, 1u16);
        // D s_0_14: cmp-eq s_0_11 s_0_13
        let s_0_14: bool = ((s_0_11) == (s_0_13));
        // D s_0_15: write-var limitedordered <= s_0_14
        fn_state.limitedordered = s_0_14;
        // D s_0_16: read-var L:u8
        let s_0_16: bool = fn_state.L;
        // D s_0_17: cast zx s_0_16 -> bv
        let s_0_17: Bits = Bits::new(s_0_16 as u128, 1u16);
        // C s_0_18: const #1u : u8
        let s_0_18: bool = true;
        // C s_0_19: cast zx s_0_18 -> bv
        let s_0_19: Bits = Bits::new(s_0_18 as u128, 1u16);
        // D s_0_20: cmp-eq s_0_17 s_0_19
        let s_0_20: bool = ((s_0_17) == (s_0_19));
        // N s_0_21: branch s_0_20 b6 b1
        if s_0_20 {
            return block_6(state, tracer, fn_state);
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
        // N s_2_11: branch s_2_10 b5 b3
        if s_2_10 {
            return block_5(state, tracer, fn_state);
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
        // D s_3_1: write-var ga#259362 <= s_3_0
        fn_state.ga_259362 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var ga#259362:i64
        let s_4_0: i64 = fn_state.ga_259362;
        // D s_4_1: read-var elsize:i64
        let s_4_1: i64 = fn_state.elsize;
        // C s_4_2: const #31s : i
        let s_4_2: i128 = 31;
        // D s_4_3: read-var n:i64
        let s_4_3: i64 = fn_state.n;
        // D s_4_4: cast zx s_4_3 -> i
        let s_4_4: i128 = (i128::try_from(s_4_3).unwrap());
        // D s_4_5: call neq_int(s_4_4, s_4_2)
        let s_4_5: bool = neq_int(state, tracer, s_4_4, s_4_2);
        // D s_4_6: cast zx s_4_1 -> i
        let s_4_6: i128 = (i128::try_from(s_4_1).unwrap());
        // D s_4_7: cast reint s_4_6 -> i64
        let s_4_7: i64 = (s_4_6 as i64);
        // D s_4_8: cast zx s_4_0 -> i
        let s_4_8: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_9: cast reint s_4_8 -> i64
        let s_4_9: i64 = (s_4_8 as i64);
        // C s_4_10: const #0s : i
        let s_4_10: i128 = 0;
        // D s_4_11: read-var limitedordered:u8
        let s_4_11: bool = fn_state.limitedordered;
        // D s_4_12: read-var memop:u32
        let s_4_12: u32 = fn_state.memop;
        // D s_4_13: read-var n:i64
        let s_4_13: i64 = fn_state.n;
        // C s_4_14: const #0u : u8
        let s_4_14: bool = false;
        // D s_4_15: read-var t:i64
        let s_4_15: i64 = fn_state.t;
        // C s_4_16: const #0u : u8
        let s_4_16: bool = false;
        // D s_4_17: call execute_aarch64_instrs_memory_ordered(s_4_7, s_4_11, s_4_12, s_4_13, s_4_10, s_4_9, s_4_14, s_4_15, s_4_5, s_4_16)
        let s_4_17: () = execute_aarch64_instrs_memory_ordered(
            state,
            tracer,
            s_4_7,
            s_4_11,
            s_4_12,
            s_4_13,
            s_4_10,
            s_4_9,
            s_4_14,
            s_4_15,
            s_4_5,
            s_4_16,
        );
        // N s_4_18: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #64s : i64
        let s_5_0: i64 = 64;
        // D s_5_1: write-var ga#259362 <= s_5_0
        fn_state.ga_259362 = s_5_0;
        // N s_5_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #0u : u32
        let s_6_0: u32 = 0;
        // D s_6_1: write-var memop <= s_6_0
        fn_state.memop = s_6_0;
        // N s_6_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
