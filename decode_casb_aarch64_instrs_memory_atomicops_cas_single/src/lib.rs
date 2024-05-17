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
use HaveAtomicExt::*;
use execute_aarch64_instrs_memory_atomicops_cas_single::*;
use common::*;
pub fn decode_casb_aarch64_instrs_memory_atomicops_cas_single<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rt: u8,
    Rn: u8,
    o0: bool,
    Rs: u8,
    L: bool,
    size: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        t: i64,
        s: i64,
        datasize: i64,
        n: i64,
        ga_250946: i64,
        Rt: u8,
        Rn: u8,
        o0: bool,
        Rs: u8,
        L: bool,
        size: u8,
    }
    let fn_state = FunctionState {
        Rt,
        Rn,
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
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call HaveAtomicExt(s_0_0)
        let s_0_1: bool = HaveAtomicExt(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b5 b1
        if s_0_2 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var Rn:u8
        let s_1_0: u8 = fn_state.Rn;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 5u16);
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (s_1_1.value() as i128);
        // D s_1_3: cast reint s_1_2 -> i64
        let s_1_3: i64 = (s_1_2 as i64);
        // D s_1_4: write-var n <= s_1_3
        fn_state.n = s_1_3;
        // D s_1_5: read-var Rt:u8
        let s_1_5: u8 = fn_state.Rt;
        // D s_1_6: cast zx s_1_5 -> bv
        let s_1_6: Bits = Bits::new(s_1_5 as u128, 5u16);
        // D s_1_7: cast zx s_1_6 -> i
        let s_1_7: i128 = (s_1_6.value() as i128);
        // D s_1_8: cast reint s_1_7 -> i64
        let s_1_8: i64 = (s_1_7 as i64);
        // D s_1_9: write-var t <= s_1_8
        fn_state.t = s_1_8;
        // D s_1_10: read-var Rs:u8
        let s_1_10: u8 = fn_state.Rs;
        // D s_1_11: cast zx s_1_10 -> bv
        let s_1_11: Bits = Bits::new(s_1_10 as u128, 5u16);
        // D s_1_12: cast zx s_1_11 -> i
        let s_1_12: i128 = (s_1_11.value() as i128);
        // D s_1_13: cast reint s_1_12 -> i64
        let s_1_13: i64 = (s_1_12 as i64);
        // D s_1_14: write-var s <= s_1_13
        fn_state.s = s_1_13;
        // D s_1_15: read-var size:u8
        let s_1_15: u8 = fn_state.size;
        // D s_1_16: cast zx s_1_15 -> bv
        let s_1_16: Bits = Bits::new(s_1_15 as u128, 2u16);
        // D s_1_17: cast zx s_1_16 -> i
        let s_1_17: i128 = (s_1_16.value() as i128);
        // D s_1_18: cast reint s_1_17 -> i64
        let s_1_18: i64 = (s_1_17 as i64);
        // C s_1_19: const #8s : i64
        let s_1_19: i64 = 8;
        // D s_1_20: lsl s_1_19 s_1_18
        let s_1_20: i64 = s_1_19 << s_1_18;
        // D s_1_21: write-var datasize <= s_1_20
        fn_state.datasize = s_1_20;
        // C s_1_22: const #64s : i
        let s_1_22: i128 = 64;
        // D s_1_23: read-var datasize:i64
        let s_1_23: i64 = fn_state.datasize;
        // D s_1_24: cast zx s_1_23 -> i
        let s_1_24: i128 = (i128::try_from(s_1_23).unwrap());
        // D s_1_25: cmp-eq s_1_24 s_1_22
        let s_1_25: bool = ((s_1_24) == (s_1_22));
        // N s_1_26: branch s_1_25 b4 b2
        if s_1_25 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #32s : i64
        let s_2_0: i64 = 32;
        // D s_2_1: write-var ga#250946 <= s_2_0
        fn_state.ga_250946 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var ga#250946:i64
        let s_3_0: i64 = fn_state.ga_250946;
        // D s_3_1: read-var L:u8
        let s_3_1: bool = fn_state.L;
        // D s_3_2: cast zx s_3_1 -> bv
        let s_3_2: Bits = Bits::new(s_3_1 as u128, 1u16);
        // C s_3_3: const #1u : u8
        let s_3_3: bool = true;
        // C s_3_4: cast zx s_3_3 -> bv
        let s_3_4: Bits = Bits::new(s_3_3 as u128, 1u16);
        // D s_3_5: cmp-eq s_3_2 s_3_4
        let s_3_5: bool = ((s_3_2) == (s_3_4));
        // D s_3_6: read-var o0:u8
        let s_3_6: bool = fn_state.o0;
        // D s_3_7: cast zx s_3_6 -> bv
        let s_3_7: Bits = Bits::new(s_3_6 as u128, 1u16);
        // C s_3_8: const #1u : u8
        let s_3_8: bool = true;
        // C s_3_9: cast zx s_3_8 -> bv
        let s_3_9: Bits = Bits::new(s_3_8 as u128, 1u16);
        // D s_3_10: cmp-eq s_3_7 s_3_9
        let s_3_10: bool = ((s_3_7) == (s_3_9));
        // C s_3_11: const #31s : i
        let s_3_11: i128 = 31;
        // D s_3_12: read-var n:i64
        let s_3_12: i64 = fn_state.n;
        // D s_3_13: cast zx s_3_12 -> i
        let s_3_13: i128 = (i128::try_from(s_3_12).unwrap());
        // D s_3_14: call neq_int(s_3_13, s_3_11)
        let s_3_14: bool = neq_int(state, tracer, s_3_13, s_3_11);
        // D s_3_15: read-var datasize:i64
        let s_3_15: i64 = fn_state.datasize;
        // D s_3_16: cast zx s_3_15 -> i
        let s_3_16: i128 = (i128::try_from(s_3_15).unwrap());
        // D s_3_17: cast reint s_3_16 -> i64
        let s_3_17: i64 = (s_3_16 as i64);
        // D s_3_18: cast zx s_3_0 -> i
        let s_3_18: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_19: cast reint s_3_18 -> i64
        let s_3_19: i64 = (s_3_18 as i64);
        // D s_3_20: read-var n:i64
        let s_3_20: i64 = fn_state.n;
        // D s_3_21: read-var s:i64
        let s_3_21: i64 = fn_state.s;
        // D s_3_22: read-var t:i64
        let s_3_22: i64 = fn_state.t;
        // D s_3_23: call execute_aarch64_instrs_memory_atomicops_cas_single(s_3_5, s_3_17, s_3_20, s_3_19, s_3_10, s_3_21, s_3_22, s_3_14)
        let s_3_23: () = execute_aarch64_instrs_memory_atomicops_cas_single(
            state,
            tracer,
            s_3_5,
            s_3_17,
            s_3_20,
            s_3_19,
            s_3_10,
            s_3_21,
            s_3_22,
            s_3_14,
        );
        // N s_3_24: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #64s : i64
        let s_4_0: i64 = 64;
        // D s_4_1: write-var ga#250946 <= s_4_0
        fn_state.ga_250946 = s_4_0;
        // N s_4_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: panic
        panic!("{:?}", ());
        // N s_5_1: return
        return;
    }
}
