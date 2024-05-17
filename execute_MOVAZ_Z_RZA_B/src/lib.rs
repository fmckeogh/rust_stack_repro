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
use ZAslice_read::*;
use ZAslice_set::*;
use X_read::*;
use Zeros::*;
use CheckStreamingSVEAndZAEnabled::*;
use Z_set::*;
use common::*;
pub fn execute_MOVAZ_Z_RZA_B<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    d: i64,
    esize: i64,
    n: i64,
    offset: i64,
    s: i64,
    vertical: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        VLshadow_6892: i64,
        VL: i64,
        d: i64,
        esize: i64,
        n: i64,
        offset: i64,
        s: i64,
        vertical: bool,
    }
    let fn_state = FunctionState {
        VL,
        d,
        esize,
        n,
        offset,
        s,
        vertical,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var VL:i64
        let s_0_0: i64 = fn_state.VL;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var VLshadow#6892 <= s_0_2
        fn_state.VLshadow_6892 = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call CheckStreamingSVEAndZAEnabled(s_0_4)
        let s_0_5: () = CheckStreamingSVEAndZAEnabled(state, tracer, s_0_4);
        // N s_0_6: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var VLshadow#6892:i64
        let s_1_0: i64 = fn_state.VLshadow_6892;
        // D s_1_1: cast zx s_1_0 -> i
        let s_1_1: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_2: read-var esize:i64
        let s_1_2: i64 = fn_state.esize;
        // D s_1_3: cast zx s_1_2 -> i
        let s_1_3: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_4: div s_1_1 s_1_3
        let s_1_4: i128 = ((s_1_1) / (s_1_3));
        // D s_1_5: cast reint s_1_4 -> i64
        let s_1_5: i64 = (s_1_4 as i64);
        // C s_1_6: const #32s : i64
        let s_1_6: i64 = 32;
        // D s_1_7: read-var s:i64
        let s_1_7: i64 = fn_state.s;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: call X_read(s_1_8, s_1_6)
        let s_1_9: Bits = X_read(state, tracer, s_1_8, s_1_6);
        // D s_1_10: cast reint s_1_9 -> u32
        let s_1_10: u32 = (s_1_9.value() as u32);
        // D s_1_11: cast zx s_1_10 -> bv
        let s_1_11: Bits = Bits::new(s_1_10 as u128, 32u16);
        // D s_1_12: cast zx s_1_11 -> i
        let s_1_12: i128 = (s_1_11.value() as i128);
        // D s_1_13: cast reint s_1_12 -> i64
        let s_1_13: i64 = (s_1_12 as i64);
        // D s_1_14: cast zx s_1_13 -> i
        let s_1_14: i128 = (i128::try_from(s_1_13).unwrap());
        // D s_1_15: read-var offset:i64
        let s_1_15: i64 = fn_state.offset;
        // D s_1_16: cast zx s_1_15 -> i
        let s_1_16: i128 = (i128::try_from(s_1_15).unwrap());
        // D s_1_17: add s_1_14 s_1_16
        let s_1_17: i128 = (s_1_14 + s_1_16);
        // D s_1_18: cast reint s_1_17 -> i64
        let s_1_18: i64 = (s_1_17 as i64);
        // D s_1_19: cast zx s_1_18 -> i
        let s_1_19: i128 = (i128::try_from(s_1_18).unwrap());
        // D s_1_20: cast zx s_1_5 -> i
        let s_1_20: i128 = (i128::try_from(s_1_5).unwrap());
        // D s_1_21: mod s_1_19 s_1_20
        let s_1_21: i128 = ((s_1_19) % (s_1_20));
        // D s_1_22: cast reint s_1_21 -> i64
        let s_1_22: i64 = (s_1_21 as i64);
        // D s_1_23: read-var esize:i64
        let s_1_23: i64 = fn_state.esize;
        // D s_1_24: cast zx s_1_23 -> i
        let s_1_24: i128 = (i128::try_from(s_1_23).unwrap());
        // D s_1_25: cast reint s_1_24 -> i64
        let s_1_25: i64 = (s_1_24 as i64);
        // D s_1_26: cast zx s_1_0 -> i
        let s_1_26: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_27: cast reint s_1_26 -> i64
        let s_1_27: i64 = (s_1_26 as i64);
        // D s_1_28: read-var n:i64
        let s_1_28: i64 = fn_state.n;
        // D s_1_29: cast zx s_1_28 -> i
        let s_1_29: i128 = (i128::try_from(s_1_28).unwrap());
        // D s_1_30: cast zx s_1_25 -> i
        let s_1_30: i128 = (i128::try_from(s_1_25).unwrap());
        // D s_1_31: cast zx s_1_22 -> i
        let s_1_31: i128 = (i128::try_from(s_1_22).unwrap());
        // D s_1_32: cast zx s_1_27 -> i
        let s_1_32: i128 = (i128::try_from(s_1_27).unwrap());
        // D s_1_33: read-var vertical:u8
        let s_1_33: bool = fn_state.vertical;
        // D s_1_34: call ZAslice_read(s_1_29, s_1_30, s_1_33, s_1_31, s_1_32)
        let s_1_34: Bits = ZAslice_read(
            state,
            tracer,
            s_1_29,
            s_1_30,
            s_1_33,
            s_1_31,
            s_1_32,
        );
        // D s_1_35: read-var esize:i64
        let s_1_35: i64 = fn_state.esize;
        // D s_1_36: cast zx s_1_35 -> i
        let s_1_36: i128 = (i128::try_from(s_1_35).unwrap());
        // D s_1_37: cast reint s_1_36 -> i64
        let s_1_37: i64 = (s_1_36 as i64);
        // D s_1_38: cast zx s_1_0 -> i
        let s_1_38: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_39: cast reint s_1_38 -> i64
        let s_1_39: i64 = (s_1_38 as i64);
        // D s_1_40: cast zx s_1_0 -> i
        let s_1_40: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_41: call Zeros(s_1_40)
        let s_1_41: Bits = Zeros(state, tracer, s_1_40);
        // D s_1_42: read-var n:i64
        let s_1_42: i64 = fn_state.n;
        // D s_1_43: cast zx s_1_42 -> i
        let s_1_43: i128 = (i128::try_from(s_1_42).unwrap());
        // D s_1_44: cast zx s_1_37 -> i
        let s_1_44: i128 = (i128::try_from(s_1_37).unwrap());
        // D s_1_45: cast zx s_1_22 -> i
        let s_1_45: i128 = (i128::try_from(s_1_22).unwrap());
        // D s_1_46: cast zx s_1_39 -> i
        let s_1_46: i128 = (i128::try_from(s_1_39).unwrap());
        // D s_1_47: read-var vertical:u8
        let s_1_47: bool = fn_state.vertical;
        // D s_1_48: call ZAslice_set(s_1_43, s_1_44, s_1_47, s_1_45, s_1_46, s_1_41)
        let s_1_48: () = ZAslice_set(
            state,
            tracer,
            s_1_43,
            s_1_44,
            s_1_47,
            s_1_45,
            s_1_46,
            s_1_41,
        );
        // D s_1_49: cast zx s_1_0 -> i
        let s_1_49: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_50: cast reint s_1_49 -> i64
        let s_1_50: i64 = (s_1_49 as i64);
        // D s_1_51: read-var d:i64
        let s_1_51: i64 = fn_state.d;
        // D s_1_52: cast zx s_1_51 -> i
        let s_1_52: i128 = (i128::try_from(s_1_51).unwrap());
        // D s_1_53: cast zx s_1_50 -> i
        let s_1_53: i128 = (i128::try_from(s_1_50).unwrap());
        // D s_1_54: call Z_set(s_1_52, s_1_53, s_1_34)
        let s_1_54: () = Z_set(state, tracer, s_1_52, s_1_53, s_1_34);
        // N s_1_55: return
        return;
    }
}
