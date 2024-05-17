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
pub fn execute_MOVA_MZ4_ZA_D1<T: Tracer>(
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
        gs_259052: bool,
        VLshadow_5587: i64,
        gs_259050: bool,
        VLshadow_5586: i64,
        gs_259060: i64,
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
        // D s_0_3: write-var VLshadow#5586 <= s_0_2
        fn_state.VLshadow_5586 = s_0_2;
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
        // D s_1_0: read-var VLshadow#5586:i64
        let s_1_0: i64 = fn_state.VLshadow_5586;
        // D s_1_1: write-var VLshadow#5587 <= s_1_0
        fn_state.VLshadow_5587 = s_1_0;
        // C s_1_2: const #4s : i
        let s_1_2: i128 = 4;
        // D s_1_3: read-var nreg:i64
        let s_1_3: i64 = fn_state.nreg;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: cmp-eq s_1_4 s_1_2
        let s_1_5: bool = ((s_1_4) == (s_1_2));
        // N s_1_6: branch s_1_5 b12 b2
        if s_1_5 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var gs#259050 <= s_2_0
        fn_state.gs_259050 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#259050:u8
        let s_3_0: bool = fn_state.gs_259050;
        // N s_3_1: branch s_3_0 b11 b4
        if s_3_0 {
            return block_11(state, tracer, fn_state);
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
        // D s_4_1: write-var gs#259052 <= s_4_0
        fn_state.gs_259052 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#259052:u8
        let s_5_0: bool = fn_state.gs_259052;
        // N s_5_1: branch s_5_0 b10 b6
        if s_5_0 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var VLshadow#5587:i64
        let s_6_0: i64 = fn_state.VLshadow_5587;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: read-var esize:i64
        let s_6_2: i64 = fn_state.esize;
        // D s_6_3: cast zx s_6_2 -> i
        let s_6_3: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_4: div s_6_1 s_6_3
        let s_6_4: i128 = ((s_6_1) / (s_6_3));
        // D s_6_5: cast reint s_6_4 -> i64
        let s_6_5: i64 = (s_6_4 as i64);
        // C s_6_6: const #32s : i64
        let s_6_6: i64 = 32;
        // D s_6_7: read-var s:i64
        let s_6_7: i64 = fn_state.s;
        // D s_6_8: cast zx s_6_7 -> i
        let s_6_8: i128 = (i128::try_from(s_6_7).unwrap());
        // D s_6_9: call X_read(s_6_8, s_6_6)
        let s_6_9: Bits = X_read(state, tracer, s_6_8, s_6_6);
        // D s_6_10: cast reint s_6_9 -> u32
        let s_6_10: u32 = (s_6_9.value() as u32);
        // D s_6_11: cast zx s_6_10 -> bv
        let s_6_11: Bits = Bits::new(s_6_10 as u128, 32u16);
        // D s_6_12: cast zx s_6_11 -> i
        let s_6_12: i128 = (s_6_11.value() as i128);
        // D s_6_13: cast reint s_6_12 -> i64
        let s_6_13: i64 = (s_6_12 as i64);
        // D s_6_14: cast zx s_6_10 -> bv
        let s_6_14: Bits = Bits::new(s_6_10 as u128, 32u16);
        // D s_6_15: cast zx s_6_14 -> i
        let s_6_15: i128 = (s_6_14.value() as i128);
        // D s_6_16: cast reint s_6_15 -> i64
        let s_6_16: i64 = (s_6_15 as i64);
        // D s_6_17: cast zx s_6_16 -> i
        let s_6_17: i128 = (i128::try_from(s_6_16).unwrap());
        // D s_6_18: read-var nreg:i64
        let s_6_18: i64 = fn_state.nreg;
        // D s_6_19: cast zx s_6_18 -> i
        let s_6_19: i128 = (i128::try_from(s_6_18).unwrap());
        // D s_6_20: mod s_6_17 s_6_19
        let s_6_20: i128 = ((s_6_17) % (s_6_19));
        // D s_6_21: cast reint s_6_20 -> i64
        let s_6_21: i64 = (s_6_20 as i64);
        // D s_6_22: cast zx s_6_13 -> i
        let s_6_22: i128 = (i128::try_from(s_6_13).unwrap());
        // D s_6_23: cast zx s_6_21 -> i
        let s_6_23: i128 = (i128::try_from(s_6_21).unwrap());
        // D s_6_24: sub s_6_22 s_6_23
        let s_6_24: i128 = ((s_6_22) - (s_6_23));
        // D s_6_25: cast reint s_6_24 -> i64
        let s_6_25: i64 = (s_6_24 as i64);
        // D s_6_26: cast zx s_6_25 -> i
        let s_6_26: i128 = (i128::try_from(s_6_25).unwrap());
        // D s_6_27: read-var offset:i64
        let s_6_27: i64 = fn_state.offset;
        // D s_6_28: cast zx s_6_27 -> i
        let s_6_28: i128 = (i128::try_from(s_6_27).unwrap());
        // D s_6_29: add s_6_26 s_6_28
        let s_6_29: i128 = (s_6_26 + s_6_28);
        // D s_6_30: cast reint s_6_29 -> i64
        let s_6_30: i64 = (s_6_29 as i64);
        // D s_6_31: cast zx s_6_30 -> i
        let s_6_31: i128 = (i128::try_from(s_6_30).unwrap());
        // D s_6_32: cast zx s_6_5 -> i
        let s_6_32: i128 = (i128::try_from(s_6_5).unwrap());
        // D s_6_33: call fmod_int(s_6_31, s_6_32)
        let s_6_33: i128 = fmod_int(state, tracer, s_6_31, s_6_32);
        // D s_6_34: write-var slice_name <= s_6_33
        fn_state.slice_name = s_6_33;
        // C s_6_35: const #0s : i64
        let s_6_35: i64 = 0;
        // C s_6_36: const #1s : i
        let s_6_36: i128 = 1;
        // D s_6_37: read-var nreg:i64
        let s_6_37: i64 = fn_state.nreg;
        // D s_6_38: cast zx s_6_37 -> i
        let s_6_38: i128 = (i128::try_from(s_6_37).unwrap());
        // D s_6_39: sub s_6_38 s_6_36
        let s_6_39: i128 = ((s_6_38) - (s_6_36));
        // D s_6_40: cast reint s_6_39 -> i64
        let s_6_40: i64 = (s_6_39 as i64);
        // D s_6_41: write-var gs#259060 <= s_6_40
        fn_state.gs_259060 = s_6_40;
        // D s_6_42: write-var r <= s_6_35
        fn_state.r = s_6_35;
        // N s_6_43: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var r:i64
        let s_7_0: i64 = fn_state.r;
        // D s_7_1: read-var gs#259060:i64
        let s_7_1: i64 = fn_state.gs_259060;
        // D s_7_2: cmp-gt s_7_0 s_7_1
        let s_7_2: bool = ((s_7_0) > (s_7_1));
        // N s_7_3: branch s_7_2 b9 b8
        if s_7_2 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var esize:i64
        let s_8_0: i64 = fn_state.esize;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: cast reint s_8_1 -> i64
        let s_8_2: i64 = (s_8_1 as i64);
        // D s_8_3: read-var r:i64
        let s_8_3: i64 = fn_state.r;
        // D s_8_4: cast zx s_8_3 -> i
        let s_8_4: i128 = (i128::try_from(s_8_3).unwrap());
        // D s_8_5: read-var slice_name:i
        let s_8_5: i128 = fn_state.slice_name;
        // D s_8_6: add s_8_5 s_8_4
        let s_8_6: i128 = (s_8_5 + s_8_4);
        // D s_8_7: read-var VLshadow#5587:i64
        let s_8_7: i64 = fn_state.VLshadow_5587;
        // D s_8_8: cast zx s_8_7 -> i
        let s_8_8: i128 = (i128::try_from(s_8_7).unwrap());
        // D s_8_9: cast reint s_8_8 -> i64
        let s_8_9: i64 = (s_8_8 as i64);
        // D s_8_10: read-var n:i64
        let s_8_10: i64 = fn_state.n;
        // D s_8_11: cast zx s_8_10 -> i
        let s_8_11: i128 = (i128::try_from(s_8_10).unwrap());
        // D s_8_12: cast zx s_8_2 -> i
        let s_8_12: i128 = (i128::try_from(s_8_2).unwrap());
        // D s_8_13: cast zx s_8_9 -> i
        let s_8_13: i128 = (i128::try_from(s_8_9).unwrap());
        // D s_8_14: read-var vertical:u8
        let s_8_14: bool = fn_state.vertical;
        // D s_8_15: call ZAslice_read(s_8_11, s_8_12, s_8_14, s_8_6, s_8_13)
        let s_8_15: Bits = ZAslice_read(
            state,
            tracer,
            s_8_11,
            s_8_12,
            s_8_14,
            s_8_6,
            s_8_13,
        );
        // D s_8_16: read-var d:i64
        let s_8_16: i64 = fn_state.d;
        // D s_8_17: cast zx s_8_16 -> i
        let s_8_17: i128 = (i128::try_from(s_8_16).unwrap());
        // D s_8_18: read-var r:i64
        let s_8_18: i64 = fn_state.r;
        // D s_8_19: cast zx s_8_18 -> i
        let s_8_19: i128 = (i128::try_from(s_8_18).unwrap());
        // D s_8_20: add s_8_17 s_8_19
        let s_8_20: i128 = (s_8_17 + s_8_19);
        // D s_8_21: cast reint s_8_20 -> i64
        let s_8_21: i64 = (s_8_20 as i64);
        // D s_8_22: read-var VLshadow#5587:i64
        let s_8_22: i64 = fn_state.VLshadow_5587;
        // D s_8_23: cast zx s_8_22 -> i
        let s_8_23: i128 = (i128::try_from(s_8_22).unwrap());
        // D s_8_24: cast reint s_8_23 -> i64
        let s_8_24: i64 = (s_8_23 as i64);
        // D s_8_25: cast zx s_8_21 -> i
        let s_8_25: i128 = (i128::try_from(s_8_21).unwrap());
        // D s_8_26: cast zx s_8_24 -> i
        let s_8_26: i128 = (i128::try_from(s_8_24).unwrap());
        // D s_8_27: call Z_set(s_8_25, s_8_26, s_8_15)
        let s_8_27: () = Z_set(state, tracer, s_8_25, s_8_26, s_8_15);
        // D s_8_28: read-var r:i64
        let s_8_28: i64 = fn_state.r;
        // C s_8_29: const #1s : i64
        let s_8_29: i64 = 1;
        // D s_8_30: add s_8_28 s_8_29
        let s_8_30: i64 = (s_8_28 + s_8_29);
        // D s_8_31: write-var r <= s_8_30
        fn_state.r = s_8_30;
        // N s_8_32: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_9_0: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_10_0: panic
        panic!("{:?}", ());
        // N s_10_1: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #128s : i
        let s_11_0: i128 = 128;
        // D s_11_1: read-var VLshadow#5587:i64
        let s_11_1: i64 = fn_state.VLshadow_5587;
        // D s_11_2: cast zx s_11_1 -> i
        let s_11_2: i128 = (i128::try_from(s_11_1).unwrap());
        // D s_11_3: cmp-eq s_11_2 s_11_0
        let s_11_3: bool = ((s_11_2) == (s_11_0));
        // D s_11_4: write-var gs#259052 <= s_11_3
        fn_state.gs_259052 = s_11_3;
        // N s_11_5: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #64s : i
        let s_12_0: i128 = 64;
        // D s_12_1: read-var esize:i64
        let s_12_1: i64 = fn_state.esize;
        // D s_12_2: cast zx s_12_1 -> i
        let s_12_2: i128 = (i128::try_from(s_12_1).unwrap());
        // D s_12_3: cmp-eq s_12_2 s_12_0
        let s_12_3: bool = ((s_12_2) == (s_12_0));
        // D s_12_4: write-var gs#259050 <= s_12_3
        fn_state.gs_259050 = s_12_3;
        // N s_12_5: jump b3
        return block_3(state, tracer, fn_state);
    }
}
