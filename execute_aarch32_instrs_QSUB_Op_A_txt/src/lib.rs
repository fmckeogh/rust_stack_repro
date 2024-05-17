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
use R_set::*;
use SignedSatQ::*;
use R_read::*;
use common::*;
pub fn execute_aarch32_instrs_QSUB_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    m: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_882715: ProductTypef506aa96a892fe52,
        d: i64,
        m: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        d,
        m,
        n,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var m:i64
        let s_0_0: i64 = fn_state.m;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: call R_read(s_0_1)
        let s_0_2: u32 = R_read(state, tracer, s_0_1);
        // D s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 32u16);
        // D s_0_4: cast sx s_0_3 -> i
        let s_0_4: i128 = {
            let sign_bit = s_0_3.length() - 1;
            let mut result = s_0_3.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_0_5: cast reint s_0_4 -> i64
        let s_0_5: i64 = (s_0_4 as i64);
        // D s_0_6: read-var n:i64
        let s_0_6: i64 = fn_state.n;
        // D s_0_7: cast zx s_0_6 -> i
        let s_0_7: i128 = (i128::try_from(s_0_6).unwrap());
        // D s_0_8: call R_read(s_0_7)
        let s_0_8: u32 = R_read(state, tracer, s_0_7);
        // D s_0_9: cast zx s_0_8 -> bv
        let s_0_9: Bits = Bits::new(s_0_8 as u128, 32u16);
        // D s_0_10: cast sx s_0_9 -> i
        let s_0_10: i128 = {
            let sign_bit = s_0_9.length() - 1;
            let mut result = s_0_9.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_0_11: cast reint s_0_10 -> i64
        let s_0_11: i64 = (s_0_10 as i64);
        // D s_0_12: cast zx s_0_5 -> i
        let s_0_12: i128 = (i128::try_from(s_0_5).unwrap());
        // D s_0_13: cast zx s_0_11 -> i
        let s_0_13: i128 = (i128::try_from(s_0_11).unwrap());
        // D s_0_14: sub s_0_12 s_0_13
        let s_0_14: i128 = ((s_0_12) - (s_0_13));
        // D s_0_15: cast reint s_0_14 -> i64
        let s_0_15: i64 = (s_0_14 as i64);
        // C s_0_16: const #32s : i64
        let s_0_16: i64 = 32;
        // D s_0_17: cast zx s_0_15 -> i
        let s_0_17: i128 = (i128::try_from(s_0_15).unwrap());
        // C s_0_18: cast zx s_0_16 -> i
        let s_0_18: i128 = (i128::try_from(s_0_16).unwrap());
        // D s_0_19: call SignedSatQ(s_0_17, s_0_18)
        let s_0_19: ProductTypef506aa96a892fe52 = SignedSatQ(
            state,
            tracer,
            s_0_17,
            s_0_18,
        );
        // D s_0_20: write-var gs#882715 <= s_0_19
        fn_state.gs_882715 = s_0_19;
        // D s_0_21: read-var gs#882715.0:struct
        let s_0_21: Bits = fn_state.gs_882715._0;
        // D s_0_22: cast reint s_0_21 -> u32
        let s_0_22: u32 = (s_0_21.value() as u32);
        // D s_0_23: read-var gs#882715.1:struct
        let s_0_23: bool = fn_state.gs_882715._1;
        // D s_0_24: read-var d:i64
        let s_0_24: i64 = fn_state.d;
        // D s_0_25: cast zx s_0_24 -> i
        let s_0_25: i128 = (i128::try_from(s_0_24).unwrap());
        // D s_0_26: call R_set(s_0_25, s_0_22)
        let s_0_26: () = R_set(state, tracer, s_0_25, s_0_22);
        // N s_0_27: branch s_0_23 b2 b1
        if s_0_23 {
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
        // C s_2_0: const #1u : u8
        let s_2_0: bool = true;
        // C s_2_1: const #16988u : u32
        let s_2_1: u32 = 16988;
        // N s_2_2: write-reg s_2_1 <= s_2_0
        let s_2_2: () = {
            state.write_register::<bool>(s_2_1 as isize, s_2_0);
            tracer.write_register(s_2_1 as isize, s_2_0);
        };
        // N s_2_3: return
        return;
    }
}
