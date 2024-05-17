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
use fmod_int::*;
use X_read::*;
use CheckStreamingSVEAndZAEnabled::*;
use Z_set::*;
use common::*;
pub fn execute_MOVA_MZ2_ZA_B1<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    d: i64,
    esize: i64,
    n: i64,
    nreg: i64,
    offset: i64,
    s: i64,
    vertical: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: i64,
        gs_258701: i64,
        VLshadow_5573: i64,
        VLshadow_5572: i64,
        slice_name: i128,
        VL: i64,
        d: i64,
        esize: i64,
        n: i64,
        nreg: i64,
        offset: i64,
        s: i64,
        vertical: bool,
    }
    let fn_state = FunctionState {
        VL,
        d,
        esize,
        n,
        nreg,
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
        // D s_0_3: write-var VLshadow#5572 <= s_0_2
        fn_state.VLshadow_5572 = s_0_2;
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
        // D s_1_0: read-var VLshadow#5572:i64
        let s_1_0: i64 = fn_state.VLshadow_5572;
        // D s_1_1: write-var VLshadow#5573 <= s_1_0
        fn_state.VLshadow_5573 = s_1_0;
        // D s_1_2: read-var VLshadow#5573:i64
        let s_1_2: i64 = fn_state.VLshadow_5573;
        // D s_1_3: cast zx s_1_2 -> i
        let s_1_3: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_4: read-var esize:i64
        let s_1_4: i64 = fn_state.esize;
        // D s_1_5: cast zx s_1_4 -> i
        let s_1_5: i128 = (i128::try_from(s_1_4).unwrap());
        // D s_1_6: div s_1_3 s_1_5
        let s_1_6: i128 = ((s_1_3) / (s_1_5));
        // D s_1_7: cast reint s_1_6 -> i64
        let s_1_7: i64 = (s_1_6 as i64);
        // C s_1_8: const #32s : i64
        let s_1_8: i64 = 32;
        // D s_1_9: read-var s:i64
        let s_1_9: i64 = fn_state.s;
        // D s_1_10: cast zx s_1_9 -> i
        let s_1_10: i128 = (i128::try_from(s_1_9).unwrap());
        // D s_1_11: call X_read(s_1_10, s_1_8)
        let s_1_11: Bits = X_read(state, tracer, s_1_10, s_1_8);
        // D s_1_12: cast reint s_1_11 -> u32
        let s_1_12: u32 = (s_1_11.value() as u32);
        // D s_1_13: cast zx s_1_12 -> bv
        let s_1_13: Bits = Bits::new(s_1_12 as u128, 32u16);
        // D s_1_14: cast zx s_1_13 -> i
        let s_1_14: i128 = (s_1_13.value() as i128);
        // D s_1_15: cast reint s_1_14 -> i64
        let s_1_15: i64 = (s_1_14 as i64);
        // D s_1_16: cast zx s_1_12 -> bv
        let s_1_16: Bits = Bits::new(s_1_12 as u128, 32u16);
        // D s_1_17: cast zx s_1_16 -> i
        let s_1_17: i128 = (s_1_16.value() as i128);
        // D s_1_18: cast reint s_1_17 -> i64
        let s_1_18: i64 = (s_1_17 as i64);
        // D s_1_19: cast zx s_1_18 -> i
        let s_1_19: i128 = (i128::try_from(s_1_18).unwrap());
        // D s_1_20: read-var nreg:i64
        let s_1_20: i64 = fn_state.nreg;
        // D s_1_21: cast zx s_1_20 -> i
        let s_1_21: i128 = (i128::try_from(s_1_20).unwrap());
        // D s_1_22: mod s_1_19 s_1_21
        let s_1_22: i128 = ((s_1_19) % (s_1_21));
        // D s_1_23: cast reint s_1_22 -> i64
        let s_1_23: i64 = (s_1_22 as i64);
        // D s_1_24: cast zx s_1_15 -> i
        let s_1_24: i128 = (i128::try_from(s_1_15).unwrap());
        // D s_1_25: cast zx s_1_23 -> i
        let s_1_25: i128 = (i128::try_from(s_1_23).unwrap());
        // D s_1_26: sub s_1_24 s_1_25
        let s_1_26: i128 = ((s_1_24) - (s_1_25));
        // D s_1_27: cast reint s_1_26 -> i64
        let s_1_27: i64 = (s_1_26 as i64);
        // D s_1_28: cast zx s_1_27 -> i
        let s_1_28: i128 = (i128::try_from(s_1_27).unwrap());
        // D s_1_29: read-var offset:i64
        let s_1_29: i64 = fn_state.offset;
        // D s_1_30: cast zx s_1_29 -> i
        let s_1_30: i128 = (i128::try_from(s_1_29).unwrap());
        // D s_1_31: add s_1_28 s_1_30
        let s_1_31: i128 = (s_1_28 + s_1_30);
        // D s_1_32: cast reint s_1_31 -> i64
        let s_1_32: i64 = (s_1_31 as i64);
        // D s_1_33: cast zx s_1_32 -> i
        let s_1_33: i128 = (i128::try_from(s_1_32).unwrap());
        // D s_1_34: cast zx s_1_7 -> i
        let s_1_34: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_35: call fmod_int(s_1_33, s_1_34)
        let s_1_35: i128 = fmod_int(state, tracer, s_1_33, s_1_34);
        // D s_1_36: write-var slice_name <= s_1_35
        fn_state.slice_name = s_1_35;
        // C s_1_37: const #0s : i64
        let s_1_37: i64 = 0;
        // C s_1_38: const #1s : i
        let s_1_38: i128 = 1;
        // D s_1_39: read-var nreg:i64
        let s_1_39: i64 = fn_state.nreg;
        // D s_1_40: cast zx s_1_39 -> i
        let s_1_40: i128 = (i128::try_from(s_1_39).unwrap());
        // D s_1_41: sub s_1_40 s_1_38
        let s_1_41: i128 = ((s_1_40) - (s_1_38));
        // D s_1_42: cast reint s_1_41 -> i64
        let s_1_42: i64 = (s_1_41 as i64);
        // D s_1_43: write-var gs#258701 <= s_1_42
        fn_state.gs_258701 = s_1_42;
        // D s_1_44: write-var r <= s_1_37
        fn_state.r = s_1_37;
        // N s_1_45: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var r:i64
        let s_2_0: i64 = fn_state.r;
        // D s_2_1: read-var gs#258701:i64
        let s_2_1: i64 = fn_state.gs_258701;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b4 b3
        if s_2_2 {
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
        // D s_3_0: read-var esize:i64
        let s_3_0: i64 = fn_state.esize;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: cast reint s_3_1 -> i64
        let s_3_2: i64 = (s_3_1 as i64);
        // D s_3_3: read-var r:i64
        let s_3_3: i64 = fn_state.r;
        // D s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // D s_3_5: read-var slice_name:i
        let s_3_5: i128 = fn_state.slice_name;
        // D s_3_6: add s_3_5 s_3_4
        let s_3_6: i128 = (s_3_5 + s_3_4);
        // D s_3_7: read-var VLshadow#5573:i64
        let s_3_7: i64 = fn_state.VLshadow_5573;
        // D s_3_8: cast zx s_3_7 -> i
        let s_3_8: i128 = (i128::try_from(s_3_7).unwrap());
        // D s_3_9: cast reint s_3_8 -> i64
        let s_3_9: i64 = (s_3_8 as i64);
        // D s_3_10: read-var n:i64
        let s_3_10: i64 = fn_state.n;
        // D s_3_11: cast zx s_3_10 -> i
        let s_3_11: i128 = (i128::try_from(s_3_10).unwrap());
        // D s_3_12: cast zx s_3_2 -> i
        let s_3_12: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_13: cast zx s_3_9 -> i
        let s_3_13: i128 = (i128::try_from(s_3_9).unwrap());
        // D s_3_14: read-var vertical:u8
        let s_3_14: bool = fn_state.vertical;
        // D s_3_15: call ZAslice_read(s_3_11, s_3_12, s_3_14, s_3_6, s_3_13)
        let s_3_15: Bits = ZAslice_read(
            state,
            tracer,
            s_3_11,
            s_3_12,
            s_3_14,
            s_3_6,
            s_3_13,
        );
        // D s_3_16: read-var d:i64
        let s_3_16: i64 = fn_state.d;
        // D s_3_17: cast zx s_3_16 -> i
        let s_3_17: i128 = (i128::try_from(s_3_16).unwrap());
        // D s_3_18: read-var r:i64
        let s_3_18: i64 = fn_state.r;
        // D s_3_19: cast zx s_3_18 -> i
        let s_3_19: i128 = (i128::try_from(s_3_18).unwrap());
        // D s_3_20: add s_3_17 s_3_19
        let s_3_20: i128 = (s_3_17 + s_3_19);
        // D s_3_21: cast reint s_3_20 -> i64
        let s_3_21: i64 = (s_3_20 as i64);
        // D s_3_22: read-var VLshadow#5573:i64
        let s_3_22: i64 = fn_state.VLshadow_5573;
        // D s_3_23: cast zx s_3_22 -> i
        let s_3_23: i128 = (i128::try_from(s_3_22).unwrap());
        // D s_3_24: cast reint s_3_23 -> i64
        let s_3_24: i64 = (s_3_23 as i64);
        // D s_3_25: cast zx s_3_21 -> i
        let s_3_25: i128 = (i128::try_from(s_3_21).unwrap());
        // D s_3_26: cast zx s_3_24 -> i
        let s_3_26: i128 = (i128::try_from(s_3_24).unwrap());
        // D s_3_27: call Z_set(s_3_25, s_3_26, s_3_15)
        let s_3_27: () = Z_set(state, tracer, s_3_25, s_3_26, s_3_15);
        // D s_3_28: read-var r:i64
        let s_3_28: i64 = fn_state.r;
        // C s_3_29: const #1s : i64
        let s_3_29: i64 = 1;
        // D s_3_30: add s_3_28 s_3_29
        let s_3_30: i64 = (s_3_28 + s_3_29);
        // D s_3_31: write-var r <= s_3_30
        fn_state.r = s_3_30;
        // N s_3_32: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_4_0: return
        return;
    }
}
