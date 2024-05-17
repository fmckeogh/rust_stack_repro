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
use HaveFeatLS64_V::*;
use execute_aarch64_instrs_memory_atomicops_st_acc_st64bv::*;
use common::*;
pub fn decode_st64bv_aarch64_instrs_memory_atomicops_st_acc_st64bv<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rt: u8,
    Rn: u8,
    Rs: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_173115: bool,
        Rt: u8,
        Rn: u8,
        Rs: u8,
    }
    let fn_state = FunctionState {
        Rt,
        Rn,
        Rs,
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
        // S s_0_1: call HaveFeatLS64_V(s_0_0)
        let s_0_1: bool = HaveFeatLS64_V(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b7 b1
        if s_0_2 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #3s : i
        let s_1_0: i128 = 3;
        // D s_1_1: read-var Rt:u8
        let s_1_1: u8 = fn_state.Rt;
        // D s_1_2: cast zx s_1_1 -> bv
        let s_1_2: Bits = Bits::new(s_1_1 as u128, 5u16);
        // C s_1_3: const #1s : i64
        let s_1_3: i64 = 1;
        // C s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // C s_1_5: const #1s : i
        let s_1_5: i128 = 1;
        // C s_1_6: add s_1_5 s_1_4
        let s_1_6: i128 = (s_1_5 + s_1_4);
        // D s_1_7: bit-extract s_1_2 s_1_0 s_1_6
        let s_1_7: Bits = (Bits::new(
            ((s_1_2) >> (s_1_0)).value(),
            u16::try_from(s_1_6).unwrap(),
        ));
        // D s_1_8: cast reint s_1_7 -> u8
        let s_1_8: u8 = (s_1_7.value() as u8);
        // D s_1_9: cast zx s_1_8 -> bv
        let s_1_9: Bits = Bits::new(s_1_8 as u128, 2u16);
        // C s_1_10: const #3u : u8
        let s_1_10: u8 = 3;
        // C s_1_11: cast zx s_1_10 -> bv
        let s_1_11: Bits = Bits::new(s_1_10 as u128, 2u16);
        // D s_1_12: cmp-eq s_1_9 s_1_11
        let s_1_12: bool = ((s_1_9) == (s_1_11));
        // N s_1_13: branch s_1_12 b6 b2
        if s_1_12 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #0s : i
        let s_2_0: i128 = 0;
        // D s_2_1: read-var Rt:u8
        let s_2_1: u8 = fn_state.Rt;
        // D s_2_2: cast zx s_2_1 -> bv
        let s_2_2: Bits = Bits::new(s_2_1 as u128, 5u16);
        // C s_2_3: const #1u : u64
        let s_2_3: u64 = 1;
        // D s_2_4: bit-extract s_2_2 s_2_0 s_2_3
        let s_2_4: Bits = (Bits::new(
            ((s_2_2) >> (s_2_0)).value(),
            u16::try_from(s_2_3).unwrap(),
        ));
        // D s_2_5: cast reint s_2_4 -> u8
        let s_2_5: bool = ((s_2_4.value()) != 0);
        // C s_2_6: const #0s : i
        let s_2_6: i128 = 0;
        // C s_2_7: const #0u : u64
        let s_2_7: u64 = 0;
        // D s_2_8: cast zx s_2_5 -> u64
        let s_2_8: u64 = (s_2_5 as u64);
        // C s_2_9: const #1u : u64
        let s_2_9: u64 = 1;
        // D s_2_10: and s_2_8 s_2_9
        let s_2_10: u64 = ((s_2_8) & (s_2_9));
        // D s_2_11: cmp-eq s_2_10 s_2_9
        let s_2_11: bool = ((s_2_10) == (s_2_9));
        // D s_2_12: lsl s_2_8 s_2_6
        let s_2_12: u64 = s_2_8 << s_2_6;
        // D s_2_13: or s_2_7 s_2_12
        let s_2_13: u64 = ((s_2_7) | (s_2_12));
        // D s_2_14: cmpl s_2_12
        let s_2_14: u64 = !s_2_12;
        // D s_2_15: and s_2_7 s_2_14
        let s_2_15: u64 = ((s_2_7) & (s_2_14));
        // D s_2_16: select s_2_11 s_2_13 s_2_15
        let s_2_16: u64 = if s_2_11 { s_2_13 } else { s_2_15 };
        // D s_2_17: cast trunc s_2_16 -> u8
        let s_2_17: bool = ((s_2_16) != 0);
        // D s_2_18: cast zx s_2_17 -> bv
        let s_2_18: Bits = Bits::new(s_2_17 as u128, 1u16);
        // C s_2_19: const #1u : u8
        let s_2_19: bool = true;
        // C s_2_20: cast zx s_2_19 -> bv
        let s_2_20: Bits = Bits::new(s_2_19 as u128, 1u16);
        // D s_2_21: cmp-eq s_2_18 s_2_20
        let s_2_21: bool = ((s_2_18) == (s_2_20));
        // D s_2_22: write-var gs#173115 <= s_2_21
        fn_state.gs_173115 = s_2_21;
        // N s_2_23: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#173115:u8
        let s_3_0: bool = fn_state.gs_173115;
        // N s_3_1: branch s_3_0 b5 b4
        if s_3_0 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var Rn:u8
        let s_4_0: u8 = fn_state.Rn;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 5u16);
        // D s_4_2: cast zx s_4_1 -> i
        let s_4_2: i128 = (s_4_1.value() as i128);
        // D s_4_3: cast reint s_4_2 -> i64
        let s_4_3: i64 = (s_4_2 as i64);
        // D s_4_4: read-var Rt:u8
        let s_4_4: u8 = fn_state.Rt;
        // D s_4_5: cast zx s_4_4 -> bv
        let s_4_5: Bits = Bits::new(s_4_4 as u128, 5u16);
        // D s_4_6: cast zx s_4_5 -> i
        let s_4_6: i128 = (s_4_5.value() as i128);
        // D s_4_7: cast reint s_4_6 -> i64
        let s_4_7: i64 = (s_4_6 as i64);
        // C s_4_8: const #1u : u32
        let s_4_8: u32 = 1;
        // D s_4_9: read-var Rs:u8
        let s_4_9: u8 = fn_state.Rs;
        // D s_4_10: cast zx s_4_9 -> bv
        let s_4_10: Bits = Bits::new(s_4_9 as u128, 5u16);
        // D s_4_11: cast zx s_4_10 -> i
        let s_4_11: i128 = (s_4_10.value() as i128);
        // D s_4_12: cast reint s_4_11 -> i64
        let s_4_12: i64 = (s_4_11 as i64);
        // C s_4_13: const #31s : i
        let s_4_13: i128 = 31;
        // D s_4_14: cast zx s_4_3 -> i
        let s_4_14: i128 = (i128::try_from(s_4_3).unwrap());
        // D s_4_15: call neq_int(s_4_14, s_4_13)
        let s_4_15: bool = neq_int(state, tracer, s_4_14, s_4_13);
        // C s_4_16: const #1200u : u32
        let s_4_16: u32 = 1200;
        // D s_4_17: read-reg s_4_16:u8
        let s_4_17: u8 = {
            let value = state.read_register::<u8>(s_4_16 as isize);
            tracer.read_register(s_4_16 as isize, value);
            value
        };
        // D s_4_18: read-var Rs:u8
        let s_4_18: u8 = fn_state.Rs;
        // D s_4_19: call execute_aarch64_instrs_memory_atomicops_st_acc_st64bv(s_4_18, s_4_17, s_4_8, s_4_3, s_4_12, s_4_7, s_4_15)
        let s_4_19: () = execute_aarch64_instrs_memory_atomicops_st_acc_st64bv(
            state,
            tracer,
            s_4_18,
            s_4_17,
            s_4_8,
            s_4_3,
            s_4_12,
            s_4_7,
            s_4_15,
        );
        // N s_4_20: return
        return;
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
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #1u : u8
        let s_6_0: bool = true;
        // D s_6_1: write-var gs#173115 <= s_6_0
        fn_state.gs_173115 = s_6_0;
        // N s_6_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: panic
        panic!("{:?}", ());
        // N s_7_1: return
        return;
    }
}
