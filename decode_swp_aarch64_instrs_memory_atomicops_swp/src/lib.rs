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
use execute_aarch64_instrs_memory_atomicops_swp::*;
use common::*;
pub fn decode_swp_aarch64_instrs_memory_atomicops_swp<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rt: u8,
    Rn: u8,
    Rs: u8,
    R: bool,
    A: bool,
    size: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        t: i64,
        regsize: i64,
        ga_269775: i64,
        s: i64,
        datasize: i64,
        n: i64,
        gs_173753: bool,
        Rt: u8,
        Rn: u8,
        Rs: u8,
        R: bool,
        A: bool,
        size: u8,
    }
    let fn_state = FunctionState {
        Rt,
        Rn,
        Rs,
        R,
        A,
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
        // N s_0_3: branch s_0_2 b8 b1
        if s_0_2 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var Rt:u8
        let s_1_0: u8 = fn_state.Rt;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 5u16);
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (s_1_1.value() as i128);
        // D s_1_3: cast reint s_1_2 -> i64
        let s_1_3: i64 = (s_1_2 as i64);
        // D s_1_4: write-var t <= s_1_3
        fn_state.t = s_1_3;
        // D s_1_5: read-var Rn:u8
        let s_1_5: u8 = fn_state.Rn;
        // D s_1_6: cast zx s_1_5 -> bv
        let s_1_6: Bits = Bits::new(s_1_5 as u128, 5u16);
        // D s_1_7: cast zx s_1_6 -> i
        let s_1_7: i128 = (s_1_6.value() as i128);
        // D s_1_8: cast reint s_1_7 -> i64
        let s_1_8: i64 = (s_1_7 as i64);
        // D s_1_9: write-var n <= s_1_8
        fn_state.n = s_1_8;
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
        // N s_1_26: branch s_1_25 b7 b2
        if s_1_25 {
            return block_7(state, tracer, fn_state);
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
        // D s_2_1: write-var ga#269775 <= s_2_0
        fn_state.ga_269775 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var ga#269775:i64
        let s_3_0: i64 = fn_state.ga_269775;
        // D s_3_1: write-var regsize <= s_3_0
        fn_state.regsize = s_3_0;
        // D s_3_2: read-var A:u8
        let s_3_2: bool = fn_state.A;
        // D s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 1u16);
        // C s_3_4: const #1u : u8
        let s_3_4: bool = true;
        // C s_3_5: cast zx s_3_4 -> bv
        let s_3_5: Bits = Bits::new(s_3_4 as u128, 1u16);
        // D s_3_6: cmp-eq s_3_3 s_3_5
        let s_3_6: bool = ((s_3_3) == (s_3_5));
        // N s_3_7: branch s_3_6 b6 b4
        if s_3_6 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var gs#173753 <= s_4_0
        fn_state.gs_173753 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#173753:u8
        let s_5_0: bool = fn_state.gs_173753;
        // D s_5_1: read-var R:u8
        let s_5_1: bool = fn_state.R;
        // D s_5_2: cast zx s_5_1 -> bv
        let s_5_2: Bits = Bits::new(s_5_1 as u128, 1u16);
        // C s_5_3: const #1u : u8
        let s_5_3: bool = true;
        // C s_5_4: cast zx s_5_3 -> bv
        let s_5_4: Bits = Bits::new(s_5_3 as u128, 1u16);
        // D s_5_5: cmp-eq s_5_2 s_5_4
        let s_5_5: bool = ((s_5_2) == (s_5_4));
        // C s_5_6: const #31s : i
        let s_5_6: i128 = 31;
        // D s_5_7: read-var n:i64
        let s_5_7: i64 = fn_state.n;
        // D s_5_8: cast zx s_5_7 -> i
        let s_5_8: i128 = (i128::try_from(s_5_7).unwrap());
        // D s_5_9: call neq_int(s_5_8, s_5_6)
        let s_5_9: bool = neq_int(state, tracer, s_5_8, s_5_6);
        // D s_5_10: read-var datasize:i64
        let s_5_10: i64 = fn_state.datasize;
        // D s_5_11: cast zx s_5_10 -> i
        let s_5_11: i128 = (i128::try_from(s_5_10).unwrap());
        // D s_5_12: cast reint s_5_11 -> i64
        let s_5_12: i64 = (s_5_11 as i64);
        // D s_5_13: read-var regsize:i64
        let s_5_13: i64 = fn_state.regsize;
        // D s_5_14: cast zx s_5_13 -> i
        let s_5_14: i128 = (i128::try_from(s_5_13).unwrap());
        // D s_5_15: cast reint s_5_14 -> i64
        let s_5_15: i64 = (s_5_14 as i64);
        // D s_5_16: read-var n:i64
        let s_5_16: i64 = fn_state.n;
        // D s_5_17: read-var s:i64
        let s_5_17: i64 = fn_state.s;
        // D s_5_18: read-var t:i64
        let s_5_18: i64 = fn_state.t;
        // D s_5_19: call execute_aarch64_instrs_memory_atomicops_swp(s_5_0, s_5_12, s_5_16, s_5_15, s_5_5, s_5_17, s_5_18, s_5_9)
        let s_5_19: () = execute_aarch64_instrs_memory_atomicops_swp(
            state,
            tracer,
            s_5_0,
            s_5_12,
            s_5_16,
            s_5_15,
            s_5_5,
            s_5_17,
            s_5_18,
            s_5_9,
        );
        // N s_5_20: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var Rt:u8
        let s_6_0: u8 = fn_state.Rt;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 5u16);
        // C s_6_2: const #31u : u8
        let s_6_2: u8 = 31;
        // C s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 5u16);
        // D s_6_4: cmp-ne s_6_1 s_6_3
        let s_6_4: bool = ((s_6_1) != (s_6_3));
        // D s_6_5: write-var gs#173753 <= s_6_4
        fn_state.gs_173753 = s_6_4;
        // N s_6_6: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #64s : i64
        let s_7_0: i64 = 64;
        // D s_7_1: write-var ga#269775 <= s_7_0
        fn_state.ga_269775 = s_7_0;
        // N s_7_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_8_0: panic
        panic!("{:?}", ());
        // N s_8_1: return
        return;
    }
}
