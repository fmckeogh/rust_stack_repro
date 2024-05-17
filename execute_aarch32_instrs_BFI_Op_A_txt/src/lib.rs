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
use R_read::*;
use u__id::*;
use vector_update_subrange_from_subrange::*;
use R_set::*;
use common::*;
pub fn execute_aarch32_instrs_BFI_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    lsbit: i64,
    msbit: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        d: i64,
        lsbit: i64,
        msbit: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        d,
        lsbit,
        msbit,
        n,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var msbit:i64
        let s_0_0: i64 = fn_state.msbit;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: call __id(s_0_1)
        let s_0_2: i128 = u__id(state, tracer, s_0_1);
        // D s_0_3: cast reint s_0_2 -> i64
        let s_0_3: i64 = (s_0_2 as i64);
        // D s_0_4: read-var lsbit:i64
        let s_0_4: i64 = fn_state.lsbit;
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
        // D s_0_10: sub s_0_8 s_0_9
        let s_0_10: i128 = ((s_0_8) - (s_0_9));
        // D s_0_11: cast reint s_0_10 -> i64
        let s_0_11: i64 = (s_0_10 as i64);
        // C s_0_12: const #0s : i
        let s_0_12: i128 = 0;
        // D s_0_13: cast zx s_0_11 -> i
        let s_0_13: i128 = (i128::try_from(s_0_11).unwrap());
        // D s_0_14: cmp-le s_0_12 s_0_13
        let s_0_14: bool = ((s_0_12) <= (s_0_13));
        // N s_0_15: assert s_0_14
        let s_0_15: () = assert!(s_0_14);
        // D s_0_16: read-var d:i64
        let s_0_16: i64 = fn_state.d;
        // D s_0_17: cast zx s_0_16 -> i
        let s_0_17: i128 = (i128::try_from(s_0_16).unwrap());
        // D s_0_18: call R_read(s_0_17)
        let s_0_18: u32 = R_read(state, tracer, s_0_17);
        // D s_0_19: read-var n:i64
        let s_0_19: i64 = fn_state.n;
        // D s_0_20: cast zx s_0_19 -> i
        let s_0_20: i128 = (i128::try_from(s_0_19).unwrap());
        // D s_0_21: call R_read(s_0_20)
        let s_0_21: u32 = R_read(state, tracer, s_0_20);
        // D s_0_22: read-var msbit:i64
        let s_0_22: i64 = fn_state.msbit;
        // D s_0_23: cast zx s_0_22 -> i
        let s_0_23: i128 = (i128::try_from(s_0_22).unwrap());
        // D s_0_24: read-var lsbit:i64
        let s_0_24: i64 = fn_state.lsbit;
        // D s_0_25: cast zx s_0_24 -> i
        let s_0_25: i128 = (i128::try_from(s_0_24).unwrap());
        // D s_0_26: sub s_0_23 s_0_25
        let s_0_26: i128 = ((s_0_23) - (s_0_25));
        // D s_0_27: cast reint s_0_26 -> i64
        let s_0_27: i64 = (s_0_26 as i64);
        // C s_0_28: const #32s : i
        let s_0_28: i128 = 32;
        // C s_0_29: const #0s : i
        let s_0_29: i128 = 0;
        // D s_0_30: cast zx s_0_18 -> bv
        let s_0_30: Bits = Bits::new(s_0_18 as u128, 32u16);
        // D s_0_31: read-var msbit:i64
        let s_0_31: i64 = fn_state.msbit;
        // D s_0_32: cast zx s_0_31 -> i
        let s_0_32: i128 = (i128::try_from(s_0_31).unwrap());
        // D s_0_33: read-var lsbit:i64
        let s_0_33: i64 = fn_state.lsbit;
        // D s_0_34: cast zx s_0_33 -> i
        let s_0_34: i128 = (i128::try_from(s_0_33).unwrap());
        // D s_0_35: cast zx s_0_21 -> bv
        let s_0_35: Bits = Bits::new(s_0_21 as u128, 32u16);
        // D s_0_36: cast zx s_0_27 -> i
        let s_0_36: i128 = (i128::try_from(s_0_27).unwrap());
        // D s_0_37: call vector_update_subrange_from_subrange(s_0_28, s_0_30, s_0_32, s_0_34, s_0_35, s_0_36, s_0_29)
        let s_0_37: Bits = vector_update_subrange_from_subrange(
            state,
            tracer,
            s_0_28,
            s_0_30,
            s_0_32,
            s_0_34,
            s_0_35,
            s_0_36,
            s_0_29,
        );
        // D s_0_38: cast reint s_0_37 -> u32
        let s_0_38: u32 = (s_0_37.value() as u32);
        // D s_0_39: read-var d:i64
        let s_0_39: i64 = fn_state.d;
        // D s_0_40: cast zx s_0_39 -> i
        let s_0_40: i128 = (i128::try_from(s_0_39).unwrap());
        // D s_0_41: call R_set(s_0_40, s_0_38)
        let s_0_41: () = R_set(state, tracer, s_0_40, s_0_38);
        // N s_0_42: return
        return;
    }
}
