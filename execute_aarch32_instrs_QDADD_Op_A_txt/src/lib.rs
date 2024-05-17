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
pub fn execute_aarch32_instrs_QDADD_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    m: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_882403: ProductTypef506aa96a892fe52,
        gs_299546: bool,
        gs_882391: ProductTypef506aa96a892fe52,
        sat2: bool,
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
        // D s_0_0: read-var n:i64
        let s_0_0: i64 = fn_state.n;
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
        // C s_0_6: const #2s : i
        let s_0_6: i128 = 2;
        // D s_0_7: cast zx s_0_5 -> i
        let s_0_7: i128 = (i128::try_from(s_0_5).unwrap());
        // D s_0_8: mul s_0_6 s_0_7
        let s_0_8: i128 = ((s_0_6) * (s_0_7));
        // D s_0_9: cast reint s_0_8 -> i64
        let s_0_9: i64 = (s_0_8 as i64);
        // C s_0_10: const #32s : i64
        let s_0_10: i64 = 32;
        // D s_0_11: cast zx s_0_9 -> i
        let s_0_11: i128 = (i128::try_from(s_0_9).unwrap());
        // C s_0_12: cast zx s_0_10 -> i
        let s_0_12: i128 = (i128::try_from(s_0_10).unwrap());
        // D s_0_13: call SignedSatQ(s_0_11, s_0_12)
        let s_0_13: ProductTypef506aa96a892fe52 = SignedSatQ(
            state,
            tracer,
            s_0_11,
            s_0_12,
        );
        // D s_0_14: write-var gs#882391 <= s_0_13
        fn_state.gs_882391 = s_0_13;
        // D s_0_15: read-var gs#882391.0:struct
        let s_0_15: Bits = fn_state.gs_882391._0;
        // D s_0_16: cast reint s_0_15 -> u32
        let s_0_16: u32 = (s_0_15.value() as u32);
        // D s_0_17: read-var gs#882391.1:struct
        let s_0_17: bool = fn_state.gs_882391._1;
        // D s_0_18: read-var m:i64
        let s_0_18: i64 = fn_state.m;
        // D s_0_19: cast zx s_0_18 -> i
        let s_0_19: i128 = (i128::try_from(s_0_18).unwrap());
        // D s_0_20: call R_read(s_0_19)
        let s_0_20: u32 = R_read(state, tracer, s_0_19);
        // D s_0_21: cast zx s_0_20 -> bv
        let s_0_21: Bits = Bits::new(s_0_20 as u128, 32u16);
        // D s_0_22: cast sx s_0_21 -> i
        let s_0_22: i128 = {
            let sign_bit = s_0_21.length() - 1;
            let mut result = s_0_21.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_0_23: cast reint s_0_22 -> i64
        let s_0_23: i64 = (s_0_22 as i64);
        // D s_0_24: cast zx s_0_16 -> bv
        let s_0_24: Bits = Bits::new(s_0_16 as u128, 32u16);
        // D s_0_25: cast sx s_0_24 -> i
        let s_0_25: i128 = {
            let sign_bit = s_0_24.length() - 1;
            let mut result = s_0_24.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_0_26: cast reint s_0_25 -> i64
        let s_0_26: i64 = (s_0_25 as i64);
        // D s_0_27: cast zx s_0_23 -> i
        let s_0_27: i128 = (i128::try_from(s_0_23).unwrap());
        // D s_0_28: cast zx s_0_26 -> i
        let s_0_28: i128 = (i128::try_from(s_0_26).unwrap());
        // D s_0_29: add s_0_27 s_0_28
        let s_0_29: i128 = (s_0_27 + s_0_28);
        // D s_0_30: cast reint s_0_29 -> i64
        let s_0_30: i64 = (s_0_29 as i64);
        // C s_0_31: const #32s : i64
        let s_0_31: i64 = 32;
        // D s_0_32: cast zx s_0_30 -> i
        let s_0_32: i128 = (i128::try_from(s_0_30).unwrap());
        // C s_0_33: cast zx s_0_31 -> i
        let s_0_33: i128 = (i128::try_from(s_0_31).unwrap());
        // D s_0_34: call SignedSatQ(s_0_32, s_0_33)
        let s_0_34: ProductTypef506aa96a892fe52 = SignedSatQ(
            state,
            tracer,
            s_0_32,
            s_0_33,
        );
        // D s_0_35: write-var gs#882403 <= s_0_34
        fn_state.gs_882403 = s_0_34;
        // D s_0_36: read-var gs#882403.0:struct
        let s_0_36: Bits = fn_state.gs_882403._0;
        // D s_0_37: cast reint s_0_36 -> u32
        let s_0_37: u32 = (s_0_36.value() as u32);
        // D s_0_38: read-var gs#882403.1:struct
        let s_0_38: bool = fn_state.gs_882403._1;
        // D s_0_39: read-var d:i64
        let s_0_39: i64 = fn_state.d;
        // D s_0_40: cast zx s_0_39 -> i
        let s_0_40: i128 = (i128::try_from(s_0_39).unwrap());
        // D s_0_41: call R_set(s_0_40, s_0_37)
        let s_0_41: () = R_set(state, tracer, s_0_40, s_0_37);
        // D s_0_42: write-var sat2 <= s_0_38
        fn_state.sat2 = s_0_38;
        // N s_0_43: branch s_0_17 b5 b1
        if s_0_17 {
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
        // D s_1_0: read-var sat2:u8
        let s_1_0: bool = fn_state.sat2;
        // D s_1_1: write-var gs#299546 <= s_1_0
        fn_state.gs_299546 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#299546:u8
        let s_2_0: bool = fn_state.gs_299546;
        // N s_2_1: branch s_2_0 b4 b3
        if s_2_0 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_3_0: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #1u : u8
        let s_4_0: bool = true;
        // C s_4_1: const #16988u : u32
        let s_4_1: u32 = 16988;
        // N s_4_2: write-reg s_4_1 <= s_4_0
        let s_4_2: () = {
            state.write_register::<bool>(s_4_1 as isize, s_4_0);
            tracer.write_register(s_4_1 as isize, s_4_0);
        };
        // N s_4_3: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #1u : u8
        let s_5_0: bool = true;
        // D s_5_1: write-var gs#299546 <= s_5_0
        fn_state.gs_299546 = s_5_0;
        // N s_5_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
