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
use set_subrange_zeros::*;
use R_read::*;
use u__id::*;
use R_set::*;
use common::*;
pub fn execute_aarch32_instrs_BFC_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    lsbit: i64,
    msbit: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        d: i64,
        lsbit: i64,
        msbit: i64,
    }
    let fn_state = FunctionState {
        d,
        lsbit,
        msbit,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var lsbit:i64
        let s_0_0: i64 = fn_state.lsbit;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: call __id(s_0_1)
        let s_0_2: i128 = u__id(state, tracer, s_0_1);
        // D s_0_3: cast reint s_0_2 -> i64
        let s_0_3: i64 = (s_0_2 as i64);
        // D s_0_4: read-var msbit:i64
        let s_0_4: i64 = fn_state.msbit;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: call __id(s_0_5)
        let s_0_6: i128 = u__id(state, tracer, s_0_5);
        // D s_0_7: cast reint s_0_6 -> i64
        let s_0_7: i64 = (s_0_6 as i64);
        // D s_0_8: cast zx s_0_3 -> i
        let s_0_8: i128 = (i128::try_from(s_0_3).unwrap());
        // D s_0_9: cast zx s_0_7 -> i
        let s_0_9: i128 = (i128::try_from(s_0_7).unwrap());
        // D s_0_10: cmp-le s_0_8 s_0_9
        let s_0_10: bool = ((s_0_8) <= (s_0_9));
        // N s_0_11: assert s_0_10
        let s_0_11: () = assert!(s_0_10);
        // D s_0_12: read-var d:i64
        let s_0_12: i64 = fn_state.d;
        // D s_0_13: cast zx s_0_12 -> i
        let s_0_13: i128 = (i128::try_from(s_0_12).unwrap());
        // D s_0_14: call R_read(s_0_13)
        let s_0_14: u32 = R_read(state, tracer, s_0_13);
        // C s_0_15: const #32s : i
        let s_0_15: i128 = 32;
        // D s_0_16: cast zx s_0_14 -> bv
        let s_0_16: Bits = Bits::new(s_0_14 as u128, 32u16);
        // D s_0_17: read-var msbit:i64
        let s_0_17: i64 = fn_state.msbit;
        // D s_0_18: cast zx s_0_17 -> i
        let s_0_18: i128 = (i128::try_from(s_0_17).unwrap());
        // D s_0_19: read-var lsbit:i64
        let s_0_19: i64 = fn_state.lsbit;
        // D s_0_20: cast zx s_0_19 -> i
        let s_0_20: i128 = (i128::try_from(s_0_19).unwrap());
        // D s_0_21: call set_subrange_zeros(s_0_15, s_0_16, s_0_18, s_0_20)
        let s_0_21: Bits = set_subrange_zeros(
            state,
            tracer,
            s_0_15,
            s_0_16,
            s_0_18,
            s_0_20,
        );
        // D s_0_22: cast reint s_0_21 -> u32
        let s_0_22: u32 = (s_0_21.value() as u32);
        // D s_0_23: read-var d:i64
        let s_0_23: i64 = fn_state.d;
        // D s_0_24: cast zx s_0_23 -> i
        let s_0_24: i128 = (i128::try_from(s_0_23).unwrap());
        // D s_0_25: call R_set(s_0_24, s_0_22)
        let s_0_25: () = R_set(state, tracer, s_0_24, s_0_22);
        // N s_0_26: return
        return;
    }
}
