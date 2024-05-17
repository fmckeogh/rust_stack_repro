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
use ConditionPassed::*;
use neq_int::*;
use execute_aarch32_instrs_PLI_r_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_PLI_r_T1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rn: u8,
    imm2: u8,
    Rm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        shift_t: u32,
        n: i64,
        shift_nshadow_7249: i128,
        ga_345504: ProductType90c39552810120fd,
        Rn: u8,
        imm2: u8,
        Rm: u8,
    }
    let fn_state = FunctionState {
        Rn,
        imm2,
        Rm,
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
        // S s_0_1: call ConditionPassed(s_0_0)
        let s_0_1: bool = ConditionPassed(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b2 b1
        if s_0_1 {
            return block_2(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_1_0: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var Rn:u8
        let s_2_0: u8 = fn_state.Rn;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 4u16);
        // C s_2_2: const #15u : u8
        let s_2_2: u8 = 15;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 4u16);
        // D s_2_4: cmp-eq s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) == (s_2_3));
        // N s_2_5: branch s_2_4 b6 b3
        if s_2_4 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var Rn:u8
        let s_3_0: u8 = fn_state.Rn;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 4u16);
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (s_3_1.value() as i128);
        // D s_3_3: cast reint s_3_2 -> i64
        let s_3_3: i64 = (s_3_2 as i64);
        // D s_3_4: write-var n <= s_3_3
        fn_state.n = s_3_3;
        // C s_3_5: const #15s : i
        let s_3_5: i128 = 15;
        // D s_3_6: read-var n:i64
        let s_3_6: i64 = fn_state.n;
        // D s_3_7: cast zx s_3_6 -> i
        let s_3_7: i128 = (i128::try_from(s_3_6).unwrap());
        // D s_3_8: call neq_int(s_3_7, s_3_5)
        let s_3_8: bool = neq_int(state, tracer, s_3_7, s_3_5);
        // N s_3_9: assert s_3_8
        let s_3_9: () = assert!(s_3_8);
        // D s_3_10: read-var Rm:u8
        let s_3_10: u8 = fn_state.Rm;
        // D s_3_11: cast zx s_3_10 -> bv
        let s_3_11: Bits = Bits::new(s_3_10 as u128, 4u16);
        // D s_3_12: cast zx s_3_11 -> i
        let s_3_12: i128 = (s_3_11.value() as i128);
        // D s_3_13: cast reint s_3_12 -> i64
        let s_3_13: i64 = (s_3_12 as i64);
        // D s_3_14: write-var m <= s_3_13
        fn_state.m = s_3_13;
        // D s_3_15: read-var imm2:u8
        let s_3_15: u8 = fn_state.imm2;
        // D s_3_16: cast zx s_3_15 -> bv
        let s_3_16: Bits = Bits::new(s_3_15 as u128, 2u16);
        // D s_3_17: cast zx s_3_16 -> i
        let s_3_17: i128 = (s_3_16.value() as i128);
        // D s_3_18: cast reint s_3_17 -> i64
        let s_3_18: i64 = (s_3_17 as i64);
        // C s_3_19: const #0u : u32
        let s_3_19: u32 = 0;
        // D s_3_20: create-product struct = ["s_3_19", "s_3_18"]
        let s_3_20: ProductType90c39552810120fd = ProductType90c39552810120fd {
            _0: s_3_19,
            _1: s_3_18,
        };
        // D s_3_21: write-var ga#345504 <= s_3_20
        fn_state.ga_345504 = s_3_20;
        // D s_3_22: read-var ga#345504.0:struct
        let s_3_22: u32 = fn_state.ga_345504._0;
        // D s_3_23: read-var ga#345504.1:struct
        let s_3_23: i64 = fn_state.ga_345504._1;
        // D s_3_24: write-var shift_t <= s_3_22
        fn_state.shift_t = s_3_22;
        // D s_3_25: cast zx s_3_23 -> i
        let s_3_25: i128 = (i128::try_from(s_3_23).unwrap());
        // D s_3_26: write-var shift_nshadow#7249 <= s_3_25
        fn_state.shift_nshadow_7249 = s_3_25;
        // C s_3_27: const #15s : i
        let s_3_27: i128 = 15;
        // D s_3_28: read-var m:i64
        let s_3_28: i64 = fn_state.m;
        // D s_3_29: cast zx s_3_28 -> i
        let s_3_29: i128 = (i128::try_from(s_3_28).unwrap());
        // D s_3_30: cmp-eq s_3_29 s_3_27
        let s_3_30: bool = ((s_3_29) == (s_3_27));
        // N s_3_31: branch s_3_30 b5 b4
        if s_3_30 {
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
        // C s_4_0: const #1u : u8
        let s_4_0: bool = true;
        // D s_4_1: read-var m:i64
        let s_4_1: i64 = fn_state.m;
        // D s_4_2: read-var n:i64
        let s_4_2: i64 = fn_state.n;
        // D s_4_3: read-var shift_nshadow#7249:i
        let s_4_3: i128 = fn_state.shift_nshadow_7249;
        // D s_4_4: read-var shift_t:u32
        let s_4_4: u32 = fn_state.shift_t;
        // D s_4_5: call execute_aarch32_instrs_PLI_r_Op_A_txt(s_4_0, s_4_1, s_4_2, s_4_3, s_4_4)
        let s_4_5: () = execute_aarch32_instrs_PLI_r_Op_A_txt(
            state,
            tracer,
            s_4_0,
            s_4_1,
            s_4_2,
            s_4_3,
            s_4_4,
        );
        // N s_4_6: return
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
        // N s_6_0: panic
        panic!("{:?}", ());
        // N s_6_1: return
        return;
    }
}
